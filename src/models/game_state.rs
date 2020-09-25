use std::collections::hash_map::HashMap;
use tbot::types::user;

pub struct GameState {
    users: HashMap<user::Id, user::User>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            users: HashMap::new(),
        }
    }

    // returns true if the user wasn't in the game already
    pub fn add_user(&mut self, user: user::User) -> bool {
        // if this returns a some it means that the user was already in the game
        self.users.insert(user.id, user).is_none()
    }

    // returns true if the user was in the game
    pub fn remove_user(&mut self, user: &user::User) -> bool {
        self.users.remove(&user.id).is_some()
    }

    pub fn get_players(&self) -> Vec<user::User> {
        self.users.values().map(user::User::to_owned).collect()
    }

    pub fn can_start(&self) -> bool {
        self.users.len() >= 4
    }
}
