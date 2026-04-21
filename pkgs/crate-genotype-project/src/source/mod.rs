use crate::prelude::internal::*;

mod fs;
pub use fs::*;

pub trait GtProjectSource {
    fn glob() -> ();

    fn search() -> ();

    fn read() -> ();
}
