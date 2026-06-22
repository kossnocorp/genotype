use crate::prelude::internal::*;

mod system;
pub use system::*;

pub trait GtcFileProvider {
    fn file_exists(&self, path: &Path) -> bool;

    fn file_write(&self, path: &Path, content: &str) -> Result<()>;
}
