use genotype_lang_py_tree::*;
use genotype_parser::tree::descriptor::GTDescriptor;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYDescriptor> for GTDescriptor {
    fn convert(&self, context: &mut PYConvertContext) -> PYDescriptor {
        match self {
            GTDescriptor::Alias(alias) => {
                let definition = alias.convert(context);
                context.hoist(definition);
                // [TODO]
                PYDescriptor::Reference(PYReference::new(alias.name.convert(context), false))
            }

            GTDescriptor::Array(array) => PYDescriptor::List(Box::new(array.convert(context))),

            GTDescriptor::InlineImport(import) => {
                // [TODO] Hoist to imports instead
                // PYDescriptor::InlineImport(import.convert(resolve, hoist))
                PYDescriptor::Reference(PYReference::new("TODO".into(), false))
            }

            GTDescriptor::Literal(literal) => PYDescriptor::Literal(literal.convert(context)),

            GTDescriptor::Object(object) => {
                let class = object.convert(context);
                let reference = PYReference::new(class.name.clone(), true);
                context.hoist(PYDefinition::Class(class));
                PYDescriptor::Reference(reference)
            }

            GTDescriptor::Primitive(primitive) => {
                PYDescriptor::Primitive(primitive.convert(context))
            }

            GTDescriptor::Reference(name) => PYDescriptor::Reference(name.convert(context)),

            GTDescriptor::Tuple(tuple) => PYDescriptor::Tuple(tuple.convert(context)),

            GTDescriptor::Union(union) => PYDescriptor::Union(union.convert(context)),
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use crate::mock::mock_context;

    use super::*;

    #[test]
    fn test_convert_alias() {
        let (hoisted, context) = mock_context();
        let mut context = context;
        assert_eq!(
            GTDescriptor::Alias(Box::new(GTAlias {
                span: (0, 0).into(),
                doc: None,
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }))
            .convert(&mut context),
            PYReference::new("Name".into(), false).into()
        );
        assert_eq!(
            hoisted.lock().unwrap().clone(),
            vec![PYDefinition::Alias(PYAlias {
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::Boolean),
            }),]
        );
    }

    #[test]
    fn test_convert_array() {
        assert_eq!(
            GTDescriptor::Array(Box::new(GTArray {
                span: (0, 0).into(),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }))
            .convert(&mut PYConvertContext::default()),
            PYDescriptor::List(Box::new(PYList {
                descriptor: PYDescriptor::Primitive(PYPrimitive::Boolean)
            }))
        );
    }

    #[test]
    fn test_convert_inline_import() {
        assert_eq!(
            GTDescriptor::InlineImport(GTInlineImport {
                span: (0, 0).into(),
                path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                name: GTIdentifier::new((0, 0).into(), "Name".into())
            })
            .convert(&mut PYConvertContext::default()),
            // [TODO]
            // PYDescriptor::InlineImport(PYInlineImport {
            //     path: "./path/to/module.ts".into(),
            //     name: "Name".into()
            // })
            PYDescriptor::Reference(PYReference::new("TODO".into(), false))
        );
    }

    #[test]
    fn test_convert_object() {
        let (hoisted, context) = mock_context();
        let mut context = context;
        assert_eq!(
            GTDescriptor::Object(GTObject {
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
            })
            .convert(&mut context),
            PYDescriptor::Reference(PYReference::new("Person".into(), true))
        );
        assert_eq!(
            hoisted.lock().unwrap().clone(),
            vec![PYDefinition::Class(PYClass {
                name: "Person".into(),
                extensions: vec![],
                properties: vec![
                    PYProperty {
                        name: "name".into(),
                        descriptor: PYPrimitive::String.into(),
                        required: true,
                    },
                    PYProperty {
                        name: "age".into(),
                        descriptor: PYPrimitive::Int.into(),
                        required: false,
                    }
                ]
            })]
        );
    }

    #[test]
    fn test_convert_primitive() {
        assert_eq!(
            GTDescriptor::Primitive(GTPrimitive::Boolean((0, 0).into()))
                .convert(&mut PYConvertContext::default()),
            PYDescriptor::Primitive(PYPrimitive::Boolean)
        );
    }

    #[test]
    fn test_convert_reference() {
        assert_eq!(
            GTDescriptor::Reference(GTIdentifier::new((0, 0).into(), "Name".into()).into())
                .convert(&mut PYConvertContext::default()),
            PYReference::new("Name".into(), true).into()
        );
        // [TODO] Depending on context, set forward reference
        assert_eq!(
            GTDescriptor::Reference(GTIdentifier::new((0, 0).into(), "Name".into()).into())
                .convert(&mut PYConvertContext::default()),
            PYReference::new("Name".into(), true).into()
        );
    }

    #[test]
    fn test_convert_tuple() {
        assert_eq!(
            GTDescriptor::Tuple(GTTuple {
                span: (0, 0).into(),
                descriptors: vec![
                    GTPrimitive::Boolean((0, 0).into()).into(),
                    GTPrimitive::String((0, 0).into()).into(),
                ]
            })
            .convert(&mut PYConvertContext::default()),
            PYDescriptor::Tuple(PYTuple {
                descriptors: vec![
                    PYDescriptor::Primitive(PYPrimitive::Boolean),
                    PYDescriptor::Primitive(PYPrimitive::String),
                ]
            })
        );
    }

    #[test]
    fn test_convert_union() {
        assert_eq!(
            GTDescriptor::Union(GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GTPrimitive::Boolean((0, 0).into()).into(),
                    GTPrimitive::String((0, 0).into()).into(),
                ]
            })
            .convert(&mut PYConvertContext::default()),
            PYDescriptor::Union(PYUnion {
                descriptors: vec![
                    PYDescriptor::Primitive(PYPrimitive::Boolean),
                    PYDescriptor::Primitive(PYPrimitive::String),
                ]
            })
        );
    }
}
