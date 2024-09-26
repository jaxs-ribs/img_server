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


fn handle_message(our: &Address, message: &Message) -> anyhow::Result<()> {
    Ok(())
}

call_init!(init);
fn init(our: Address) {
    init_logging(&our, Level::DEBUG, Level::INFO, None).unwrap();
    info!("begin");

    loop {
        match await_message() {
            Err(send_error) => error!("got SendError: {send_error}"),
            Ok(ref message) => match handle_message(&our, message) {
                Ok(_) => {}
                Err(e) => error!("got error while handling message: {e:?}"),
            },
        }
    }
}
