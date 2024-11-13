use crate::RSPath;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RSDependency {
    Local(RSPath),
    Runtime,
    Serde,
    Typing,
    TypingExtensions,
    Rsdantic,
    Std(RSPath),
}

impl RSDependency {
    pub fn as_path(&self) -> RSPath {
        match self {
            Self::Local(path) => path.clone(),
            Self::Runtime => "genotype".into(),
            Self::Serde => "serde".into(),
            Self::Typing => "typing".into(),
            Self::TypingExtensions => "typing_extensions".into(),
            Self::Rsdantic => "rsdantic".into(),
            Self::Std(path) => RSPath::from("std").join(&path),
        }
    }

    pub fn external_str(&self) -> Option<String> {
        match self {
            Self::Runtime => Some(r#"genotype-runtime = "^0.4""#.into()),
            Self::TypingExtensions => Some(r#"typing-extensions = "^4""#.into()),
            Self::Rsdantic => Some(r#"rsdantic = "^2.9""#.into()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_path() {
        assert_eq!(
            RSDependency::Local("foo".into()).as_path(),
            RSPath::from("foo")
        );
        assert_eq!(RSDependency::Runtime.as_path(), RSPath::from("genotype"));
        assert_eq!(RSDependency::Typing.as_path(), RSPath::from("typing"));
        assert_eq!(
            RSDependency::TypingExtensions.as_path(),
            RSPath::from("typing_extensions")
        );
        assert_eq!(RSDependency::Rsdantic.as_path(), RSPath::from("rsdantic"));
        assert_eq!(
            RSDependency::Std("collections".into()).as_path(),
            RSPath::from("std::collections")
        );
    }
}
