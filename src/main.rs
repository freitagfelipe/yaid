use dotenv;
use std::env;
use yaid::Bot;

fn main() {
    dotenv::dotenv().ok();

    let token = env::var("TOKEN").expect("TOKEN must be set!");

    let bot: &'static Bot = Bot::new(&token).expect("Invalid token!");

    bot.get_updates();
}
