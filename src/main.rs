use sthin::{configuration::*, startup::Application};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Configs::get().expect("Failed to read configuration");

    let server = Application::build(config).await?;
    server.run_until_stopped().await?;

    Ok(())
}
