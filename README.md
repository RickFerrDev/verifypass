# verifypass
This is a simple Rust library for verifying passwords against a set of criteria.
##  Usage
 * Add the library to your Cargo.toml file:
```rust
[dependencies]
verify-password = "0.1.0"
```
 * Use the verify function to check a password:
```rust
use verify_password::{verify, structures::PasswordCriterias};

fn main() {
    let criterias = PasswordCriterias {
        min_length: 8,
        max_length: 32,
        require_uppercase: true,
        require_lowercase: true,
        require_digits: true,
        require_special_char: true,
    };

    let password = "MyStrongPassword123!";
    let (is_valid, reasons) = verify(password, &criterias);

    if is_valid {
        println!("Password is valid!");
    } else {
        println!("Password is invalid:");
        for reason in reasons {
            println!("- {}", reason);
        }
    }
}
```
# Features
 * Checks password length
 * Checks for uppercase, lowercase, digits, and special characters
 * Provides clear error messages
# PRs
We would love your help to improve this project! If you find a bug, have a suggestion or want to add new features, please open an issue or send a pull request.

