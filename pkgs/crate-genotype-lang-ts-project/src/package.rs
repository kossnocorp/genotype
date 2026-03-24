use crate::prelude::internal::*;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct TSPackage {
    pub types: String,
    // [TODO] Merge with package?
    // pub files: Vec<String>,
}
