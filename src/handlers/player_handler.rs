use actix_identity::IdentityExt;
use actix_identity::Identity;
use actix_session::SessionExt;
use actix_web::HttpMessage;
use actix_web::HttpRequest;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::{db::player_db, error::AppError, models::player_model::{PlayerData, PlayerModel}};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/players", web::post().to(create_player))
        .route("/players", web::get().to(get_all_players))
        .route("/players/login", web::post().to(player_login))
        .route("/players/{id}", web::get().to(get_player_by_id))
        .route("/players/{id}", web::put().to(update_player))
        .route("/players/pwdroles/{id}", web::put().to(update_player_pwd_roles))
        .route("/players/{id}", web::delete().to(delete_player));
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub async fn player_login( req: HttpRequest, form: web::Json<LoginRequest>, db_pool: web::Data<SqlitePool>, id: Option<Identity>, ) -> Result<HttpResponse, AppError> {
    if let Some(_id) = id {
        println!("session: {:?}",req.get_session().entries());
        let player=req.get_session().get::<PlayerData>("player");
        if player.is_ok() {
            return Ok(HttpResponse::Ok().json(player.unwrap()));
        }else{
            return Err(AppError::Unauthorized("Invalid credentials".to_owned()));
        }
    }
    match player_db::get_player_login(&db_pool, &form.username).await {
        Ok(player) => {
            if player.as_ref().is_some_and(|pl| pl.password == form.password) {
                let player=player.unwrap();
                let player_id=player.id.to_string();

                match Identity::login(&req.extensions(), player_id) {
                    Ok(_) => println!("login ok"),
                    Err(ko) => println!("login ko {:?}",ko),
                };
                let resp: PlayerData = PlayerData {
                    id: player.id,
                    name: player.name.clone(),
                    color: player.color,
                    roles: player.roles,
                };
                let _ = req.get_session().insert("player", resp.clone());

                Ok(HttpResponse::Ok().json(resp))
            }else{
                Err(AppError::Unauthorized("Invalid credentials".to_owned()))
            }
        },
        Err(_) => {
            Err(AppError::Unauthorized("Invalid credentials".to_owned()))
        }
    }
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

async fn get_all_players(db_pool: web::Data<SqlitePool>, id: Option<Identity>) -> Result<HttpResponse, AppError> {
    if let Some(_id) = id {
        let players = player_db::get_all_players(db_pool.get_ref()).await
            .map_err(|e| AppError::DbError(e))?;

        return Ok(HttpResponse::Ok().json(players));
    }
    Err(AppError::Unauthorized("You must be logged on!".to_owned()))
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
