pub fn get_content(message: &str) -> Option<&str> {
    let content = message.split(' ').skip(1).last();

    match content {
        Some(res) => Some(res.trim()),
        None => None,
    }
}
