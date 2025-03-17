use actix_web::dev::Server;
use actix_web::web::get;
use actix_web::{App, HttpServer};
use std::net::TcpListener;

use crate::configuration::Settings;
use crate::routes::health_check;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(config: Settings) -> Result<Application, anyhow::Error> {
        let address = format!("{}:{}", config.application.host, config.application.port);
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        let server = Self::run(listener).await?;

        Ok(Self { server, port })
    }

    async fn run(listener: TcpListener) -> Result<Server, anyhow::Error> {
        let server = HttpServer::new(move || App::new().route("/healthz", get().to(health_check)))
            .listen(listener)?
            .run();

        Ok(server)
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
