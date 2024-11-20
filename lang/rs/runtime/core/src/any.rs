use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub enum Any {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Any>),
    Object(BTreeMap<String, Any>),
}

impl Default for Any {
    fn default() -> Self {
        Any::Null
    }
}

impl PartialEq for Any {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Any::Null, Any::Null) => true,

            (Any::Bool(a), Any::Bool(b)) => a == b,

            (Any::Int(a), Any::Int(b)) => a == b,

            (Any::Float(a), Any::Float(b)) => {
                // Normalize -0.0 to 0.0
                let a = if a == &-0.0 { 0.0 } else { *a };
                let b = if b == &-0.0 { 0.0 } else { *b };

                // Treat NaN as equal to NaN
                if a.is_nan() && b.is_nan() {
                    true
                } else {
                    a == b
                }
            }

            (Any::String(a), Any::String(b)) => a == b,

            (Any::Array(a), Any::Array(b)) => a == b,

            (Any::Object(a), Any::Object(b)) => a == b,

            _ => false,
        }
    }
}

impl Eq for Any {}

impl Hash for Any {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Any::Null => state.write_u8(0),

            Any::Bool(value) => {
                value.hash(state);
            }

            Any::Int(value) => {
                value.hash(state);
            }

            Any::Float(value) => {
                state.write_u8(3);
                let mut bits = value.to_bits();

                // Treat all NaN values the same
                if value.is_nan() {
                    bits = f64::NAN.to_bits();
                } else if bits == (-0.0f64).to_bits() {
                    // Normalize -0.0 to 0.0
                    bits = 0.0f64.to_bits();
                }

                bits.hash(state);
            }

            Any::String(value) => {
                value.hash(state);
            }

            Any::Array(value) => {
                value.hash(state);
            }

            Any::Object(value) => {
                value.hash(state);
            }
        }
    }
}

impl Serialize for Any {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Any::Null => serializer.serialize_none(),

            Any::Bool(value) => serializer.serialize_bool(*value),

            Any::Int(value) => serializer.serialize_i64(*value),

            Any::Float(value) => serializer.serialize_f64(*value),

            Any::String(value) => serializer.serialize_str(value),

            Any::Array(arr) => arr.serialize(serializer),

