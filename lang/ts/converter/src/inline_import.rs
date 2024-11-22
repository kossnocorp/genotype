use genotype_lang_ts_tree::inline_import::TSInlineImport;
use genotype_parser::tree::inline_import::GTInlineImport;

use crate::{context::TSConvertContext, convert::TSConvert};

impl TSConvert<TSInlineImport> for GTInlineImport {
    fn convert(&self, context: &mut TSConvertContext) -> TSInlineImport {
        TSInlineImport {
            path: self.path.convert(context),
            name: self.name.convert(context),
        }
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
            .convert(&mut Default::default()),
            TSInlineImport {
                path: "./path/to/module.ts".into(),
                name: "Name".into(),
            }
        );
    }
}
