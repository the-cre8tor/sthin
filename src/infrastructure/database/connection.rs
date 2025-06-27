use crate::{configuration::DatabaseSettings, error::AppError};
use sqlx::postgres::{PgPool, PgPoolOptions};

pub struct DatabasePool {
    pub pool: PgPool,
}

impl DatabasePool {
    pub async fn new(config: &DatabaseSettings) -> Result<Self, AppError> {
        let pool = PgPoolOptions::new().connect_lazy(&config.connection_string())?;

        Ok(Self { pool })
    }
}
