use litty_macro::{
    DeserializeLiterals, Literals, SerializeLiterals, deserialize_literal, literal,
    serialize_literal,
};
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
#[allow(clippy::approx_constant)]
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
#[allow(clippy::approx_constant)]
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
    pub enum Abc {
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

    assert_eq!(serde_json::to_string_pretty(&Abc::B(B)).unwrap(), r#""b""#);
    assert_eq!(serde_json::from_str::<Abc>(r#""b""#).unwrap(), Abc::B(B));
}

#[test]
fn test_enum_variants() {
    #[derive(Debug, PartialEq, Literals)]
    pub enum Abc {
        #[literal("a")]
        A,
        #[literal("b")]
        B,
        #[literal("c")]
        C,
    }

    assert_eq!(serde_json::to_string_pretty(&Abc::B).unwrap(), r#""b""#);
    assert_eq!(serde_json::from_str::<Abc>(r#""b""#).unwrap(), Abc::B);
}

#[test]
fn test_enum_serialize_literals() {
    #[derive(Debug, PartialEq, SerializeLiterals)]
    pub enum Abc {
        #[literal("a")]
        A,
        #[literal("b")]
        B,
    }

    assert_eq!(serde_json::to_string_pretty(&Abc::A).unwrap(), r#""a""#);
    assert_eq!(serde_json::to_string_pretty(&Abc::B).unwrap(), r#""b""#);
}

#[test]
fn test_enum_deserialize_literals() {
    #[derive(Debug, PartialEq, DeserializeLiterals)]
    pub enum Abc {
        #[literal("a")]
        A,
        #[literal("b")]
        B,
    }

    assert_eq!(serde_json::from_str::<Abc>(r#""b""#).unwrap(), Abc::B);
}

#[test]
fn test_literal_fields() {
    #[derive(Debug, PartialEq, Literals)]
    #[literals(ok = true, version = 1)]
    struct SuccessV1 {
        message: String,
    }

    let value = SuccessV1 {
        message: "hello".to_string(),
    };

    assert_eq!(
        serde_json::to_string(&value).unwrap(),
        r#"{"message":"hello","ok":true,"version":1}"#
    );

    let parsed: SuccessV1 =
        serde_json::from_str(r#"{"message":"hello","ok":true,"version":1}"#).unwrap();
    assert_eq!(parsed, value);

    assert!(
        serde_json::from_str::<SuccessV1>(r#"{"message":"hello","ok":false,"version":1}"#).is_err()
    );
}

#[test]
fn test_serialize_literals() {
    #[derive(Debug, PartialEq, SerializeLiterals)]
    #[literals(ok = true, version = 1)]
    struct SuccessV1 {
        message: String,
    }

    let value = SuccessV1 {
        message: "hello".to_string(),
    };

    assert_eq!(
        serde_json::to_string(&value).unwrap(),
        r#"{"message":"hello","ok":true,"version":1}"#
    );
}

#[test]
fn test_deserialize_literals() {
    #[derive(Debug, PartialEq, DeserializeLiterals)]
    #[literals(ok = true, version = 1)]
    struct SuccessV1 {
        message: String,
    }

    let parsed: SuccessV1 =
        serde_json::from_str(r#"{"message":"hello","ok":true,"version":1}"#).unwrap();
    assert_eq!(
        parsed,
        SuccessV1 {
            message: "hello".to_string()
        }
    );

    assert!(
        serde_json::from_str::<SuccessV1>(r#"{"message":"hello","ok":false,"version":1}"#).is_err()
    );
}

#[test]
fn test_literal_fields_with_null() {
    #[derive(Debug, PartialEq, Literals)]
    #[literals(kind = "demo", enabled = true, code = 200, empty = null)]
    struct LiteralBag {
        value: String,
    }

    let value = LiteralBag {
        value: "hello".to_string(),
    };

    assert_eq!(
        serde_json::to_string(&value).unwrap(),
        r#"{"value":"hello","kind":"demo","enabled":true,"code":200,"empty":null}"#
    );

    let parsed: LiteralBag = serde_json::from_str(
        r#"{"value":"hello","kind":"demo","enabled":true,"code":200,"empty":null}"#,
    )
    .unwrap();
    assert_eq!(parsed, value);
}

#[test]
fn test_serialize_literal_struct_attribute() {
    #[serialize_literal("hello")]
    struct Hello;

    assert_eq!(serde_json::to_string_pretty(&Hello).unwrap(), r#""hello""#);
}

#[test]
fn test_deserialize_literal_struct_attribute() {
    #[deserialize_literal("hello")]
    struct Hello;

    assert_eq!(serde_json::from_str::<Hello>(r#""hello""#).unwrap(), Hello);
}
