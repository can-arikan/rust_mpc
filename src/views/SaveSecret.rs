
use std::str::FromStr;

use crate::{models::PartialSecret::PartialSecret, database::SecretRepository::SecretRepository};

use actix_web::{post, web::{Data}, HttpResponse, HttpRequest};
use bigdecimal::BigDecimal;
use mongodb::bson::oid::ObjectId;

#[post("/save/{user_id}/{public_key}/{partial_secret}/{degree}")]
pub async fn save_secret(db: Data<SecretRepository>, req: HttpRequest) -> HttpResponse {
    let pub_key = req.match_info().get("public_key").unwrap();
    let user_id = req.match_info().get("user_id").unwrap();
    let partial_secret = req.match_info().get("partial_secret").unwrap();
    let secret_degree: u8 = req.match_info().get("partial_secret").unwrap().parse().unwrap();
    let data = PartialSecret {
        id: None,
        user_id: ObjectId::from_str(user_id).unwrap(),
        partial_secret: partial_secret.to_owned(),
        public_key: pub_key.to_owned(),
        secret_degree: secret_degree
    };
    let partial_secret_detail = db.save_secret(data).await;
    match partial_secret_detail {
        Ok(partial_secret) => HttpResponse::Ok().json(partial_secret),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn inner_save_secret(db: Data<SecretRepository>, pub_key: &str, user_id: &str, partial_secret: Vec<Vec<BigDecimal>>, secret_degree: u8) -> HttpResponse {
    let user_id = ObjectId::from_str(user_id).unwrap();
    let mapped: Vec<PartialSecret> = partial_secret.iter()
        .map(|x| PartialSecret {
            id: None,
            user_id: user_id,
            partial_secret: (x[0].to_string() + "||" + x[1].to_string().as_str()).to_string(),
            public_key: pub_key.to_owned(),
            secret_degree: secret_degree
        })
        .collect();
    let partial_secret_detail = db.save_muliple_secret(mapped).await;
    match partial_secret_detail {
        Ok(partial_secret) => HttpResponse::Ok().json(partial_secret),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}