use anyhow::Result;

use kinode_process_lib::{
    await_message, call_init, get_state, kiprintln,
    logging::{error, info, init_logging, Level},
    set_state, Address, Message, ProcessId,
};
use std::str::FromStr;

pub mod helpers;
pub mod structs;
pub mod kino_msg_handlers;

pub use helpers::*;
pub use structs::*;
pub use kino_msg_handlers::*;

wit_bindgen::generate!({
    path: "target/wit",
    world: "img-server-template-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

fn handle_message(_our: &Address, message: Message, state: &mut State) -> Result<()> {
    kiprintln!("Received a message");
    helpers::send_immediate_response();

    match message {
        Message::Request { body, source, .. } => handle_request(body, &source, state),
        Message::Response { .. } => todo!(),
    }
}


fn handle_request(body: Vec<u8>, source: &Address, state: &mut State) -> Result<()> {
    let http_server_address = ProcessId::from_str("http_server:distro:sys")?;
    if source.process.eq(&http_server_address) {
        handle_http_server_request(state)
    } else {
        handle_kinode_request(&body, state)
    }
}

// TODO: Zena: We need to move this to hq and forward it this kinode
fn handle_http_server_request(state: &mut State) -> anyhow::Result<()> {
    let jpeg_bytes = helpers::get_jpeg_bytes()?;
    let hash_hex = helpers::calculate_sha256_hash(&jpeg_bytes);

    kiprintln!("SHA-256 hash of JPEG: {}", hash_hex);
    state.images.insert(hash_hex, jpeg_bytes);
    save_state(state)?;
    Ok(())
}

fn handle_kinode_request(body: &[u8], state: &mut State) -> anyhow::Result<()> {
    let request: ImgServerRequest = serde_json::from_slice(body)?;
    match request {
        ImgServerRequest::UploadImage(_upload_image_request) => Ok(()), // TODO: make this kinode msg
        ImgServerRequest::GetImage(get_image_request) => handle_get_image_request(get_image_request, state),
    }
}

call_init!(init);

fn init(our: Address) {
    init_logging(&our, Level::DEBUG, Level::INFO, None).unwrap();
    kiprintln!("begin1");

    if let Err(e) = helpers::setup_http_server() {
        info!("Failed to start server: {}", e);
    }

    let mut state: State = if let Some(state) = get_state() {
        if let Ok(state) = serde_json::from_slice::<State>(&state) {
            kiprintln!("Successfully loaded state");
            state
        } else {
            kiprintln!("Failed to deserialize state, using default");
            State::default()
        }
    } else {
        kiprintln!("No state found, using default");
        State::default()
    };

    loop {
        match await_message() {
            Err(send_error) => error!("got SendError: {send_error}"),
            Ok(message) => {
                if let Err(e) = handle_message(&our, message, &mut state) {
                    error!("got error while handling message: {e:?}");
                }
            }
        }
    }
}
