use super::game_state::GameState;
use std::collections::hash_map::HashMap;
use tbot::types::{chat, message, user};
use tokio::sync::RwLock;

pub struct BotState {
    username: String,
    groups: RwLock<HashMap<chat::Id, GameState>>,
    pending_join: RwLock<HashMap<user::Id, (message::Id, chat::Id)>>,
}

impl BotState {
    pub fn new(username: String) -> BotState {
        BotState {
            username,
            groups: RwLock::new(HashMap::new()),
            pending_join: RwLock::new(HashMap::new()),
        }
    }

    pub async fn add_user_to_pending_join(
        &self,
        group_id: &chat::Id,
        user: &user::User,
        message_id: &message::Id,
    ) {
        let mut pending_join = self.pending_join.write().await;
        pending_join.insert(user.id, (*message_id, *group_id));
    }

    pub async fn remove_user_from_pending_join(
        &self,
        user: &user::User,
    ) -> Option<(message::Id, chat::Id)> {
        let mut pending_join = self.pending_join.write().await;
        pending_join.remove(&user.id)
    }

    pub fn bot_username(&self) -> &String {
        &self.username
    }

    // ----------- Game methods -----------
    pub async fn new_game(&self, group_id: chat::Id) -> bool {
        let mut groups = self.groups.write().await;

        if groups.contains_key(&group_id) {
            false
        } else {
            groups.insert(group_id, GameState::new());
            true
        }
    }

    /// returns true if a new user is added to the game
    pub async fn add_user_to_game(&self, group_id: &chat::Id, user: user::User) -> bool {
        let mut groups = self.groups.write().await;

        if let Some(game_state) = groups.get_mut(group_id) {
            game_state.add_user(user)
        } else {
            false
        }
    }

    /// returns true if an user was removed from the game
    pub async fn remove_user_from_game(&self, group_id: &chat::Id, user: &user::User) -> bool {
        let mut groups = self.groups.write().await;

        if let Some(game_state) = groups.get_mut(group_id) {
            game_state.remove_user(user)
        } else {
            false
        }
    }

    pub async fn get_game_players(&self, group_id: &chat::Id) -> Option<Vec<user::User>> {
        let groups = self.groups.read().await;

        groups
            .get(group_id)
            .map(|game_state| game_state.get_players())
    }

    pub async fn game_can_start(&self, group_id: &chat::Id) -> bool {
        let groups = self.groups.read().await;

        groups
            .get(group_id)
            .map(|game_state| game_state.can_start())
            .unwrap_or(false)
    }
}
