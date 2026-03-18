use crate::prelude::internal::*;

impl TSConvert<TSAny> for GTAny {
    fn convert(&self, _context: &mut TSConvertContext) -> TSAny {
        TSAny
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(GTAny((0, 0).into()).convert(&mut Default::default()), @"TSAny");
    }
}
