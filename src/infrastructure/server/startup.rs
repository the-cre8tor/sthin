use actix_web::dev::Server;
use actix_web::web::{Data, get};
use actix_web::{App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::configuration::Settings;
use crate::domain::services::UrlService;
use crate::infrastructure::database::repositories::url_repository::UrlRepository;
use crate::interfaces::http::Routes;
use crate::interfaces::http::handlers::health_check;

pub struct WebServer {
    port: u16,
    server: Server,
}

impl WebServer {
    pub async fn build(
        config: Settings,
        connection_pool: PgPool,
    ) -> Result<WebServer, anyhow::Error> {
        let address = format!("{}:{}", config.application.host, config.application.port);
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        let server = Self::run(listener, connection_pool).await?;

        Ok(Self { server, port })
    }

    async fn run(listener: TcpListener, pool: PgPool) -> Result<Server, anyhow::Error> {
        let url_repository = UrlRepository::new(pool);
        let url_service = Data::new(UrlService::new(url_repository));

        let server = HttpServer::new(move || {
            App::new()
                .wrap(TracingLogger::default())
                .route("healthz", get().to(health_check))
                .configure(Routes::configure_routes)
                .app_data(url_service.clone())
        })
        .listen(listener)?
        .run();

        Ok(server)
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
