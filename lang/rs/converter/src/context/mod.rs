use genotype_lang_rs_config::RSLangConfig;
use genotype_lang_rs_tree::*;
use genotype_parser::{GTDefinitionId, GTIdentifier, GTModuleId, GTPath};
use naming::RSContextParent;

use crate::resolve::RSConvertResolve;

pub mod attributing;
pub mod hoisting;
pub mod ids;
pub mod naming;

pub struct RSConvertContext {
    resolve: RSConvertResolve,
    config: RSLangConfig,
    imports: Vec<RSUse>,
    definitions: Vec<RSDefinition>,
    defined: Vec<RSIdentifier>,
    hoisting: bool,
    hoist_defined: Vec<RSIdentifier>,
    hoisted: Vec<RSDefinition>,
    dependencies: Vec<(RSDependency, RSIdentifier)>,
    doc: Option<RSDoc>,
    parents: Vec<RSContextParent>,
    module_id: GTModuleId,
    definition_id: Option<GTDefinitionId>,
    field_attributes: Vec<RSAttribute>,
}

impl RSContext for RSConvertContext {
    fn import(&mut self, dependency: RSDependency, name: RSIdentifier) {
        let dependency = (dependency, name);
        if !self.dependencies.contains(&dependency) {
            self.dependencies.push(dependency);
        }
    }

    fn render_derive(&self) -> String {
        let mut traits = self
            .config
            .derive
            .iter()
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
        Self::new(module_id, Default::default(), Default::default())
    }

    pub fn new(module_id: GTModuleId, resolve: RSConvertResolve, config: RSLangConfig) -> Self {
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

#[cfg(test)]
mod tests {
    use genotype_lang_rs_tree::*;
    use pretty_assertions::assert_eq;

    use super::*;

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
