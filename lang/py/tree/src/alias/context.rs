use crate::{PYContext, PYContextResolve, PYVersion};

use super::PYAlias;

impl PYContextResolve for PYAlias {
    fn resolve(self, context: &mut PYContext, options: &crate::PYOptions) -> Self {
        if let PYVersion::Legacy = options.version {
            context
                .imports
                .insert(("typing".into(), "TypeAlias".into()));
        }
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
        let alias = PYAlias {
            name: "Foo".into(),
            descriptor: PYPrimitive::String.into(),
        };
        alias.resolve(&mut context, &PYOptions::default());
        assert_eq!(context, PYContext::new());
    }

    #[test]
    fn test_resolve_legacy() {
        let mut context = PYContext::new();
        let alias = PYAlias {
            name: "Foo".into(),
            descriptor: PYPrimitive::String.into(),
        };
        alias.resolve(&mut context, &PYOptions::new(PYVersion::Legacy));
        assert_eq!(
            context,
            PYContext {
                imports: HashSet::from_iter(vec![("typing".into(), "TypeAlias".into())]),
                ..PYContext::new()
            }
        );
    }
}
