use genotype_lang_py_tree::{definition::PYDefinition, path::PYPath};
use genotype_parser::tree::path::GTPath;

use crate::{convert::PYConvert, resolve::PYConvertResolve};

impl PYConvert<PYPath> for GTPath {
    fn convert<HoistFn>(&self, resolve: &PYConvertResolve, _hoist: &HoistFn) -> PYPath
    where
        HoistFn: Fn(PYDefinition),
    {
        PYPath(resolve.paths.get(&self).unwrap_or(self).as_str().to_owned() + ".ts")
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert_base() {
        assert_eq!(
            PYPath("./path/to/module.ts".into()),
            GTPath::parse((0, 0).into(), "./path/to/module")
                .unwrap()
                .convert(&PYConvertResolve::new(), &|_| {}),
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut resolve = PYConvertResolve::new();
        resolve.paths.insert(
            GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            GTPath::parse((0, 0).into(), "./path/to/module/index").unwrap(),
        );
        assert_eq!(
            PYPath("./path/to/module/index.ts".into()),
            GTPath::parse((0, 0).into(), "./path/to/module")
                .unwrap()
                .convert(&resolve, &|_| {}),
        );
    }
}
