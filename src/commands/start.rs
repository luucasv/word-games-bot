use crate::messages;
use std::sync::Arc;
use tbot::contexts::{methods::ChatMethods, Command, Text};

pub async fn handler(message: Arc<Command<Text>>) {
    let response = messages::welcome(&message.from);
    let result = message.send_message(&response).call().await;
    if let Err(error) = result {
        dbg!(error);
    }
}
