use crate::prelude::internal::*;

impl TSConvert<TSPath> for GTPath {
    fn convert(&self, context: &mut TSConvertContext) -> TSPath {
        TSPath(context.resolve_path(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert_base() {
        assert_eq!(
            TSPath("./path/to/module.ts".into()),
            GTPath::parse((0, 0).into(), "./path/to/module")
                .unwrap()
                .convert(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut resolve = TSConvertResolve::new();
        resolve.paths.insert(
            GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            GTPath::parse((0, 0).into(), "./path/to/module/index").unwrap(),
        );
        assert_eq!(
            TSPath("./path/to/module/index.ts".into()),
            GTPath::parse((0, 0).into(), "./path/to/module")
                .unwrap()
                .convert(&mut TSConvertContext::new(resolve, Default::default())),
        );
    }
}
