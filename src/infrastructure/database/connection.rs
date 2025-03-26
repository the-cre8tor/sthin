use crate::configuration::DatabaseSettings;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub struct Database;

impl Database {
    pub async fn establish_connection(config: &DatabaseSettings) -> Result<PgPool, sqlx::Error> {
        PgPoolOptions::new().connect_lazy(&config.connection_string())
    }
}
