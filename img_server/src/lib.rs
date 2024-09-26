use std::collections::HashMap;

use crate::kinode::process::img_server::{
    ImgServerMessage, Request as ImgServerRequest, Response as ImgServerResponse, SendRequest,
};
use kinode_process_lib::logging::{error, info, init_logging, Level};
use kinode_process_lib::{await_message, call_init, println, Address, Message, Request, Response};

wit_bindgen::generate!({
    path: "target/wit",
    world: "img-server-template-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

type MessageArchive = HashMap<String, Vec<ImgServerMessage>>;

fn handle_message(
    our: &Address,
    message: &Message,
    message_archive: &mut MessageArchive,
) -> anyhow::Result<()> {
    if !message.is_request() {
        return Err(anyhow::anyhow!("unexpected Response: {:?}", message));
    }

    let body = message.body();
    let source = message.source();
    match body.try_into()? {
        ImgServerRequest::Send(SendRequest {
            ref target,
            ref message,
        }) => {
            if target == &our.node {
                println!("{}: {}", source.node, message);
                let message = ImgServerMessage {
                    author: source.node.clone(),
                    content: message.into(),
                };
                message_archive
                    .entry(source.node.clone())
                    .and_modify(|e| e.push(message.clone()))
                    .or_insert(vec![message]);
            } else {
                let _ = Request::new()
                    .target(Address {
                        node: target.clone(),
                        process: "img_server:img_server:template.os".parse()?,
                    })
                    .body(body)
                    .send_and_await_response(5)?
                    .unwrap();
                let message = ImgServerMessage {
                    author: our.node.clone(),
                    content: message.into(),
                };
                message_archive
                    .entry(target.clone())
                    .and_modify(|e| e.push(message.clone()))
                    .or_insert(vec![message]);
            }
            Response::new().body(ImgServerResponse::Send).send().unwrap();
        }
        ImgServerRequest::History(ref node) => {
            Response::new()
                .body(ImgServerResponse::History(
                    message_archive
                        .get(node)
                        .map(|msgs| msgs.clone())
                        .unwrap_or_default(),
                ))
                .send()
                .unwrap();
        }
    }
    Ok(())
}

call_init!(init);
fn init(our: Address) {
    init_logging(&our, Level::DEBUG, Level::INFO, None).unwrap();
    info!("begin");

    let mut message_archive = HashMap::new();

    loop {
        match await_message() {
            Err(send_error) => error!("got SendError: {send_error}"),
            Ok(ref message) => match handle_message(&our, message, &mut message_archive) {
                Ok(_) => {}
                Err(e) => error!("got error while handling message: {e:?}"),
            },
        }
    }
}
