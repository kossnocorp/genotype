use crate::RSPath;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RSDependency {
    Local(RSPath),
    Runtime,
    Typing,
    TypingExtensions,
    Rsdantic,
}

impl RSDependency {
    pub fn as_path(&self) -> RSPath {
        match self {
            Self::Local(path) => path.clone(),
            Self::Runtime => "genotype".into(),
            Self::Typing => "typing".into(),
            Self::TypingExtensions => "typing_extensions".into(),
            Self::Rsdantic => "rsdantic".into(),
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
