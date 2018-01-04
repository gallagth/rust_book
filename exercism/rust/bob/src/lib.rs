pub fn is_alphabetic(message: &str) -> bool {
    message.chars().any(|c| c.is_alphabetic())
}

pub fn is_yelling(message: &str) -> bool {
    is_alphabetic(message) && message == message.to_uppercase()
}

pub fn is_asking(message: &str) -> bool {
    message.trim().ends_with('?')
}

pub fn is_silent(message: &str) -> bool {
    message.trim().is_empty()
}

pub fn reply(message: &str) -> &str {
    let mut reply = "Whatever.";
    if is_silent(message) {
        reply = "Fine. Be that way!";
    } else if is_asking(message) && is_yelling(message) {
        reply = "Calm down, I know what I'm doing!";
    } else if is_asking(message) {
        reply = "Sure.";
    } else if is_yelling(message) {
        reply = "Whoa, chill out!";
    }
    reply
}
