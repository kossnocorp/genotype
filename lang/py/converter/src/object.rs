use genotype_lang_py_tree::*;
use genotype_parser::*;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYClass> for GTObject {
    fn convert(&self, context: &mut PYConvertContext) -> PYClass {
        let name = match &self.name {
            GTObjectName::Named(identifier) => identifier.convert(context),
            GTObjectName::Alias(identifier, _) => identifier.convert(context),
            _ => panic!("Invalid object name"),
        };

        PYClass {
            name,
            extensions: self.extensions.iter().map(|e| e.convert(context)).collect(),
            properties: self.properties.iter().map(|p| p.convert(context)).collect(),
        }
        .resolve(&mut context.tree, &context.options)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use genotype_lang_py_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use crate::mock::mock_context;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTObject {
                span: (0, 0).into(),
                name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![
                    GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        name: GTKey::new((0, 0).into(), "name".into()),
                        descriptor: GTPrimitive::String((0, 0).into()).into(),
                        required: true,
                    },
                    GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        name: GTKey::new((0, 0).into(), "age".into()),
                        descriptor: GTPrimitive::Int((0, 0).into()).into(),
                        required: false,
                    }
                ]
            }
            .convert(&mut PYConvertContext::default()),
            PYClass {
                name: "Person".into(),
                extensions: vec![],
                properties: vec![
                    PYProperty {
                        name: "name".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                        required: true,
                    },
                    PYProperty {
                        name: "age".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::Int),
                        required: false,
                    }
                ]
            }
        );
    }

    #[test]
    fn test_convert_resolve() {
        let (_, context) = mock_context();
        let mut context = context;
        assert_eq!(
            GTObject {
                span: (0, 0).into(),
                name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![]
            }
            .convert(&mut context),
            PYClass {
                name: "Person".into(),
                extensions: vec![],
                properties: vec![]
            }
        );
        assert_eq!(
            context.tree.imports,
            HashSet::from_iter(vec![("dataclasses".into(), "dataclass".into())]),
        );
    }
}
