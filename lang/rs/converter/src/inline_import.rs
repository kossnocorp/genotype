use genotype_lang_rs_tree::RSInlineUse;
use genotype_parser::tree::inline_import::GTInlineImport;
use miette::Result;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSInlineUse> for GTInlineImport {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSInlineUse> {
        let path = self.path.convert(context)?;
        let name = self.name.convert(context)?;
        Ok(RSInlineUse { path, name })
    }
}

#[cfg(test)]
mod tesrs {
    use genotype_lang_rs_tree::RSPath;
    use genotype_parser::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTInlineImport {
                span: (0, 0).into(),
                path: GTPath::new(
                    (0, 0).into(),
                    GTPathModuleId::Resolved("module/path".into()),
                    "./path/to/module".into()
                ),
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
            }
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            RSInlineUse {
                path: RSPath("module/path".into(), "super::path::to::module".into()),
                name: "Name".into(),
            }
        );
    }
}
