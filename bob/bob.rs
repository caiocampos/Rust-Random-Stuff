pub fn reply(message: &str) -> &str {
    match (
        message.trim_end().ends_with("?"),
        message.find(char::is_uppercase),
        message.find(char::is_lowercase),
        message.find(char::is_alphanumeric),
    ) {
        (true, Some(_), None, Some(_)) => "Calm down, I know what I'm doing!",
        (true, _, _, _) => "Sure.",
        (false, Some(_), None, _) => "Whoa, chill out!",
        (false, None, None, None) => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
