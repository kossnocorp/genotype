use genotype_lang_ts_tree::{
    definition::TSDefinition, descriptor::TSDescriptor, primitive::TSPrimitive,
    reference::TSReference, union::TSUnion,
};
use genotype_parser::tree::descriptor::GTDescriptor;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSDescriptor> for GTDescriptor {
    fn convert<HoistFn>(&self, resolve: &TSConvertResolve, hoist: &HoistFn) -> TSDescriptor
    where
        HoistFn: Fn(TSDefinition),
    {
        match self {
            GTDescriptor::Alias(alias) => {
                hoist(alias.convert(resolve, hoist));
                TSDescriptor::Reference(TSReference(alias.name.convert(resolve, hoist)))
            }

            GTDescriptor::Array(array) => {
                TSDescriptor::Array(Box::new(array.convert(resolve, hoist)))
            }

            GTDescriptor::InlineImport(import) => {
                TSDescriptor::InlineImport(import.convert(resolve, hoist))
            }

            GTDescriptor::Nullable(nullable) => TSDescriptor::Union(TSUnion {
                descriptors: vec![
                    nullable.convert(resolve, hoist),
                    TSDescriptor::Primitive(TSPrimitive::Null),
                ],
            }),

            GTDescriptor::Object(object) => TSDescriptor::Object(object.convert(resolve, hoist)),

            GTDescriptor::Primitive(primitive) => {
                TSDescriptor::Primitive(primitive.convert(resolve, hoist))
            }

            GTDescriptor::Reference(name) => TSDescriptor::Reference(name.convert(resolve, hoist)),

            GTDescriptor::Tuple(tuple) => TSDescriptor::Tuple(tuple.convert(resolve, hoist)),

            GTDescriptor::Union(union) => TSDescriptor::Union(union.convert(resolve, hoist)),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Mutex;

    use genotype_lang_ts_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use crate::resolve::TSConvertResolve;

    use super::*;

    #[test]
    fn test_convert_alias() {
        let hoisted = Mutex::new(vec![]);
        assert_eq!(
            GTDescriptor::Alias(Box::new(GTAlias {
                doc: None,
                name: "Name".into(),
                descriptor: GTPrimitive::Boolean.into(),
            }))
            .convert(&TSConvertResolve::new(), &|definition| {
                let mut hoisted = hoisted.lock().unwrap();
                hoisted.push(definition);
            }),
            TSDescriptor::Reference("Name".into())
        );
        assert_eq!(
            hoisted.lock().unwrap().clone(),
            vec![TSDefinition::Alias(TSAlias {
                name: "Name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::Boolean),
            }),]
        );
    }

    #[test]
    fn test_convert_array() {
        assert_eq!(
            GTDescriptor::Array(Box::new(GTArray {
                descriptor: GTPrimitive::Boolean.into(),
            }))
            .convert(&TSConvertResolve::new(), &|_| {}),
            TSDescriptor::Array(Box::new(TSArray {
                descriptor: TSDescriptor::Primitive(TSPrimitive::Boolean)
            }))
        );
    }

    #[test]
    fn test_convert_inline_import() {
        assert_eq!(
            GTDescriptor::InlineImport(GTInlineImport {
                path: "./path/to/module".into(),
                name: "Name".into()
            })
            .convert(&TSConvertResolve::new(), &|_| {}),
            TSDescriptor::InlineImport(TSInlineImport {
                path: "./path/to/module.ts".into(),
                name: "Name".into()
            })
        );
    }

    #[test]
    fn test_convert_nullable() {
        assert_eq!(
            GTDescriptor::Nullable(Box::new(GTDescriptor::Primitive(GTPrimitive::Boolean)))
                .convert(&TSConvertResolve::new(), &|_| {}),
            TSDescriptor::Union(TSUnion {
                descriptors: vec![
                    TSDescriptor::Primitive(TSPrimitive::Boolean),
                    TSDescriptor::Primitive(TSPrimitive::Null),
                ]
            })
        );
    }

    #[test]
    fn test_convert_object() {
        assert_eq!(
            GTDescriptor::Object(GTObject {
                extensions: vec![],
                properties: vec![
                    GTProperty {
                        doc: None,
                        name: "name".into(),
                        descriptor: GTPrimitive::String.into(),
                        required: true,
                    },
                    GTProperty {
                        doc: None,
                        name: "age".into(),
                        descriptor: GTPrimitive::Int.into(),
                        required: false,
                    }
                ]
            })
            .convert(&TSConvertResolve::new(), &|_| {}),
            TSDescriptor::Object(TSObject {
                properties: vec![
                    TSProperty {
                        name: "name".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                        required: true,
                    },
                    TSProperty {
                        name: "age".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::Number),
                        required: false,
                    }
                ]
            })
        );
    }

    #[test]
    fn test_convert_primitive() {
        assert_eq!(
            GTDescriptor::Primitive(GTPrimitive::Boolean)
                .convert(&TSConvertResolve::new(), &|_| {}),
            TSDescriptor::Primitive(TSPrimitive::Boolean)
        );
    }

    #[test]
    fn test_convert_reference() {
        assert_eq!(
            GTDescriptor::Reference("Name".into()).convert(&TSConvertResolve::new(), &|_| {}),
            TSDescriptor::Reference("Name".into())
        );
    }

    #[test]
    fn test_convert_tuple() {
        assert_eq!(
            GTDescriptor::Tuple(GTTuple {
                descriptors: vec![GTPrimitive::Boolean.into(), GTPrimitive::String.into(),]
            })
            .convert(&TSConvertResolve::new(), &|_| {}),
            TSDescriptor::Tuple(TSTuple {
                descriptors: vec![
                    TSDescriptor::Primitive(TSPrimitive::Boolean),
                    TSDescriptor::Primitive(TSPrimitive::String),
                ]
            })
        );
    }

    #[test]
    fn test_convert_union() {
        assert_eq!(
            GTDescriptor::Union(GTUnion {
                descriptors: vec![GTPrimitive::Boolean.into(), GTPrimitive::String.into(),]
            })
            .convert(&TSConvertResolve::new(), &|_| {}),
            TSDescriptor::Union(TSUnion {
                descriptors: vec![
                    TSDescriptor::Primitive(TSPrimitive::Boolean),
                    TSDescriptor::Primitive(TSPrimitive::String),
                ]
            })
        );
    }
}
