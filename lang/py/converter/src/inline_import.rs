use genotype_lang_py_tree::{definition::PYDefinition, PYReference};
use genotype_parser::tree::inline_import::GTInlineImport;

use crate::{convert::PYConvert, resolve::PYConvertResolve};

impl PYConvert<PYReference> for GTInlineImport {
    fn convert<HoistFn>(&self, resolve: &PYConvertResolve, hoist: &HoistFn) -> PYReference
    where
        HoistFn: Fn(PYDefinition),
    {
        // [TODO] Pull the dependency
        // PYInlineImport {
        //     path: self.path.convert(resolve, hoist),
        //     name: self.name.convert(resolve, hoist),
        // }
        PYReference::new("TODO".into(), false)
    }
}

#[cfg(test)]
mod tests {
    use genotype_parser::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTInlineImport {
                span: (0, 0).into(),
                path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
            }
            .convert(&PYConvertResolve::new(), &|_| {}),
            // [TODo]
            PYReference::new("TODO".into(), false),
        );
    }
}
