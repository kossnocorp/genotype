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
            // [TODO] Consider using the parent property's doc for that
            doc: None,
            name,
            extensions: self.extensions.iter().map(|e| e.convert(context)).collect(),
            properties: self.properties.iter().map(|p| p.convert(context)).collect(),
        }
        .resolve(context)
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

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
                        attributes: vec![],
                        name: GTKey::new((0, 0).into(), "name".into()),
                        descriptor: GTPrimitive::String((0, 0).into()).into(),
                        required: true,
                    },
                    GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTKey::new((0, 0).into(), "age".into()),
                        descriptor: GTPrimitive::Int((0, 0).into()).into(),
                        required: false,
                    }
                ]
            }
            .convert(&mut PYConvertContext::default()),
            PYClass {
                doc: None,
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
        let mut context = PYConvertContext::default();
        assert_eq!(
            GTObject {
                span: (0, 0).into(),
                name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![]
            }
            .convert(&mut context),
            PYClass {
                doc: None,
                name: "Person".into(),
                extensions: vec![],
                properties: vec![]
            }
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(PYDependency::Runtime, "Model".into())]
        );
    }
}
