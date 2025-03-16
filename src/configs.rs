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

        let env: Env = std::env::var("APPLICATION_ENV")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .expect("Failed to parse APPLICATION_ENV");

        let env_filename = format!("{}.yaml", env.as_str());

        let settings = config::Config::builder()
            .add_source(config::File::from(config_dir.join("base.yaml")))
            .add_source(config::File::from(config_dir.join(env_filename)))
            .add_source(config::Environment::with_prefix("APP").separator("__"))
            .build()?;

        settings.try_deserialize::<Settings>()
    }
}

pub enum Env {
    Local,
    Production,
}

impl Env {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Production => "production",
        }
    }
}

impl TryFrom<String> for Env {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either 'local' or 'production'",
                other
            )),
        }
    }
}
