use genotype_lang_ts_tree::TSExtension;
use genotype_parser::tree::GTExtension;

use crate::{context::TSConvertContext, convert::TSConvert};

impl TSConvert<TSExtension> for GTExtension {
    fn convert(&self, context: &mut TSConvertContext) -> TSExtension {
        TSExtension {
            reference: self.reference.convert(context),
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::*;
    use genotype_parser::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            TSExtension {
                reference: "Name".into()
            },
            GTExtension {
                span: (0, 0).into(),
                reference: GTReference {
                    span: (0, 0).into(),
                    id: GTReferenceId("module".into(), (0, 0).into()),
                    definition_id: GTReferenceDefinitionId::Resolved(GTDefinitionId(
                        "module".into(),
                        "Name".into()
                    )),
                    identifier: GTIdentifier::new((0, 0).into(), "Name".into())
                }
            }
            .convert(&mut Default::default()),
        );
    }
}
