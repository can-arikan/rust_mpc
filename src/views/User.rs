use crate::{models::User::{User, Wallet}, database::{UserRepository::UserRepository, SecretRepository::SecretRepository}, services::{WalletService::WalletService, SecretService}, views::SaveSecret::inner_save_secret};

use actix_web::{post, web::{Data}, HttpResponse, HttpRequest};

#[post("/create_user/{degree}/{holders_count}")]
pub async fn create_user(db: Data<UserRepository>, db2: Data<SecretRepository>, req: HttpRequest) -> HttpResponse {
    let eth_wallet = WalletService::createEthWallet();
    let btc_wallet = WalletService::createBitcoinWallet();

    let degree: u8 = req.match_info().get("degree").unwrap().parse().unwrap();
    let holders_count: u8 = req.match_info().get("holders_count").unwrap().parse().unwrap();
    assert!(!(holders_count < degree + 1), "Number of holders {} must be greater than or equal to the degree of the polynomial plus one: {} + 1 => {}", holders_count, degree, (degree+1));

    let data = User {
        id: None,
        wallets: vec![Wallet::new(eth_wallet[0].clone(), degree), Wallet::new(btc_wallet[0].clone(), degree)]
    };

    let user_detail = db.create_user(data.clone()).await;

    match user_detail {
        Ok(user_detail) => {
            let partitions = SecretService::SecretService::secretPartition(data.wallets[0].degree, eth_wallet[1].clone(), holders_count);
            inner_save_secret(db2, data.wallets[0].pub_key.as_str(), user_detail.as_object_id().unwrap().to_hex().as_str(), partitions, data.to_owned().wallets[0].degree).await;
            HttpResponse::Ok().json(user_detail)
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}