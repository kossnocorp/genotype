use genotype_json_schema::json::*;
use genotype_parser::*;

use crate::{GtjConvert, GtjConvertContext};

impl GtjConvert<GTArray> for GtjArray {
    fn convert(&self, context: &mut GtjConvertContext) -> GTArray {
        let name = context.claim_name(self.name.clone(), "Array");

        let descriptor =
            context.enter_name_context(GTNamingContextName::Identifier(name.clone()), |context| {
                context.enter_name_context(
                    GTNamingContextName::Transitive("Element".into()),
                    |context| self.descriptor.convert(context),
                )
            });

        GTArray {
            span: GTSpan::default(),
            descriptor,
        }
    }
}

impl GtjConvert<GTDescriptor> for GtjArray {
    fn convert(&self, context: &mut GtjConvertContext) -> GTDescriptor {
        GTDescriptor::Array(Box::new(self.convert(context)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_json_schema::json::{GtjAny, GtjArrayKindArray, GtjNull, GtjNullKindNull};
    use genotype_parser::GTArray;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let array = GtjArray {
            kind: GtjArrayKindArray,
            descriptor: GtjAny::GtjNull(GtjNull {
                kind: GtjNullKindNull,
                name: None,
                doc: None,
            }),
            name: None,
            doc: None,
        };
        assert_eq!(
            GTArray {
                span: Default::default(),
                descriptor: GTPrimitive::Null(Default::default()).into()
            },
            array.convert(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_descriptor() {
        let array = GtjArray {
            kind: GtjArrayKindArray,
            descriptor: GtjAny::GtjNull(GtjNull {
                kind: GtjNullKindNull,
                name: None,
                doc: None,
            }),
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Array(Box::new(GTArray {
                span: Default::default(),
                descriptor: GTPrimitive::Null(Default::default()).into()
            })),
            array.convert(&mut Default::default())
        );
    }

    #[test]
    fn test_convert_naming_unnamed() {
        let mut context = GtjConvertContext::new();

        let array = GtjArray {
            kind: GtjArrayKindArray,
            descriptor: GtjAny::GtjObject(GtjObject {
                kind: GtjObjectKindObject,
                name: None,
                doc: None,
                properties: vec![],
            }),
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Array(Box::new(GTArray {
                span: Default::default(),
                descriptor: GTObject {
                    span: Default::default(),
                    name: GTObjectName::Named(GTIdentifier(
                        Default::default(),
                        "RootElement".into()
                    )),
                    extensions: vec![],
                    properties: vec![],
                }
                .into()
            })),
            array.convert(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_unnamed_nested() {
        let mut context = GtjConvertContext::new();

        let array = GtjArray {
            kind: GtjArrayKindArray,
            descriptor: GtjAny::GtjArray(Box::new(GtjArray {
                kind: GtjArrayKindArray,
                descriptor: GtjAny::GtjObject(GtjObject {
                    kind: GtjObjectKindObject,
                    name: None,
                    doc: None,
                    properties: vec![GtjProperty {
                        kind: GtjPropertyKindProperty,
                        name: "world".into(),
                        doc: None,
                        descriptor: GtjAny::GtjObject(GtjObject {
                            name: None,
                            doc: None,
                            kind: GtjObjectKindObject,
                            properties: vec![],
                        }),
                        required: false,
                    }],
                }),
                name: None,
                doc: None,
            })),
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Array(Box::new(GTArray {
                span: Default::default(),
                descriptor: GTArray {
                    span: Default::default(),
                    descriptor: GTObject {
                        span: Default::default(),
                        name: GTObjectName::Named(GTIdentifier(
                            Default::default(),
                            "RootElementElement".into()
                        )),
                        extensions: vec![],
                        properties: vec![GTProperty {
                            span: Default::default(),
                            descriptor: GTObject {
                                span: Default::default(),
                                name: GTObjectName::Named(GTIdentifier(
                                    Default::default(),
                                    "RootElementElementWorld".into()
                                )),
                                extensions: vec![],
                                properties: vec![],
                            }
                            .into(),
                            attributes: Default::default(),
                            required: false,
                            name: GTKey(Default::default(), "world".into()),
                            doc: None,
                        }],
                    }
                    .into()
                }
                .into()
            })),
            array.convert(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_named() {
        let mut context = GtjConvertContext::new();

        let array = GtjArray {
            kind: GtjArrayKindArray,
            descriptor: GtjAny::GtjObject(GtjObject {
                kind: GtjObjectKindObject,
                name: None,
                doc: None,
                properties: vec![],
            }),
            name: Some("Hello".into()),
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Array(Box::new(GTArray {
                span: Default::default(),
                descriptor: GTObject {
                    span: Default::default(),
                    name: GTObjectName::Named(GTIdentifier(
                        Default::default(),
                        "HelloElement".into()
                    )),
                    extensions: vec![],
                    properties: vec![],
                }
                .into()
            })),
            array.convert(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_named_nested() {
        let mut context = GtjConvertContext::new();

        let array = GtjArray {
            kind: GtjArrayKindArray,
            descriptor: GtjAny::GtjArray(Box::new(GtjArray {
                kind: GtjArrayKindArray,
                descriptor: GtjAny::GtjObject(GtjObject {
                    kind: GtjObjectKindObject,
                    name: Some("Hey".into()),
                    doc: None,
                    properties: vec![GtjProperty {
                        kind: GtjPropertyKindProperty,
                        name: "world".into(),
                        doc: None,
                        descriptor: GtjAny::GtjObject(GtjObject {
                            name: None,
                            doc: None,
                            kind: GtjObjectKindObject,
                            properties: vec![],
                        }),
                        required: false,
                    }],
                }),
                name: Some("Hi".into()),
                doc: None,
            })),
            name: Some("Hello".into()),
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Array(Box::new(GTArray {
                span: Default::default(),
                descriptor: GTArray {
                    span: Default::default(),
                    descriptor: GTObject {
                        span: Default::default(),
                        name: GTObjectName::Named(GTIdentifier(Default::default(), "Hey".into())),
                        extensions: vec![],
                        properties: vec![GTProperty {
                            span: Default::default(),
                            descriptor: GTObject {
                                span: Default::default(),
                                name: GTObjectName::Named(GTIdentifier(
                                    Default::default(),
                                    "HeyWorld".into()
                                )),
                                extensions: vec![],
                                properties: vec![],
                            }
                            .into(),
                            attributes: Default::default(),
                            required: false,
                            name: GTKey(Default::default(), "world".into()),
                            doc: None,
                        }],
                    }
                    .into()
                }
                .into()
            })),
            array.convert(&mut context),
        );
    }
}
