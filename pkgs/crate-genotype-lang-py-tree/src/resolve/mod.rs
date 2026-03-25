use crate::prelude::internal::*;

mod convert;
pub use convert::*;

pub trait PyContextResolve {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PyConvertContextConstraint;
}
