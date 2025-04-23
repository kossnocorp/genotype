use crate::prelude::internal::*;

impl TSConvert<TSDoc> for GTDoc {
    fn convert(&self, _context: &mut TSConvertContext) -> TSDoc {
        TSDoc(self.1.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        assert_eq!(
            TSDoc("Hello, world!".into()),
            GTDoc((0, 0).into(), "Hello, world!".into()).convert(&mut Default::default()),
        );
    }
}
