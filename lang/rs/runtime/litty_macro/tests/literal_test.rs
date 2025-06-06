use litty_macro::literal;
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::hash::{DefaultHasher, Hasher};

#[test]
fn test_str() {
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
    #[literal(true)]
    pub struct Yeah;

    assert_eq!(serde_json::to_string_pretty(&Yeah).unwrap(), "true");
    assert_eq!(serde_json::from_str::<Yeah>("true").unwrap(), Yeah);
}

#[test]
fn test_int() {
    #[literal(1)]
    pub struct V1;

    assert_eq!(serde_json::to_string_pretty(&V1).unwrap(), "1");
    assert_eq!(serde_json::from_str::<V1>("1").unwrap(), V1);
}

#[test]
fn test_float() {
    #[literal(3.14159)]
    pub struct Pi;

    assert_eq!(serde_json::to_string_pretty(&Pi).unwrap(), "3.14159");
    assert_eq!(serde_json::from_str::<Pi>("3.14159").unwrap(), Pi);
}

#[test]
fn test_null() {
    #[literal(null)]
    pub struct Null;

    assert_eq!(serde_json::to_string_pretty(&Null).unwrap(), "null");
    assert_eq!(serde_json::from_str::<Null>("null").unwrap(), Null);
}

#[test]
fn test_hash() {
    #[literal("a")]
    pub struct A;

    #[literal("b")]
    pub struct B;

    let mut hasher = DefaultHasher::new();
    A.hash(&mut hasher);
    let a_hash1 = hasher.finish();

    let mut hasher = DefaultHasher::new();
    A.hash(&mut hasher);
    let a_hash2 = hasher.finish();

    assert_eq!(a_hash1, a_hash2);

    let mut hasher = DefaultHasher::new();
    B.hash(&mut hasher);
    let b_hash = hasher.finish();

    assert_ne!(a_hash1, b_hash);
}

#[test]
fn test_default() {
    #[literal("a")]
    pub struct A;
    let _a: A = Default::default();
}

#[test]
fn test_eq() {
    #[literal("a")]
    pub struct A;

    assert!(A == A);
}

#[test]
fn test_debug() {
    #[literal("a")]
    pub struct A;

    assert_eq!(format!("{:?}", A), r#""a""#);

    #[literal(42)]
    pub struct B;

    assert_eq!(format!("{:?}", B), r#"42"#);

    #[literal(3.14159)]
    pub struct C;

    assert_eq!(format!("{:?}", C), r#"3.14159"#);
}

#[test]
fn test_clone() {
    #[literal("a")]
    pub struct A;
    let _a = A.clone();
}

#[test]
fn test_enum_structs() {
    #[derive(PartialEq, Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum ABC {
        A(A),
        B(B),
        C(C),
    }

    #[literal("a")]
    pub struct A;

    #[literal("b")]
    pub struct B;

    #[literal("c")]
    pub struct C;

    assert_eq!(serde_json::to_string_pretty(&ABC::B(B)).unwrap(), r#""b""#);
    assert_eq!(serde_json::from_str::<ABC>(r#""b""#).unwrap(), ABC::B(B));
}

#[test]
fn test_enum_variants() {
    #[literal]
    #[derive(Debug, PartialEq)]
    pub enum ABC {
        #[literal("a")]
        A,
        #[literal("b")]
        B,
        #[literal("c")]
        C,
    }

    assert_eq!(serde_json::to_string_pretty(&ABC::B).unwrap(), r#""b""#);
    assert_eq!(serde_json::from_str::<ABC>(r#""b""#).unwrap(), ABC::B);
}

#[test]
fn test_enum_variants_debug() {
    #[literal]
    #[derive(Debug, PartialEq)]
    pub enum ABC {
        #[literal("a")]
        A,
        B,
        C(String, usize),
        D {
            x: usize,
            y: usize,
        },
    }

    assert_eq!(format!("{:?}", ABC::A), r#""a""#);
    assert_eq!(format!("{:?}", ABC::B), "B");
    assert_eq!(
        format!("{:?}", ABC::C("test".to_string(), 42)),
        "C(\"test\", 42)"
    );
    assert_eq!(format!("{:?}", ABC::D { x: 1, y: 2 }), "D { x: 1, y: 2 }");
}

#[test]
fn test_enum_variants_hash() {
    #[literal]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum ABC {
        #[literal("a")]
        A,
        B,
        C(String, usize),
        D {
            x: usize,
            y: usize,
        },
    }

    let mut hasher = DefaultHasher::new();
    ABC::A.hash(&mut hasher);
    let a_hash1 = hasher.finish();

    let mut hasher = DefaultHasher::new();
    ABC::A.hash(&mut hasher);
    let a_hash2 = hasher.finish();

    assert_eq!(a_hash1, a_hash2);

    let mut hasher = DefaultHasher::new();
    ABC::B.hash(&mut hasher);
    let b_hash = hasher.finish();

    assert_ne!(a_hash1, b_hash);
}
