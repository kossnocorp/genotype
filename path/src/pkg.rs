use crate::prelude::internal::*;

// Package path relative to the working directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtPkgPath(GtCwdRelativePath);

/// Path relative to the target package directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtPkgRelativePath(RelativePathBuf);
