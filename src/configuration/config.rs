use crate::configuration::Env;
use config::ConfigError;

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize, Debug)]
pub struct ApplicationSettings {
    pub name: String,
    pub port: u16,
    pub host: String,
}

pub struct Configs;

impl Configs {
    pub fn get() -> Result<Settings, ConfigError> {
        let base_path = std::env::current_dir().expect("Failed to determin the current directory");
        let config_dir = base_path.join("configs");

        let env: Env = std::env::var("APP_ENV")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .expect("Failed to parse APP_ENV");

        let env_filename = format!("{}.yaml", env.as_str());

        let settings = config::Config::builder()
            .add_source(config::File::from(config_dir.join("base.yaml")))
            .add_source(config::File::from(config_dir.join(env_filename)))
            .add_source(config::Environment::with_prefix("APP").separator("__"))
            .build()?;

        settings.try_deserialize::<Settings>()
    }
}
