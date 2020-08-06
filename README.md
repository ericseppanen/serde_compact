## serde-compact: derive macros for compact Serialize and Deserialize

`serde-compact` provides macros that can derive the `Serialize` and `Deserialize` traits
on structs.  The resulting serialized data has only the member values, and not names.
This can reduce the encoded size of the struct, and make serialization and deserialization
faster.

## Example:
```rust
use serde_compact::{Serialize_compact, Deserialize_compact};

#[derive(Debug, PartialEq, Serialize_compact, Deserialize_compact)]
pub struct Person {
    name: String,
    age: u32,
}

let gg = Person {
    name: "Galileo".to_string(),
    age: 456,
};

let serialized = serde_json::to_string(&gg).unwrap();
assert_eq!(serialized, r#"["Galileo",456]"#);

let deserialized: Person = serde_json::from_str(&serialized).unwrap();
assert_eq!(deserialized, gg);
```

Q: Does this do the same thing as [serde_tuple](https://docs.rs/serde_tuple/latest)?

A: Yes, but with fewer features and more bugs.  You should probably use that instead.
