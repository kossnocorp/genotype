use crate::prelude::internal::*;

impl TSConvert<TSKey> for GTKey {
    fn convert(&self, _context: &mut TSConvertContext) -> TSKey {
        TSKey(self.1.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        assert_eq!(
            TSKey("foo".into()),
            GTKey::new((0, 0).into(), "foo".into()).convert(&mut Default::default()),
        );
    }
}
