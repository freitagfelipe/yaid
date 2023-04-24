# [YAID BOT](https://t.me/yaid_bot)

- YAID BOT is a Telegram bot that can download Instagram posts and stories from public users using the [YAID API](https://github.com/freitagfelipe/yaid-api).

## How YAID BOT was made

- YAID BOT is written in Rust, using [frankenstein](https://crates.io/crates/frankenstein) crate to communicate with the Telegram API and other crates like:
    - [dotenv](https://crates.io/crates/dotenv)
    - [reqwest](https://crates.io/crates/reqwest)
    - [serde](https://crates.io/crates/serde)
    - [tokio](https://crates.io/crates/tokio)

## Commands help

- If you just send a message without a command YAID will ask you what you want to do with three options, you will hit the first option if was a message with a post url, or the second option if was a message with an username, or the third if the message was a mistake.

### start

- Description: explain to you how to use me.
- Usage: /start

#### download-post

- Description: download a post from a public user.
- Usage: /download_post <post-link\>

#### download-stories

- Description: download all stories from a public user. 
- Usage: /download_stories <user-name\>

#### help
- Description: reminds you how to use me.
- Usage: /help

## Acknowledgments

- [Antonio Lucas](https://github.com/antoniolucas30) for the README typo review!
- [Gabriela Machado](https://www.instagram.com/thanosdecalcinha/) for the bot's profile picture!