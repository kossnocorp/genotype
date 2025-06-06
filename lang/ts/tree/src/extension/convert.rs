use crate::prelude::internal::*;

impl TSConvert<TSExtension> for GTExtension {
    fn convert(&self, context: &mut TSConvertContext) -> TSExtension {
        TSExtension {
            reference: self.reference.convert(context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
