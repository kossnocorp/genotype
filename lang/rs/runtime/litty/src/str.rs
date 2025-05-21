use serde::de;
use std::fmt;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

pub trait LitStr {
    const LIT: &'static str;

    fn lit_serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(Self::LIT)
    }

    fn lit_deserialize<'de, D>(deserializer: D) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
        Self: Sized,
    {
        deserializer.deserialize_str(LitStrVisitor::<Self>(PhantomData))
    }

    fn lit_hash<H: Hasher>(state: &mut H) {
        Self::LIT.to_string().hash(state);
    }

    fn lit_fmt(f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, Self::LIT)
    }
}

struct LitStrVisitor<T>(PhantomData<T>);

impl<'de, T: LitStr> de::Visitor<'de> for LitStrVisitor<T> {
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str(&format!("string {:?}", T::LIT))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == T::LIT {
            Ok(())
        } else {
            Err(E::custom(format!("expected {:?}, got {:?}", T::LIT, v)))
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
    fn test_str_serde() {
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
    fn test_str_hash() {
        let mut hasher = DefaultHasher::new();
        Hello.hash(&mut hasher);
        let hash1 = hasher.finish();

        let mut hasher = DefaultHasher::new();
        Hello.hash(&mut hasher);
        let hash2 = hasher.finish();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_default() {
        let _hello: Hello = Default::default();
    }

    #[test]
    fn test_str_eq() {
        assert!(Hello == Hello);
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", Hello), r#""Hello, world!""#);
    }

    #[test]
    fn test_clone() {
        let _hello = Hello.clone();
    }

    #[derive(Default, PartialEq, Eq, Clone)]
    struct Hello;

    impl LitStr for Hello {
        const LIT: &'static str = "Hello, world!";
    }

    impl Serialize for Hello {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Self::lit_serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for Hello {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Self::lit_deserialize(deserializer)?;
            Ok(Self)
        }
    }

    impl Hash for Hello {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            Self::lit_hash(state)
        }
    }

    impl Debug for Hello {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Self::lit_fmt(f)
        }
    }
}
