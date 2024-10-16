use genotype_lang_py_tree::{definition::PYDefinition, PYReference};
use genotype_parser::tree::object::GTObject;

use crate::{convert::PYConvert, resolve::PYConvertResolve};

impl PYConvert<PYReference> for GTObject {
    fn convert<HoistFn>(&self, resolve: &PYConvertResolve, hoist: &HoistFn) -> PYReference
    where
        HoistFn: Fn(PYDefinition),
    {
        // [TODO] Implement this
        // PYObject {
        //     properties: self
        //         .properties
        //         .iter()
        //         .map(|property| property.convert(resolve, hoist))
        //         .collect(),
        // }
        PYReference::new("TODO".into(), false)
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
            }
            .convert(&PYConvertResolve::new(), &|_| {}),
            // [TODO]
            // PYObject {
            //     properties: vec![
            //         PYProperty {
            //             name: "name".into(),
            //             descriptor: PYDescriptor::Primitive(PYPrimitive::String),
            //             required: true,
            //         },
            //         PYProperty {
            //             name: "age".into(),
            //             descriptor: PYDescriptor::Primitive(PYPrimitive::Int),
            //             required: false,
            //         }
            //     ]
            // }
            PYReference::new("TODO".into(), false)
        );
    }
}
