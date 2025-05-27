use crate::prelude::internal::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RSDependencyIdent {
    Local(RSPath),
    Runtime,
    Litty,
    Serde,
    Std(String),
}

impl RSDependencyIdent {
    pub fn as_path(&self) -> String {
        match self {
            Self::Local(path) => path.1.clone(),
            Self::Runtime => "genotype_runtime".into(),
            Self::Litty => "litty".into(),
            Self::Serde => "serde".into(),
            Self::Std(path) => format!("std::{path}"),
        }
    }

    pub fn external(&self) -> Option<RSDependencyExternal> {
        match self {
            Self::Runtime => Some(RSDependencyExternal {
                name: "genotype_runtime".into(),
                version: "0.4".into(),
                features: vec![],
            }),
            Self::Litty => Some(RSDependencyExternal {
                name: "litty".into(),
                version: "0.2".into(),
                features: vec![],
            }),
            Self::Serde => Some(RSDependencyExternal {
                name: "serde".into(),
                version: "1".into(),
                features: vec!["derive".into()],
            }),
            _ => None,
        }
    }
}

pub struct RSDependencyExternal {
    pub name: String,
    pub version: String,
    pub features: Vec<String>,
}

impl GtlDependencyIdent for RSDependencyIdent {}
