use word_games_bot::commands::*;
use word_games_bot::models::BotState;

macro_rules! add_commands {
    ($bot:ident => [ $( $cmd:path ),* ] ) => {
        $($bot.command(<$cmd>::NAME, <$cmd>::final_handler);)*
    }
}

#[tokio::main]
async fn main() {
    let bot = tbot::Bot::from_env("BOT_TOKEN");
    let bot_username = bot.get_me().call().await.unwrap().user.username.unwrap();

    let mut bot = bot.stateful_event_loop(BotState::new(bot_username.clone()));
    bot.username(bot_username);

    bot.start(StartCommand::final_handler);
    add_commands!(bot => [NewGameCommand]);

    bot.data_callback(DataCallback::handler);

    bot.fetch_username().await.unwrap();
    bot.polling().start().await.unwrap();
}
