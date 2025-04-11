use crate::features::{
    url_stats::{
        repository::UrlStatsRepository,
        service::{IUrlStatsService, UrlStatsService},
    },
    urls::models::Url,
};
use std::{sync::Arc, time::Instant};
use tokio::sync::mpsc;

pub struct StatsEvent {
    pub data: Url,
    pub timestamp: Instant,
}

#[derive(Clone)]
pub struct StatsProcessor {
    pub sender: mpsc::Sender<StatsEvent>,
}

impl StatsProcessor {
    pub fn new(capacity: usize, service: Arc<UrlStatsService<UrlStatsRepository>>) -> Self {
        let (sender, mut receiver) = mpsc::channel::<StatsEvent>(capacity);

        tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                if let Err(error) = service.record_url_access(event.data).await {
                    println!("Failed to record stats: {}", error)
                }
            }
        });

        Self { sender }
    }
}
