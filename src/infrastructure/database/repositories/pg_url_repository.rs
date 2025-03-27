use sqlx::PgPool;

use crate::{
    domain::{errors::DomainError, models::Url, repositories::UrlRepository},
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

            "#,
        );

        Ok()
    }

    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<uuid::Uuid>, DomainError> {
        todo!()
    }

    async fn find_by_short_code(
        &self,
        short_code: &crate::domain::value_objects::ShortCode,
    ) -> Result<Option<Url>, DomainError> {
        todo!()
    }

    async fn find_by_original_url(
        &self,
        original_url: &crate::domain::value_objects::ValidUrl,
    ) -> Result<Option<Url>, DomainError> {
        todo!()
    }

    async fn update(&self, url: &Url) -> Result<Url, DomainError> {
        todo!()
    }

    async fn delete_by_short_code(
        &self,
        short_code: &crate::domain::value_objects::ShortCode,
    ) -> Result<bool, DomainError> {
        todo!()
    }

    async fn exists_by_short_code(
        &self,
        short_code: &crate::domain::value_objects::ShortCode,
    ) -> Result<bool, DomainError> {
        todo!()
    }
}
