use crate::prelude::internal::*;

pub fn rs_parse_module_path(path: String) -> String {
    path.replace("../", "super::super::")
        .replace("./", "super::")
        .replace("/", "::")
}

impl RsConvert<RsPath> for GtPath {
    fn convert(&self, context: &mut RsConvertContext) -> Result<RsPath> {
        let Some(module_id) = context.resolve_path_module_id(self) else {
            return Err(RsConverterError::UnresolvedPath(self.span).into());
        };

        Ok(RsPath(
            module_id,
            rs_parse_module_path(context.resolve_path(self)).into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;
    use std::collections::HashMap;

    #[test]
    fn test_convert_base() {
        assert_ron_snapshot!(
            GtPath::new(
                (0, 0).into(),
                GtPathModuleId::new((0, 0).into(), "module".into()),
                "./path/to/module".into()
            )
            .convert(&mut RsConvertContext::new(
                "module".into(),
                RsConvertResolve {
                    path_module_ids: HashMap::from([(
                        GtPathModuleId::new((0, 0).into(), "module".into()),
                        "module/path".into(),
                    )]),
                    ..Default::default()
                },
                Default::default(),
                Default::default(),
            ))
            .unwrap(),
            @r#"RsPath(GtModuleId("module/path"), "super::path::to::module")"#,
        );
    }

    #[test]
    fn test_convert_absolute() {
        assert_ron_snapshot!(
            GtPath::new(
                (0, 0).into(),
                GtPathModuleId::new((0, 0).into(), "module".into()),
                "path/to/module".into()
            )
            .convert(&mut RsConvertContext::new(
                "module".into(),
                RsConvertResolve {
                    path_module_ids: HashMap::from([(
                        GtPathModuleId::new((0, 0).into(), "module".into()),
                        "module/path".into(),
                    )]),
                    ..Default::default()
                },
                Default::default(),
                Default::default(),
            ))
            .unwrap(),
            @r#"RsPath(GtModuleId("module/path"), "path::to::module")"#,
        );
    }

    #[test]
    fn test_convert_up() {
        assert_ron_snapshot!(
            GtPath::new(
                (0, 0).into(),
                GtPathModuleId::new((0, 0).into(), "module".into()),
                "../path/to/module".into(),
            )
            .convert(&mut RsConvertContext::new(
                "module".into(),
                RsConvertResolve {
                    path_module_ids: HashMap::from([(
                        GtPathModuleId::new((0, 0).into(), "module".into()),
                        "module/path".into(),
                    )]),
                    ..Default::default()
                },
                Default::default(),
                Default::default(),
            ))
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
                GtPathModuleId::new((0, 0).into(), "module".into()),
                "./path/to/module".into(),
            ),
            GtPath::new(
                (0, 0).into(),
                GtPathModuleId::new((0, 0).into(), "module".into()),
                "./path/to/another/module".into(),
            ),
        );
        resolve.path_module_ids.insert(
            GtPathModuleId::new((0, 0).into(), "module".into()),
            "module/path".into(),
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
                GtPathModuleId::new((0, 0).into(), "module".into()),
                "./path/to/module".into()
            )
            .convert(&mut context)
            .unwrap(),
            @r#"RsPath(GtModuleId("module/path"), "super::path::to::another::module")"#,
        );
    }
}
