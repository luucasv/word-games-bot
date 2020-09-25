use tbot::markup::{markdown_v2, mention};
use tbot::types::{chat, Chat, User};

use super::message_literals::*;
use super::string_formatter::*;

pub fn welcome(user: &Option<User>) -> String {
    let name = match user {
        Some(user) => user.first_name.as_str(),
        None => "",
    };
    Welcome::format(&Lang::Pt, name)
}

pub fn join_private(chat: &Chat) -> String {
    let chat_title = match &chat.kind {
        chat::Kind::Group { title, .. } => title,
        chat::Kind::Supergroup { title, .. } => title,
        _ => "",
    };
    JoinPrivate::format(&Lang::Pt, chat_title)
}

pub fn not_in_group_error() -> &'static str {
    NotInGroupError::get(&Lang::Pt)
}

pub fn game_lobby(joined: &Vec<User>) -> String {
    let mentions: Vec<_> = joined
        .iter()
        .map(|user| (mention(&*user.first_name, user.id), ", "))
        .collect();
    let mentions_string = markdown_v2(mentions).to_string();
    GameLobby::format(&Lang::Pt, (joined.len(), &mentions_string))
}

pub fn already_in_game_error() -> &'static str {
    AlreadyInGameError::get(&Lang::Pt)
}
