use genotype_lang_ts_tree::{
    definition::TSDefinition, descriptor::TSDescriptor, primitive::TSPrimitive,
    reference::TSReference, union::TSUnion,
};
use genotype_parser::tree::descriptor::GTDescriptor;

use crate::convert::TSConvert;

impl TSConvert<TSDescriptor> for GTDescriptor {
    fn convert<HoistFn>(&self, hoist: &HoistFn) -> TSDescriptor
    where
        HoistFn: Fn(TSDefinition),
    {
        match self {
            GTDescriptor::Alias(alias) => {
                hoist(alias.convert(hoist));
                TSDescriptor::Reference(TSReference::Local(alias.name.convert(hoist)))
            }

            GTDescriptor::Array(array) => TSDescriptor::Array(Box::new(array.convert(hoist))),

            GTDescriptor::InlineImport(import) => TSDescriptor::InlineImport(import.convert(hoist)),

            GTDescriptor::Nullable(nullable) => TSDescriptor::Union(TSUnion {
                descriptors: vec![
                    nullable.convert(hoist),
                    TSDescriptor::Primitive(TSPrimitive::Null),
                ],
            }),

            GTDescriptor::Object(object) => TSDescriptor::Object(object.convert(hoist)),

            GTDescriptor::Primitive(primitive) => TSDescriptor::Primitive(primitive.convert(hoist)),

            GTDescriptor::Reference(name) => TSDescriptor::Reference(name.convert(hoist)),

            GTDescriptor::Tuple(tuple) => TSDescriptor::Tuple(tuple.convert(hoist)),

            GTDescriptor::Union(union) => TSDescriptor::Union(union.convert(hoist)),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Mutex;

    use genotype_lang_ts_tree::{
        alias::TSAlias, array::TSArray, inline_import::TSInlineImport, object::TSObject,
        path::TSPath, property::TSProperty, reference::TSReference, tuple::TSTuple,
    };
    use genotype_parser::tree::{
        alias::GTAlias, array::GTArray, inline_import::GTInlineImport, object::GTObject,
        primitive::GTPrimitive, property::GTProperty, reference::GTReference, tuple::GTTuple,
        GTUnion,
    };
    use pretty_assertions::assert_eq;

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
            .convert(&|definition| {
                let mut hoisted = hoisted.lock().unwrap();
                hoisted.push(definition);
            }),
            TSDescriptor::Reference(TSReference::Local("Name".into()))
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
            .convert(&|_| {}),
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
            .convert(&|_| {}),
            TSDescriptor::InlineImport(TSInlineImport {
                path: TSPath::Unresolved("./path/to/module".into()),
                name: "Name".into()
            })
        );
    }

    #[test]
    fn test_convert_nullable() {
        assert_eq!(
            GTDescriptor::Nullable(Box::new(GTDescriptor::Primitive(GTPrimitive::Boolean)))
                .convert(&|_| {}),
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
            .convert(&|_| {}),
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
            GTDescriptor::Primitive(GTPrimitive::Boolean).convert(&|_| {}),
            TSDescriptor::Primitive(TSPrimitive::Boolean)
        );
    }

    #[test]
    fn test_convert_reference() {
        assert_eq!(
            GTDescriptor::Reference(GTReference::Unresolved("Name".into())).convert(&|_| {}),
            TSDescriptor::Reference(TSReference::Unresolved("Name".into()))
        );
    }

    #[test]
    fn test_convert_tuple() {
        assert_eq!(
            GTDescriptor::Tuple(GTTuple {
                descriptors: vec![GTPrimitive::Boolean.into(), GTPrimitive::String.into(),]
            })
            .convert(&|_| {}),
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
            .convert(&|_| {}),
            TSDescriptor::Union(TSUnion {
                descriptors: vec![
                    TSDescriptor::Primitive(TSPrimitive::Boolean),
                    TSDescriptor::Primitive(TSPrimitive::String),
                ]
            })
        );
    }
}
