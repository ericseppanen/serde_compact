use serde_struct_compact::{DeserializeCompact, SerializeCompact};
use serde_test::{assert_tokens, Token};

#[test]
fn basic_struct() {
    #[derive(Debug, PartialEq, SerializeCompact, DeserializeCompact)]
    pub struct MyStruct {
        name: String,
        age: u32,
        alive: bool,
        friends: Vec<String>,
    }

    let gg = MyStruct {
        name: "Galileo".to_string(),
        age: 456,
        alive: false,
        friends: vec!["Cigoli".to_owned(), "Castelli".to_owned()],
    };

    assert_tokens(
        &gg,
        &[
            Token::Tuple { len: 4 },
            Token::Str("Galileo"),
            Token::U32(456),
            Token::Bool(false),
            Token::Seq { len: Some(2) },
            Token::Str("Cigoli"),
            Token::Str("Castelli"),
            Token::SeqEnd,
            Token::TupleEnd,
        ],
    );
}
