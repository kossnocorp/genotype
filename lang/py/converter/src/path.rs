use genotype_lang_py_tree::path::PYPath;
use genotype_parser::tree::path::GTPath;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYPath> for GTPath {
    fn convert(&self, context: &mut PYConvertContext) -> PYPath {
        PYPath(
            context
                .resolve
                .paths
                .get(&self)
                .unwrap_or(self)
                .as_str()
                .to_owned()
                + ".ts",
        )
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::context::PYConvertContext;

    use super::*;

    #[test]
    fn test_convert_base() {
        assert_eq!(
            PYPath("./path/to/module.ts".into()),
            GTPath::parse((0, 0).into(), "./path/to/module")
                .unwrap()
                .convert(&mut PYConvertContext::default()),
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = PYConvertContext::default();
        context.resolve.paths.insert(
            GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            GTPath::parse((0, 0).into(), "./path/to/module/index").unwrap(),
        );
        assert_eq!(
            PYPath("./path/to/module/index.ts".into()),
            GTPath::parse((0, 0).into(), "./path/to/module")
                .unwrap()
                .convert(&mut context),
        );
    }
}
