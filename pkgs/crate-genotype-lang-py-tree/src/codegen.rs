use crate::*;
use genotype_lang_core_tree::*;
use genotype_parser::*;
use miette::{Context, Report};

#[derive(Default)]
pub struct PyCodegen {
    module: PyModule,
    convert_context: PyConvertContext,
}

impl GtlCodegen for PyCodegen {
    type Import = PyImport;

    type Definition = PyDefinition;

    fn register_import(&mut self, import: Self::Import) {
        self.module.imports.push(import);
    }

    fn register_definition(&mut self, definition: Self::Definition) {
        self.module.definitions.push(definition);
    }

    fn inject_descriptor(&mut self, descriptor: GtDescriptor) -> Result<String, Report> {
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
    use genotype_test::*;

    #[test]
    fn test_register_import() {
        let mut codegen = PyCodegen::default();
        codegen.register_import(PyImport::new("dependency".into(), "Name".into()));
        codegen.register_import(PyImport::new("another".into(), "AlsoName".into()));
        assert_snapshot!(
            codegen.render_module().unwrap(),
            @"
        from __future__ import annotations


        from dependency import Name
        from another import AlsoName
        "
        );
    }

    #[test]
    fn test_register_definition() {
        let mut codegen = PyCodegen::default();
        codegen.register_definition(
            PyAlias {
                doc: None,
                name: "Name".into(),
                descriptor: PyAny.into(),
                references: vec![],
            }
            .into(),
        );
        assert_snapshot!(
            codegen.render_module().unwrap(),
            @"
        from __future__ import annotations


        type Name = Any
        "
        );
    }

    #[test]
    fn test_inject_descriptor() {
        let mut codegen = PyCodegen::default();
        let primitive = GtDescriptor::Primitive(Gt::primitive_string());
        let result = codegen.inject_descriptor(primitive).unwrap();
        assert_snapshot!(
            result,
            @"str"
        );
        assert_snapshot!(
            codegen.render_module().unwrap(),
            @"from __future__ import annotations"
        );
    }

    #[test]
    fn test_inject_descriptor_with_hoisted() {
        let mut codegen = PyCodegen::default();
        let alias = GtDescriptor::Alias(Box::new(GtAlias {
            id: GtDefinitionId(GtModuleId("module".into()), "Hello".into()),
            span: Default::default(),
            doc: None,
            attributes: vec![],
            name: GtIdentifier::new(Default::default(), "Hello".into()),
            descriptor: Gt::primitive_string().into(),
        }));
        let result = codegen.inject_descriptor(alias).unwrap();
        assert_snapshot!(
            result,
            @"Hello"
        );

        assert_snapshot!(
            codegen.render_module().unwrap(),
            @"
        from __future__ import annotations


        type Hello = str
        "
        );
    }

    #[test]
    fn test_inject_descriptor_with_imports() {
        let mut codegen = PyCodegen::default();
        let alias = GtDescriptor::Alias(Box::new(GtAlias {
            id: GtDefinitionId(GtModuleId("module".into()), "Hello".into()),
            span: Default::default(),
            doc: None,
            attributes: vec![],
            name: GtIdentifier::new(Default::default(), "Hello".into()),
            descriptor: Gt::literal_string("hello").into(),
        }));
        let result = codegen.inject_descriptor(alias).unwrap();
        assert_snapshot!(
            result,
            @"Hello"
        );
        assert_snapshot!(
            codegen.render_module().unwrap(),
            @r#"
        from __future__ import annotations


        from typing import Literal


        type Hello = Literal["hello"]
        "#
        );
    }
}
