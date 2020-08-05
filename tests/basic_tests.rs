use serde_struct_compact::{DeserializeCompact, SerializeCompact};
use serde_test::{assert_ser_tokens, assert_tokens, Token};

#[test]
fn basic() {
    #[derive(Debug, PartialEq, SerializeCompact, DeserializeCompact)]
    pub struct Basic {
        name: String,
        age: u32,
        alive: bool,
        friends: Vec<String>,
    }

    let gg = Basic {
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

#[test]
fn basic2() {
    #[derive(Debug, PartialEq, SerializeCompact, DeserializeCompact)]
    pub struct Basic2 {
        unit: (),
        array: [u8; 3],
        opti: Option<i64>,
        optf: Option<f64>,
        tup: (u8, f64),
    }

    let instance = Basic2 {
        unit: (),
        array: [42u8, 43u8, 44u8],
        opti: Some(0x7FFF_0000_FFFF_0000),
        optf: None,
        tup: (45, 1.5e100),
    };

    assert_tokens(
        &instance,
        &[
            Token::Tuple { len: 5 },
            Token::Unit,
            Token::Tuple { len: 3 },
            Token::U8(42),
            Token::U8(43),
            Token::U8(44),
            Token::TupleEnd,
            Token::Some,
            Token::I64(0x7FFF_0000_FFFF_0000),
            Token::None,
            Token::Tuple { len: 2 },
            Token::U8(45),
            Token::F64(1.5e100),
            Token::TupleEnd,
            Token::TupleEnd,
        ],
    );
}

#[test]
fn serialize_by_ref() {
    // This struct can be serialized but not deserialized, because
    // it doesn't own its contents.
    #[derive(Debug, PartialEq, SerializeCompact)]
    pub struct Basic3 {
        strref: &'static str,
        slice: &'static [u8],
    }

    let instance = Basic3 {
        strref: "hello",
        slice: &[45u8, 46u8],
    };

    assert_ser_tokens(
        &instance,
        &[
            Token::Tuple { len: 2 },
            Token::Str("hello"),
            Token::Seq { len: Some(2) },
            Token::U8(45),
            Token::U8(46),
            Token::SeqEnd,
            Token::TupleEnd,
        ],
    );
}
