use anyhow::Result;
use kinode_process_lib::{get_blob, kiprintln};
use kinode_process_lib::{
    http::server::{HttpBindingConfig, HttpServer},
    logging::info, set_state,
};
use sha2::{Digest, Sha256};

pub fn save_state(state: &State) -> anyhow::Result<()> {
    set_state(&serde_json::to_vec(state)?);
    Ok(())
}

pub fn send_immediate_response() {
    use kinode_process_lib::http::{server::send_response, StatusCode};
    use std::collections::HashMap;

    send_response(
        StatusCode::OK,
        Some(HashMap::from([(
            String::from("Content-Type"),
            String::from("application/json"),
        )])),
        vec![],
    );
    kiprintln!("Sent response");
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

pub fn setup_http_server() -> Result<()> {
    let mut http_server = HttpServer::new(5);
    let http_config = HttpBindingConfig::new(false, false, false, None);
    http_server.bind_http_path("/", http_config)?;
    info!("Server started successfully");
    Ok(())
}
