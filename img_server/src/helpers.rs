use crate::Address;
use crate::State;
use anyhow::Result;
use kinode_process_lib::{get_blob, kiprintln};
use kinode_process_lib::{
    http::server::{send_response, HttpBindingConfig, HttpServer},
    http::StatusCode,
    set_state,
};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

pub fn save_state(state: &State) -> anyhow::Result<()> {
    set_state(&serde_json::to_vec(state)?);
    Ok(())
}

pub fn send_http_json_response<T: serde::Serialize>(
    status: StatusCode,
    data: &T,
) -> anyhow::Result<()> {
    let json_data = serde_json::to_vec(data)?;
    send_response(
        status,
        Some(HashMap::from([(
            String::from("Content-Type"),
            String::from("application/json"),
        )])),
        json_data,
    );
    Ok(())
}

pub fn get_jpeg_bytes() -> Result<Vec<u8>> {
    if let Some(blob) = get_blob() {
        kiprintln!("got jpeg bytes of len {}", blob.bytes.len());
        Ok(blob.bytes)
    } else {
        Err(anyhow::anyhow!("Failed to get blob"))
    }
}

pub fn calculate_sha256_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash_result = hasher.finalize();
    format!("{:x}", hash_result)
}

pub fn setup_http_server(our: &Address) -> Result<()> {
    let mut http_server = HttpServer::new(5);
    let http_config = HttpBindingConfig::new(false, false, false, None);
    http_server.bind_http_path("/", http_config.clone())?;

    http_server
        .serve_ui(&our, "ui", vec!["/main.html"], http_config.clone())
        .expect("Failed to serve UI");
    Ok(())
}
