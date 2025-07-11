use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;

use crate::{db::player_db, error::AppError, models::player_model::{PlayerData, PlayerModel}};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/players", web::post().to(create_player))
        .route("/players", web::get().to(get_all_players))
        .route("/players/{id}", web::get().to(get_player_by_id))
        .route("/players/{id}", web::put().to(update_player))
        .route("/players/pwdroles/{id}", web::put().to(update_player_pwd_roles))
        .route("/players/{id}", web::delete().to(delete_player));
}

async fn create_player(db_pool: web::Data<SqlitePool>, new_player: web::Json<PlayerModel>,) -> Result<HttpResponse, AppError> {
    match player_db::create_player(&db_pool, new_player.into_inner()).await {
        Ok(id) => Ok(HttpResponse::Ok().json(id)),
        Err(e) => {
            eprintln!("DB error: {}", e);
            Err(AppError::DbError(e))
        }
    }
}

async fn get_all_players(db_pool: web::Data<SqlitePool>,) -> Result<HttpResponse, AppError> {
    let players = player_db::get_all_players(db_pool.get_ref()).await
        .map_err(|e| AppError::DbError(e))?;

    Ok(HttpResponse::Ok().json(players))
}

async fn get_player_by_id(db_pool: web::Data<SqlitePool>, path: web::Path<i64>,) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    match player_db::get_player_by_id(&db_pool, id).await {
        Ok(Some(session)) => Ok(HttpResponse::Ok().json(session)),
        Ok(None) => Err(AppError::NotFound(format!("Player with id {} not found", id))),
        Err(e) => Err(AppError::DbError(e)),
    }
}

async fn update_player(db_pool: web::Data<SqlitePool>, path: web::Path<i64>, player: web::Json<PlayerData>,) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    match player_db::update_player(&db_pool, id, player.into_inner()).await {
        Ok(()) => Ok(HttpResponse::Ok().json(id)),
        Err(e) => Err(AppError::DbError(e)),
    }
}

async fn update_player_pwd_roles(db_pool: web::Data<SqlitePool>, path: web::Path<i64>, player: web::Json<PlayerModel>,) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    match player_db::update_player_password(&db_pool, id, player.into_inner()).await {
        Ok(()) => Ok(HttpResponse::Ok().json(id)),
        Err(e) => Err(AppError::DbError(e)),
    }
}

async fn delete_player(db_pool: web::Data<SqlitePool>, path: web::Path<i64>,) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    match player_db::delete_player(&db_pool, id).await {
        Ok(()) => Ok(HttpResponse::Ok().json(id)),
        Err(e) => Err(AppError::DbError(e)),
    }
}
