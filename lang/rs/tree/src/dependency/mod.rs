use crate::RSPath;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RSDependency {
    Local(RSPath),
    Runtime,
    Serde,
    SerdeJson,
    Std(String),
}

impl RSDependency {
    pub fn as_path(&self) -> String {
        match self {
            Self::Local(path) => path.1.clone(),
            Self::Runtime => "genotype_runtime".into(),
            Self::Serde => "serde".into(),
            Self::SerdeJson => "serde_json".into(),
            Self::Std(path) => format!("std::{path}"),
        }
    }

    pub fn external_str(&self) -> Option<String> {
        match self {
            Self::Runtime => Some(r#"genotype_runtime = "0.1""#.into()),
            Self::Serde => Some(r#"serde = { version = "1", features = ["derive"] }"#.into()),
            Self::SerdeJson => Some(r#"serde_json = "1""#.into()),
            _ => None,
        }
    }
}
