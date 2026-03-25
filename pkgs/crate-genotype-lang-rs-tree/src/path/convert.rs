use crate::prelude::internal::*;

pub fn rs_parse_module_path(path: String) -> String {
    path.replace("../", "super::super::")
        .replace("./", "super::")
        .replace("/", "::")
}

impl RsConvert<RsPath> for GtPath {
    fn convert(&self, context: &mut RsConvertContext) -> Result<RsPath> {
        match &self.1 {
            GtPathModuleId::Resolved(module_id) => Ok(RsPath(
                module_id.clone(),
                rs_parse_module_path(context.resolve_path(self)).into(),
            )),

            GtPathModuleId::Unresolved => {
                Err(RsConverterError::UnresolvedPath(self.0.clone()).into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_base() {
        assert_ron_snapshot!(
            GtPath::new(
                (0, 0).into(),
                GtPathModuleId::Resolved("module/path".into()),
                "./path/to/module".into()
            )
            .convert(&mut RsConvertContext::empty("module".into()))
            .unwrap(),
            @r#"RsPath(GtModuleId("module/path"), "super::path::to::module")"#,
        );
    }

    #[test]
    fn test_convert_absolute() {
        assert_ron_snapshot!(
            GtPath::new(
                (0, 0).into(),
                GtPathModuleId::Resolved("module/path".into()),
                "path/to/module".into()
            )
            .convert(&mut RsConvertContext::empty("module".into()))
            .unwrap(),
            @r#"RsPath(GtModuleId("module/path"), "path::to::module")"#,
        );
    }

    #[test]
    fn test_convert_up() {
        assert_ron_snapshot!(
            GtPath::new(
                (0, 0).into(),
                GtPathModuleId::Resolved("module/path".into()),
                "../path/to/module".into(),
            )
            .convert(&mut RsConvertContext::empty("module".into()))
            .unwrap(),
            @r#"RsPath(GtModuleId("module/path"), "super::super::path::to::module")"#,
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut resolve = RsConvertResolve::default();
        resolve.paths.insert(
            GtPath::new(
                (0, 0).into(),
                GtPathModuleId::Resolved("module/path".into()),
                "./path/to/module".into(),
            ),
            GtPath::new(
                (0, 0).into(),
                GtPathModuleId::Resolved("module/path".into()),
                "./path/to/another/module".into(),
            ),
        );
        let mut context = RsConvertContext::new(
            "module".into(),
            resolve,
            Default::default(),
            Default::default(),
        );
        assert_ron_snapshot!(
            GtPath::new(
                (0, 0).into(),
                GtPathModuleId::Resolved("module/path".into()),
                "./path/to/module".into()
            )
            .convert(&mut context)
            .unwrap(),
            @r#"RsPath(GtModuleId("module/path"), "super::path::to::another::module")"#,
        );
    }
}
