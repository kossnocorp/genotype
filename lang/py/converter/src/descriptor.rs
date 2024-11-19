use genotype_lang_py_tree::*;
use genotype_parser::tree::descriptor::GTDescriptor;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYDescriptor> for GTDescriptor {
    fn convert(&self, context: &mut PYConvertContext) -> PYDescriptor {
        match self {
            GTDescriptor::Alias(alias) => context.hoist(|context| alias.convert(context)).into(),

            GTDescriptor::Array(array) => array.convert(context).into(),

            GTDescriptor::InlineImport(import) => {
                let reference = import.convert(context);
                context.track_reference(&reference);
                reference.into()
            }

            GTDescriptor::Literal(literal) => literal.convert(context).into(),

            GTDescriptor::Object(object) => context
                .hoist(|context| object.convert(context).into())
                .into(),

            GTDescriptor::Primitive(primitive) => primitive.convert(context).into(),

            GTDescriptor::Record(record) => record.convert(context).into(),

            GTDescriptor::Reference(name) => {
                let reference = name.convert(context);
                context.track_reference(&reference);
                reference.into()
            }

            GTDescriptor::Tuple(tuple) => tuple.convert(context).into(),

            GTDescriptor::Union(union) => union.convert(context).into(),

            GTDescriptor::Any(any) => any.convert(context).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert_alias() {
        let mut context = PYConvertContext::default();
        assert_eq!(
            GTDescriptor::Alias(Box::new(GTAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }))
            .convert(&mut context),
            PYReference::new("Name".into(), true).into()
        );
        let hoisted = context.drain_hoisted();
        assert_eq!(
            hoisted,
            vec![PYDefinition::Alias(PYAlias {
                doc: None,
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::Boolean),
                references: vec![],
            })]
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
        let mut context = PYConvertContext::default();
        assert_eq!(
            GTDescriptor::InlineImport(GTInlineImport {
                span: (0, 0).into(),
                path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                name: GTIdentifier::new((0, 0).into(), "Name".into())
            })
            .convert(&mut context),
            PYDescriptor::Reference(PYReference::new("Name".into(), false))
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(PYDependency::Local(".path.to.module".into()), "Name".into()),]
        );
    }

    #[test]
    fn test_convert_object() {
        let mut context = PYConvertContext::default();
        assert_eq!(
            GTDescriptor::Object(GTObject {
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
                ],
            })
            .convert(&mut context),
            PYDescriptor::Reference(PYReference::new("Person".into(), true))
        );
        let hoisted = context.drain_hoisted();
        assert_eq!(
            hoisted,
            vec![PYDefinition::Class(PYClass {
                doc: None,
                name: "Person".into(),
                extensions: vec![],
                properties: vec![
                    PYProperty {
                        doc: None,
                        name: "name".into(),
                        descriptor: PYPrimitive::String.into(),
                        required: true,
                    },
                    PYProperty {
                        doc: None,
                        name: "age".into(),
                        descriptor: PYPrimitive::Int.into(),
                        required: false,
                    }
                ],
                references: vec![],
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
            GTDescriptor::Reference(
                GTReference {
                    span: (0, 0).into(),
                    id: GTReferenceId("module".into(), (0, 0).into()),
                    definition_id: GTReferenceDefinitionId::Resolved(GTDefinitionId(
                        "module".into(),
                        "Name".into()
                    )),
                    identifier: GTIdentifier::new((0, 0).into(), "Name".into())
                }
                .into()
            )
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
                ],
                discriminator: None
            })
        );
    }
}
