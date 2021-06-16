pub fn reply(message: &str) -> &str {
    if message.trim().len() == 0 {
        "Fine. Be that way!"
    } else if message
        .chars()
        .filter(|x| x.is_ascii_alphabetic() && x.is_ascii_lowercase())
        .collect::<String>()
        .len()
        == 0
        && message.chars().any(|x| x.is_ascii_uppercase())
    {
        if message.trim_end().chars().last() == Some('?') {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        }
    } else if message.trim_end().chars().last() == Some('?') {
        "Sure."
    } else {
        "Whatever."
    } 
}
