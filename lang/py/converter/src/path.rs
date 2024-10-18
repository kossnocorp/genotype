use genotype_lang_py_tree::path::PYPath;
use genotype_parser::tree::path::GTPath;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYPath> for GTPath {
    fn convert(&self, context: &mut PYConvertContext) -> PYPath {
        PYPath(
            context
                .resolve_path(self)
                .replace("../", "..")
                .replace("./", ".")
                .replace("/", "."),
        )
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{context::PYConvertContext, resolve::PYConvertResolve};

    use super::*;

    #[test]
    fn test_convert_base() {
        assert_eq!(
            PYPath(".path.to.module".into()),
            GTPath::parse((0, 0).into(), "./path/to/module")
                .unwrap()
                .convert(&mut PYConvertContext::default()),
        );
    }

    #[test]
    fn test_convert_absolute() {
        assert_eq!(
            PYPath("module.path".into()),
            GTPath::parse((0, 0).into(), "module/path")
                .unwrap()
                .convert(&mut PYConvertContext::default()),
        );
    }

    #[test]
    fn test_convert_up() {
        assert_eq!(
            PYPath("..path.to.module".into()),
            GTPath::parse((0, 0).into(), "../path/to/module")
                .unwrap()
                .convert(&mut PYConvertContext::default()),
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
        assert_eq!(
            PYPath(".path.to.another.module".into()),
            GTPath::parse((0, 0).into(), "./path/to/module")
                .unwrap()
                .convert(&mut context),
        );
    }
}
