use crate::prelude::internal::*;

impl TsConvert<TsPath> for GtPath {
    fn convert(&self, context: &mut TsConvertContext) -> TsPath {
        TsPath(context.resolve_path(self).into())
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
            GtPath::parse((0, 0).into(), "./path/to/module")
                .unwrap()
                .convert(&mut Default::default()),
            @r#"TsPath("./path/to/module")"#
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut resolve = TsConvertResolve::new();
        resolve.paths.insert(
            GtPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            GtPath::parse((0, 0).into(), "./path/to/module/index").unwrap(),
        );
        assert_ron_snapshot!(
            GtPath::parse((0, 0).into(), "./path/to/module")
                .unwrap()
                .convert(&mut TsConvertContext::new(resolve, Default::default())),
            @r#"TsPath("./path/to/module/index")"#
        );
    }
}
