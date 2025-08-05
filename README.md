# 🔐 passcheck

**passcheck** is a flexible and lightweight password validation crate for Rust.  
It allows you to define custom rules for checking password strength and structure — use only what you need!

---

## 🚀 Features

- ✅ Rule-based validation system  
- ✅ Add only the checks you want: length, uppercase/lowercase, number, special char, etc.  
- ✅ Supports custom error messages per rule via `Option<&str>`  
- ✅ Lightweight and dependency-free  
- ✅ Easy to use and extend with a fluent API  

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
passcheck = "0.1.1"
```
# 🎯 Usage Example 
```rust 
use passcheck::PasswordChecker;

fn main() {
    let checker = PasswordChecker::new()
        .min_length(8, None)  // Use default error message with None OR use Some(str) for use custom message
        .require_upper_lower(None)
        .require_number(Some("Password must include at least one digit")) // Custom message
        .require_special_char(None);

    let password = "Ali@123";

    match checker.validate(password) {
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
min_length(n, Option<&str>) → Requires password to be at least n characters

require_upper_lower(Option<&str>) → Requires at least one uppercase and one lowercase letter

require_number(Option<&str>) → Requires at least one digit

require_special_char(Option<&str>) → Requires at least one special character

---
📄 License
MIT
---
💻 Author
Made with ❤ + ☕ by Ali

