#![allow(non_snake_case)]
mod views;
mod models;
mod database;
mod services;
mod util;

use actix_web::{HttpServer, App, middleware::Logger, web::{self, Data}};
use std::env;
use dotenv::dotenv;
use views::Default::not_found;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // LOAD ENVIRONMENT
    dotenv().ok();

    let host: String = env::var("HOST").expect("HOST IS NOT IN ENV");
    let port: u16 = env::var("PORT").expect("PORT IS NOT IN ENV").parse().expect("PORT IS NOT IN CORRECT FORMAT");

    // INITIALIZE LOGGER
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // INITIALIZE DB

    // Secret Repository
    let secret_repository = database::SecretRepository::SecretRepository::init().await;
    let secret_data = Data::new(secret_repository);

    // User Repositoty
    let user_repository = database::UserRepository::UserRepository::init().await;
    let user_data = Data::new(user_repository);

    // INITIALIZE SERVICES
    let wallet_service = services::WalletService::WalletService;
    let wallet_service_data = Data::new(wallet_service);
    
    // START SERVER
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(wallet_service_data.clone())
            .app_data(secret_data.clone())
            .app_data(user_data.clone())
            .service(views::SaveSecret::save_secret)
            .service(views::User::create_user)
            .default_service(web::to(|| not_found()))
    })
        .bind((host, port))?
        .run()
        .await
}