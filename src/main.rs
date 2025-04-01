use anyhow::Result;
use sthin::configuration::*;
use sthin::infrastructure::database::connection::DatabasePool;
use sthin::infrastructure::server::WebServer;
use sthin::infrastructure::telemetry::Telemetry;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Configs::get().expect("Failed to read configuration");

    Telemetry::init_subscriber(&config.application.name, "info".into(), std::io::stdout);

    let pool = DatabasePool::new(&config.database).await?;

    let server = WebServer::build(config, pool).await?;
    server.run_until_stopped().await?;

    Ok(())
}
