use crate::prelude::internal::*;

pub trait PYContextResolve {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PYConvertContextConstraint;
}
