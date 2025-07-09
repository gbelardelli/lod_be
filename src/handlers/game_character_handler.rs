use actix_web::{web, HttpResponse, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/characters", web::post().to(create_character))
        .route("/characters", web::get().to(get_all_characters))
        .route("/characters/{id}", web::get().to(get_character_by_id))
        .route("/characters/{id}", web::put().to(update_character))
        .route("/characters/{id}", web::delete().to(delete_character));
}

async fn create_character() -> impl Responder { HttpResponse::Ok().body("create_character") }
async fn get_all_characters() -> impl Responder { HttpResponse::Ok().body("get_all_characters") }
async fn get_character_by_id() -> impl Responder { HttpResponse::Ok().body("get_character_by_id") }
async fn update_character() -> impl Responder { HttpResponse::Ok().body("update_character") }
async fn delete_character() -> impl Responder { HttpResponse::Ok().body("delete_character") }