use crate::{application::CreateUrlDto, domain::services::IUrlService};

pub struct CreateUrlCommand<U: IUrlService + Send + Sync> {
    url_service: U,
}

impl<U: IUrlService> CreateUrlCommand<U> {
    pub fn new(url_service: U) -> Self {
        Self { url_service }
    }

    pub async fn execute(&self, dto: CreateUrlDto) {}
}
