use std::time::Duration;

use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::{Key}, middleware::Logger, web, App, HttpServer};
use sqlx::SqlitePool;

mod models;
mod handlers;
mod db;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let db_pool = SqlitePool::connect("sqlite://lodapp.db").await.expect("Failed to connect to database");
    //sqlx::migrate!("./migrations").run(&db_pool).await.expect("Failed to run migrations");    
    let secret_key = Key::generate();
    let expiration = Duration::from_secs(24 * 60 * 60);
    
    HttpServer::new(move || {
        let cors = Cors::permissive();
        let session_mw =
            SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                // disable secure cookie for local testing
                .cookie_secure(false)
                .cookie_name("lodapp".to_string())
                // Set a ttl for the cookie if the identity should live longer than the user session
                .session_lifecycle(
                    PersistentSession::default().session_ttl(expiration.try_into().unwrap()),
                )
                .build();
        let identity_mw = IdentityMiddleware::builder()
                .visit_deadline(Some(expiration))
                .build();            

        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(identity_mw)
            .wrap(session_mw)
            .wrap(Logger::default())
            .wrap(cors)
            .configure(handlers::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}

