use structures::PasswordCriterias;

pub mod internal;
pub mod structures;

pub fn verify(password: &str, criterias: &PasswordCriterias) -> (bool, Vec<String>) {
    let mut reasons = Vec::new();

    if password.len() < criterias.min_length || password.len() > criterias.max_length {
        reasons.push(format!(
            "The password must have a minimum length of {} and a maximum length of {} characters",
            criterias.min_length, criterias.max_length
        ));
    }

    if criterias.require_uppercase && !internal::is_uppercase(password) {
        reasons.push(String::from(
            "Password must contain at least one uppercase letter.",
        ));
    }

    if criterias.require_lowercase && !internal::is_lowercase(password) {
        reasons.push(String::from(
            "Password must contain at least one lowercase letter.",
        ));
    }

    if criterias.require_digits && !internal::contains_digits(password) {
        reasons.push(String::from("Password must contain at least one digit."));
    }

    if criterias.require_special_char && !internal::contains_special_char(password) {
        reasons.push(String::from(
            "Password must contain at least one special character.",
        ));
    }

    (reasons.is_empty(), reasons)
}

#[cfg(test)]
mod tests {
    use self::structures::PasswordCriterias;

    use super::*;

    #[test]
    fn it_min_length_and_max_length_work() {
        let criterias = PasswordCriterias {
            min_length: 8,
            max_length: 12,
            require_digits: false,
            require_uppercase: false,
            require_lowercase: true,
            require_special_char: false,
        };

        let (is_safe, _) = verify(&"rickferrdev", &criterias);
        assert_eq!(is_safe, true)
    }

    #[test]
    fn it_uppercase_and_lowercase_work() {
        let criterias = PasswordCriterias {
            min_length: 8,
            max_length: 12,
            require_digits: false,
            require_uppercase: true,
            require_lowercase: true,
            require_special_char: false,
        };

        let (is_safe, _) = verify(&"RickFerrDev", &criterias);
        assert_eq!(is_safe, true)
    }

    #[test]
    fn it_digits_and_special_chars_work() {
        let criterias = PasswordCriterias {
            min_length: 0,
            max_length: 12,
            require_digits: true,
            require_uppercase: false,
            require_lowercase: false,
            require_special_char: true,
        };

        let (is_safe, _) = verify(&"@123", &criterias);
        assert_eq!(is_safe, true)
    }
}
