use genotype_lang_core_codegen::*;
use genotype_lang_core_tree::*;
use genotype_lang_py_tree::*;
use genotype_parser::*;
use miette::{Context, Report};

pub mod prelude;

#[derive(Default)]
pub struct PyCodegen {
    module: PYModule,
    convert_context: PYConvertContext,
}

impl PyCodegen {}

impl GtlCodegen for PyCodegen {
    type Import = PYImport;

    type Definition = PYDefinition;

    fn register_import(&mut self, import: Self::Import) {
        self.module.imports.push(import);
    }

    fn register_definition(&mut self, definition: Self::Definition) {
        self.module.definitions.push(definition);
    }

    fn inject_descriptor(&mut self, descriptor: GTDescriptor) -> Result<String, Report> {
        let descriptor = descriptor.convert(&mut self.convert_context);

        let imports = self.convert_context.drain_imports();
        self.module.imports.extend(imports);

        let definitions = self.convert_context.drain_hoisted();
        self.module.definitions.extend(definitions);

        descriptor
            .render(Default::default(), &mut Default::default())
            .wrap_err("Failed to render descriptor")
    }

    fn render_module(&self) -> Result<String, Report> {
        self.module
            .render(Default::default(), &mut Default::default())
            .wrap_err("Failed to render module")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_register_import() {
        let mut codegen = PyCodegen::default();
        codegen.register_import(PYImport::new("dependency".into(), "Name".into()));
        codegen.register_import(PYImport::new("another".into(), "AlsoName".into()));
        assert_eq!(
            codegen.render_module().unwrap(),
            r#"from dependency import Name
from another import AlsoName
"#
        );
    }

    #[test]
    fn test_register_definition() {
        let mut codegen = PyCodegen::default();
        codegen.register_definition(
            PYAlias {
                doc: None,
                name: "Name".into(),
                descriptor: PYAny.into(),
                references: vec![],
            }
            .into(),
        );
        assert_eq!(
            codegen.render_module().unwrap(),
            r#"type Name = Any
"#
        );
    }

    #[test]
    fn test_inject_descriptor() {
        let mut codegen = PyCodegen::default();
        let primitive = GTDescriptor::Primitive(GTPrimitive::String(Default::default()));
        let result = codegen.inject_descriptor(primitive).unwrap();
        assert_eq!(result, "str");
        assert_eq!(codegen.render_module().unwrap(), "\n");
    }

    #[test]
    fn test_inject_descriptor_with_hoisted() {
        let mut codegen = PyCodegen::default();
        let alias = GTDescriptor::Alias(Box::new(GTAlias {
            id: GTDefinitionId(GTModuleId("module".into()), "Hello".into()),
            span: Default::default(),
            doc: None,
            attributes: vec![],
            name: GTIdentifier::new(Default::default(), "Hello".into()),
            descriptor: GTPrimitive::String(Default::default()).into(),
        }));
        let result = codegen.inject_descriptor(alias).unwrap();
        assert_eq!(result, "Hello");
        assert_eq!(
            codegen.render_module().unwrap(),
            r#"type Hello = str
"#
        );
    }

    #[test]
    fn test_inject_descriptor_with_imports() {
        let mut codegen = PyCodegen::default();
        let alias = GTDescriptor::Alias(Box::new(GTAlias {
            id: GTDefinitionId(GTModuleId("module".into()), "Hello".into()),
            span: Default::default(),
            doc: None,
            attributes: vec![],
            name: GTIdentifier::new(Default::default(), "Hello".into()),
            descriptor: GTLiteral::String(Default::default(), "hello".into()).into(),
        }));
        let result = codegen.inject_descriptor(alias).unwrap();
        assert_eq!(result, "Hello");
        assert_eq!(
            codegen.render_module().unwrap(),
            r#"from typing import Literal


type Hello = Literal["hello"]
"#
        );
    }
}
