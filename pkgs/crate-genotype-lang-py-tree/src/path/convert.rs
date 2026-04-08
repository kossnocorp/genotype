use crate::prelude::internal::*;

pub fn py_parse_module_path(path: String) -> String {
    path.replace("../", "..")
        .replace("./", ".")
        .replace("/", ".")
}

impl PyConvert<PyPath> for GtPath {
    fn convert(&self, context: &mut PyConvertContext) -> PyPath {
        PyPath(py_parse_module_path(context.resolve_path(self)).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;
    

    #[test]
    fn test_convert_base() {
        assert_ron_snapshot!(
            GtPath::parse((0, 0).into(), &"module".into(), "./path/to/module")
                .unwrap()
                .convert(&mut PyConvertContext::default()),
            @r#"PyPath(".path.to.module")"#
        );
    }

    #[test]
    fn test_convert_absolute() {
        assert_ron_snapshot!(
            GtPath::parse((0, 0).into(), &"module".into(), "module/path")
                .unwrap()
                .convert(&mut PyConvertContext::default()),
            @r#"PyPath("module.path")"#
        );
    }

    #[test]
    fn test_convert_up() {
        assert_ron_snapshot!(
            GtPath::parse((0, 0).into(), &"module".into(), "../path/to/module")
                .unwrap()
                .convert(&mut PyConvertContext::default()),
            @r#"PyPath("..path.to.module")"#
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut resolve = PyConvertResolve::default();
        resolve.paths.insert(
            GtPath::parse((0, 0).into(), &"module".into(), "./path/to/module").unwrap(),
            GtPath::parse((0, 0).into(), &"module".into(), "./path/to/another/module").unwrap(),
        );
        let mut context = PyConvertContext::new(resolve, Default::default());
        assert_ron_snapshot!(
            GtPath::parse((0, 0).into(), &"module".into(), "./path/to/module")
                .unwrap()
                .convert(&mut context),
            @r#"PyPath(".path.to.another.module")"#
        );
    }
}
