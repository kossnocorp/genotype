use std::path::PathBuf;

use miette::Result;

use crate::error::GTWError;

#[derive(Hash, Eq, PartialEq)]
pub struct GTWPath {
    path: PathBuf,
}

impl GTWPath {
    pub fn new(path: &String) -> Result<GTWPath> {
        let absolute_path = PathBuf::from(path)
            .canonicalize()
            .map_err(|_| GTWError::Canonicalize(path.clone()))?;

        Ok(GTWPath {
            path: absolute_path,
        })
    }
}
