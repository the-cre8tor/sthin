use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::{
        errors::DomainError,
        models::Url,
        repositories::UrlRepository,
        value_objects::{ShortCode, ValidUrl},
    },
    infrastructure::database::models::db_url::DbUrl,
};

pub struct PgUrlRepository {
    pool: PgPool,
}

impl PgUrlRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UrlRepository for PgUrlRepository {
    async fn save(&self, url: &Url) -> Result<Url, DomainError> {
        let db_url = DbUrl::from_domain(url);

        let saved_url = sqlx::query_as!(
            DbUrl,
            r#"
            INSERT INTO urls (original_url, short_code, created_at, updated_at)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (short_code) DO UPDATE
            SET original_url = EXCLUDED.original_url,
                updated_at = EXCLUDED.updated_at
            RETURNING *
            "#,
            db_url.original_url,
            db_url.short_code,
            db_url.created_at,
            db_url.updated_at
        )
        .fetch_one(&self.pool)
        .await
        .map_err(DomainError::UnexpectedError)?;

        saved_url.to_domain()
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Uuid>, DomainError> {
        let result = sqlx::query!("SELECT id FROM urls WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(DomainError::UnexpectedError)?;

        Ok(result.map(|row| row.id))
    }

    async fn find_by_short_code(&self, short_code: &ShortCode) -> Result<Option<Url>, DomainError> {
        let result = sqlx::query_as!(
            DbUrl,
            "SELECT * FROM urls WHERE short_code = $1",
            short_code.as_ref()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(DomainError::UnexpectedError)?;

        result.map(|db_url| db_url.to_domain()).transpose()
    }

    async fn find_by_original_url(
        &self,
        original_url: &ValidUrl,
    ) -> Result<Option<Url>, DomainError> {
        let result = sqlx::query_as!(
            DbUrl,
            "SELECT * FROM urls WHERE original_url = $1",
            original_url.as_ref()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(DomainError::UnexpectedError)?;

        result.map(|db_url| db_url.to_domain()).transpose()
    }

    async fn delete_by_short_code(&self, short_code: &ShortCode) -> Result<bool, DomainError> {
        let result = sqlx::query!(
            "DELETE FROM urls WHERE short_code = $1",
            short_code.as_str()
        )
        .execute(&self.pool)
        .await
        .map_err(DomainError::UnexpectedError)?;

        Ok(result.rows_affected() > 0)
    }

    async fn exists_by_short_code(&self, short_code: &ShortCode) -> Result<bool, DomainError> {
        let result = self.find_by_short_code(short_code).await?;

        Ok(result.is_none())
    }

    // async fn update(&self, url: &Url) -> Result<Url, DomainError> {
    //     todo!()
    // }
}
