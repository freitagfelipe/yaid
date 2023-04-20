use dotenvy::dotenv;
use std::env;
use yaid::Bot;

#[tokio::main]
async fn main() {
    setup();

    let token = env::var("BOT_TOKEN").unwrap();

    let bot: &'static Bot = Bot::new(&token).expect("Invalid token or no internet connection!");

    bot.get_updates();
}

fn setup() {
    dotenv().ok();

    env::var("BOT_TOKEN").expect("BOT_TOKEN must be set!");
    env::var("BASE_URL").expect("BASE_URL must be set!");
}
