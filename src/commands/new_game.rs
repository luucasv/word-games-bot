use super::command::{Command, HandlerCommandArg, HandlerResult, HandlerStateArg};
use super::helper::HelperMethods;
use crate::strings::{keyboards, messages};
use async_trait::async_trait;
use tbot::contexts::methods::ChatMethods;
use tbot::types::parameters::Text;

pub struct NewGameCommand;

#[async_trait]
impl Command for NewGameCommand {
    const NAME: &'static str = "newgame";
    const DESCRIPTION: &'static str = "Create a new game in a group";

    async fn handler(message: HandlerCommandArg, state: HandlerStateArg) -> HandlerResult {
        if !message.from_group() {
            let answer = messages::not_in_group_error();
            message.send_message_in_reply(answer).call().await?;
            return Ok(());
        }
        if !state.new_game(message.chat.id).await {
            let answer = messages::already_in_game_error();
            message.send_message_in_reply(answer).call().await?;
        } else {
            // TODO: get from state?
            let empty = Vec::new();
            let answer = messages::game_lobby(&empty);
            let markup = keyboards::game_lobby_keyboard(false);
            to_slice!(markup);
            message
                .send_message(Text::markdown_v2(&answer))
                .reply_markup(markup)
                .call()
                .await?;
        }
        Ok(())
    }
}
