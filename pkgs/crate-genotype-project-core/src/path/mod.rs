//! Project paths module. It contains all the path types used in the project, organized in
//! a hierarchy based on their relation to the project structure.
//!
//! Hierarchy:
//!
//! - [cwd]
//!   - [config]
//!     - [root]
//!       - [src]
//!         - [entry]
//!         - [module]
//!       - [dist]
//!         - [pkg]
//!          - [pkg_src]

use crate::prelude::internal::*;

// region: Base paths

mod cwd;
pub use cwd::*;

mod config;
pub use config::*;

mod root;
pub use root::*;

// endregion

// region: Src paths

mod src;
pub use src::*;

mod entry;
pub use entry::*;

mod module;
pub use module::*;

// endregion

// region: Dist paths

mod dist;
pub use dist::*;

mod pkg;
pub use pkg::*;

mod pkg_src;
pub use pkg_src::*;

// endregion

// region: Traits

pub trait GtpRelativePath {
    fn new(path: RelativePathBuf) -> Self;

    fn relative_path(&self) -> &RelativePathBuf;

    fn as_str(&self) -> &str {
        self.relative_path().as_str()
    }

    fn with_extension<Str: AsRef<str>>(&self, ext: Str) -> Self
    where
        Self: Sized,
    {
        Self::new(self.relative_path().with_extension(ext))
    }

    fn join_relative_path(&self, path: &RelativePathBuf) -> Self
    where
        Self: Sized,
    {
        Self::new(self.relative_path().join_normalized(path))
    }

    fn join_str(&self, path: &str) -> Self
    where
        Self: Sized,
    {
        self.join_relative_path(&path.into())
    }

    fn to_parent(&self) -> Option<Self>
    where
        Self: Sized,
    {
        self.relative_path()
            .parent()
            .map(|parent| Self::new(parent.into()))
    }

    fn to_self_with_parents(&self) -> Successors<Self, fn(&Self) -> Option<Self>>
    where
        Self: Sized + Clone,
    {
        std::iter::successors(Some(self.clone()), Self::to_parent)
    }

    fn to_parents(&self) -> Successors<Self, fn(&Self) -> Option<Self>>
    where
        Self: Sized + Clone,
    {
        std::iter::successors(self.to_parent(), Self::to_parent)
    }

    fn display(&self) -> String {
        format!("{}", self.relative_path())
    }

    fn to_path_buf(&self) -> PathBuf {
        self.relative_path().to_path(".")
    }
}

// endregion
