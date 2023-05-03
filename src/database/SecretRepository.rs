extern crate dotenv;

use crate::models::PartialSecret::PartialSecret;

use std::env;
use dotenv::dotenv;
use mongodb::{bson::{extjson::de::Error, Bson}, results::{InsertOneResult}, Client, Collection};

pub struct SecretRepository {
    col: Collection<PartialSecret>,
}

impl SecretRepository {
    fn get_connection_string() -> String {
        let host = env::var("MONGO_HOST").expect("MONGO_HOST env not set.");
        let port = env::var("MONGO_PORT").expect("MONGO_PORT env not set."); 
        let user = env::var("MONGO_USER").expect("MONGO_USER env not set.");
        let pass = env::var("MONGO_PASS").expect("MONGO_PASS env not set.");
        "mongodb://".to_owned() + &user + ":" + &pass + "@" + &host + ":" + &port
    }

    pub async fn init() -> Self {
        dotenv().ok();
        let uri = Self::get_connection_string();
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("RustDB");
        let col: Collection<PartialSecret> = db.collection("PartialSecrets");
        SecretRepository { col }
    }

    pub async fn save_muliple_secret(&self, new_secrets: Vec<PartialSecret>) -> Result<Vec<Bson>, Error> {
        let insertions = self
            .col
            .insert_many(new_secrets, None)
            .await
            .ok()
            .expect("Error saving partial secrets");
        let x: Vec<Bson> = insertions.inserted_ids.into_iter().map(|(_x, y)| y).collect();
        Ok(x)
    }

    pub async fn save_secret(&self, new_secret: PartialSecret) -> Result<InsertOneResult, Error> {
        let user = self
            .col
            .insert_one(new_secret, None)
            .await
            .ok()
            .expect("Error saving partial secret");
        Ok(user)
    }
}