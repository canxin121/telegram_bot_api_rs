use anyhow::Result;
use telegram_bot_api_rs::{available_methods::payload::SendMessagePayload, bot::Bot};
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let token = std::env::var("TOKEN").expect("TOKEN is not Set");
    let bot = Bot::new(token);
    bot.send_message(&SendMessagePayload {
        chat_id: todo!(),
        text: todo!(),
        ..Default::default()
    })
    .await
    .unwrap();
    Ok(())
}
