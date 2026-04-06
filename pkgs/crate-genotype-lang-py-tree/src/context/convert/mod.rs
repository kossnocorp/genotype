use crate::prelude::internal::*;
use indexmap::IndexSet;

mod hoisting;
mod references;

pub struct PyConvertContext {
    resolve: PyConvertResolve,
    config: PyConfig,
    // dependencies_config: HashMap<String, String>,
    imports: Vec<PyImport>,
    definitions: Vec<PyDefinition>,
    defined: Vec<PyIdentifier>,
    hoisting: bool,
    hoist_defined: Vec<PyIdentifier>,
    hoisted: Vec<PyDefinition>,
    doc: Option<PyDoc>,
    references: Vec<IndexSet<PyIdentifier>>,
}

impl PyConvertContext {
    pub fn new(resolve: PyConvertResolve, config: PyConfig) -> Self {
        Self {
            resolve,
            config,
            imports: vec![],
            definitions: vec![],
            defined: vec![],
            hoisting: false,
            hoist_defined: vec![],
            hoisted: vec![],
            doc: None,
            references: vec![],
        }
    }

    pub fn provide_doc(&mut self, doc: Option<PyDoc>) {
        self.doc = doc;
    }

    pub fn consume_doc(&mut self) -> Option<PyDoc> {
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
            if let Some(dependency) = self.config.common.dependencies.get(&package_path) {
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

    pub fn is_version(&self, version: PyVersion) -> bool {
        self.config.lang.version == version
    }

    pub fn push_defined(&mut self, identifier: &PyIdentifier) {
        if self.hoisting {
            self.hoist_defined.push(identifier.clone());
        } else {
            self.defined.push(identifier.clone());
        }
    }

    pub fn is_forward_identifier(
        &self,
        identifier: &PyIdentifier,
        original: &GtIdentifier,
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

    pub fn push_definition(&mut self, definition: PyDefinition) {
        self.definitions.push(definition);
        let hoisted_definitions = self.drain_hoisted();
        self.definitions.extend(hoisted_definitions);
    }

    pub fn drain_definitions(&mut self) -> Vec<PyDefinition> {
        self.definitions.drain(..).collect()
    }
}

impl GtlConvertContext for PyConvertContext {
    type Import = PyImport;

    fn imports(&self) -> &Vec<Self::Import> {
        &self.imports
    }

    fn imports_mut(&mut self) -> &mut Vec<Self::Import> {
        &mut self.imports
    }
}

impl Default for PyConvertContext {
    fn default() -> Self {
        Self::new(PyConvertResolve::default(), Default::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_hoist() {
        let mut context = PyConvertContext::default();
        context.hoist(|_| {
            PyDefinition::Alias(PyAlias {
                doc: None,
                name: "Name".into(),
                descriptor: PyDescriptor::Primitive(PyPrimitive::Boolean),
                references: vec![],
            })
        });
        let hoisted = context.drain_hoisted();
        assert_eq!(
            hoisted,
            vec![PyDefinition::Alias(PyAlias {
                doc: None,
                name: "Name".into(),
                descriptor: PyDescriptor::Primitive(PyPrimitive::Boolean),
                references: vec![],
            })],
        );
    }

    #[test]
    fn test_define() {
        let mut context = PyConvertContext::default();
        context.push_defined(&"Name".into());
        assert_eq!(context.defined, vec!["Name".into()]);
    }

    #[test]
    fn test_is_forward() {
        let mut context = PyConvertContext::default();
        context.push_defined(&"Name".into());
        assert_eq!(
            context.is_forward_identifier(
                &"Name".into(),
                &GtIdentifier::new((0, 0).into(), "Name".into())
            ),
            false
        );
        assert_eq!(
            context.is_forward_identifier(
                &"Other".into(),
                &GtIdentifier::new((0, 0).into(), "Name".into())
            ),
            true
        );
    }

    #[test]
    fn test_is_forward_resolve() {
        let mut resolve = PyConvertResolve::default();
        resolve
            .imported
            .insert(GtIdentifier((0, 0).into(), "Name".into()));
        let context = PyConvertContext::new(resolve, Default::default());
        assert_eq!(
            context.is_forward_identifier(
                &"Other".into(),
                &GtIdentifier::new((0, 0).into(), "Name".into())
            ),
            false
        );
    }
}
