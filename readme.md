# Form / Struct Validator

## INFO

This is currently experimental code, the module validator-derive is generated with AI (Junie) and be used for educative purpose. Base inspiration are The Java and Spring annotations for validation. Purpose is to share the validations for Frontend and Backend.

Take a look on The Little Book of Rust Macros (https://lukaswirth.dev/tlborm/).

During the the development, I found also an existing crate: https://crates.io/crates/validator in case you like a productive ready. One general difference I currently see to my approach is that my validator-derive is neutral to the validation implementations and build on a general purpose. All logic is delegated to the validator itself. 

##

Playground

* Creating a way for validating Backend and Frontend with the same validation rules

* Validating a structure

Sample for validating a structure

```rust
let v = Positive;
assert!(v.validate(5).is_ok(), "5 should be valid (> 0)");
```

```rust
#[derive(validator::Validate)]
struct User {
    #[validate(Email)]
    email: String,
    #[validate(Positive)]
    age: i32,
    // ignored (no attribute)
    note: String,
}
```

```shell
cargo install cargo-expand

```

```shell
# For a binary target named `app`
cargo expand --bin app

# For the library (default)
cargo expand --lib

# For tests in the lib crate
cargo expand --tests
```

## Next steps

* Defining a full or partial validation
* ...