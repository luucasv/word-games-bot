use crate::models::BotState;
use crate::strings::{keyboards, messages};
use std::sync::Arc;
use tbot::errors::MethodCall;
use tbot::types::parameters::Text;
use tbot::types::{chat, keyboard::inline::Keyboard, message};
use tbot::Bot;

/// update the game lobby message
///
/// # Panics
///
/// panics if game doesn't exist
pub async fn update_game_lobby_message(
    bot: &Arc<Bot>,
    state: Arc<BotState>,
    group_id: chat::Id,
    message_id: message::Id,
) -> Result<(), MethodCall> {
    let can_start = state.game_can_start(&group_id).await;
    let joined = state.get_game_players(&group_id).await.unwrap();
    let text = messages::game_lobby(&joined);
    let keyboard = keyboards::game_lobby_keyboard(can_start);
    to_slice!(keyboard);
    bot.edit_message_text(group_id, message_id, Text::markdown_v2(&text))
        .reply_markup(Keyboard::from(keyboard))
        .call()
        .await?;
    Ok(())
}
