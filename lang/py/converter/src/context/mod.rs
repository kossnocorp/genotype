use crate::resolve::PYConvertResolve;
use genotype_lang_core_tree::*;
use genotype_lang_py_config::*;
use genotype_lang_py_tree::*;
use genotype_parser::*;
use indexmap::IndexSet;
use std::collections::HashMap;

mod hoisting;
mod references;

pub struct PYConvertContext {
    resolve: PYConvertResolve,
    config: PYLangConfig,
    dependencies_config: HashMap<String, String>,
    imports: Vec<PYImport>,
    definitions: Vec<PYDefinition>,
    defined: Vec<PYIdentifier>,
    hoisting: bool,
    hoist_defined: Vec<PYIdentifier>,
    hoisted: Vec<PYDefinition>,
    dependencies: Vec<(PYDependencyIdent, PYIdentifier)>,
    doc: Option<PYDoc>,
    references: Vec<IndexSet<PYIdentifier>>,
}

impl PYConvertContext {
    pub fn new(
        resolve: PYConvertResolve,
        config: PYLangConfig,
        dependencies_config: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            resolve,
            config,
            dependencies_config: dependencies_config.unwrap_or_default(),
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

    pub fn provide_doc(&mut self, doc: Option<PYDoc>) {
        self.doc = doc;
    }

    pub fn consume_doc(&mut self) -> Option<PYDoc> {
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
        }
    }

    #[cfg(test)]
    pub fn as_dependencies(&self) -> Vec<(PYDependencyIdent, PYIdentifier)> {
        self.dependencies.clone().into_iter().collect()
    }

    pub fn push_defined(&mut self, identifier: &PYIdentifier) {
        if self.hoisting {
            self.hoist_defined.push(identifier.clone());
        } else {
            self.defined.push(identifier.clone());
        }
    }

    pub fn is_forward_identifier(
        &self,
        identifier: &PYIdentifier,
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

    pub fn push_import(&mut self, import: PYImport) {
        self.imports.push(import);
    }

    pub fn drain_imports(&mut self) -> Vec<PYImport> {
        let mut imports: Vec<_> = self.imports.drain(..).collect();

        let dependencies = self.dependencies.drain(..);
        for (dependency, name) in dependencies {
            let import = imports
                .iter_mut()
                .find(|import| import.dependency == dependency);

            if let Some(import) = import {
                if let PYImportReference::Named(names) = &mut import.reference {
                    names.push(name.into());
                    continue;
                }
            }
            imports.push(PYImport {
                reference: PYImportReference::Named(vec![name.into()]),
                dependency,
            });
        }

        imports
    }

    pub fn push_definition(&mut self, definition: PYDefinition) {
        self.definitions.push(definition);
        let hoisted_definitions = self.drain_hoisted();
        self.definitions.extend(hoisted_definitions);
    }

    pub fn drain_definitions(&mut self) -> Vec<PYDefinition> {
        self.definitions.drain(..).collect()
    }
}

impl PYConvertContextMockable for PYConvertContext {
    fn is_version(&self, version: PYVersion) -> bool {
        self.config.version == version
    }
}

impl GtlConvertContext for PYConvertContext {
    type DependencyIdent = PYDependencyIdent;

    type DependencyRef = PYIdentifier;

    fn add_import(self: &mut Self, ident: Self::DependencyIdent, r#ref: Self::DependencyRef) {
        let dependency = (ident, r#ref);
        if !self.dependencies.contains(&dependency) {
            self.dependencies.push(dependency);
        }
    }
}

impl PYConvertContextConstraint for PYConvertContext {}

impl Default for PYConvertContext {
    fn default() -> Self {
        Self::new(PYConvertResolve::default(), Default::default(), None)
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_tree::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_hoist() {
        let mut context = PYConvertContext::default();
        context.hoist(|_| {
            PYDefinition::Alias(PYAlias {
                doc: None,
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::Boolean),
                references: vec![],
            })
        });
        let hoisted = context.drain_hoisted();
        assert_eq!(
            hoisted,
            vec![PYDefinition::Alias(PYAlias {
                doc: None,
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::Boolean),
                references: vec![],
            })],
        );
    }

    #[test]
    fn test_define() {
        let mut context = PYConvertContext::default();
        context.push_defined(&"Name".into());
        assert_eq!(context.defined, vec!["Name".into()]);
    }

    #[test]
    fn test_is_forward() {
        let mut context = PYConvertContext::default();
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
        let mut resolve = PYConvertResolve::default();
        resolve
            .imported
            .insert(GTIdentifier((0, 0).into(), "Name".into()));
        let context = PYConvertContext::new(resolve, Default::default(), None);
        assert_eq!(
            context.is_forward_identifier(
                &"Other".into(),
                &GTIdentifier::new((0, 0).into(), "Name".into())
            ),
            false
        );
    }
}
