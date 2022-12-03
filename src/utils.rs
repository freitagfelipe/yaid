use frankenstein::Message;

pub fn get_content(message: &Message) -> Result<&str, ()> {
    let content = message.text.as_ref().unwrap().split(' ').skip(1).last();

    match content {
        Some(res) => Ok(res.trim()),
        None => Err(()),
    }
}
