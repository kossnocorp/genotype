use std::vec;

use genotype_lang_py_tree::*;
use genotype_parser::{GTIdentifier, GTPath};

use crate::resolve::PYConvertResolve;

pub struct PYConvertContext {
    resolve: PYConvertResolve,
    options: PYOptions,
    imports: Vec<PYImport>,
    definitions: Vec<PYDefinition>,
    defined: Vec<PYIdentifier>,
    hoisting: bool,
    hoist_defined: Vec<PYIdentifier>,
    hoisted: Vec<PYDefinition>,
    dependencies: Vec<(PYDependency, PYIdentifier)>,
}

impl PYContext for PYConvertContext {
    fn is_version(&self, version: PYVersion) -> bool {
        self.options.version == version
    }

    fn import(&mut self, dependency: PYDependency, name: PYIdentifier) {
        let dependency = (dependency, name);
        if !self.dependencies.contains(&dependency) {
            self.dependencies.push(dependency);
        }
    }
}

impl PYConvertContext {
    pub fn new(resolve: PYConvertResolve, options: PYOptions) -> Self {
        Self {
            resolve,
            options,
            imports: vec![],
            definitions: vec![],
            defined: vec![],
            hoisting: false,
            hoist_defined: vec![],
            hoisted: vec![],
            dependencies: vec![],
        }
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
    pub fn as_dependencies(&self) -> Vec<(PYDependency, PYIdentifier)> {
        self.dependencies.clone().into_iter().collect()
    }

    pub fn hoist<HoistFn>(&mut self, mut hoist_fn: HoistFn) -> PYReference
    where
        HoistFn: FnMut(&mut PYConvertContext) -> PYDefinition,
    {
        self.hoisting = true;
        let definition = hoist_fn(self);
        let reference = PYReference::new(definition.name().clone(), true);
        self.hoisted.push(definition);
        self.hoisting = false;
        reference
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
                .find(|import| import.path == dependency.as_path());

            if let Some(import) = import {
                if let PYImportReference::Named(names) = &mut import.reference {
                    names.push(name.into());
                    continue;
                }
            }
            imports.push(PYImport {
                path: dependency.as_path(),
                reference: PYImportReference::Named(vec![name.into()]),
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

    pub fn drain_hoisted(&mut self) -> Vec<PYDefinition> {
        self.defined.extend(self.hoist_defined.drain(..));
        self.hoisted.drain(..).collect()
    }
}

impl Default for PYConvertContext {
    fn default() -> Self {
        Self::new(PYConvertResolve::default(), PYOptions::default())
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
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::Boolean),
            })
        });
        let hoisted = context.drain_hoisted();
        assert_eq!(
            hoisted,
            vec![PYDefinition::Alias(PYAlias {
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::Boolean),
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
        let context = PYConvertContext::new(resolve, PYOptions::default());
        assert_eq!(
            context.is_forward_identifier(
                &"Other".into(),
                &GTIdentifier::new((0, 0).into(), "Name".into())
            ),
            false
        );
    }
}
