use crate::*;

use super::PYTuple;

impl PYContextResolve for PYTuple {
    fn resolve(self, context: &mut PYContext, options: &PYOptions) -> Self {
        if let PYVersion::Legacy = options.version {
            context.imports.insert(("typing".into(), "Tuple".into()));
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
        let tuple = PYTuple {
            descriptors: vec![PYPrimitive::String.into()],
        };
        tuple.resolve(&mut context, &PYOptions::default());
        assert_eq!(context, PYContext::new());
    }

    #[test]
    fn test_resolve_legacy() {
        let mut context = PYContext::new();
        let tuple = PYTuple {
            descriptors: vec![PYPrimitive::String.into()],
        };
        tuple.resolve(&mut context, &PYOptions::new(PYVersion::Legacy));
        assert_eq!(
            context,
            PYContext {
                imports: HashSet::from_iter(vec![("typing".into(), "Tuple".into())]),
            }
        );
    }
}
