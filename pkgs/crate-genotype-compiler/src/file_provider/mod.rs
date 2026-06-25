use crate::prelude::internal::*;

mod system;
pub use system::*;

pub trait GtcFileProvider {
    fn file_exists(&self, path: &RelativePathBuf) -> Result<bool>;

    fn file_write(&self, path: &RelativePathBuf, content: &str) -> Result<()>;
}
