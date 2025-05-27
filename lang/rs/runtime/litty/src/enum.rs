#[cfg(test)]
mod tests {
    use super::*;
    use crate::LitStr;
    use litty_macro::literal;
    use pretty_assertions::assert_eq;
    use serde::{Deserialize, Serialize};
    use std::fmt::Debug;
    use std::hash::{DefaultHasher, Hash, Hasher};

    mod litty {
        pub use crate::*;
    }

    #[test]
    fn test_str_serde() {
        assert_eq!(
            serde_json::to_string_pretty(&Version::V1).unwrap(),
            r#""v1""#
        );
        assert_eq!(
            serde_json::from_str::<Version>(r#""v2""#).unwrap(),
            Version::V2
        );
    }

    #[test]
    fn test_str_hash() {
        let mut hasher = DefaultHasher::new();
        Version::V1.hash(&mut hasher);
        let hash1 = hasher.finish();

        let mut hasher = DefaultHasher::new();
        Version::V1.hash(&mut hasher);
        let hash2 = hasher.finish();

        let mut hasher = DefaultHasher::new();
        Version::V2.hash(&mut hasher);
        let hash3 = hasher.finish();

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_default() {
        let v2: Version = Default::default();
        assert_eq!(v2, Version::V2);
    }

    #[test]
    fn test_str_eq() {
        assert!(Version::V1 == Version::V1);
        assert!(Version::V1 != Version::V2);
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", Version::V1), r#""v1""#);
        assert_eq!(format!("{:?}", Version::V2), r#""v2""#);
        assert_eq!(
            format!("{:?}", Version::Custom(CustomVersion(42))),
            "Custom(CustomVersion(42))"
        );
        assert_eq!(
            format!("{:?}", Version::Strict(CustomVersion(1), 2, 3)),
            "Strict(CustomVersion(1), 2, 3)"
        );
        assert_eq!(format!("{:?}", Version::Unknown), "Unknown");
    }

    #[test]
    fn test_clone() {
        let _v1 = Version::V1.clone();
    }

    #[derive(Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
    #[serde(untagged)]
    enum Version {
        #[serde(
            serialize_with = "<VersionV1Lit as LitStr>::lit_serialize",
            deserialize_with = "<VersionV1Lit as LitStr>::lit_deserialize"
        )]
        V1,
        #[default]
        #[serde(
            serialize_with = "<VersionV2Lit as LitStr>::lit_serialize",
            deserialize_with = "<VersionV2Lit as LitStr>::lit_deserialize"
        )]
        V2,
        Custom(CustomVersion),
        Strict(CustomVersion, u8, u8),
        Literal {
            str: String,
        },
        Unknown,
    }

    impl Debug for Version {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Version::V1 => VersionV1Lit.fmt(f),
                Version::V2 => VersionV2Lit.fmt(f),
                Version::Custom(custom) => {
                    write!(f, "Custom({:?})", custom)
                }
                Version::Strict(a, b, c) => {
                    write!(f, "Strict({:?}, {:?}, {:?})", a, b, c)
                }
                Version::Literal { str } => {
                    write!(f, "Literal {{ str: {:?} }}", str)
                }
                Version::Unknown => {
                    write!(f, "Unknown")
                }
            }
        }
    }

    impl Hash for Version {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            match self {
                Version::V1 => VersionV1Lit.hash(state),
                Version::V2 => VersionV2Lit.hash(state),
                Version::Custom(custom) => custom.hash(state),
                Version::Strict(a, b, c) => {
                    a.hash(state);
                    b.hash(state);
                    c.hash(state);
                }
                Version::Literal { str } => str.hash(state),
                Version::Unknown => {}
            }
        }
    }

    #[literal("v1")]
    struct VersionV1Lit;

    #[literal("v2")]
    struct VersionV2Lit;

    #[derive(Debug, Default, PartialEq, Hash, Eq, Clone, Serialize, Deserialize)]
    struct CustomVersion(u8);
}
