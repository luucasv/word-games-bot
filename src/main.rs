use word_games_bot::commands;

#[tokio::main]
async fn main() {
  let mut bot = tbot::Bot::from_env("BOT_TOKEN").event_loop();
  bot.start(commands::start::handler);

  bot.fetch_username().await.unwrap();
  bot.polling().start().await.unwrap(); 
}
