use tbot::contexts::methods::ChatMethods;

#[tokio::main]
async fn main() {
  let mut bot = tbot::Bot::from_env("BOT_TOKEN").event_loop();
  bot.text(|context| async move {
    let send_result = context.send_message("Hi, I am the word games bot!").call().await;
    if let Err(err) = send_result {
      dbg!(err);
    }
  });

  bot.polling().start().await.unwrap(); 
}
