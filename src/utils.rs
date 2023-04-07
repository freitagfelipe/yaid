use frankenstein::Message;

pub fn get_content(message: &Message) -> Option<&str> {
    let content = message.text.as_ref().unwrap().split(' ').skip(1).last();

    match content {
        Some(res) => Some(res.trim()),
        None => None,
    }
}
