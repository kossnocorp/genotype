use crate::prelude::internal::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Visitor)]
pub enum RsDependencyIdent {
    Local(#[visit] RsPath),
    Runtime,
    Litty,
    Serde,
    OrderedFloat,
    Std(String),
}

pub struct RsDependencyExternal {
    pub name: String,
    pub version: String,
    pub features: Vec<String>,
}

impl RsDependencyIdent {
    pub fn as_path_str(&self) -> String {
        match self {
            Self::Local(path) => path.1.to_string(),
            Self::Runtime => "genotype_runtime".into(),
            Self::Litty => "litty".into(),
            Self::Serde => "serde".into(),
            Self::OrderedFloat => "ordered_float".into(),
            Self::Std(path) => format!("std::{path}"),
        }
    }
}

impl GtlDependencyIdent for RsDependencyIdent {}
