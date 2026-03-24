use std::{env, path::PathBuf};

use miette::Result;

use crate::error::GTWError;

/// Workspace path. It holds the absolute canonical path of a file or directory.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct GTWPath {
    /// Absolute canonical path.
    path: PathBuf,
}

impl GTWPath {
    /// Creates a new `GTWPath` from a string. A relative path will resolve to
    /// the passed working directory or the current directory. The path will
    /// be canonicalized.
    pub fn try_new(path_str: &String, cwd: Option<&GTWPath>) -> Result<GTWPath> {
        let path = PathBuf::from(path_str);

        let path = match path.is_absolute() {
            true => path,

            false => match cwd {
                Some(root) => root.path.join(path),
                None => match cwd {
                    Some(root) => root.path.join(path),
                    None => match env::current_dir() {
                        Ok(cwd) => cwd.join(path),
                        Err(_) => return Err(GTWError::ResolvePath(path_str.clone()).into()),
                    },
                },
            },
        };

        let path = path
            .canonicalize()
            .map_err(|_| GTWError::CanonicalizePath(path_str.clone()))?;

        Ok(GTWPath { path })
    }

    pub fn as_path(&self) -> &PathBuf {
        &self.path
    }
}

impl From<GTWPath> for String {
    fn from(path: GTWPath) -> String {
        path.path.display().to_string()
    }
}

impl From<&GTWPath> for String {
    fn from(path: &GTWPath) -> String {
        path.path.display().to_string()
    }
}
