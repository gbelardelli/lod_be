use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;

use crate::{db::character_db, models::character_model::NewCharacter};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/characters", web::post().to(create_character))
        .route("/characters", web::get().to(get_all_characters))
        .route("/characters/{id}", web::get().to(get_character_by_id))
        .route("/characters/{id}", web::put().to(update_character))
        .route("/characters/{id}", web::delete().to(delete_character));
}

async fn create_character(db_pool: web::Data<SqlitePool>, new_character: web::Json<NewCharacter>,) -> impl Responder {
    match character_db::create_character(&db_pool, new_character.into_inner()).await {
        Ok(id) => HttpResponse::Created().json(id),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to create character")
        }
    }
}

async fn get_all_characters() -> impl Responder { HttpResponse::Ok().body("get_all_characters") }
async fn get_character_by_id() -> impl Responder { HttpResponse::Ok().body("get_character_by_id") }
async fn update_character() -> impl Responder { HttpResponse::Ok().body("update_character") }
async fn delete_character() -> impl Responder { HttpResponse::Ok().body("delete_character") }