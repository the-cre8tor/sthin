use crate::{configuration::DatabaseSettings, error::AppError};
use sqlx::postgres::{PgPool, PgPoolOptions};

pub struct Database;

impl Database {
    pub async fn establish_connection(config: &DatabaseSettings) -> Result<PgPool, AppError> {
        PgPoolOptions::new()
            .connect_lazy(&config.connection_string())
            .map_err(|error| AppError::DatabaseConnectionError(error.to_string()))
    }
}
