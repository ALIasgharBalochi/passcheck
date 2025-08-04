# 🔐 passcheck

**passcheck** is a flexible and lightweight password validation crate for Rust.  
It allows you to define custom rules for checking password strength and structure — use only what you need!

---

## 🚀 Features

- ✅ Rule-based validation system
- ✅ Add only the checks you want: length, number, special char, etc.
- ✅ Lightweight and dependency-free
- ✅ Easy to use and extend

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
passcheck = "0.1.1"  
```
## example
```rust
use passcheck::PasswordChecker;

fn main() {
    let checker = PasswordChecker::new()
        .min_length(8)
        .require_number()
        .require_special_char();

    match checker.validate("Ali@123") {
        Ok(_) => println!("✅ Strong password!"),
        Err(errors) => {
            println!("❌ Invalid password:");
            for err in errors {
                println!("  - {}", err);
            }
        }
    }
}
```
---
# 🔧 Available Rules
min_length(n) → Requires password to be at least n characters

require_upper_lower() → Requires at least one uppercase and one lowercase letter

require_number() → Requires at least one digit

require_special_char() → Requires at least one special character

---
 📄 License
MIT
---
💻 Author
Made with ❤ + ☕ by Ali
