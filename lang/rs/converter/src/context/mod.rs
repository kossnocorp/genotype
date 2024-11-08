use genotype_lang_rs_config::{RSLangConfig, RSVersion};
use genotype_lang_rs_tree::*;
use genotype_parser::{GTIdentifier, GTPath};
use indexmap::IndexSet;

use crate::resolve::RSConvertResolve;

mod hoisting;
mod references;

pub struct RSConvertContext {
    resolve: RSConvertResolve,
    config: RSLangConfig,
    imports: Vec<RSImport>,
    definitions: Vec<RSDefinition>,
    defined: Vec<RSIdentifier>,
    hoisting: bool,
    hoist_defined: Vec<RSIdentifier>,
    hoisted: Vec<RSDefinition>,
    dependencies: Vec<(RSDependency, RSIdentifier)>,
    doc: Option<RSDoc>,
    references: Vec<IndexSet<RSIdentifier>>,
}

impl RSContext for RSConvertContext {
    fn is_version(&self, version: RSVersion) -> bool {
        self.config.version == version
    }

    fn import(&mut self, dependency: RSDependency, name: RSIdentifier) {
        let dependency = (dependency, name);
        if !self.dependencies.contains(&dependency) {
            self.dependencies.push(dependency);
        }
    }
}

impl RSConvertContext {
    pub fn new(resolve: RSConvertResolve, config: RSLangConfig) -> Self {
        Self {
            resolve,
            config,
            imports: vec![],
            definitions: vec![],
            defined: vec![],
            hoisting: false,
            hoist_defined: vec![],
            hoisted: vec![],
            dependencies: vec![],
            doc: None,
            references: vec![],
        }
    }

    pub fn provide_doc(&mut self, doc: Option<RSDoc>) {
        self.doc = doc;
    }

    pub fn consume_doc(&mut self) -> Option<RSDoc> {
        self.doc.take()
    }

    pub fn resolve_identifier(&self, identifier: &GTIdentifier) -> String {
        self.resolve
            .identifiers
            .get(identifier)
            .unwrap_or(identifier)
            .1
            .clone()
    }

    pub fn resolve_path(&self, path: &GTPath) -> String {
        self.resolve
            .paths
            .get(path)
            .unwrap_or(path)
            .as_str()
            .to_owned()
    }

    #[cfg(test)]
    pub fn as_dependencies(&self) -> Vec<(RSDependency, RSIdentifier)> {
        self.dependencies.clone().into_iter().collect()
    }

    pub fn push_defined(&mut self, identifier: &RSIdentifier) {
        if self.hoisting {
            self.hoist_defined.push(identifier.clone());
        } else {
            self.defined.push(identifier.clone());
        }
    }

    pub fn is_forward_identifier(
        &self,
        identifier: &RSIdentifier,
        original: &GTIdentifier,
    ) -> bool {
        let is_defined = self
            .resolve
            .imported
            .iter()
            .find(|identifier| identifier.1 == original.1)
            .is_some()
            || self.defined.contains(identifier)
            || (self.hoisting && self.hoist_defined.contains(identifier));
        !is_defined
    }

    pub fn push_import(&mut self, import: RSImport) {
        self.imports.push(import);
    }

    pub fn drain_imports(&mut self) -> Vec<RSImport> {
        let mut imports: Vec<_> = self.imports.drain(..).collect();

        let dependencies = self.dependencies.drain(..);
        for (dependency, name) in dependencies {
            let import = imports
                .iter_mut()
                .find(|import| import.path == dependency.as_path());

            if let Some(import) = import {
                if let RSImportReference::Named(names) = &mut import.reference {
                    names.push(name.into());
                    continue;
                }
            }
            imports.push(RSImport {
                path: dependency.as_path(),
                reference: RSImportReference::Named(vec![name.into()]),
                dependency,
            });
        }

        imports
    }

    pub fn push_definition(&mut self, definition: RSDefinition) {
        self.definitions.push(definition);
        let hoisted_definitions = self.drain_hoisted();
        self.definitions.extend(hoisted_definitions);
    }

    pub fn drain_definitions(&mut self) -> Vec<RSDefinition> {
        self.definitions.drain(..).collect()
    }
}

impl Default for RSConvertContext {
    fn default() -> Self {
        Self::new(RSConvertResolve::default(), Default::default())
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_tree::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_hoist() {
        let mut context = RSConvertContext::default();
        context.hoist(|_| {
            RSDefinition::Alias(RSAlias {
                doc: None,
                name: "Name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean),
                references: vec![],
            })
        });
        let hoisted = context.drain_hoisted();
        assert_eq!(
            hoisted,
            vec![RSDefinition::Alias(RSAlias {
                doc: None,
                name: "Name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean),
                references: vec![],
            })],
        );
    }

    #[test]
    fn test_define() {
        let mut context = RSConvertContext::default();
        context.push_defined(&"Name".into());
        assert_eq!(context.defined, vec!["Name".into()]);
    }

    #[test]
    fn test_is_forward() {
        let mut context = RSConvertContext::default();
        context.push_defined(&"Name".into());
        assert_eq!(
            context.is_forward_identifier(
                &"Name".into(),
                &GTIdentifier::new((0, 0).into(), "Name".into())
            ),
            false
        );
        assert_eq!(
            context.is_forward_identifier(
                &"Other".into(),
                &GTIdentifier::new((0, 0).into(), "Name".into())
            ),
            true
        );
    }

    #[test]
    fn test_is_forward_resolve() {
        let mut resolve = RSConvertResolve::default();
        resolve
            .imported
            .insert(GTIdentifier((0, 0).into(), "Name".into()));
        let context = RSConvertContext::new(resolve, Default::default());
        assert_eq!(
            context.is_forward_identifier(
                &"Other".into(),
                &GTIdentifier::new((0, 0).into(), "Name".into())
            ),
            false
        );
    }
}
