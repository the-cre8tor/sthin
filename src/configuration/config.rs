use crate::configuration::Env;
use config::ConfigError;
use redact::Secret;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub name: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

pub struct Configs;

impl Configs {
    pub fn get() -> Result<Settings, ConfigError> {
        let base_path = std::env::current_dir().expect("Failed to determine the current directory");
        let config_dir = base_path.join("configs");

        let env: Env = std::env::var("APP_ENV")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .expect("Failed to parse APP_ENV");

        let env_filename = format!("{}.yaml", env.as_str());

        let settings = config::Config::builder()
            .add_source(config::File::from(config_dir.join("base.yaml")))
            .add_source(config::File::from(config_dir.join(env_filename)))
            .add_source(config::Environment::default().separator("__"))
            .build()?;

        settings.try_deserialize::<Settings>()
    }
}

impl DatabaseSettings {
    pub fn connect_options(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
            .database(&self.database_name)
    }

    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}?sslmode={}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name,
            if self.require_ssl {
                "require"
            } else {
                "prefer"
            }
        )
    }
}
