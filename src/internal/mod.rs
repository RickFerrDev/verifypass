use regex::Regex;

pub fn is_uppercase(text: &str) -> bool {
    text.chars().any(|t| t.is_uppercase())
}

pub fn is_lowercase(text: &str) -> bool {
    text.chars().any(|t| t.is_lowercase())
}

pub fn contains_digits(text: &str) -> bool {
    text.chars().any(|t| t.is_ascii_digit())
}

pub fn contains_special_char(text: &str) -> bool {
    Regex::new(r"[^\w\s]").unwrap().is_match(text)
}
