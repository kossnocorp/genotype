use genotype_lang_py_tree::{PYContext, PYDependency, PYReference};
use genotype_parser::tree::inline_import::GTInlineImport;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYReference> for GTInlineImport {
    fn convert(&self, context: &mut PYConvertContext) -> PYReference {
        let name = self.name.convert(context);
        let path = self.path.convert(context);
        context.import(PYDependency::Local(path), name.clone());
        PYReference::new(name, false)
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_tree::PYDependency;
    use genotype_parser::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        let mut context = PYConvertContext::default();
        assert_eq!(
            GTInlineImport {
                span: (0, 0).into(),
                path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
            }
            .convert(&mut context),
            PYReference::new("Name".into(), false),
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(PYDependency::Local(".path.to.module".into()), "Name".into())]
        );
    }
}
