use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;

use crate::{db::character_db, error::AppError, models::character_model::{CharacterModel, NewCharacter}};

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

async fn get_all_characters(db_pool: web::Data<SqlitePool>,) -> Result<HttpResponse, AppError> {
    let characters = character_db::get_all_characters(db_pool.get_ref()).await
        .map_err(|e| AppError::DbError(e))?;

    Ok(HttpResponse::Ok().json(characters))
}

async fn get_character_by_id(db_pool: web::Data<SqlitePool>, path: web::Path<i64>,) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    match character_db::get_character_by_id(&db_pool, id).await {
        Ok(Some(session)) => Ok(HttpResponse::Ok().json(session)),
        Ok(None) => Err(AppError::NotFound(format!("Character with id {} not found", id))),
        Err(e) => Err(AppError::DbError(e)),
    }
}

async fn update_character(db_pool: web::Data<SqlitePool>, path: web::Path<i64>, character: web::Json<CharacterModel>,) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    match character_db::update_character(&db_pool, id, character.into_inner()).await {
        Ok(()) => Ok(HttpResponse::Ok().json(id)),
        Err(e) => Err(AppError::DbError(e)),
    }
}

async fn delete_character(db_pool: web::Data<SqlitePool>, path: web::Path<i64>,) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    match character_db::delete_character(&db_pool, id).await {
        Ok(()) => Ok(HttpResponse::Ok().json(id)),
        Err(e) => Err(AppError::DbError(e)),
    }
}
