use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web, App, HttpServer};
use sqlx::SqlitePool;

mod models;
mod handlers;
mod db;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = SqlitePool::connect("sqlite://lodapp.db").await.expect("Failed to connect to database");
    //sqlx::migrate!("./migrations").run(&db_pool).await.expect("Failed to run migrations");    

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                Key::generate(), // in produzione usa una chiave salvata in modo sicuro!
            ))
            .wrap(IdentityMiddleware::default())            
            .configure(handlers::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

