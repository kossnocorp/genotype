use serde::de;
use std::fmt;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

pub trait LitFloat {
    const LIT: f64;

    fn lit_serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_f64(Self::LIT)
    }

    fn lit_deserialize<'de, D>(deserializer: D) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
        Self: Sized,
    {
        deserializer.deserialize_f64(LitFloatVisitor::<Self>(PhantomData))
    }

    fn lit_hash<H: Hasher>(state: &mut H) {
        // Special handling for floating point values to ensure consistent hashing
        let mut bits = Self::LIT.to_bits();

        // Treat all NaN values the same
        if Self::LIT.is_nan() {
            bits = f64::NAN.to_bits();
        } else if bits == (-0.0f64).to_bits() {
            // Normalize -0.0 to 0.0
            bits = 0.0f64.to_bits();
        }

        bits.hash(state);
    }

    fn lit_fmt(f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::LIT)
    }
}

struct LitFloatVisitor<T>(PhantomData<T>);

impl<'de, T: LitFloat> de::Visitor<'de> for LitFloatVisitor<T> {
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str(&format!("float {}", T::LIT))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        // Handle exact equality for floats
        if (v - T::LIT).abs() < std::f64::EPSILON {
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
    fn test_float_serde() {
        assert_eq!(serde_json::to_string_pretty(&Pi).unwrap(), "3.14159");
        assert_eq!(serde_json::from_str::<Pi>("3.14159").unwrap(), Pi);
    }

    #[test]
    fn test_float_hash() {
        let mut hasher = DefaultHasher::new();
        Pi.hash(&mut hasher);
        let hash1 = hasher.finish();

        let mut hasher = DefaultHasher::new();
        Pi.hash(&mut hasher);
        let hash2 = hasher.finish();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_default() {
        let _pi: Pi = Default::default();
    }

    #[test]
    fn test_float_eq() {
        assert!(Pi == Pi);
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", Pi), "3.14159");
    }

    #[test]
    fn test_clone() {
        let _pi = Pi.clone();
    }

    #[derive(Default, PartialEq, Eq, Clone)]
    struct Pi;

    impl LitFloat for Pi {
        const LIT: f64 = 3.14159;
    }

    impl Serialize for Pi {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Self::lit_serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for Pi {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Self::lit_deserialize(deserializer)?;
            Ok(Self)
        }
    }

    impl Hash for Pi {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            Self::lit_hash(state)
        }
    }

    impl Debug for Pi {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Self::lit_fmt(f)
        }
    }
}
