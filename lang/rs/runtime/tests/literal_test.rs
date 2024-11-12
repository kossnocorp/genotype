use genotype_runtime::literal;
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};

#[test]
fn test_str() {
    #[derive(PartialEq, Debug)]
    #[literal("Hello, world!")]
    pub struct Hello;

    assert_eq!(
        serde_json::to_string_pretty(&Hello).unwrap(),
        r#""Hello, world!""#
    );
    assert_eq!(
        serde_json::from_str::<Hello>(r#""Hello, world!""#).unwrap(),
        Hello
    );
}

#[test]
fn test_bool() {
    #[derive(PartialEq, Debug)]
    #[literal(true)]
    pub struct Yeah;

    assert_eq!(serde_json::to_string_pretty(&Yeah).unwrap(), "true");
    assert_eq!(serde_json::from_str::<Yeah>("true").unwrap(), Yeah);
}

#[test]
fn test_int() {
    #[derive(PartialEq, Debug)]
    #[literal(1)]
    pub struct V1;

    assert_eq!(serde_json::to_string_pretty(&V1).unwrap(), "1");
    assert_eq!(serde_json::from_str::<V1>("1").unwrap(), V1);
}

#[test]
fn test_enum() {
    #[derive(PartialEq, Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum ABC {
        A(A),
        B(B),
        C(C),
    }

    #[derive(PartialEq, Debug)]
    #[literal("a")]
    pub struct A;

    #[derive(PartialEq, Debug)]
    #[literal("b")]
    pub struct B;

    #[derive(PartialEq, Debug)]
    #[literal("c")]
    pub struct C;

    assert_eq!(serde_json::to_string_pretty(&ABC::B(B)).unwrap(), r#""b""#);
    assert_eq!(serde_json::from_str::<ABC>(r#""b""#).unwrap(), ABC::B(B));
}