            Any::Object(obj) => {
                let mut obj_ser = serializer.serialize_map(Some(obj.len()))?;
                for (key, value) in obj {
                    obj_ser.serialize_entry(key, value)?;
                }
                obj_ser.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for Any {
    fn deserialize<D>(deserializer: D) -> Result<Any, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AnyVisitor;

        impl<'de> Visitor<'de> for AnyVisitor {
            type Value = Any;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid JSON value")
            }

            fn visit_none<E>(self) -> Result<Any, E> {
                Ok(Any::Null)
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Any, D::Error>
            where
                D: Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }

            fn visit_unit<E>(self) -> Result<Any, E> {
                Ok(Any::Null)
            }

            fn visit_bool<E>(self, value: bool) -> Result<Any, E> {
                Ok(Any::Bool(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Any, E> {
                Ok(Any::Int(value))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Any, E> {
                Ok(Any::Int(value as i64))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Any, E> {
                Ok(Any::Float(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<Any, E>
            where
                E: serde::de::Error,
            {
                Ok(Any::String(value.to_owned()))
            }

            fn visit_string<E>(self, value: String) -> Result<Any, E> {
                Ok(Any::String(value))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Any, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut array = Vec::new();
                while let Some(item) = seq.next_element()? {
                    array.push(item);
                }
                Ok(Any::Array(array))
            }

            fn visit_map<M>(self, mut map: M) -> Result<Any, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut obj = BTreeMap::new();
                while let Some((key, value)) = map.next_entry()? {
                    obj.insert(key, value);
                }
                Ok(Any::Object(obj))
            }
        }

        deserializer.deserialize_any(AnyVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json;
    use std::{collections::BTreeMap, hash::DefaultHasher};

    #[test]
    fn test_default() {
        let value = Any::default();
        assert_eq!(value, Any::Null);
    }

    #[test]
    fn test_serialize_null() {
        let value = Any::Null;
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "null");
    }

    #[test]
    fn test_serialize_bool() {
        let value = Any::Bool(true);
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "true");
    }

    #[test]
    fn test_serialize_int() {
        let value = Any::Int(42);
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "42");
    }

    #[test]
    fn test_serialize_float() {
        let value = Any::Float(42.5);
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "42.5");
    }

    #[test]
    fn test_serialize_string() {
        let value = Any::String("Hello, World!".into());
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, r#""Hello, World!""#);
    }

    #[test]
    fn test_serialize_array() {
        let value = Any::Array(vec![
            Any::Float(1.0),
            Any::Bool(false),
            Any::String("test".into()),
        ]);
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "[1.0,false,\"test\"]");
    }

    #[test]
    fn test_serialize_object() {
        let value = Any::Object(BTreeMap::from_iter(vec![
            ("number".into(), Any::Float(1.0)),
            ("bool".into(), Any::Bool(false)),
            ("string".into(), Any::String("test".into())),
        ]));
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, r#"{"bool":false,"number":1.0,"string":"test"}"#);
    }

    #[test]
    fn test_deserialize_null() {
        let value: Any = serde_json::from_str("null").unwrap();
        assert_eq!(value, Any::Null);
    }

    #[test]
    fn test_deserialize_bool() {
        let value: Any = serde_json::from_str("true").unwrap();
        assert_eq!(value, Any::Bool(true));
    }

    #[test]
    fn test_deserialize_int() {
        let value: Any = serde_json::from_str("42").unwrap();
        assert_eq!(value, Any::Int(42));
    }

    #[test]
    fn test_deserialize_float() {
        let value: Any = serde_json::from_str("42.5").unwrap();
        assert_eq!(value, Any::Float(42.5));
    }

    #[test]
    fn test_deserialize_string() {
        let value: Any = serde_json::from_str(r#""Hello, World!""#).unwrap();
        assert_eq!(value, Any::String("Hello, World!".into()));
    }

    #[test]
    fn test_deserialize_array() {
        let value: Any = serde_json::from_str(r#"[1.0, false, "test"]"#).unwrap();
        assert_eq!(
            value,
            Any::Array(vec![
                Any::Float(1.0),
                Any::Bool(false),
                Any::String("test".into()),
            ])
        );
    }

    #[test]
    fn test_deserialize_object() {
        let value: Any =
            serde_json::from_str(r#"{ "number": 1.0, "bool": false, "string": "test" }"#).unwrap();
        assert_eq!(
            value,
            Any::Object(BTreeMap::from_iter(vec![
                ("number".into(), Any::Float(1.0)),
                ("bool".into(), Any::Bool(false)),
                ("string".into(), Any::String("test".into())),
            ]))
        );
    }

    #[test]
    fn test_nested() {
        let json = r#"{
            "null": null,
            "bool": true,
            "number": 123.456,
            "string": "text",
            "array": [1, "two", false],
            "object": {
                "nested_number": 789,
                "nested_array": [3, 4, 5]
            }
        }"#;
        let json_any: Any = serde_json::from_str(json).unwrap();
        let custom_any = Any::Object(BTreeMap::from_iter(vec![
            ("null".into(), Any::Null),
            ("bool".into(), Any::Bool(true)),
            ("number".into(), Any::Float(123.456)),
            ("string".into(), Any::String("text".into())),
            (
                "array".into(),
                Any::Array(vec![
                    Any::Int(1),
                    Any::String("two".into()),
                    Any::Bool(false),
                ]),
            ),
            (
                "object".into(),
                Any::Object(BTreeMap::from_iter(vec![
                    ("nested_number".into(), Any::Int(789)),
                    (
                        "nested_array".into(),
                        Any::Array(vec![Any::Int(3), Any::Int(4), Any::Int(5)]),
                    ),
                ])),
            ),
        ]));
        assert_eq!(json_any, custom_any);

        let serialized_any = serde_json::to_string_pretty(&json_any).unwrap();
        let deserialized_any: Any = serde_json::from_str(&serialized_any).unwrap();
        assert_eq!(deserialized_any, json_any);
    }

    #[test]
    fn test_invalid() {
        let result: Result<Any, serde_json::Error> = serde_json::from_str("{invalid json}");
        assert!(result.is_err());
    }

    #[test]
    fn test_hash() {
        let value_a = Any::Int(42);
        let value_b = Any::Int(43);
        let value_null = Any::Null;

        let mut hasher = DefaultHasher::new();
        value_a.hash(&mut hasher);
        let hash_a1 = hasher.finish();

        let mut hasher = DefaultHasher::new();
        value_a.hash(&mut hasher);
        let hash_a2 = hasher.finish();

        assert_eq!(hash_a1, hash_a2);

        let mut hasher = DefaultHasher::new();
        value_b.hash(&mut hasher);
        let hash_b = hasher.finish();

        assert_ne!(hash_a1, hash_b);

        let mut hasher = DefaultHasher::new();
        value_null.hash(&mut hasher);
        let hash_null1 = hasher.finish();

        let mut hasher = DefaultHasher::new();
        value_null.hash(&mut hasher);
        let hash_null2 = hasher.finish();

        assert_eq!(hash_null1, hash_null2);

        #[derive(Hash)]
        struct Nope;

        let mut hasher = DefaultHasher::new();
        Nope.hash(&mut hasher);
        let hash_nope = hasher.finish();

        assert_ne!(hash_null1, hash_nope);
    }

    #[test]
    fn test_float_equality() {
        assert_eq!(Any::Float(0.0), Any::Float(-0.0));
        assert_eq!(Any::Float(f64::NAN), Any::Float(f64::NAN));
        assert_eq!(Any::Float(f64::INFINITY), Any::Float(f64::INFINITY));
        assert_eq!(Any::Float(f64::NEG_INFINITY), Any::Float(f64::NEG_INFINITY),);

        assert_ne!(Any::Float(1.0), Any::Float(2.0));
        assert_ne!(Any::Float(f64::NAN), Any::Float(1.0));
        assert_ne!(Any::Float(f64::INFINITY), Any::Float(f64::NEG_INFINITY),);
    }

    #[test]
    fn test_int_equality() {
        assert_eq!(Any::Int(42), Any::Int(42));
        assert_eq!(Any::Int(-100), Any::Int(-100));

        assert_ne!(Any::Int(42), Any::Int(43));
        assert_ne!(Any::Int(0), Any::Int(1));
    }

    #[test]
    fn test_bool_equality() {
        assert_eq!(Any::Bool(true), Any::Bool(true));
        assert_eq!(Any::Bool(false), Any::Bool(false));

        assert_ne!(Any::Bool(true), Any::Bool(false));
    }

    #[test]
    fn test_string_equality() {
        assert_eq!(Any::String("test".into()), Any::String("test".into()));
        assert_eq!(Any::String("".into()), Any::String("".into()));

        assert_ne!(Any::String("test".into()), Any::String("different".into()));
        assert_ne!(Any::String("Test".into()), Any::String("test".into()));
    }

    #[test]
    fn test_null_equality() {
        assert_eq!(Any::Null, Any::Null);

        assert_ne!(Any::Null, Any::Int(0));
        assert_ne!(Any::Null, Any::Bool(false));
        assert_ne!(Any::Null, Any::String("null".into()));
    }

    #[test]
    fn test_array_equality() {
        assert_eq!(
            Any::Array(vec![Any::Int(1), Any::Int(2)]),
            Any::Array(vec![Any::Int(1), Any::Int(2)])
        );
        assert_eq!(Any::Array(vec![]), Any::Array(vec![]));

        assert_ne!(
            Any::Array(vec![Any::Int(1), Any::Int(2)]),
            Any::Array(vec![Any::Int(2), Any::Int(1)])
        );
        assert_ne!(
            Any::Array(vec![Any::Int(1)]),
            Any::Array(vec![Any::Int(1), Any::Int(2)])
        );
    }

    #[test]
    fn test_object_equality() {
        assert_eq!(
            Any::Object(BTreeMap::from_iter(vec![
                ("key1".into(), Any::Int(1)),
                ("key2".into(), Any::Int(2)),
            ])),
            Any::Object(BTreeMap::from_iter(vec![
                ("key2".into(), Any::Int(2)),
                ("key1".into(), Any::Int(1)),
            ]))
        );
        assert_eq!(Any::Object(BTreeMap::new()), Any::Object(BTreeMap::new()));

        assert_ne!(
            Any::Object(BTreeMap::from_iter(vec![
                ("key1".into(), Any::Int(1)),
                ("key2".into(), Any::Int(2)),
            ])),
            Any::Object(BTreeMap::from_iter(vec![
                ("key1".into(), Any::Int(2)),
                ("key2".into(), Any::Int(1)),
            ]))
        );
        assert_ne!(
            Any::Object(BTreeMap::from_iter(vec![("key1".into(), Any::Int(1)),])),
            Any::Object(BTreeMap::from_iter(vec![
                ("key1".into(), Any::Int(1)),
                ("key2".into(), Any::Int(2)),
            ]))
        );
    }
}
