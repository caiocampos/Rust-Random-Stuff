pub fn reply(message: &str) -> &str {
    let message = message.trim_end();
    match (
        message.ends_with("?"),
        message.contains(char::is_uppercase),
        message.contains(char::is_lowercase),
        message.is_empty()
    ) {
        (true, true, false, false) => "Calm down, I know what I'm doing!",
        (true, _, _, _) => "Sure.",
        (false, true, false, _) => "Whoa, chill out!",
        (false, false, false, true) => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
