use actix_web::{web, App, HttpServer};
use sqlx::SqlitePool;

mod models;
mod handlers;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = SqlitePool::connect("sqlite://lodapp.db").await.expect("Failed to connect to database");
    sqlx::migrate!("./migrations").run(&db_pool).await.expect("Failed to run migrations");    

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .configure(handlers::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

