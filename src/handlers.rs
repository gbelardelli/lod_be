
pub mod session_handler;
pub mod character_handler;
pub mod player_handler;

#[cfg(test)]
pub mod session_tests;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(session_handler::config)
            .configure(character_handler::config)
            .configure(player_handler::config)
    );
}
