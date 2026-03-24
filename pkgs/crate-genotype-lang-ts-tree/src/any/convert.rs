use crate::prelude::internal::*;

impl TSConvert<TSAny> for GTAny {
    fn convert(&self, _context: &mut TSConvertContext) -> TSAny {
        TSAny
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
            @"TSAny"
        );
    }
}
