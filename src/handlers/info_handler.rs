use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;

use crate::{db::info_db, error::AppError};


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/info", web::get().to(get_all_info));
}

async fn get_all_info(db_pool: web::Data<SqlitePool>, id: Option<Identity>) -> Result<HttpResponse, AppError> {
    if let Some(_id) = id {
        let infos = info_db::get_all_info(db_pool.get_ref()).await
            .map_err(|e| AppError::DbError(e))?;

        return Ok(HttpResponse::Ok().json(infos));
    }
    Err(AppError::Unauthorized("You must be logged on!".to_owned()))
}