use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub images: HashMap<URI, Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImgServerRequest {
    UploadImage,
    GetImage(GetImageRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImgServerResponse {
    UploadImage(UploadImageResponse),
    GetImage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadImageResponse {
    pub uri: URI,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetImageRequest {
    pub uri: URI,
}

pub type URI = String;
