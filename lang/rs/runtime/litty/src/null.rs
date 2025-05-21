use serde::de;
use std::fmt;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

pub trait LitNull {
    const LIT: () = ();

    fn lit_serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_unit()
    }

    fn lit_deserialize<'de, D>(deserializer: D) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
        Self: Sized,
    {
        deserializer.deserialize_unit(LitNullVisitor::<Self>(PhantomData))
    }

    fn lit_hash<H: Hasher>(state: &mut H) {
        ().hash(state);
    }

    fn lit_fmt(f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "null")
    }
}

struct LitNullVisitor<T>(PhantomData<T>);

impl<'de, T: LitNull> de::Visitor<'de> for LitNullVisitor<T> {
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("null")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde::{Deserialize, Serialize};
    use std::fmt::Debug;
    use std::hash::{DefaultHasher, Hash};

    #[test]
    fn test_null_hash() {
        let mut hasher = DefaultHasher::new();
        Null.hash(&mut hasher);
        let hash1 = hasher.finish();

        let mut hasher = DefaultHasher::new();
        Null.hash(&mut hasher);
        let hash2 = hasher.finish();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_default() {
        let _null: Null = Default::default();
    }

    #[test]
    fn test_null_eq() {
        assert!(Null == Null);
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", Null), "null");
    }

    #[test]
    fn test_clone() {
        let _null = Null.clone();
    }

    #[derive(Default, PartialEq, Eq, Clone)]
    struct Null;

    impl LitNull for Null {}

    impl Serialize for Null {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Self::lit_serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for Null {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Self::lit_deserialize(deserializer)?;
            Ok(Self)
        }
    }

    impl std::hash::Hash for Null {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            Self::lit_hash(state);
        }
    }

    impl Debug for Null {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Self::lit_fmt(f)
        }
    }

    #[test]
    fn test_null_serde() {
        assert_eq!(serde_json::to_string_pretty(&Null).unwrap(), "null");
        assert_eq!(serde_json::from_str::<Null>("null").unwrap(), Null);
    }
}
