use genotype_lang_core_codegen::*;
use genotype_lang_core_tree::*;
use genotype_lang_ts_tree::*;
use genotype_parser::*;
use miette::Result;

pub mod prelude;

#[derive(Default)]
pub struct TsCodegen {
    module: TSModule,
    convert_context: TSConvertContext,
}

impl GtlCodegen for TsCodegen {
    type Import = TSImport;

    type Definition = TSDefinition;

    fn register_import(&mut self, import: Self::Import) {
        self.module.imports.push(import);
    }

    fn register_definition(&mut self, definition: Self::Definition) {
        self.module.definitions.push(definition);
    }

    fn inject_descriptor(&mut self, descriptor: GTDescriptor) -> Result<String> {
        let descriptor = descriptor.convert(&mut self.convert_context);

        // [TODO] Drain imports when needed

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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_register_import() {
        let mut codegen = TsCodegen::default();
        codegen.register_import(TSImport::new("dependency".into(), "Name".into()));
        codegen.register_import(TSImport::new("another".into(), "AlsoName".into()));
        assert_eq!(
            codegen.render_module().unwrap(),
            r#"import { Name } from "dependency";
import { AlsoName } from "another";
"#
        );
    }

    #[test]
    fn test_register_definition() {
        let mut codegen = TsCodegen::default();
        codegen.register_definition(
            TSAlias {
                doc: None,
                name: "Name".into(),
                descriptor: TSAny.into(),
            }
            .into(),
        );
        assert_eq!(
            codegen.render_module().unwrap(),
            r#"export type Name = any;
"#
        );
    }

    #[test]
    fn test_inject_descriptor() {
        let mut codegen = TsCodegen::default();
        let primitive = GTDescriptor::Primitive(GTPrimitive::String(Default::default()));
        let result = codegen.inject_descriptor(primitive).unwrap();
        assert_eq!(result, "string");
        assert_eq!(codegen.render_module().unwrap(), "");
    }

    #[test]
    fn test_inject_descriptor_with_hoisted() {
        let mut codegen = TsCodegen::default();
        let alias = GTDescriptor::Alias(Box::new(GTAlias {
            id: GTDefinitionId("module".into(), "Hello".into()),
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
            r#"export type Hello = string;
"#
        );
    }
}
