use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize,sqlx::FromRow)]
pub struct SessionModel {
    pub id: i64,
    pub description: String,
    pub completed: i64,
    pub date_start: String,
    pub last_date: Option<String>,
    pub session_data: serde_json::Value,
}

impl From<web::Json<SessionModel>> for SessionModel {
    fn from(value: web::Json<SessionModel>) -> Self {
        value.0
    }
}

#[derive(Deserialize)]
pub struct NewSessionData {
    pub description: String,
    pub date_start: String,
    pub last_date: Option<String>,
    pub session_data: serde_json::Value,
}