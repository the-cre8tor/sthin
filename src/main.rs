use sthin::configuration::*;
use sthin::infrastructure::telemetry::Telemetry;
use sthin::startup::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Configs::get().expect("Failed to read configuration");

    Telemetry::init_subscriber(&config.application.name, "info".into(), std::io::stdout);

    let server = Application::build(config).await?;
    server.run_until_stopped().await?;

    Ok(())
}
