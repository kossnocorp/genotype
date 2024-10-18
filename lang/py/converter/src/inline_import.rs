use genotype_lang_py_tree::PYReference;
use genotype_parser::tree::inline_import::GTInlineImport;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYReference> for GTInlineImport {
    fn convert(&self, context: &mut PYConvertContext) -> PYReference {
        let name = self.name.convert(context);
        let path = self.path.convert(context);
        context.add_dependency(path, name.clone());
        PYReference::new(name, false)
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_tree::{PYImport, PYImportReference};
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
            vec![PYImport {
                path: ".path.to.module".into(),
                reference: PYImportReference::Named(vec!["Name".into()]),
            }]
        );
    }
}
