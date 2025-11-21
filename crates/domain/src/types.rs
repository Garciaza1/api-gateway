// crates/domain/src/types.rs

use serde::{Serialize, Deserialize};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserID(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub email: String,
    pub password: String, 
    pub last_login: DateTime<Utc>, 
    pub created_at: DateTime<Utc>, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String, 
}