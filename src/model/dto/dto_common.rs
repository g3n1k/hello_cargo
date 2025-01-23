use serde::Serialize;




#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub status: bool,
    pub message: String,
    pub data: Option<T>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct MessageResponse {
    pub success: bool,
    pub message: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DataResponse<T> {
    pub success: bool,
    pub data: T,
}