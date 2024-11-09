use genotype_lang_rs_tree::{RSContext, RSDependency, RSReference};
use genotype_parser::tree::inline_import::GTInlineImport;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSReference> for GTInlineImport {
    fn convert(&self, context: &mut RSConvertContext) -> RSReference {
        let name = self.name.convert(context);
        let path = self.path.convert(context);
        context.import(RSDependency::Local(path), name.clone());
        RSReference::new(name)
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_tree::RSDependency;
    use genotype_parser::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        let mut context = RSConvertContext::default();
        assert_eq!(
            GTInlineImport {
                span: (0, 0).into(),
                path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
            }
            .convert(&mut context),
            RSReference::new("Name".into()),
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(
                RSDependency::Local("self::path::to::module".into()),
                "Name".into()
            )]
        );
    }
}
