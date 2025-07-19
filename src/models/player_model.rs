use actix_web::web;
use serde::{Deserialize, Serialize};

pub enum Roles {
    ADMIN,
    SESSIONMGR,
    PLAYER
}

#[derive(Debug, Deserialize, Serialize,sqlx::FromRow)]
pub struct PlayerModel {
    pub id: i64,
    pub name: String,
    pub color: i64,
    pub roles: i64,
    pub password: String,
}

impl From<web::Json<PlayerModel>> for PlayerModel {
    fn from(value: web::Json<PlayerModel>) -> Self {
        value.0
    }
}

#[derive(Deserialize,Serialize,Clone,sqlx::FromRow)]
pub struct PlayerData {
    pub id: i64,
    pub name: String,
    pub color: i64,
    pub roles: i64,
}
