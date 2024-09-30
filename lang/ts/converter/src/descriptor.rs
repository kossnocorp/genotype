use genotype_lang_ts_tree::{
    definition::TSDefinition, primitive::TSPrimitive, type_descriptor::TSTypeDescriptor,
    union::TSUnion,
};
use genotype_parser::tree::descriptor::GTDescriptor;

use crate::convert::TSConvert;

impl TSConvert<TSTypeDescriptor> for GTDescriptor {
    fn convert<HoistFn>(&self, hoist: &HoistFn) -> TSTypeDescriptor
    where
        HoistFn: Fn(TSDefinition),
    {
        match self {
            GTDescriptor::Primitive(primitive) => {
                TSTypeDescriptor::Primitive(primitive.convert(hoist))
            }

            GTDescriptor::Name(name) => TSTypeDescriptor::Name(name.convert(hoist)),

            GTDescriptor::Nullable(nullable) => TSTypeDescriptor::Union(TSUnion {
                descriptors: vec![
                    nullable.convert(hoist),
                    TSTypeDescriptor::Primitive(TSPrimitive::Null),
                ],
            }),

            GTDescriptor::Object(object) => {
                TSTypeDescriptor::Object(Box::new(object.convert(hoist)))
            }

            GTDescriptor::Array(array) => TSTypeDescriptor::Array(Box::new(array.convert(hoist))),

            GTDescriptor::Tuple(tuple) => TSTypeDescriptor::Tuple(Box::new(tuple.convert(hoist))),

            GTDescriptor::Alias(alias) => {
                hoist(alias.convert(hoist));
                TSTypeDescriptor::Name(alias.name.convert(hoist))
            }

            GTDescriptor::InlineImport(import) => {
                TSTypeDescriptor::InlineImport(import.convert(hoist))
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Mutex;

    use genotype_lang_ts_tree::{
        alias::TSAlias, array::TSArray, definition, definition_descriptor::TSDefinitionDescriptor,
        inline_import::TSInlineImport, name::TSName, object::TSObject, property::TSProperty,
        tuple::TSTuple,
    };
    use genotype_parser::tree::{
        alias::GTAlias, array::GTArray, inline_import::GTInlineImport, name::GTName,
        object::GTObject, primitive::GTPrimitive, property::GTProperty, tuple::GTTuple,
    };
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert_primitive() {
        assert_eq!(
            GTDescriptor::Primitive(GTPrimitive::Boolean).convert(&|_| {}),
            TSTypeDescriptor::Primitive(TSPrimitive::Boolean)
        );
    }

    #[test]
    fn test_convert_name() {
        assert_eq!(
            GTDescriptor::Name(GTName("Name".to_string())).convert(&|_| {}),
            TSTypeDescriptor::Name(TSName("Name".to_string()))
        );
    }

    #[test]
    fn test_convert_nullable() {
        assert_eq!(
            GTDescriptor::Nullable(Box::new(GTDescriptor::Primitive(GTPrimitive::Boolean)))
                .convert(&|_| {}),
            TSTypeDescriptor::Union(TSUnion {
                descriptors: vec![
                    TSTypeDescriptor::Primitive(TSPrimitive::Boolean),
                    TSTypeDescriptor::Primitive(TSPrimitive::Null),
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
                        name: GTName("name".to_string()),
                        descriptor: GTDescriptor::Primitive(GTPrimitive::String),
                        required: true,
                    },
                    GTProperty {
                        doc: None,
                        name: GTName("age".to_string()),
                        descriptor: GTDescriptor::Primitive(GTPrimitive::Int),
                        required: false,
                    }
                ]
            })
            .convert(&|_| {}),
            TSTypeDescriptor::Object(Box::new(TSObject {
                properties: vec![
                    TSProperty {
                        name: TSName("name".to_string()),
                        descriptor: TSTypeDescriptor::Primitive(TSPrimitive::String),
                        required: true,
                    },
                    TSProperty {
                        name: TSName("age".to_string()),
                        descriptor: TSTypeDescriptor::Primitive(TSPrimitive::Number),
                        required: false,
                    }
                ]
            }))
        );
    }

    #[test]
    fn test_convert_array() {
        assert_eq!(
            GTDescriptor::Array(Box::new(GTArray {
                descriptor: GTDescriptor::Primitive(GTPrimitive::Boolean),
            }))
            .convert(&|_| {}),
            TSTypeDescriptor::Array(Box::new(TSArray {
                descriptor: TSTypeDescriptor::Primitive(TSPrimitive::Boolean)
            }))
        );
    }

    #[test]
    fn test_convert_tuple() {
        assert_eq!(
            GTDescriptor::Tuple(GTTuple {
                descriptors: vec![
                    GTDescriptor::Primitive(GTPrimitive::Boolean),
                    GTDescriptor::Primitive(GTPrimitive::String),
                ]
            })
            .convert(&|_| {}),
            TSTypeDescriptor::Tuple(Box::new(TSTuple {
                descriptors: vec![
                    TSTypeDescriptor::Primitive(TSPrimitive::Boolean),
                    TSTypeDescriptor::Primitive(TSPrimitive::String),
                ]
            }))
        );
    }

    #[test]
    fn test_convert_alias() {
        let hoisted = Mutex::new(vec![]);
        assert_eq!(
            GTDescriptor::Alias(Box::new(GTAlias {
                doc: None,
                name: GTName("Name".to_string()),
                descriptor: GTDescriptor::Primitive(GTPrimitive::Boolean),
            }))
            .convert(&|definition| {
                let mut hoisted = hoisted.lock().unwrap();
                hoisted.push(definition);
            }),
            TSTypeDescriptor::Name(TSName("Name".to_string()))
        );
        assert_eq!(
            hoisted.lock().unwrap().clone(),
            vec![definition::TSDefinition {
                doc: None,
                descriptor: TSDefinitionDescriptor::Alias(TSAlias {
                    name: TSName("Name".to_string()),
                    descriptor: TSTypeDescriptor::Primitive(TSPrimitive::Boolean),
                }),
            }]
        );
    }

    #[test]
    fn test_convert_inline_import() {
        assert_eq!(
            GTDescriptor::InlineImport(GTInlineImport {
                path: "./path/to/module".to_string(),
                name: GTName("Name".to_string())
            })
            .convert(&|_| {}),
            TSTypeDescriptor::InlineImport(TSInlineImport {
                path: "./path/to/module".to_string(),
                name: TSName("Name".to_string())
            })
        );
    }
}
