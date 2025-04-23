use crate::prelude::internal::*;

impl TSConvert<TSAny> for GTAny {
    fn convert(&self, _context: &mut TSConvertContext) -> TSAny {
        TSAny
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        assert_eq!(GTAny((0, 0).into()).convert(&mut Default::default()), TSAny);
    }
}
