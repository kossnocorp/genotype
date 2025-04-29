use crate::prelude::internal::*;

impl TSConvertContext {
    pub fn resolve_path(&self, path: &GTPath) -> String {
        // [TODO] Refactor `resolve_path` between Python, Rust and TypeScript
        if let Some((package_path, inner_path)) = path.package_path() {
            if let Some(dependency) = self.dependencies_config.get(&package_path) {
                match inner_path {
                    Some(inner_path) => format!("{dependency}/{inner_path}"),
                    None => dependency.to_owned(),
                }
            } else {
                path.source_str().to_owned()
            }
        } else {
            self.resolve
                .paths
                .get(path)
                .unwrap_or(path)
                .source_str()
                .to_owned()
                + ".ts"
        }
    }

    pub fn resolve_glob(&self, import: &GTImport) -> String {
        // [TODO] Add errors to TS convert!
        self.resolve.globs.get(&import.path).unwrap().clone()
    }

    pub fn resolve_identifier(&self, identifier: &GTIdentifier) -> String {
        self.resolve
            .identifiers
            .get(identifier)
            .unwrap_or(identifier)
            .1
            .clone()
    }
}
