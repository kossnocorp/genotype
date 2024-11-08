use genotype_lang_rs_tree::path::RSPath;
use genotype_parser::tree::path::GTPath;

use crate::{context::RSConvertContext, convert::RSConvert};

pub fn rs_parse_module_path(path: String) -> String {
    path.replace("../", "..")
        .replace("./", ".")
        .replace("/", ".")
}

impl RSConvert<RSPath> for GTPath {
    fn convert(&self, context: &mut RSConvertContext) -> RSPath {
        RSPath(rs_parse_module_path(context.resolve_path(self)))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{context::RSConvertContext, resolve::RSConvertResolve};

    use super::*;

    #[test]
    fn test_convert_base() {
        assert_eq!(
            RSPath(".path.to.module".into()),
            GTPath::parse((0, 0).into(), "./path/to/module")
                .unwrap()
                .convert(&mut RSConvertContext::default()),
        );
    }

    #[test]
    fn test_convert_absolute() {
        assert_eq!(
            RSPath("module.path".into()),
            GTPath::parse((0, 0).into(), "module/path")
                .unwrap()
                .convert(&mut RSConvertContext::default()),
        );
    }

    #[test]
    fn test_convert_up() {
        assert_eq!(
            RSPath("..path.to.module".into()),
            GTPath::parse((0, 0).into(), "../path/to/module")
                .unwrap()
                .convert(&mut RSConvertContext::default()),
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut resolve = RSConvertResolve::default();
        resolve.paths.insert(
            GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            GTPath::parse((0, 0).into(), "./path/to/another/module").unwrap(),
        );
        let mut context = RSConvertContext::new(resolve, Default::default());
        assert_eq!(
            RSPath(".path.to.another.module".into()),
            GTPath::parse((0, 0).into(), "./path/to/module")
                .unwrap()
                .convert(&mut context),
        );
    }
}
