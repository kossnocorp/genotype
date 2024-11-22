use genotype_lang_ts_tree::*;
use genotype_parser::GTAny;

use crate::{context::TSConvertContext, convert::TSConvert};

impl TSConvert<TSAny> for GTAny {
    fn convert(&self, _context: &mut TSConvertContext) -> TSAny {
        TSAny
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(GTAny((0, 0).into()).convert(&mut Default::default()), TSAny);
    }
}
