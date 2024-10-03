use std::{
    hash::Hash,
    path::{Component, Path, PathBuf},
};

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct GTPath(String);

impl GTPath {
    pub fn new(path: &str) -> Self {
        GTPath(normalize(path))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for GTPath {
    fn from(str: &str) -> Self {
        GTPath::new(str)
    }
}

fn normalize(path: &str) -> String {
    let mut result = PathBuf::new();
    let mut components = Path::new(path).components().peekable();

    let mut leading = Vec::new();
    while let Some(&component) = components.peek() {
        match component {
            Component::CurDir | Component::ParentDir => {
                leading.push(component.as_os_str().to_owned());
                components.next();
            }
            _ => break,
        }
    }

    for component in components {
        match component {
            Component::CurDir => {
                // Skip redundant .
            }

            Component::ParentDir => {
                // Pop the last component if possible, else push ..
                if !result.pop() {
                    result.push("..");
                }
            }

            Component::Normal(c) => {
                result.push(c);
            }

            _ => {}
        }
    }

    let mut normalized = PathBuf::new();
    for component in leading {
        normalized.push(component);
    }
    normalized.push(result);

    normalized.as_os_str().to_str().unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_normalize() {
        assert_eq!(GTPath::new("./path/to/../module").as_str(), "./path/module");
        assert_eq!(
            GTPath::new("./path/./to/./module").as_str(),
            "./path/to/module"
        );
        assert_eq!(
            GTPath::new("path/./to/./module/../module").as_str(),
            "path/to/module"
        );
        assert_eq!(
            GTPath::new("./././path/./to/./module/../module").as_str(),
            "./path/to/module"
        );
        assert_eq!(
            GTPath::new("../../../path/./to/./module/../module").as_str(),
            "../../../path/to/module"
        );
    }
}
