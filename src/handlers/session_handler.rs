use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;

use crate::{db::session_db, error::AppError, models::session_model::{NewSessionData, SessionModel}};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/sessions", web::post().to(create_session))
        .route("/sessions", web::get().to(get_all_sessions))
        .route("/sessions/{id}", web::get().to(get_session_by_id))
        .route("/sessions/{id}", web::put().to(update_session))
        .route("/sessions/{id}", web::delete().to(delete_session));
}

async fn create_session(db_pool: web::Data<SqlitePool>, new_session: web::Json<NewSessionData>,) -> Result<HttpResponse, AppError> {
    match session_db::create_session(&db_pool, new_session.into_inner()).await {
        Ok(id) => Ok(HttpResponse::Ok().json(id)),
        Err(e) => {
            eprintln!("DB error: {}", e);
            Err(AppError::DbError(e))
        }
    }
}

async fn get_all_sessions(db_pool: web::Data<SqlitePool>,) -> Result<HttpResponse, AppError> { 
    let sessions = session_db::get_all_sessions(db_pool.get_ref()).await
        .map_err(|e| AppError::DbError(e))?;

    Ok(HttpResponse::Ok().json(sessions))
}

async fn get_session_by_id(db_pool: web::Data<SqlitePool>, path: web::Path<i64>, ) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    match session_db::get_session_by_id(&db_pool, id).await {
        Ok(Some(session)) => Ok(HttpResponse::Ok().json(session)),
        Ok(None) => Err(AppError::NotFound(format!("Session with id {} not found", id))),
        Err(e) => Err(AppError::DbError(e)),
    }
}

async fn update_session(db_pool: web::Data<SqlitePool>, path: web::Path<i64>, session: web::Json<SessionModel>,) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    match session_db::update_session(&db_pool, id, session.into_inner()).await {
        Ok(()) => Ok(HttpResponse::Ok().json(id)),
        Err(e) => Err(AppError::DbError(e)),
    }
}

async fn delete_session(db_pool: web::Data<SqlitePool>, path: web::Path<i64>, ) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    match session_db::delete_session(&db_pool, id).await {
        Ok(()) => Ok(HttpResponse::Ok().json(id)),
        Err(e) => Err(AppError::DbError(e)),
    }
}

