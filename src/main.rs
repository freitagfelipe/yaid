use dotenv;
use std::env;
use yaid::Bot;

#[tokio::main]
async fn main() {
    setup();

    let token = env::var("TOKEN").unwrap();

    let bot: &'static Bot = Bot::new(&token).expect("Invalid token or no internet connection!");

    bot.get_updates();
}

fn setup() {
    dotenv::dotenv().ok();

    env::var("TOKEN").expect("TOKEN must be set!");
    env::var("URL").expect("URL must be set!");
    env::var("AUTH_TOKEN").expect("AUTH_TOKEN must be set!");
}
