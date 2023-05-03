extern crate dotenv;

use crate::models::{User::User};

use std::env;
use dotenv::dotenv;
use mongodb::{bson::{extjson::de::Error, doc, Bson}, Client, Collection};

pub struct UserRepository {
    col: Collection<User>,
}

impl UserRepository {
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
        let col: Collection<User> = db.collection("User");
        UserRepository { col }
    }

    pub async fn create_user(&self, new_user: User) -> Result<Bson, Error> {
        let exist = self
            .col
            .find(new_user.to_document(), None)
            .await
            .ok();

        if !exist.is_none() {
            let mut exist = exist.unwrap();
            let tmp = exist.advance().await;
            if tmp.is_ok() {
                if tmp.unwrap() == true {
                    let cursor = exist;
                    let user = cursor.deserialize_current();
                    if user.is_ok() {
                        let user = user.ok().unwrap();
                        let x = doc!{ "insertedId": user.id.unwrap().to_string() };
                        return Ok(x.into())
                    }
                }
            }
        }
            
        let user = self
            .col
            .insert_one(new_user, None)
            .await
            .ok()
            .expect("Error creating new user");
        Ok(user.inserted_id)
    }
}