use crate::prelude::internal::*;

mod convert;
pub use convert::*;

pub trait PyContextResolve {
    fn resolve(self, context: &mut PyConvertContext) -> Self;
}
