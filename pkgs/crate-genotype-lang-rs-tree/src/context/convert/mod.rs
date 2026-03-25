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

mod config;
pub use config::*;

#[derive(Default)]
pub struct RsConvertContext {
    resolve: RsConvertResolve,
    config: RsConfigLang,
    imports: Vec<RsUse>,
    definitions: Vec<RsDefinition>,
    defined: Vec<RsIdentifier>,
    hoisting: bool,
    hoist_defined: Vec<RsIdentifier>,
    hoisted: Vec<RsDefinition>,
    dependencies: Vec<(RsDependencyIdent, RsIdentifier)>,
    doc: Option<RsDoc>,
    parents: Vec<RsContextParent>,
    module_id: GtModuleId,
    definition_id: Option<GtDefinitionId>,
    field_attributes: Vec<RsAttribute>,
    dependencies_config: HashMap<String, String>,
}

impl RsConvertContextMockable for RsConvertContext {
    fn render_derive(
        &self,
        mode: RsContextRenderDeriveTypeMode,
        serde_mode: RsContextRenderDeriveSerdeMode,
    ) -> String {
        let mut traits = self
            .config
            .derive
            .iter()
            .filter(|f| mode != RsContextRenderDeriveTypeMode::UnionEnum || *f != "Default")
            .map(|derive| derive.as_str())
            .collect::<Vec<&str>>();

        // All types need to have serialize and deserialize derive macros.
        match serde_mode {
            RsContextRenderDeriveSerdeMode::Serde => {
                traits.push("Serialize");
                traits.push("Deserialize");
            }

            RsContextRenderDeriveSerdeMode::Litty => {
                // Literals combines SerializeLiterals and DeserializeLiterals.
                traits.push("Literals");
            }
        }
        let traits = traits.join(", ");

        format!("derive({traits})")
    }
}

impl RsConvertContext {
    pub fn empty(module_id: GtModuleId) -> Self {
        Self::new(
            module_id,
            Default::default(),
            Default::default(),
            Default::default(),
        )
    }

    pub fn new(
        module_id: GtModuleId,
        resolve: RsConvertResolve,
        config: RsConfigLang,
        dependencies_config: HashMap<String, String>,
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
            dependencies_config,
        }
    }

    pub fn provide_doc(&mut self, doc: Option<RsDoc>) {
        self.doc = doc;
    }

    pub fn consume_doc(&mut self) -> Option<RsDoc> {
        self.doc.take()
    }

    pub fn resolve_identifier(&self, identifier: &GtIdentifier) -> String {
        self.resolve
            .identifiers
            .get(identifier)
            .unwrap_or(identifier)
            .1
            .to_string()
    }

    pub fn resolve_path(&self, path: &GtPath) -> String {
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
    pub fn as_dependencies(&self) -> Vec<(RsDependencyIdent, RsIdentifier)> {
        self.dependencies.clone().into_iter().collect()
    }

    pub fn push_defined(&mut self, identifier: &RsIdentifier) {
        if self.hoisting {
            self.hoist_defined.push(identifier.clone());
        } else {
            self.defined.push(identifier.clone());
        }
    }

    pub fn push_import(&mut self, import: RsUse) {
        self.imports.push(import);
    }

    pub fn drain_imports(&mut self) -> Vec<RsUse> {
        let mut imports: Vec<_> = self.imports.drain(..).collect();

        let dependencies = self.dependencies.drain(..);
        for (dependency, name) in dependencies {
            let import = imports
                .iter_mut()
                .find(|import| import.dependency == dependency);

            if let Some(import) = import {
                if let RsUseReference::Named(names) = &mut import.reference {
                    names.push(name.into());
                    continue;
                }
            }
            imports.push(RsUse {
                reference: RsUseReference::Named(vec![name.into()]),
                dependency,
            });
        }

        imports
    }

    pub fn push_definition(&mut self, definition: RsDefinition) {
        self.definitions.push(definition);
        let hoisted_definitions = self.drain_hoisted();
        self.definitions.extend(hoisted_definitions);
    }

    pub fn drain_definitions(&mut self) -> Vec<RsDefinition> {
        self.definitions.drain(..).collect()
    }
}

impl GtlConvertContext for RsConvertContext {
    type DependencyIdent = RsDependencyIdent;

    type DependencyRef = RsIdentifier;

    fn add_import(self: &mut Self, ident: Self::DependencyIdent, r#ref: Self::DependencyRef) {
        let dependency = (ident, r#ref);
        if !self.dependencies.contains(&dependency) {
            self.dependencies.push(dependency);
        }
    }
}

impl RsConvertContextConstraint for RsConvertContext {}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_hoist() {
        let mut context = RsConvertContext::empty("module".into());
        let _ = context.hoist(|_| {
            Ok((
                RsDefinition::Alias(RsAlias {
                    id: GtDefinitionId("module".into(), "Name".into()),
                    doc: None,
                    name: "Name".into(),
                    descriptor: RsDescriptor::Primitive(RsPrimitive::Boolean),
                }),
                (0, 0).into(),
            ))
        });
        let hoisted = context.drain_hoisted();
        assert_eq!(
            hoisted,
            vec![RsDefinition::Alias(RsAlias {
                id: GtDefinitionId("module".into(), "Name".into()),
                doc: None,
                name: "Name".into(),
                descriptor: RsDescriptor::Primitive(RsPrimitive::Boolean),
            })],
        );
    }

    #[test]
    fn test_define() {
        let mut context = RsConvertContext::empty("module".into());
        context.push_defined(&"Name".into());
        assert_eq!(context.defined, vec!["Name".into()]);
    }
}
