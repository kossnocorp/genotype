use crate::prelude::internal::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Visitor)]
pub enum RsDependencyIdent {
    Local(#[visit] RsPath),
    Runtime,
    Litty,
    Serde,
    Std(String),
}

impl RsDependencyIdent {
    pub fn as_path(&self) -> String {
        match self {
            Self::Local(path) => path.1.to_string(),
            Self::Runtime => "genotype_runtime".into(),
            Self::Litty => "litty".into(),
            Self::Serde => "serde".into(),
            Self::Std(path) => format!("std::{path}"),
        }
    }

    pub fn external(&self) -> Option<RsDependencyExternal> {
        match self {
            Self::Runtime => Some(RsDependencyExternal {
                name: "genotype_runtime".into(),
                version: "0.4".into(),
                features: vec![],
            }),
            Self::Litty => Some(RsDependencyExternal {
                name: "litty".into(),
                version: "0.3".into(),
                features: vec![],
            }),
            Self::Serde => Some(RsDependencyExternal {
                name: "serde".into(),
                version: "1".into(),
                features: vec!["derive".into()],
            }),
            _ => None,
        }
    }
}

pub struct RsDependencyExternal {
    pub name: String,
    pub version: String,
    pub features: Vec<String>,
}

impl GtlDependencyIdent for RsDependencyIdent {}
