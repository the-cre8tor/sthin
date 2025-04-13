use axum::{Router, routing::get};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use crate::configuration::Settings;
use crate::features::url_stats::queue::StatsProcessor;
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

#[derive(Clone)]
pub struct QueueProcessor {
    pub stats_processor: StatsProcessor,
}

#[derive(Clone)]
pub struct AppState {
    pub processors: QueueProcessor,
    pub services: AppServices,
}

pub struct WebServer {
    _port: u16,
    // listener: TcpListener,
}

impl WebServer {
    pub async fn build(
        config: Settings,
        connection_pool: PgPool,
    ) -> Result<WebServer, anyhow::Error> {
        let address = format!("{}:{}", config.application.host, config.application.port);
        let listener = TcpListener::bind(address).await?;
        let port = listener.local_addr()?.port();

        let _ = Self::run(listener, connection_pool).await?;

        Ok(Self {
            _port: port,
            // listener,
        })
    }

    async fn run(listener: TcpListener, pool: PgPool) -> Result<(), anyhow::Error> {
        // Create repositories
        let url_repository = Arc::new(UrlRepository::new(pool.clone()));
        let url_stats_repository = Arc::new(UrlStatsRepository::new(pool.clone()));

        // Create services
        let url_service = Arc::new(UrlService::new(url_repository));
        let url_stats_service = Arc::new(UrlStatsService::new(url_stats_repository));

        // Task Queues
        let stats_processor = StatsProcessor::new(100, url_stats_service.clone());

        // App State
        let services = AppServices {
            url_service,
            url_stats_service,
        };

        // App Queue
        let processors = QueueProcessor { stats_processor };

        let state = Arc::new(AppState {
            processors,
            services,
        });

        let app = Router::new()
            .route("/healthz", get(health_check))
            .merge(Routes::configure_routes())
            .with_state(state)
            .layer(TraceLayer::new_for_http());

        axum::serve(listener, app).await?;

        Ok(())
    }
}
