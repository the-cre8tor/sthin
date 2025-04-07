use actix_web::dev::Server;
use actix_web::web::{Data, get};
use actix_web::{App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use std::sync::Arc;
use tracing_actix_web::TracingLogger;

use crate::configuration::Settings;
use crate::features::url_stats::repository::UrlStatsRepository;
use crate::features::url_stats::service::UrlStatsService;
use crate::features::urls::handlers::health_check;
use crate::features::urls::repository::UrlRepository;
use crate::features::urls::routes::Routes;
use crate::features::urls::service::UrlService;

#[derive(Clone)]
pub struct AppServices {
    pub url_service: Arc<UrlService<UrlRepository>>,
    pub url_stats_service: Arc<UrlStatsService<UrlStatsRepository>>,
}

pub struct WebServer {
    _port: u16,
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

        Ok(Self {
            _port: port,
            server,
        })
    }

    async fn run(listener: TcpListener, pool: PgPool) -> Result<Server, anyhow::Error> {
        // Create repositories
        let url_repository = Arc::new(UrlRepository::new(pool.clone()));
        let url_stats_repository = Arc::new(UrlStatsRepository::new(pool.clone()));

        // Create services
        let url_service = Arc::new(UrlService::new(url_repository));
        let url_stats_service = Arc::new(UrlStatsService::new(url_stats_repository));

        // App State
        let services = AppServices {
            url_service,
            url_stats_service,
        };

        let server = HttpServer::new(move || {
            App::new()
                .wrap(TracingLogger::default())
                .route("healthz", get().to(health_check))
                .configure(Routes::configure_routes)
                .app_data(Data::new(services.clone()))
        })
        .listen(listener)?
        .run();

        Ok(server)
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
