use anyhow::Result;

use kinode_process_lib::{
    await_message, call_init, get_state,
    http::StatusCode,
    kiprintln,
    logging::{error, info, init_logging, Level},
    Address, Message, ProcessId, Response,
};
use std::str::FromStr;

pub mod helpers;
pub mod msg_handlers;
pub mod structs;

pub use helpers::*;
pub use msg_handlers::*;
pub use structs::*;

wit_bindgen::generate!({
    path: "target/wit",
    world: "img-server-template-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

fn handle_message(_our: &Address, message: Message, state: &mut State) -> Result<()> {
    kiprintln!("Received a message");

    match message {
        Message::Request { body, source, .. } => handle_request(body, &source, state),
        Message::Response { .. } => todo!(),
    }
}

fn handle_request(body: Vec<u8>, source: &Address, state: &mut State) -> Result<()> {
    let http_server_address = ProcessId::from_str("http_server:distro:sys")?;
    if source.process.eq(&http_server_address) {
        handle_http_server_request(body, state)
    } else {
        handle_kinode_request(&body, state)
    }
}

// TODO: Zena: We need to move this to hq and forward it this kinode
fn handle_http_server_request(body: Vec<u8>, state: &mut State) -> anyhow::Result<()> {
    if let Ok(ImgServerRequest::GetImage(uri)) =
        serde_json::from_slice::<ImgServerRequest>(&body)
    {
        match get_img(uri, state) {
            Ok(img_bytes) => send_http_json_response(StatusCode::OK, &img_bytes),
            // TODO: Zena: Later: Use payload to send the image bytes instead of jsonified bytes...
            Err(e) => send_http_json_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
        }
    } else {
        match upload_img(state) {
            Ok(uri) => send_http_json_response(StatusCode::OK, &uri),
            Err(e) => send_http_json_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
        }
    }
}

fn handle_kinode_request(body: &[u8], state: &mut State) -> anyhow::Result<()> {
    let request: ImgServerRequest = serde_json::from_slice(body)?;
    let response_body: ImgServerResponse = match request {
        ImgServerRequest::UploadImage => match upload_img(state) {
            Ok(uri) => ImgServerResponse::UploadImage(Ok(uri)),
            Err(e) => ImgServerResponse::UploadImage(Err(e.to_string())),
        },
        ImgServerRequest::GetImage(get_image_request) => match get_img(get_image_request, state) {
            Ok(img_bytes) => ImgServerResponse::GetImage(Ok(img_bytes)),
            Err(e) => ImgServerResponse::GetImage(Err(e.to_string())),
        },
    };

    Ok(Response::new().body(response_body).send()?)
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
