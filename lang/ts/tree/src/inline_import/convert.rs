use crate::prelude::internal::*;

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
    use super::*;
    use pretty_assertions::assert_eq;

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
                path: "./path/to/module".into(),
                name: "Name".into(),
            }
        );
    }
}
