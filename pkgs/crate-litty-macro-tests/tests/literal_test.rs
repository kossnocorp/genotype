#![allow(deprecated)]

use litty_macro::{
    DeserializeLiterals, Literals, SerializeLiterals, deserialize_literal, literal, serde_literal,
    serde_literals, serialize_literal,
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
fn test_serde_literal_struct_attribute() {
    #[serde_literal("hello")]
    #[derive(Serialize, Deserialize)]
    struct Hello;

    assert_eq!(serde_json::to_string_pretty(&Hello).unwrap(), r#""hello""#);
    assert_eq!(serde_json::from_str::<Hello>(r#""hello""#).unwrap(), Hello);
}

#[test]
fn test_serde_literal_struct_attribute_serialize_only() {
    #[serde_literal("hello")]
    #[derive(Serialize)]
    struct Hello;

    assert_eq!(serde_json::to_string_pretty(&Hello).unwrap(), r#""hello""#);
}

#[test]
fn test_serde_literal_struct_attribute_deserialize_only() {
    #[serde_literal("hello")]
    #[derive(Deserialize)]
    struct Hello;

    assert_eq!(serde_json::from_str::<Hello>(r#""hello""#).unwrap(), Hello);
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
    assert_eq!(Abc::A.as_str(), "a");
    assert_eq!(Abc::B.as_str(), "b");
    assert_eq!(Abc::C.as_str(), "c");
}

#[test]
fn test_serde_literal_enum_as_str() {
    #[serde_literals]
    #[derive(Serialize, Deserialize)]
    enum Status {
        #[literal("ok")]
        Ok,
        #[literal("hello-world")]
        HelloWorld,
    }

    assert_eq!(Status::Ok.as_str(), "ok");
    assert_eq!(Status::HelloWorld.as_str(), "hello-world");
    assert_eq!(Status::Ok.as_ref(), "ok");
    assert_eq!(AsRef::<str>::as_ref(&Status::HelloWorld), "hello-world");
}

#[test]
#[allow(clippy::approx_constant)]
fn test_serde_literal_enum_primitive_accessors() {
    #[serde_literals]
    #[derive(Serialize, Deserialize)]
    enum Status {
        #[literal("ok")]
        Ok,
    }

    #[serde_literals]
    #[derive(Serialize, Deserialize)]
    enum Toggle {
        #[literal(true)]
        On,
        #[literal(false)]
        Off,
    }

    #[serde_literals]
    #[derive(Serialize, Deserialize)]
    enum Version {
        #[literal(1)]
        V1,
        #[literal(2)]
        V2,
    }

    #[serde_literals]
    #[derive(Serialize, Deserialize)]
    enum Ratio {
        #[literal(3.14)]
        Pi,
        #[literal(2.71)]
        E,
    }

    const STATUS: &str = Status::Ok.as_str();
    const TOGGLE: bool = Toggle::On.as_bool();
    const VERSION: i64 = Version::V2.as_i64();
    const RATIO: f64 = Ratio::Pi.as_f64();

    assert_eq!(STATUS, "ok");
    assert_eq!(TOGGLE, Toggle::On.as_bool());
    assert_eq!(Toggle::Off.as_bool(), false);
    assert_eq!(VERSION, 2);
    assert_eq!(RATIO, 3.14);
    assert_eq!(Ratio::E.as_f64(), 2.71);
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
    assert_eq!(Abc::A.as_str(), "a");
    assert_eq!(Abc::B.as_str(), "b");
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
    assert_eq!(Abc::A.as_str(), "a");
    assert_eq!(Abc::B.as_str(), "b");
}

#[test]
fn test_serde_literal_enum_as_str_with_single_serde_mode() {
    #[serde_literals]
    #[derive(Serialize)]
    enum SerializeOnly {
        #[literal("serialize")]
        Value,
    }

    #[serde_literals]
    #[derive(Deserialize)]
    enum DeserializeOnly {
        #[literal("deserialize")]
        Value,
    }

    assert_eq!(SerializeOnly::Value.as_str(), "serialize");
    assert_eq!(DeserializeOnly::Value.as_str(), "deserialize");
}

#[test]
fn test_serde_literal_enum_as_str_with_const_generic() {
    #[serde_literals]
    #[derive(Serialize, Deserialize)]
    enum Status<const VERSION: usize> {
        #[literal("ok")]
        Ok,
    }

    assert_eq!(Status::<1>::Ok.as_str(), "ok");
}

#[test]
fn test_serde_literals_enum_with_mixed_literal_and_nested_literal_variants() {
    #[serde_literals]
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum ExecutorKind {
        #[literal("cargo")]
        Cargo,
        Node(ExecutorKindNode),
        Python(ExecutorKindPython),
    }

    #[serde_literals]
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum ExecutorKindNode {
        #[literal("pnpm")]
        Pnpm,
        #[literal("npx")]
        Npx,
    }

    #[serde_literals]
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum ExecutorKindPython {
        #[literal("pip")]
        Pip,
        #[literal("uv")]
        Uv,
    }

    assert_eq!(
        serde_json::from_str::<ExecutorKind>(r#""cargo""#).unwrap(),
        ExecutorKind::Cargo,
    );

    assert_eq!(
        serde_json::from_str::<ExecutorKind>(r#""pnpm""#).unwrap(),
        ExecutorKind::Node(ExecutorKindNode::Pnpm),
    );

    assert_eq!(
        serde_json::from_str::<ExecutorKind>(r#""uv""#).unwrap(),
        ExecutorKind::Python(ExecutorKindPython::Uv),
    );

    assert_eq!(
        serde_json::to_string(&ExecutorKind::Cargo).unwrap(),
        r#""cargo""#,
    );

    assert_eq!(
        serde_json::to_string(&ExecutorKind::Node(ExecutorKindNode::Pnpm)).unwrap(),
        r#""pnpm""#,
    );

    assert_eq!(
        serde_json::to_string(&ExecutorKind::Python(ExecutorKindPython::Uv)).unwrap(),
        r#""uv""#,
    );
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
fn test_literal_fields_with_serde_field_attrs() {
    #[serde_literals]
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[literals(kind = "success-response")]
    struct SuccessResponse {
        #[serde(rename = "messageText")]
        message_text: String,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        details: Option<String>,
    }

    let value = SuccessResponse {
        message_text: "hello".to_string(),
        details: None,
    };

    assert_eq!(
        serde_json::to_string(&value).unwrap(),
        r#"{"messageText":"hello","kind":"success-response"}"#
    );

    let parsed: SuccessResponse =
        serde_json::from_str(r#"{"messageText":"hello","kind":"success-response"}"#).unwrap();
    assert_eq!(parsed, value);
}

#[test]
fn test_literal_fields_with_renamed_literal_field() {
    #[serde_literals]
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[literals(request_type("remove-file", rename = "requestType"))]
    struct RemoveFileRequest {
        #[serde(rename = "filePath")]
        file_path: String,
    }

    let value = RemoveFileRequest {
        file_path: "src/main.type".to_string(),
    };

    assert_eq!(
        serde_json::to_string(&value).unwrap(),
        r#"{"filePath":"src/main.type","requestType":"remove-file"}"#
    );

    let parsed: RemoveFileRequest =
        serde_json::from_str(r#"{"filePath":"src/main.type","requestType":"remove-file"}"#)
            .unwrap();
    assert_eq!(parsed, value);
}

#[test]
fn test_serialize_literals_with_serde_field_attrs() {
    #[serde_literals]
    #[derive(Debug, PartialEq, Serialize)]
    #[literals(kind = "success-response")]
    struct SuccessResponse {
        #[serde(rename = "messageText")]
        message_text: String,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        details: Option<String>,
    }

    let value = SuccessResponse {
        message_text: "hello".to_string(),
        details: None,
    };

    assert_eq!(
        serde_json::to_string(&value).unwrap(),
        r#"{"messageText":"hello","kind":"success-response"}"#
    );
}

#[test]
fn test_deserialize_literals_with_serde_field_attrs() {
    #[serde_literals]
    #[derive(Debug, PartialEq, Deserialize)]
    #[literals(kind = "success-response")]
    struct SuccessResponse {
        #[serde(rename = "messageText")]
        message_text: String,

        #[serde(default)]
        details: Option<String>,
    }

    let parsed: SuccessResponse =
        serde_json::from_str(r#"{"messageText":"hello","kind":"success-response"}"#).unwrap();
    assert_eq!(
        parsed,
        SuccessResponse {
            message_text: "hello".to_string(),
            details: None,
        }
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
#[allow(clippy::approx_constant)]
fn test_literal_field_methods() {
    #[serde_literals]
    #[derive(Serialize, Deserialize)]
    #[literals(
        kind = "demo",
        enabled = true,
        code = 200,
        ratio = 3.14,
        empty = null
    )]
    struct LiteralBag<T> {
        value: T,
    }

    let value = LiteralBag { value: "hello" };

    assert_eq!(value.kind(), "demo");
    assert!(value.enabled());
    assert_eq!(value.code(), 200);
    assert_eq!(value.ratio(), 3.14);
    assert_eq!(value.empty(), ());
}

#[test]
fn test_renamed_literal_field_method_uses_rust_name() {
    #[serde_literals]
    #[derive(Serialize, Deserialize)]
    #[literals(request_type("remove-file", rename = "requestType"))]
    struct RemoveFileRequest {}

    assert_eq!(RemoveFileRequest {}.request_type(), "remove-file");
}

#[test]
fn test_literal_field_methods_with_single_serde_mode() {
    #[serde_literals]
    #[derive(Serialize)]
    #[literals(kind = "serialize")]
    struct SerializeOnly {}

    #[serde_literals]
    #[derive(Deserialize)]
    #[literals(kind = "deserialize")]
    struct DeserializeOnly {}

    assert_eq!(SerializeOnly {}.kind(), "serialize");
    assert_eq!(DeserializeOnly {}.kind(), "deserialize");
}

#[test]
fn test_literal_fields_on_unit_struct() {
    #[serde_literals]
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[literals(kind = "unit")]
    struct Unit;

    assert_eq!(serde_json::to_string(&Unit).unwrap(), r#"{"kind":"unit"}"#);
    assert_eq!(
        serde_json::from_str::<Unit>(r#"{"kind":"unit"}"#).unwrap(),
        Unit,
    );
    assert_eq!(Unit.kind(), "unit");
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
