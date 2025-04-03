use sqlx::PgPool;
use uuid::Uuid;

use crate::features::urls::{
    entities::UrlEntity,
    errors::UrlError,
    models::Url,
    value_objects::{ShortCode, ValidUrl},
};

use super::url_repository_trait::IUrlRepository;

pub struct UrlRepository {
    pool: PgPool,
}

impl UrlRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl IUrlRepository for UrlRepository {
    async fn save(&self, url: &Url) -> Result<Url, UrlError> {
        let db_url = UrlEntity::from_domain(url);

        let saved_url = sqlx::query_as!(
            UrlEntity,
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
        .await?;

        saved_url.to_domain()
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Uuid>, UrlError> {
        let result = sqlx::query!("SELECT id FROM urls WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.map(|row| row.id))
    }

    async fn find_by_short_code(&self, short_code: &ShortCode) -> Result<Option<Url>, UrlError> {
        let result = sqlx::query_as!(
            UrlEntity,
            "SELECT * FROM urls WHERE short_code = $1",
            short_code.as_str()
        )
        .fetch_optional(&self.pool)
        .await?;

        result.map(|db_url| db_url.to_domain()).transpose()
    }

    async fn find_by_original_url(&self, original_url: &ValidUrl) -> Result<Option<Url>, UrlError> {
        let result = sqlx::query_as!(
            UrlEntity,
            "SELECT * FROM urls WHERE original_url = $1",
            original_url.as_ref()
        )
        .fetch_optional(&self.pool)
        .await?;

        result.map(|db_url| db_url.to_domain()).transpose()
    }

    async fn delete_by_short_code(&self, short_code: &ShortCode) -> Result<bool, UrlError> {
        let result = sqlx::query!(
            "DELETE FROM urls WHERE short_code = $1",
            short_code.as_str()
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn exists_by_short_code(&self, short_code: &ShortCode) -> Result<bool, UrlError> {
        let result = self.find_by_short_code(short_code).await?;

        Ok(result.is_some())
    }

    async fn update(&self, url: &Url, valid_url: &ValidUrl) -> Result<Url, UrlError> {
        let result = sqlx::query_as!(
            UrlEntity,
            r#"
            UPDATE urls SET original_url = $1
            WHERE short_code = $2
            RETURNING *
            "#,
            valid_url.as_str(),
            url.short_code.as_str()
        )
        .fetch_one(&self.pool)
        .await?;

        result.to_domain()
    }
}
