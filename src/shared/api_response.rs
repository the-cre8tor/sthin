#[derive(serde::Serialize)]
pub struct Success<'a, T> {
    pub status: &'a str,
    pub message: &'a str,
    pub data: T,
}
