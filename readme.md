# Form / Struct Validator

A tiny, experimental validation library for Rust structs with a derive macro.
Inspired by Java/Spring-style annotations. The derive macro is intentionally
simple and delegates all logic to validator types.

Crates in this workspace:
- validator: core types, errors, and built-in validators
- validator-derive: proc-macro that generates Validate impls
- validator-backend: small demo for Backend
- validator-leptos: small demo for Frontend Leptos

## Quick start

- Build and test: `cargo build` and `cargo test`

### Basic usage
```rust
use validator::prelude::*;
use validator::validators::*;

#[derive(validator::Validate)]
struct User {
    #[validate(Email)]
    email: String,
    #[validate(MinLength(3), MaxLength(10))]
    name: String,
    #[validate(Positive)]
    age: i32,
}

fn main() {
    let u = User { email: "john@example.com".into(), age: 30 };
    u.validate().unwrap(); // short-circuits on first error
}
```

### Collect all errors
`complete_validate()` validates all rules and aggregates errors per field.
It returns `Err(HashMap<String, Vec<ValidationError>>)` when invalid.
```rust
let u = User { email: "invalid@".into(), name: "Mr. Smith".into(), age: 0 };
if let Err(map) = u.complete_validate() {
    // map.get("email"), map.get("age") -> Vec<ValidationError>
}
```

### Build your own validator
You can create your own validators by implementing the `Validator<T>` trait.
Keep them small and focused; the derive macro just wires them to your fields.

- Implement `Validator<T> for YourType` and return `Ok(())` when valid or a `ValidationError` when not.
- Provide `Default` for zero-config validators (used as `#[validate(YourType)]`).
- Optionally provide a constructor like `new(...)` to allow arguments (used as `#[validate(YourType(arg1, arg2))]`).

Example 1: zero-config validator for strings without digits
```rust
use validator::prelude::*;

#[derive(Default)]
pub struct NoDigits;

impl Validator<String> for NoDigits {
    fn validate(&self, value: &String) -> Result<(), ValidationError> {
        if value.chars().any(|c| c.is_ascii_digit()) {
            Err(ValidationError::new("no_digits", "Digits are not allowed"))
        } else {
            Ok(())
        }
    }
}

#[derive(validator::Validate)]
struct Form {
    #[validate(NoDigits)]
    username: String,
}
```

Example 2: configurable validator with arguments
```rust
use validator::prelude::*;
use std::borrow::Cow;

pub struct MyMinLength { pub limit: u32, pub message: Option<Cow<'static, str>> }
impl MyMinLength { pub fn new(limit: u32) -> Self { Self { limit, message: None } } }
impl Default for MyMinLength { fn default() -> Self { Self { limit: 0, message: None } } }

impl Validator<String> for MyMinLength {
    fn validate(&self, value: &String) -> Result<(), ValidationError> {
        if (value.len() as u32) < self.limit {
            let msg = self.message.as_deref().unwrap_or("Too short");
            Err(ValidationError::new("min_length", msg)
                .with_param("limit", self.limit.to_string())
                .with_param("len", value.len().to_string()))
        } else { Ok(()) }
    }
}

#[derive(validator::Validate)]
struct Account {
    #[validate(MyMinLength(3))]
    handle: String,
}
```

Tip: You can also validate standalone values using the `With<V, T>` adapter:
```rust
use validator::prelude::*;
use validator::validate::With;

let value = With::<NoDigits, _>::new("hello".to_string());
value.validate().unwrap();
```

## Notes
- This is for learning and experimentation. For production, consider the
  established `validator` crate on crates.io (https://crates.io/crates/validator).
- Take a look on The Little Book of Rust Macros (https://lukaswirth.dev/tlborm/).
- You can inspect generated code with `cargo expand` if desired.