use actix_web::web::{ServiceConfig, post, scope};

use super::handlers::UrlHandler;

pub struct Routes;

impl Routes {
    pub fn configure_routes(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/api")
                .service(scope("/shorten").route("", post().to(UrlHandler::create_short_url))),
        );
    }
}
