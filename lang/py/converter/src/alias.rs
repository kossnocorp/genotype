use genotype_lang_py_tree::*;
use genotype_parser::*;

use crate::{convert::PYConvert, resolve::PYConvertResolve};

impl PYConvert<PYDefinition> for GTAlias {
    fn convert<HoistFn>(&self, resolve: &PYConvertResolve, hoist: &HoistFn) -> PYDefinition
    where
        HoistFn: Fn(PYDefinition),
    {
        match &self.descriptor {
            GTDescriptor::Object(object) => PYDefinition::Interface(PYClass {
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
    use std::vec;

    use genotype_lang_py_tree::*;
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::tree::*;

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
    fn test_convert_interface() {
        assert_eq!(
            GTAlias {
                span: (0, 0).into(),
                doc: None,
                name: GTIdentifier::new((0, 0).into(), "Book".into()),
                descriptor: GTDescriptor::Object(GTObject {
                    span: (0, 0).into(),
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
            PYDefinition::Interface(PYClass {
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
    fn test_convert_extensions() {
        assert_eq!(
            GTAlias {
                span: (0, 0).into(),
                doc: None,
                name: GTIdentifier::new((0, 0).into(), "Book".into()),
                descriptor: GTDescriptor::Object(GTObject {
                    span: (0, 0).into(),
                    extensions: vec![GTExtension {
                        span: (0, 0).into(),
                        reference: GTIdentifier::new((0, 0).into(), "Good".into()).into()
                    }],
                    properties: vec![GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        name: GTKey::new((0, 0).into(), "author".into()),
                        descriptor: GTPrimitive::String((0, 0).into()).into(),
                        required: true,
                    }]
                })
            }
            .convert(&PYConvertResolve::new(), &|_| {}),
            PYDefinition::Interface(PYClass {
                name: "Book".into(),
                extensions: vec![PYReference::new("Good".into(), false).into()],
                properties: vec![PYProperty {
                    name: "author".into(),
                    descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                    required: true,
                }]
            }),
        );

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
                            extensions: vec![GTExtension {
                                span: (0, 0).into(),
                                reference: GTIdentifier::new((0, 0).into(), "Good".into()).into()
                            }],
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
                },)
            }
            .convert(&PYConvertResolve::new(), &|_| {}),
            PYDefinition::Alias(PYAlias {
                name: "Book".into(),
                descriptor: PYUnion {
                    descriptors: vec![
                        // [TODO] Hoist class instead of converting to a intersection
                        // PYIntersection {
                        //     descriptors: vec![
                        //         PYClass {
                        //             properties: vec![PYProperty {
                        //                 name: "author".into(),
                        //                 descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                        //                 required: true,
                        //             }]
                        //         }
                        //         .into(),
                        //         "Good".into()
                        //     ],
                        // }
                        // .into(),
                        PYPrimitive::String.into(),
                    ]
                }
                .into(),
            }),
        );
    }
}
