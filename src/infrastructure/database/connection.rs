use std::fmt::Error;

use crate::configuration::{Settings};
use sqlx::postgres::{PgPool, PgPoolOptions};

pub struct Database;

impl Database {
    pub async fn establish_connection(config: &Settings) -> Result<PgPool, Error> {
        let pool = PgPoolOptions::new().max_connections(config.);
    }
}
