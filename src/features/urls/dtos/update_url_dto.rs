use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateUrlDto {
    #[serde(default)]
    pub url: String,
}
