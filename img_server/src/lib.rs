use std::collections::HashMap;

use kinode_process_lib::get_blob;
use kinode_process_lib::http::server::send_response;
use kinode_process_lib::http::server::HttpBindingConfig;
use kinode_process_lib::http::server::HttpServer;
use kinode_process_lib::http::StatusCode;
use kinode_process_lib::logging::{error, info, warn, init_logging, Level};
use kinode_process_lib::{await_message, call_init, kiprintln, Address, Message, Request, Response};
use serde_json::Value;

pub mod structs;

pub use structs::*;

wit_bindgen::generate!({
    path: "target/wit",
    world: "img-server-template-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

fn handle_message(our: &Address, message: Message) -> anyhow::Result<()> {
    kiprintln!("Received a message");
    // Send response right away so the mobile app can proceed
    send_response(
        StatusCode::OK,
        Some(HashMap::from([(
            String::from("Content-Type"),
            String::from("application/json"),
        )])),
        vec![],
    );
    kiprintln!("Sent response");
    match message {
        Message::Request {
            source,
            expects_response,
            body,
            metadata,
            capabilities,
        } => {
            kiprintln!("Decoding body1");
            let decoded_body =
                String::from_utf8(body).unwrap_or_else(|_| "Invalid UTF-8".to_string());

            let json_body: Value = serde_json::from_str(&decoded_body)?;
            kiprintln!("Decoded body");
            let website = json_body["Http"]["headers"]["website"]
                .as_str()
                .unwrap_or("No website found");
            kiprintln!("Website: {}", website);

            let base64_image = if let Some(blob) = get_blob() {
                let bytes = blob.bytes();
                println!("Received blob: {:?} bytes", bytes.len());
                Ok(base64::encode(&bytes))
            } else {
                Err(anyhow::anyhow!("Failed to get blob"))
            };
        }
        Message::Response {
            source,
            body,
            metadata,
            context,
            capabilities,
        } => todo!(),
    }
    Ok(())
}

call_init!(init);
fn init(our: Address) {
    init_logging(&our, Level::DEBUG, Level::INFO, None).unwrap();
    kiprintln!("begin1");
    let mut http_server = HttpServer::new(5);
    let http_config = HttpBindingConfig::new(false, false, false, None);
    match http_server.bind_http_path("/", http_config.clone()) {
        Ok(_) => info!("Server started successfully"),
        Err(e) => info!("Failed to start server: {}", e),
    }

    loop {
        match await_message() {
            Err(send_error) => error!("got SendError: {send_error}"),
            Ok(message) => match handle_message(&our, message) {
                Ok(_) => {}
                Err(e) => error!("got error while handling message: {e:?}"),
            },
        }
    }
}
