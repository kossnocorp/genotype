use crate::prelude::internal::*;

impl PyConvert<PyPath> for GtPath {
    fn convert(&self, context: &mut PyConvertContext) -> PyPath {
        PyPath(PyPath::to_py_module_path(context.resolve_path(self)).into())
    }
}

impl PyPath {
    pub fn to_py_module_path(path: String) -> String {
        path.replace("../", "..")
            .replace("./", ".")
            .replace("/", ".")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_base() {
        assert_ron_snapshot!(
            Gt::path_anon("./path/to/module")
                .convert(&mut PyConvertContext::default()),
            @r#"PyPath(".path.to.module")"#
        );
    }

    #[test]
    fn test_convert_absolute() {
        assert_ron_snapshot!(
            Gt::path_anon("module/path")
                .convert(&mut PyConvertContext::default()),
            @r#"PyPath("module.path")"#
        );
    }

    #[test]
    fn test_convert_up() {
        assert_ron_snapshot!(
            Gt::path_anon("../path/to/module")
                .convert(&mut PyConvertContext::default()),
            @r#"PyPath("..path.to.module")"#
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut resolve = PyConvertResolve::default();
        resolve.paths.insert(
            Gt::path_anon("./path/to/module"),
            Gt::path_anon("./path/to/another/module"),
        );
        let mut context = PyConvertContext::new(resolve, Default::default());
        assert_ron_snapshot!(
            Gt::path_anon("./path/to/module")
                .convert(&mut context),
            @r#"PyPath(".path.to.another.module")"#
        );
    }

    #[test]
    fn test_convert_naming() {
        assert_ron_snapshot!(
            Gt::path_anon("./ShopGoods/OrderItem")
                .convert(&mut PyConvertContext::default()),
            @r#"PyPath(".shop_goods.order_item")"#
        );
    }
}
