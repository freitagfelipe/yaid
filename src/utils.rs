pub enum Error {
    NoSecondParameter,
    TooMuchParameters,
}

pub fn get_content(message: &str) -> Result<&str, Error> {
    let content: Vec<&str> = message
        .split_terminator(' ')
        .skip(1)
        .filter(|x| !x.is_empty())
        .collect();

    if content.is_empty() {
        return Err(Error::NoSecondParameter);
    } else if content.len() > 1 {
        return Err(Error::TooMuchParameters);
    }

    Ok(content.first().expect("Expect an element at the front"))
}
