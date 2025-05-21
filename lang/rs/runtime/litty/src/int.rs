use serde::de;
use std::fmt;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

pub trait LitInt {
    const LIT: i64;

    fn lit_serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(Self::LIT)
    }

    fn lit_deserialize<'de, D>(deserializer: D) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
        Self: Sized,
    {
        deserializer.deserialize_i64(LitIntVisitor::<Self>(PhantomData))
    }

    fn lit_hash<H: Hasher>(state: &mut H) {
        Self::LIT.hash(state);
    }

    fn lit_fmt(f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::LIT)
    }
}

struct LitIntVisitor<T>(PhantomData<T>);

impl<'de, T: LitInt> de::Visitor<'de> for LitIntVisitor<T> {
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str(&format!("integer {}", T::LIT))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == T::LIT {
            Ok(())
        } else {
            Err(E::custom(format!("expected {}, got {}", T::LIT, v)))
        }
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v as i64 == T::LIT {
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

    #[derive(Default, PartialEq, Eq, Clone)]
    struct FortyTwo;

    impl LitInt for FortyTwo {
        const LIT: i64 = 42;
    }

    impl Serialize for FortyTwo {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Self::lit_serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for FortyTwo {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Self::lit_deserialize(deserializer)?;
            Ok(Self)
        }
    }

    impl Hash for FortyTwo {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            Self::lit_hash(state)
        }
    }

    impl Debug for FortyTwo {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Self::lit_fmt(f)
        }
    }

    #[test]
    fn test_int_serde() {
        assert_eq!(
            serde_json::to_string_pretty(&FortyTwo).unwrap(),
            "42"
        );
        assert_eq!(
            serde_json::from_str::<FortyTwo>("42").unwrap(),
            FortyTwo
        );
    }

    #[test]
    fn test_int_hash() {
        let mut hasher = DefaultHasher::new();
        FortyTwo.hash(&mut hasher);
        let hash1 = hasher.finish();

        let mut hasher = DefaultHasher::new();
        FortyTwo.hash(&mut hasher);
        let hash2 = hasher.finish();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_default() {
        let _forty_two: FortyTwo = Default::default();
    }

    #[test]
    fn test_int_eq() {
        assert!(FortyTwo == FortyTwo);
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", FortyTwo), "42");
    }

    #[test]
    fn test_clone() {
        let _forty_two = FortyTwo.clone();
    }
}
