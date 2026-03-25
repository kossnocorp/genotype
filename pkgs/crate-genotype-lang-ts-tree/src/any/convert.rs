use crate::prelude::internal::*;

impl TsConvert<TsAny> for GtAny {
    fn convert(&self, _context: &mut TsConvertContext) -> TsAny {
        TsAny
    }
}

#[cfg(test)]
mod tests {
    use crate::test::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            convert_node(Gt::any()),
            @"TsAny"
        );
    }
}
