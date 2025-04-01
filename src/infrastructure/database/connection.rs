use crate::{configuration::DatabaseSettings, error::AppError};
use sqlx::postgres::{PgPool, PgPoolOptions};

pub struct DatabasePool;

impl DatabasePool {
    pub async fn new(config: &DatabaseSettings) -> Result<PgPool, AppError> {
        PgPoolOptions::new()
            .connect_lazy(&config.connection_string())
            .map_err(|error| AppError::DatabaseConnectionError(error.to_string()))
    }

    // async fn _transaction<F, Fut, R, E>(&self, callback: F) -> Result<R, E>
    // where
    //     F: FnOnce(&mut Transaction<'_, Postgres>) -> Fut,
    //     Fut: Future<Output = Result<R, E>>,
    //     E: From<sqlx::Error>,
    // {
    //     let mut tx = self.pool.begin().await?;

    //     match callback(&mut tx).await {
    //         Ok(result) => {
    //             tx.commit().await?;
    //             Ok(result)
    //         }
    //         Err(e) => {
    //             tx.rollback().await?;
    //             Err(e)
    //         }
    //     }
    // }
}
