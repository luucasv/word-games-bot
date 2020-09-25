use super::util;
use crate::models::BotState;
use crate::strings::data_callback_literals::*;
use crate::strings::messages;
use std::sync::Arc;
use tbot::contexts;
use tbot::contexts::methods::Callback;
use tbot::errors::MethodCall;
use tbot::types::message::Message;

pub struct DataCallback;
impl DataCallback {
    pub async fn handler(data: Arc<contexts::DataCallback>, state: Arc<BotState>) {
        if let Some(message) = data.origin.clone().message() {
            let result = match data.data.as_str() {
                JOIN_BTN_DATA => Self::add_user(data, state, &message).await,
                LEAVE_BTN_DATA => Self::remove_user(data, state, &message).await,
                START_BTN_DATA => Ok(()),
                _ => {
                    dbg!(format!("Invalid callback data received: {:#?}", data));
                    Ok(())
                }
            };

            if let Err(error) = result {
                dbg!(error);
            }
        } else {
            dbg!(format!("Invalid callback data origin: {:#?}", data.origin));
        }
    }

    async fn add_user(
        data: Arc<contexts::DataCallback>,
        state: Arc<BotState>,
        message: &Box<Message>,
    ) -> Result<(), MethodCall> {
        // send private message saying the user joined, this will help check if the bot can talk to the user
        let answer = messages::join_private(&message.chat);
        let result = data.bot.send_message(data.from.id, &answer).call().await;

        let group_id = message.chat.id;
        let user = data.clone().from.clone();
        match result {
            Err(error) => {
                if error.is_request_error() {
                    // the bot can't talk to the user
                    state
                        .add_user_to_pending_join(&group_id, &user, &message.id)
                        .await;
                    let start_url = format!(
                        "t.me/{username}?start=join",
                        username = state.bot_username()
                    );
                    data.open_url(&start_url).call().await?;
                } else {
                    return Err(error);
                }
            }
            Ok(join_message) => {
                let added_user = state.add_user_to_game(&group_id, user).await;
                if added_user {
                    util::update_game_lobby_message(&data.bot, state, message.chat.id, message.id)
                        .await?;
                } else {
                    data.bot
                        .delete_message(join_message.chat.id, join_message.id)
                        .call()
                        .await?;
                }
                data.ignore().call().await?;
            }
        }
        Ok(())
    }

    async fn remove_user(
        data: Arc<contexts::DataCallback>,
        state: Arc<BotState>,
        message: &Box<Message>,
    ) -> Result<(), MethodCall> {
        let result = state
            .remove_user_from_game(&message.chat.id, &data.from)
            .await;
        if result {
            util::update_game_lobby_message(&data.bot, state, message.chat.id, message.id).await?;
        }
        data.ignore().call().await?;
        Ok(())
    }
}
