use serde::Serialize;




#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub status: bool,
    pub message: String,
    pub data: Option<T>,
}