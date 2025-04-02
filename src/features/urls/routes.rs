use actix_web::web::{ServiceConfig, get, post, scope};

use super::handlers::UrlHandler;

pub struct Routes;

impl Routes {
    pub fn configure_routes(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/api").service(
                scope("/shorten")
                    .route("", post().to(UrlHandler::create_short_url))
                    .route("/{code}", get().to(UrlHandler::retreive_url_by_short_code)),
            ),
        );
    }
}
