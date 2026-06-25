use crate::prelude::internal::*;

mod system;
pub use system::*;

pub trait GtCompiler<Input, Output> {
    fn build_once(input: Input) -> Result<Output>;
}
