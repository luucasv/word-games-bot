use std::sync::Arc;
use tbot::contexts::{Command, Text};
use tbot::types::chat::Kind;

pub trait HelperMethods {
    fn from_group(&self) -> bool;
}

impl HelperMethods for Arc<Command<Text>> {
    fn from_group(&self) -> bool {
        match self.chat.kind {
            Kind::Group { .. } => true,
            Kind::Supergroup { .. } => true,
            _ => false,
        }
    }
}
