use crate::*;
use genotype_lang_core_tree::*;
use genotype_parser::*;
use miette::Result;

#[derive(Default)]
pub struct TsCodegen {
    module: TsModule,
    convert_context: TsConvertContext,
}

impl GtlCodegen for TsCodegen {
    type Import = TsImport;

    type Definition = TsDefinition;

    fn register_import(&mut self, import: Self::Import) {
        self.module.imports.push(import);
    }

    fn register_definition(&mut self, definition: Self::Definition) {
        self.module.definitions.push(definition);
    }

    fn inject_descriptor(&mut self, descriptor: GtDescriptor) -> Result<String> {
        let descriptor = descriptor.convert(&mut self.convert_context);

        let definitions = self.convert_context.drain_hoisted();
        self.module.definitions.extend(definitions);

        descriptor.render(Default::default(), &mut Default::default())
    }

    fn render_module(&self) -> Result<String> {
        self.module
            .render(Default::default(), &mut Default::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;

    #[test]
    fn test_register_import() {
        let mut codegen = TsCodegen::default();
        codegen.register_import(TsImport::new(
            TsDependencyIdent::Local("dependency".into()),
            "Name".into(),
        ));
        codegen.register_import(TsImport::new(
            TsDependencyIdent::Local("another".into()),
            "AlsoName".into(),
        ));
        assert_snapshot!(
            codegen.render_module().unwrap(),
            @r#"
        import { Name } from "dependency";
        import { AlsoName } from "another";
        "#
        );
    }

    #[test]
    fn test_register_definition() {
        let mut codegen = TsCodegen::default();
        codegen.register_definition(
            TsAlias {
                doc: None,
                name: "Name".into(),
                generics: vec![],
                descriptor: TsAny.into(),
            }
            .into(),
        );
        assert_snapshot!(
            codegen.render_module().unwrap(),
            @"export type Name = any;"
        );
    }

    #[test]
    fn test_inject_descriptor() {
        let mut codegen = TsCodegen::default();
        let primitive = GtDescriptor::Primitive(Gt::primitive_string());
        let result = codegen.inject_descriptor(primitive).unwrap();
        assert_snapshot!(
            result,
            @"string"
        );
        assert_snapshot!(
            codegen.render_module().unwrap(),
            @""
        );
    }

    #[test]
    fn test_inject_descriptor_with_hoisted() {
        let mut codegen = TsCodegen::default();
        let alias = GtDescriptor::Alias(Box::new(GtAlias {
            id: GtDefinitionId("module".into(), "Hello".into()),
            span: Default::default(),
            doc: None,
            attributes: vec![],
            name: GtIdentifier::new(Default::default(), "Hello".into()),
            generics: vec![],
            descriptor: Gt::primitive_string().into(),
        }));
        let result = codegen.inject_descriptor(alias).unwrap();
        assert_snapshot!(
            result,
            @"Hello"
        );
        assert_snapshot!(
            codegen.render_module().unwrap(),
            @"export type Hello = string;"
        );
    }
}
