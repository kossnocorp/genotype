use genotype_lang_ts_tree::*;
use genotype_parser::GTAny;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSAny> for GTAny {
    fn convert<HoistFn>(&self, _resolve: &TSConvertResolve, _hoist: &HoistFn) -> TSAny
    where
        HoistFn: Fn(TSDefinition),
    {
        TSAny
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::resolve::TSConvertResolve;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTAny((0, 0).into()).convert(&TSConvertResolve::new(), &|_| {}),
            TSAny
        );
    }
}
