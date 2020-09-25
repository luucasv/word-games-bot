use super::command::{Command, HandlerCommandArg, HandlerResult, HandlerStateArg};
use super::util;
use crate::strings::messages;
use async_trait::async_trait;
use tbot::contexts::methods::ChatMethods;

pub struct StartCommand;

#[async_trait]
impl Command for StartCommand {
    const NAME: &'static str = "start";
    const DESCRIPTION: &'static str = "The start command";

    async fn handler(message: HandlerCommandArg, state: HandlerStateArg) -> HandlerResult {
        let user = message.from.clone().unwrap();
        if let Some((message_id, group_id)) = state.remove_user_from_pending_join(&user).await {
            if state.add_user_to_game(&group_id, user).await {
                util::update_game_lobby_message(&message.bot, state, group_id, message_id).await?;
            }
        } else {
            let answer = messages::welcome(&message.from);
            message.send_message(&answer).call().await?;
        }
        Ok(())
    }
}
