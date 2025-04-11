use std::sync::Arc;

use crate::features::{
    url_stats::{error::UrlStatsError, model::UrlStats, repository::IUrlStatsRepository},
    urls::models::Url,
};

pub trait IUrlStatsService: Send + Sync {
    fn record_url_access(
        &self,
        url: Url,
    ) -> impl Future<Output = Result<UrlStats, UrlStatsError>> + Send;
}

#[derive(Clone)]
pub struct UrlStatsService<T> {
    repository: Arc<T>,
}

impl<T: IUrlStatsRepository> UrlStatsService<T> {
    pub fn new(repository: Arc<T>) -> Self {
        Self { repository }
    }
}

impl<T: IUrlStatsRepository> IUrlStatsService for UrlStatsService<T> {
    async fn record_url_access(&self, url: Url) -> Result<UrlStats, UrlStatsError> {
        let url_id = url.id.unwrap();
        let mut access_count = self.repository.find_one(url_id).await?;
        access_count = access_count + 1;

        self.repository.save(url_id, access_count).await
    }
}
