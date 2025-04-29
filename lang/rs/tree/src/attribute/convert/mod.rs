use crate::prelude::internal::*;
use std::collections::HashMap;

mod attributing;
pub use attributing::*;

mod hoisting;
pub use hoisting::*;

mod ids;
pub use ids::*;

mod naming;
pub use naming::*;

pub struct RSConvertContext {
    resolve: RSConvertResolve,
    config: RSLangConfig,
    imports: Vec<RSUse>,
    definitions: Vec<RSDefinition>,
    defined: Vec<RSIdentifier>,
    hoisting: bool,
    hoist_defined: Vec<RSIdentifier>,
    hoisted: Vec<RSDefinition>,
    dependencies: Vec<(RSDependencyIdent, RSIdentifier)>,
    doc: Option<RSDoc>,
    parents: Vec<RSContextParent>,
    module_id: GTModuleId,
    definition_id: Option<GTDefinitionId>,
    field_attributes: Vec<RSAttribute>,
    dependencies_config: HashMap<String, String>,
}

impl RSConvertContextMockable for RSConvertContext {
    fn render_derive(&self, mode: RSContextRenderDeriveMode) -> String {
        let mut traits = self
            .config
            .derive
            .iter()
            .filter(|f| mode != RSContextRenderDeriveMode::UnionEnum || *f != "Default")
            .map(|derive| derive.as_str())
            .collect::<Vec<&str>>();

        // We always need to derive Serialize and Deserialize
        traits.extend(vec!["Serialize", "Deserialize"]);
        let traits = traits.join(", ");

        format!("derive({traits})")
    }
}

impl RSConvertContext {
    pub fn empty(module_id: GTModuleId) -> Self {
        Self::new(module_id, Default::default(), Default::default(), None)
    }

    pub fn new(
        module_id: GTModuleId,
        resolve: RSConvertResolve,
        config: RSLangConfig,
        dependencies_config: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            module_id,
            definition_id: None,
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
            parents: vec![],
            field_attributes: vec![],
            dependencies_config: dependencies_config.unwrap_or_default(),
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
    pub fn as_dependencies(&self) -> Vec<(RSDependencyIdent, RSIdentifier)> {
        self.dependencies.clone().into_iter().collect()
    }

    pub fn push_defined(&mut self, identifier: &RSIdentifier) {
        if self.hoisting {
            self.hoist_defined.push(identifier.clone());
        } else {
            self.defined.push(identifier.clone());
        }
    }

    pub fn push_import(&mut self, import: RSUse) {
        self.imports.push(import);
    }

    pub fn drain_imports(&mut self) -> Vec<RSUse> {
        let mut imports: Vec<_> = self.imports.drain(..).collect();

        let dependencies = self.dependencies.drain(..);
        for (dependency, name) in dependencies {
            let import = imports
                .iter_mut()
                .find(|import| import.dependency == dependency);

            if let Some(import) = import {
                if let RSUseReference::Named(names) = &mut import.reference {
                    names.push(name.into());
                    continue;
                }
            }
            imports.push(RSUse {
                reference: RSUseReference::Named(vec![name.into()]),
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

impl GtlConvertContext for RSConvertContext {
    type DependencyIdent = RSDependencyIdent;

    type DependencyRef = RSIdentifier;

    fn add_import(self: &mut Self, ident: Self::DependencyIdent, r#ref: Self::DependencyRef) {
        let dependency = (ident, r#ref);
        if !self.dependencies.contains(&dependency) {
            self.dependencies.push(dependency);
        }
    }
}

impl RSConvertContextConstraint for RSConvertContext {}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_hoist() {
        let mut context = RSConvertContext::empty("module".into());
        let _ = context.hoist(|_| {
            Ok((
                RSDefinition::Alias(RSAlias {
                    id: GTDefinitionId("module".into(), "Name".into()),
                    doc: None,
                    name: "Name".into(),
                    descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean),
                }),
                (0, 0).into(),
            ))
        });
        let hoisted = context.drain_hoisted();
        assert_eq!(
            hoisted,
            vec![RSDefinition::Alias(RSAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                doc: None,
                name: "Name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean),
            })],
        );
    }

    #[test]
    fn test_define() {
        let mut context = RSConvertContext::empty("module".into());
        context.push_defined(&"Name".into());
        assert_eq!(context.defined, vec!["Name".into()]);
    }
}
