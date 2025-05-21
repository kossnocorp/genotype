use serde::de;
use std::fmt;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

pub trait LitBool {
    const LIT: bool;

    fn lit_serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bool(Self::LIT)
    }

    fn lit_deserialize<'de, D>(deserializer: D) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
        Self: Sized,
    {
        deserializer.deserialize_bool(LitBoolVisitor::<Self>(PhantomData))
    }

    fn lit_hash<H: Hasher>(state: &mut H) {
        Self::LIT.hash(state);
    }

    fn lit_fmt(f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::LIT)
    }
}

struct LitBoolVisitor<T>(PhantomData<T>);

impl<'de, T: LitBool> de::Visitor<'de> for LitBoolVisitor<T> {
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str(&format!("boolean {}", T::LIT))
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == T::LIT {
            Ok(())
        } else {
            Err(E::custom(format!("expected {}, got {}", T::LIT, v)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde::{Deserialize, Serialize};
    use std::fmt::Debug;
    use std::hash::DefaultHasher;

    #[test]
    fn test_bool_serde() {
        assert_eq!(serde_json::to_string_pretty(&True).unwrap(), "true");
        assert_eq!(serde_json::from_str::<True>("true").unwrap(), True);
    }

    #[test]
    fn test_bool_hash() {
        let mut hasher = DefaultHasher::new();
        True.hash(&mut hasher);
        let hash1 = hasher.finish();

        let mut hasher = DefaultHasher::new();
        True.hash(&mut hasher);
        let hash2 = hasher.finish();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_default() {
        let _true: True = Default::default();
    }

    #[test]
    fn test_bool_eq() {
        assert!(True == True);
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", True), "true");
    }

    #[test]
    fn test_clone() {
        let _true = True.clone();
    }

    #[derive(Default, PartialEq, Eq, Clone)]
    struct True;

    impl LitBool for True {
        const LIT: bool = true;
    }

    impl Serialize for True {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Self::lit_serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for True {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Self::lit_deserialize(deserializer)?;
            Ok(Self)
        }
    }

    impl Hash for True {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            Self::lit_hash(state)
        }
    }

    impl Debug for True {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Self::lit_fmt(f)
        }
    }
}
