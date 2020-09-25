use tbot::contexts;
use std::sync::Arc;
use async_trait::async_trait;
use crate::models::BotState;

pub type HandlerCommandArg = Arc<contexts::Command<contexts::Text>>;
pub type HandlerStateArg = Arc<BotState>;
pub type HandlerResult = Result<(), tbot::errors::MethodCall>;

#[async_trait]
pub trait Command {
    const NAME: &'static str;
    const DESCRIPTION: &'static str;
    async fn handler(command: HandlerCommandArg, state: HandlerStateArg) -> HandlerResult;
    async fn final_handler(command: HandlerCommandArg, state: HandlerStateArg) {
        let result = Self::handler(command, state).await;
        if let Err(error) = result {
            // TODO: log, using the name of the command
            dbg!(error);
        }
    }
}
