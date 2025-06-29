use std::sync::Arc;

use crate::features::{
    url_stats::{
        error::UrlStatsError,
        model::{LogList, UrlStatsModel},
        queue::StatsEvent,
        repository::IUrlStatsRepository,
    },
    urls::value_objects::ShortCode,
};

pub trait IUrlStatsService: Send + Sync {
    fn record_url_access_and_log(
        &self,
        event: StatsEvent,
    ) -> impl Future<Output = Result<UrlStatsModel, UrlStatsError>> + Send;
    fn fetch_stats(
        &self,
        short_code: ShortCode,
    ) -> impl Future<Output = Result<Option<LogList>, UrlStatsError>> + Send;
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
    async fn record_url_access_and_log(
        &self,
        event: StatsEvent,
    ) -> Result<UrlStatsModel, UrlStatsError> {
        let url_id = event.url.id.unwrap();
        let mut access_count = self.repository.find_one(url_id).await?;
        access_count = access_count + 1;

        self.repository.save(&event, access_count).await
    }

    async fn fetch_stats(&self, short_code: ShortCode) -> Result<Option<LogList>, UrlStatsError> {
        let response = self.repository.fetch_stats(short_code).await;

        response
    }
}
