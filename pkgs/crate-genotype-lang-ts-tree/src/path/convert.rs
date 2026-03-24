use crate::prelude::internal::*;

impl TSConvert<TSPath> for GTPath {
    fn convert(&self, context: &mut TSConvertContext) -> TSPath {
        TSPath(context.resolve_path(self).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert_base() {
        assert_ron_snapshot!(
            GTPath::parse((0, 0).into(), "./path/to/module")
                .unwrap()
                .convert(&mut Default::default()),
            @r#"TSPath("./path/to/module")"#
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut resolve = TSConvertResolve::new();
        resolve.paths.insert(
            GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            GTPath::parse((0, 0).into(), "./path/to/module/index").unwrap(),
        );
        assert_ron_snapshot!(
            GTPath::parse((0, 0).into(), "./path/to/module")
                .unwrap()
                .convert(&mut TSConvertContext::new(resolve, Default::default())),
            @r#"TSPath("./path/to/module/index")"#
        );
    }
}
