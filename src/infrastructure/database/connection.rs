use crate::{configuration::DatabaseSettings, error::AppError};
use sqlx::{
    Postgres, Transaction,
    postgres::{PgPool, PgPoolOptions},
};

pub struct DatabasePool {
    pub pool: PgPool,
}

impl DatabasePool {
    pub async fn new(config: &DatabaseSettings) -> Result<Self, AppError> {
        let pool = PgPoolOptions::new().connect_lazy(&config.connection_string())?;

        Ok(Self { pool })
    }

    pub async fn transaction<F, Fut, R, E>(&self, callback: F) -> Result<R, E>
    where
        F: FnOnce(&mut Transaction<'_, Postgres>) -> Fut,
        Fut: Future<Output = Result<R, E>>,
        E: From<sqlx::Error>,
    {
        let mut tx = self.pool.begin().await?;

        match callback(&mut tx).await {
            Ok(result) => {
                tx.commit().await?;
                Ok(result)
            }
            Err(e) => {
                tx.rollback().await?;
                Err(e)
            }
        }
    }
}
