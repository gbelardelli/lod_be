use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize,sqlx::FromRow)]
pub struct GameSession {
    pub id: u32,
    pub description: String,
    pub completed: String,
    pub date_start: String,
    pub last_date: Option<String>,
    pub session_data: serde_json::Value,
}

impl From<web::Json<GameSession>> for GameSession {
    fn from(value: web::Json<GameSession>) -> Self {
        value.0
    }
}

#[derive(Deserialize)]
pub struct NewGameSession {
    pub description: String,
    pub date_start: String,
    pub last_date: Option<String>,
    pub session_data: serde_json::Value,
}