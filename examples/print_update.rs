use anyhow::Result;
use telegram_bot_api_rs::bot::Bot;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let token = std::env::var("TOKEN").expect("TOKEN is not Set");
    let bot = Bot::new(token);
    bot.start_get_updates(Default::default()).await?;
    let mut subscriber = bot.subscribe_updates();
    while let Ok(update) = subscriber.recv().await {
        println!("{:#?}", update);
    }

    Ok(())
}
