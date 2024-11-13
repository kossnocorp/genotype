use genotype_runtime::{literal, union};
use pretty_assertions::assert_eq;
use serde::{de::IntoDeserializer, Deserialize};

#[test]
fn test_enum() {
    #[derive(PartialEq, Debug)]
    // #[union]
    pub enum ABC {
        // #[union(literal"a")]
        A,
        // #[union_literal("b")]
        B,
        C(C),
    }

    #[derive(PartialEq, Debug)]
    #[literal("c")]
    pub struct C;

    impl serde::Serialize for ABC {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match self {
                ABC::A => serializer.serialize_str("a"),
                ABC::B => serializer.serialize_str("b"),
                ABC::C(c) => c.serialize(serializer),
                // _ => Err(serde::de::Error::unknown_variant(
                //     &value,
                //     &["a", "b", "???"],
                // )),
            }
        }
    }

    impl<'de> serde::Deserialize<'de> for ABC {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_any(ABCVisitor)
        }
    }

    struct ABCVisitor;

    impl<'de> serde::de::Visitor<'de> for ABCVisitor {
        type Value = ABC;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an ABC variant")
        }

        fn visit_str<E>(self, value: &str) -> Result<ABC, E>
        where
            E: serde::de::Error,
        {
            if value == "a" {
                Ok(ABC::A)
            } else if value == "b" {
                Ok(ABC::B)
            } else {
                match C::deserialize(serde::de::value::StrDeserializer::<E>::new(value)) {
                    Ok(c_value) => Ok(ABC::C(c_value)),
                    Err(_) => Err(serde::de::Error::custom(format!(
                        "Unexpected string: {}",
                        value
                    ))),
                }
            }
        }

        // Handle deserialization when the input is an integer
        fn visit_i64<E>(self, value: i64) -> Result<ABC, E>
        where
            E: serde::de::Error,
        {
            if value == 2 {
                Ok(ABC::B)
            } else {
                Err(serde::de::Error::custom(format!(
                    "Unexpected integer: {}",
                    value
                )))
            }
        }

        fn visit_u64<E>(self, value: u64) -> Result<ABC, E>
        where
            E: serde::de::Error,
        {
            if value == 2 {
                Ok(ABC::B)
            } else {
                Err(serde::de::Error::custom(format!(
                    "Unexpected unsigned integer: {}",
                    value
                )))
            }
        }

        fn visit_map<M>(self, map: M) -> Result<ABC, M::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            let c_value = C::deserialize(serde::de::value::MapAccessDeserializer::new(map))?;
            Ok(ABC::C(c_value))
        }

        fn visit_seq<A>(self, seq: A) -> Result<ABC, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let c_value = C::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))?;
            Ok(ABC::C(c_value))
        }

        fn visit_bool<E>(self, value: bool) -> Result<ABC, E>
        where
            E: serde::de::Error,
        {
            Err(serde::de::Error::custom(format!(
                "Unexpected boolean: {}",
                value
            )))
        }

        fn visit_f64<E>(self, value: f64) -> Result<ABC, E>
        where
            E: serde::de::Error,
        {
            Err(serde::de::Error::custom(format!(
                "Unexpected float: {}",
                value
            )))
        }

        fn visit_unit<E>(self) -> Result<ABC, E>
        where
            E: serde::de::Error,
        {
            Err(serde::de::Error::custom("Unexpected unit value"))
        }

        fn visit_none<E>(self) -> Result<ABC, E>
        where
            E: serde::de::Error,
        {
            Err(serde::de::Error::custom("Unexpected None value"))
        }

        fn visit_some<D>(self, deserializer: D) -> Result<ABC, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Err(serde::de::Error::custom("Unexpected Some value"))
        }
    }

    assert_eq!(serde_json::to_string_pretty(&ABC::C(C)).unwrap(), r#""c""#);
    assert_eq!(serde_json::from_str::<ABC>(r#""c""#).unwrap(), ABC::C(C));

    assert_eq!(serde_json::to_string_pretty(&ABC::B).unwrap(), r#""b""#);
    assert_eq!(serde_json::from_str::<ABC>(r#""b""#).unwrap(), ABC::B);
}
