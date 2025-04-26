use crate::prelude::internal::*;

mod convert;
pub use convert::*;

pub trait PYContextResolve {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PYConvertContextConstraint;
}
