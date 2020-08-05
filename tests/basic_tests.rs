use serde_json;
use serde_struct_compact::SerializeCompact;

#[test]
fn basic_struct() {
    #[derive(SerializeCompact)]
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
    let serialized = serde_json::to_string(&gg).unwrap();
    assert_eq!(serialized, r#"["Galileo",456,false,["Cigoli","Castelli"]]"#);
}
