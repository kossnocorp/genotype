use genotype_lang_py_tree::*;
use genotype_parser::tree::descriptor::GTDescriptor;

use crate::{convert::PYConvert, resolve::PYConvertResolve};

impl PYConvert<PYDescriptor> for GTDescriptor {
    fn convert<HoistFn>(&self, resolve: &PYConvertResolve, hoist: &HoistFn) -> PYDescriptor
    where
        HoistFn: Fn(PYDefinition),
    {
        match self {
            GTDescriptor::Alias(alias) => {
                hoist(alias.convert(resolve, hoist));
                // [TODO]
                PYDescriptor::Reference(PYReference::new(alias.name.convert(resolve, hoist), false))
            }

            GTDescriptor::Array(array) => {
                PYDescriptor::List(Box::new(array.convert(resolve, hoist)))
            }

            GTDescriptor::InlineImport(import) => {
                // [TODO] Hoist to imports instead
                // PYDescriptor::InlineImport(import.convert(resolve, hoist))
                PYDescriptor::Reference(PYReference::new("TODO".into(), false))
            }

            GTDescriptor::Literal(literal) => {
                PYDescriptor::Literal(literal.convert(resolve, hoist))
            }

            GTDescriptor::Object(object) => {
                // [TODO] Resolve to class or hoist reference
                // let descriptor = PYDescriptor::Object(object.convert(resolve, hoist));
                // if object.extensions.is_empty() {
                //     descriptor
                // } else {
                //     let mut descriptors: Vec<PYDescriptor> = vec![descriptor];
                //     let extensions = object
                //         .extensions
                //         .iter()
                //         .map(|extension| {
                //             PYDescriptor::from(extension.reference.convert(resolve, hoist))
                //         })
                //         .collect::<Vec<PYDescriptor>>();
                //     descriptors.extend(extensions);
                //     PYDescriptor::Intersection(PYIntersection { descriptors })
                // }
                PYDescriptor::Reference(PYReference::new("TODO".into(), false))
            }

            GTDescriptor::Primitive(primitive) => {
                PYDescriptor::Primitive(primitive.convert(resolve, hoist))
            }

            GTDescriptor::Reference(name) => PYDescriptor::Reference(name.convert(resolve, hoist)),

            GTDescriptor::Tuple(tuple) => PYDescriptor::Tuple(tuple.convert(resolve, hoist)),

            GTDescriptor::Union(union) => PYDescriptor::Union(union.convert(resolve, hoist)),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Mutex;

    use genotype_lang_py_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use crate::resolve::PYConvertResolve;

    use super::*;

    #[test]
    fn test_convert_alias() {
        let hoisted = Mutex::new(vec![]);
        assert_eq!(
            GTDescriptor::Alias(Box::new(GTAlias {
                span: (0, 0).into(),
                doc: None,
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }))
            .convert(&PYConvertResolve::new(), &|definition| {
                let mut hoisted = hoisted.lock().unwrap();
                hoisted.push(definition);
            }),
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
            .convert(&PYConvertResolve::new(), &|_| {}),
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
            .convert(&PYConvertResolve::new(), &|_| {}),
            // [TODO]
            // PYDescriptor::InlineImport(PYInlineImport {
            //     path: "./path/to/module.ts".into(),
            //     name: "Name".into()
            // })
            PYDescriptor::Reference(PYReference::new("NOPE".into(), false))
        );
    }

    #[test]
    fn test_convert_object() {
        assert_eq!(
            GTDescriptor::Object(GTObject {
                span: (0, 0).into(),
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
            .convert(&PYConvertResolve::new(), &|_| {}),
            // [TODO]
            // PYDescriptor::Object(PYObject {
            //     properties: vec![
            //         PYProperty {
            //             name: "name".into(),
            //             descriptor: PYPrimitive::String.into(),
            //             required: true,
            //         },
            //         PYProperty {
            //             name: "age".into(),
            //             descriptor: PYPrimitive::Number.into(),
            //             required: false,
            //         }
            //     ]
            // })
            PYDescriptor::Reference(PYReference::new("NOPE".into(), false))
        );

        assert_eq!(
            GTDescriptor::Object(GTObject {
                span: (0, 0).into(),
                extensions: vec![GTExtension {
                    span: (0, 0).into(),
                    reference: GTIdentifier::new((0, 0).into(), "Good".into()).into()
                }],
                properties: vec![GTProperty {
                    span: (0, 0).into(),
                    doc: None,
                    name: GTKey::new((0, 0).into(), "title".into()),
                    descriptor: GTPrimitive::String((0, 0).into()).into(),
                    required: true,
                },]
            })
            .convert(&PYConvertResolve::new(), &|_| {}),
            // [TODO]
            // PYDescriptor::Intersection(PYIntersection {
            //     descriptors: vec![
            //         PYObject {
            //             properties: vec![PYProperty {
            //                 name: "title".into(),
            //                 descriptor: PYPrimitive::String.into(),
            //                 required: true,
            //             },]
            //         }
            //         .into(),
            //         "Good".into()
            //     ]
            // })
            PYDescriptor::Reference(PYReference::new("NOPE".into(), false))
        );
    }

    #[test]
    fn test_convert_primitive() {
        assert_eq!(
            GTDescriptor::Primitive(GTPrimitive::Boolean((0, 0).into()))
                .convert(&PYConvertResolve::new(), &|_| {}),
            PYDescriptor::Primitive(PYPrimitive::Boolean)
        );
    }

    #[test]
    fn test_convert_reference() {
        assert_eq!(
            GTDescriptor::Reference(GTIdentifier::new((0, 0).into(), "Name".into()).into())
                .convert(&PYConvertResolve::new(), &|_| {}),
            PYReference::new("Name".into(), false).into()
        );
        // [TODO] Depending on context, set forward reference
        assert_eq!(
            GTDescriptor::Reference(GTIdentifier::new((0, 0).into(), "Name".into()).into())
                .convert(&PYConvertResolve::new(), &|_| {}),
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
            .convert(&PYConvertResolve::new(), &|_| {}),
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
            .convert(&PYConvertResolve::new(), &|_| {}),
            PYDescriptor::Union(PYUnion {
                descriptors: vec![
                    PYDescriptor::Primitive(PYPrimitive::Boolean),
                    PYDescriptor::Primitive(PYPrimitive::String),
                ]
            })
        );
    }
}
