use crate::{PYContext, PYContextResolve, PYOptions};

use super::PYClass;

impl PYContextResolve for PYClass {
    fn resolve(self, context: &mut PYContext, _options: &PYOptions) -> Self {
        context
            .imports
            .insert(("dataclasses".into(), "dataclass".into()));
        self
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_resolve() {
        let mut context = PYContext::new();
        let alias = PYClass {
            name: "Foo".into(),
            extensions: vec![],
            properties: vec![],
        };
        alias.resolve(&mut context, &PYOptions::default());
        assert_eq!(
            context,
            PYContext {
                imports: HashSet::from_iter(vec![("dataclasses".into(), "dataclass".into())]),
            }
        );
    }
}
