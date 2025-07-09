use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;

use crate::{db::game_session_db, models::game_session_model::NewGameSession};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/sessions", web::post().to(create_session))
        .route("/sessions", web::get().to(get_all_sessions))
        .route("/sessions/{id}", web::get().to(get_session_by_id))
        .route("/sessions/{id}", web::put().to(update_session))
        .route("/sessions/{id}", web::delete().to(delete_session));
}

async fn create_session(db_pool: web::Data<SqlitePool>, new_session: web::Json<NewGameSession>,) -> impl Responder {
    match game_session_db::create_session(&db_pool, new_session.into_inner()).await {
        Ok(id) => HttpResponse::Created().json(id),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to create session")
        }
    }
}

async fn get_all_sessions() -> impl Responder { HttpResponse::Ok().body("get_all_sessions") }
async fn get_session_by_id() -> impl Responder { HttpResponse::Ok().body("get_session_by_id") }
async fn update_session() -> impl Responder { HttpResponse::Ok().body("update_session") }
async fn delete_session() -> impl Responder { HttpResponse::Ok().body("delete_session") }

