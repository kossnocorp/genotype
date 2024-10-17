use genotype_lang_py_tree::*;
use genotype_parser::*;

use crate::{convert::PYConvert, resolve::PYConvertResolve};

impl PYConvert<PYDefinition> for GTAlias {
    fn convert<HoistFn>(&self, resolve: &PYConvertResolve, hoist: &HoistFn) -> PYDefinition
    where
        HoistFn: Fn(PYDefinition),
    {
        match &self.descriptor {
            GTDescriptor::Object(object) => PYDefinition::Class(PYClass {
                name: self.name.convert(resolve, hoist),
                extensions: object
                    .extensions
                    .iter()
                    .map(|e| e.convert(resolve, hoist))
                    .collect(),
                properties: object
                    .properties
                    .iter()
                    .map(|p| p.convert(resolve, hoist))
                    .collect(),
            }),

            _ => PYDefinition::Alias(PYAlias {
                name: self.name.convert(resolve, hoist),
                descriptor: self.descriptor.convert(resolve, hoist),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use genotype_lang_py_tree::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert_alias() {
        assert_eq!(
            GTAlias {
                span: (0, 0).into(),
                doc: None,
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }
            .convert(&PYConvertResolve::new(), &|_| {}),
            PYDefinition::Alias(PYAlias {
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::Boolean),
            }),
        );
    }

    #[test]
    fn test_convert_class() {
        assert_eq!(
            GTAlias {
                span: (0, 0).into(),
                doc: None,
                name: GTIdentifier::new((0, 0).into(), "Book".into()),
                descriptor: GTDescriptor::Object(GTObject {
                    span: (0, 0).into(),
                    name: GTIdentifier::new((0, 0).into(), "Book".into()).into(),
                    extensions: vec![],
                    properties: vec![
                        GTProperty {
                            span: (0, 0).into(),
                            doc: None,
                            name: GTKey::new((0, 0).into(), "title".into()),
                            descriptor: GTPrimitive::String((0, 0).into()).into(),
                            required: true,
                        },
                        GTProperty {
                            span: (0, 0).into(),
                            doc: None,
                            name: GTKey::new((0, 0).into(), "author".into()),
                            descriptor: GTPrimitive::String((0, 0).into()).into(),
                            required: true,
                        }
                    ]
                })
            }
            .convert(&PYConvertResolve::new(), &|_| {}),
            PYDefinition::Class(PYClass {
                name: "Book".into(),
                extensions: vec![],
                properties: vec![
                    PYProperty {
                        name: "title".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                        required: true,
                    },
                    PYProperty {
                        name: "author".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                        required: true,
                    }
                ]
            }),
        );
    }

    #[test]
    fn test_convert_hoisting() {
        let hoisted = Mutex::new(vec![]);
        assert_eq!(
            GTAlias {
                span: (0, 0).into(),
                doc: None,
                name: GTIdentifier::new((0, 0).into(), "Book".into()),
                descriptor: GTDescriptor::Union(GTUnion {
                    span: (0, 0).into(),
                    descriptors: vec![
                        GTObject {
                            span: (0, 0).into(),
                            name: GTObjectName::Named(GTIdentifier::new(
                                (0, 0).into(),
                                "BookObj".into()
                            )),
                            extensions: vec![],
                            properties: vec![GTProperty {
                                span: (0, 0).into(),
                                doc: None,
                                name: GTKey::new((0, 0).into(), "author".into()),
                                descriptor: GTPrimitive::String((0, 0).into()).into(),
                                required: true,
                            }]
                        }
                        .into(),
                        GTPrimitive::String((0, 0).into()).into(),
                    ]
                })
            }
            .convert(&PYConvertResolve::new(), &|definition| {
                let mut hoisted = hoisted.lock().unwrap();
                hoisted.push(definition);
            }),
            PYDefinition::Alias(PYAlias {
                name: "Book".into(),
                descriptor: PYUnion {
                    descriptors: vec![
                        // [TODO]
                        PYReference::new(
                            /* [TODO] BookObj */ "TODO".into(),
                            /* [TODO] true */ false
                        )
                        .into(),
                        PYPrimitive::String.into(),
                    ]
                }
                .into(),
            })
        );
        assert_eq!(
            hoisted.into_inner().unwrap(),
            // [TODO]
            // vec![PYDefinition::Class(PYClass {
            //     name: "BookObj".into(),
            //     extensions: vec![],
            //     properties: vec![PYProperty {
            //         name: "author".into(),
            //         descriptor: PYDescriptor::Primitive(PYPrimitive::String),
            //         required: true,
            //     }]
            // })]
            vec![]
        );
    }
}
