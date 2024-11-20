use genotype_lang_rs_tree::path::RSPath;
use genotype_parser::{tree::path::GTPath, GTPathModuleId};
use miette::Result;

use crate::{context::RSConvertContext, convert::RSConvert, error::RSConverterError};

pub fn rs_parse_module_path(path: String) -> String {
    path.replace("../", "super::super::")
        .replace("./", "super::")
        .replace("/", "::")
}

impl RSConvert<RSPath> for GTPath {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSPath> {
        match &self.1 {
            GTPathModuleId::Resolved(module_id) => Ok(RSPath(
                module_id.clone(),
                rs_parse_module_path(context.resolve_path(self)),
            )),

            GTPathModuleId::Unresolved => {
                Err(RSConverterError::UnresolvedPath(self.0.clone()).into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_parser::GTModuleId;
    use pretty_assertions::assert_eq;

    use crate::{context::RSConvertContext, resolve::RSConvertResolve};

    use super::*;

    #[test]
    fn test_convert_base() {
        assert_eq!(
            RSPath(
                GTModuleId("module/path".into()),
                "super::path::to::module".into()
            ),
            GTPath::new(
                (0, 0).into(),
                GTPathModuleId::Resolved("module/path".into()),
                "./path/to/module".into()
            )
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
        );
    }

    #[test]
    fn test_convert_absolute() {
        assert_eq!(
            RSPath(GTModuleId("module/path".into()), "path::to::module".into()),
            GTPath::new(
                (0, 0).into(),
                GTPathModuleId::Resolved("module/path".into()),
                "path/to/module".into()
            )
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
        );
    }

    #[test]
    fn test_convert_up() {
        assert_eq!(
            RSPath(
                GTModuleId("module/path".into()),
                "super::super::path::to::module".into()
            ),
            GTPath::new(
                (0, 0).into(),
                GTPathModuleId::Resolved("module/path".into()),
                "../path/to/module".into(),
            )
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut resolve = RSConvertResolve::default();
        resolve.paths.insert(
            GTPath::new(
                (0, 0).into(),
                GTPathModuleId::Resolved("module/path".into()),
                "./path/to/module".into(),
            ),
            GTPath::new(
                (0, 0).into(),
                GTPathModuleId::Resolved("module/path".into()),
                "./path/to/another/module".into(),
            ),
        );
        let mut context = RSConvertContext::new("module".into(), resolve, Default::default());
        assert_eq!(
            RSPath(
                GTModuleId("module/path".into()),
                "super::path::to::another::module".into()
            ),
            GTPath::new(
                (0, 0).into(),
                GTPathModuleId::Resolved("module/path".into()),
                "./path/to/module".into()
            )
            .convert(&mut context)
            .unwrap(),
        );
    }
}
