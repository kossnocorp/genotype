use genotype_lang_ts_tree::{definition::TSDefinition, path::TSPath};
use genotype_parser::tree::path::GTPath;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSPath> for GTPath {
    fn convert<HoistFn>(&self, resolve: &TSConvertResolve, _hoist: &HoistFn) -> TSPath
    where
        HoistFn: Fn(TSDefinition),
    {
        TSPath(resolve.paths.get(&self).unwrap_or(self).as_str().to_owned() + ".ts")
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert_base() {
        assert_eq!(
            TSPath("./path/to/module.ts".into()),
            GTPath::new("./path/to/module".into()).convert(&TSConvertResolve::new(), &|_| {}),
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut resolve = TSConvertResolve::new();
        resolve
            .paths
            .insert("./path/to/module".into(), "./path/to/module/index".into());
        assert_eq!(
            TSPath("./path/to/module/index.ts".into()),
            GTPath::new("./path/to/module".into()).convert(&resolve, &|_| {}),
        );
    }
}
