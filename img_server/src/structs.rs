use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImgServerRequest {
    UploadImage(UploadImageRequest),
    GetImage(GetImageRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImgServerResponse {
    UploadImage(UploadImageResponse),
    GetImage(GetImageResponse),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadImageRequest {
    pub image: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadImageResponse {
    pub uri: URI,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetImageRequest {
    pub uri: URI,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetImageResponse {
    pub image: Vec<u8>,
}

pub type URI = String;