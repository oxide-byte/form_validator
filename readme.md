# Form / Struct Validator

## INFO

This is currently experimental code, the module validator-derive is generated with AI (Junie) and be used for educative
purpose.

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

* Not only showing first error, but collect in a HashMap ?
* Not only add one validator, but an array of validators ?