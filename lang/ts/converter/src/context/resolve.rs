use genotype_parser::{GTIdentifier, GTImport, GTPath};

use super::TSConvertContext;

impl TSConvertContext {
    pub fn resolve_path(&self, path: &GTPath) -> String {
        self.resolve
            .paths
            .get(path)
            .unwrap_or(path)
            .as_str()
            .to_owned()
            + ".ts"
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
