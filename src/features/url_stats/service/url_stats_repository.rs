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
    repository: T,
}

impl<T: IUrlStatsRepository> IUrlStatsService for UrlStatsService<T> {
    async fn record_url_access(&self, url: Url) -> Result<UrlStats, UrlStatsError> {
        let url = url.id.unwrap();
        self.repository.save(url, 1).await
    }
}
