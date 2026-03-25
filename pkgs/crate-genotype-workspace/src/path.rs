use std::{env, path::PathBuf};

use miette::Result;

use crate::error::GtwError;

/// Workspace path. It holds the absolute canonical path of a file or directory.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct GtwPath {
    /// Absolute canonical path.
    path: PathBuf,
}

impl GtwPath {
    /// Creates a new `GtwPath` from a string. A relative path will resolve to
    /// the passed working directory or the current directory. The path will
    /// be canonicalized.
    pub fn try_new(path_str: &String, cwd: Option<&GtwPath>) -> Result<GtwPath> {
        let path = PathBuf::from(path_str);

        let path = match path.is_absolute() {
            true => path,

            false => match cwd {
                Some(root) => root.path.join(path),
                None => match cwd {
                    Some(root) => root.path.join(path),
                    None => match env::current_dir() {
                        Ok(cwd) => cwd.join(path),
                        Err(_) => return Err(GtwError::ResolvePath(path_str.clone()).into()),
                    },
                },
            },
        };

        let path = path
            .canonicalize()
            .map_err(|_| GtwError::CanonicalizePath(path_str.clone()))?;

        Ok(GtwPath { path })
    }

    pub fn as_path(&self) -> &PathBuf {
        &self.path
    }
}

impl From<GtwPath> for String {
    fn from(path: GtwPath) -> String {
        path.path.display().to_string()
    }
}

impl From<&GtwPath> for String {
    fn from(path: &GtwPath) -> String {
        path.path.display().to_string()
    }
}
