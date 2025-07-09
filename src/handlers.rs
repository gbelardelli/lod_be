
pub mod game_session_handler;
pub mod game_character_handler;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(game_session_handler::config)
            .configure(game_character_handler::config)
    );
}
