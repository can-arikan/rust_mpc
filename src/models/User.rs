#![allow(dead_code)]
use mongodb::bson::{oid::ObjectId, Document, to_bson};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Wallet {
    pub pub_key: String,
    pub degree: u8
}

impl Wallet {
    pub fn new(pub_key: String, degree: u8) -> Self {
        Self { pub_key, degree }
    }

    pub fn copy(&self) -> Wallet {
        Wallet { pub_key: self.pub_key.clone(), degree: self.degree.clone() }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub wallets: Vec<Wallet>
}

impl User {
    pub fn to_document(&self) -> Document {
        to_bson(self).unwrap().as_document().unwrap().clone()
    }

    pub fn copy(&self) -> User {
        User { id: self.id.clone(), wallets: self.wallets.iter().map(|x| x.copy()).collect() }
    }
}