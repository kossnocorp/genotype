use crate::prelude::internal::*;

pub fn py_parse_module_path(path: String) -> String {
    path.replace("../", "..")
        .replace("./", ".")
        .replace("/", ".")
}

impl PYConvert<PYPath> for GTPath {
    fn convert(&self, context: &mut PYConvertContext) -> PYPath {
        PYPath(py_parse_module_path(context.resolve_path(self)).into())
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
                .convert(&mut PYConvertContext::default()),
            @r#"PYPath(".path.to.module")"#
        );
    }

    #[test]
    fn test_convert_absolute() {
        assert_ron_snapshot!(
            GTPath::parse((0, 0).into(), "module/path")
                .unwrap()
                .convert(&mut PYConvertContext::default()),
            @r#"PYPath("module.path")"#
        );
    }

    #[test]
    fn test_convert_up() {
        assert_ron_snapshot!(
            GTPath::parse((0, 0).into(), "../path/to/module")
                .unwrap()
                .convert(&mut PYConvertContext::default()),
            @r#"PYPath("..path.to.module")"#
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut resolve = PYConvertResolve::default();
        resolve.paths.insert(
            GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            GTPath::parse((0, 0).into(), "./path/to/another/module").unwrap(),
        );
        let mut context = PYConvertContext::new(resolve, Default::default());
        assert_ron_snapshot!(
            GTPath::parse((0, 0).into(), "./path/to/module")
                .unwrap()
                .convert(&mut context),
            @r#"PYPath(".path.to.another.module")"#
        );
    }
}
