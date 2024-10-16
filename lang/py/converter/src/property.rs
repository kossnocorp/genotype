use genotype_lang_py_tree::{definition::PYDefinition, property::PYProperty};
use genotype_parser::tree::property::GTProperty;

use crate::{convert::PYConvert, resolve::PYConvertResolve};

impl PYConvert<PYProperty> for GTProperty {
    fn convert<HoistFn>(&self, resolve: &PYConvertResolve, hoist: &HoistFn) -> PYProperty
    where
        HoistFn: Fn(PYDefinition),
    {
        PYProperty {
            name: self.name.convert(resolve, hoist),
            descriptor: self.descriptor.convert(resolve, hoist),
            required: self.required,
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_tree::*;
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::tree::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTProperty {
                span: (0, 0).into(),
                doc: None,
                name: GTKey::new((0, 0).into(), "name".into()),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
                required: false,
            }
            .convert(&PYConvertResolve::new(), &|_| {}),
            PYProperty {
                name: "name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                required: false,
            }
        );
    }
}
