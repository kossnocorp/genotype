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
//!
//! Traits:
//!
//! - [GtpRelativePath]: Base trait for all relative paths.
//! - [GtpRelativePathWrapper]: Trait for types that wrap a type that wraps [GtpRelativePath].

use crate::prelude::internal::*;

// region: Macros

macro_rules! gtp_relative_path_newtype {
    ($(#[$meta:meta])* $vis:vis struct $name:ident; parent: $parent:ty;) => {
        gtp_relative_path_newtype!(
            $(#[$meta])*
            $vis struct $name;
        );

        impl GtpParentRelativePath for $name {
            type ParentDirPath = $parent;
        }
    };

    ($(#[$meta:meta])* $vis:vis struct $name:ident;) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        $vis struct $name(RelativePathBuf);

        impl GtpRelativePath for $name {
            fn new(path: RelativePathBuf) -> Self {
                Self(path.normalize())
            }

            fn relative_path(&self) -> &RelativePathBuf {
                &self.0
            }
        }

        impl From<RelativePathBuf> for $name {
            fn from(path: RelativePathBuf) -> Self {
                Self::new(path)
            }
        }

        impl From<&str> for $name {
            fn from(path: &str) -> Self {
                Self::new(path.into())
            }
        }
    };
}

macro_rules! gtp_relative_path_wrapper_newtype {
    ($(#[$meta:meta])* $vis:vis struct $name:ident($inner:ty); parent: $parent:ty;) => {
        gtp_relative_path_wrapper_newtype!(
            $(#[$meta])*
            $vis struct $name($inner);
        );

        impl GtpParentRelativePath for $name {
            type ParentDirPath = $parent;
        }
    };

    ($(#[$meta:meta])* $vis:vis struct $name:ident($inner:ty); marker: $marker:path; parent: $parent:ty;) => {
        gtp_relative_path_wrapper_newtype!(
            $(#[$meta])*
            $vis struct $name($inner);
            marker: $marker;
        );

        impl GtpParentRelativePath for $name {
            type ParentDirPath = $parent;
        }
    };

    ($(#[$meta:meta])* $vis:vis struct $name:ident($inner:ty);) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        $vis struct $name($inner);

        impl GtpRelativePathWrapper for $name {
            type Inner = $inner;

            fn from_inner(path: Self::Inner) -> Self {
                Self(path)
            }

            fn inner(&self) -> &Self::Inner {
                &self.0
            }
        }

        impl From<$inner> for $name {
            fn from(path: $inner) -> Self {
                Self(path)
            }
        }

        impl From<&str> for $name {
            fn from(path: &str) -> Self {
                Self::new(path.into())
            }
        }
    };

    ($(#[$meta:meta])* $vis:vis struct $name:ident($inner:ty); marker: $marker:path;) => {
        gtp_relative_path_wrapper_newtype!(
            $(#[$meta])*
            $vis struct $name($inner);
        );

        impl $marker for $name {}
    };
}

macro_rules! gtp_cwd_relative_path_wrapper_newtype {
    ($(#[$meta:meta])* $vis:vis struct $name:ident($inner:ty);) => {
        gtp_relative_path_wrapper_newtype!(
            $(#[$meta])*
            $vis struct $name(GtpCwdRelativePath);
        );


        impl $name {
            pub fn from_cwd_relative_path(path: GtpCwdRelativePath) -> Self {
                Self::from_inner(path)
            }

            pub fn cwd_relative_path(&self) -> &GtpCwdRelativePath {
                self.inner()
            }
        }

        impl AsRef<GtpCwdRelativePath> for $name {
            fn as_ref(&self) -> &GtpCwdRelativePath {
                self.cwd_relative_path()
            }
        }
    };
}

macro_rules! gtp_cwd_relative_dir_path_wrapper_newtype {
    ($(#[$meta:meta])* $vis:vis struct $name:ident($inner:ty);) => {
        gtp_cwd_relative_path_wrapper_newtype!(
            $(#[$meta])*
            $vis struct $name($inner);
        );

        impl GtpDirPath for $name {}
    };
}

// endregion

// region: Modules

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

// endregion

// region: Traits

pub trait GtpRelativePath {
    fn new(path: RelativePathBuf) -> Self;

    fn from_str(path: &str) -> Self
    where
        Self: Sized,
    {
        Self::new(path.into())
    }

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

pub trait GtpRelativePathWrapper {
    type Inner: GtpRelativePath + From<RelativePathBuf>;

    fn from_inner(path: Self::Inner) -> Self;

    fn inner(&self) -> &Self::Inner;
}

impl<Type: GtpRelativePathWrapper> GtpRelativePath for Type {
    fn new(path: RelativePathBuf) -> Self {
        Self::from_inner(path.into())
    }

    fn relative_path(&self) -> &RelativePathBuf {
        self.inner().relative_path()
    }
}

// region: Cwd-relative dir path trait

pub trait GtpDirPath: GtpRelativePath {
    fn join_as_cwd_relative_path<ChildPath>(&self, path: &ChildPath) -> GtpCwdRelativePath
    where
        ChildPath: GtpParentRelativePath<ParentDirPath = Self>,
        Self: Sized,
    {
        GtpCwdRelativePath::new(self.relative_path().join_normalized(path.relative_path()))
    }

    fn join_str_as_cwd_relative_path(&self, path: &str) -> GtpCwdRelativePath {
        let relative_path = RelativePathBuf::from(path);
        GtpCwdRelativePath::new(self.relative_path().join_normalized(&relative_path))
    }
}

// endregion

// region: Parent-relative path trait

pub trait GtpParentRelativePath: GtpRelativePath + Sized {
    type ParentDirPath: GtpDirPath;

    fn to_cwd_relative_path(&self, parent_dir_path: &Self::ParentDirPath) -> GtpCwdRelativePath {
        parent_dir_path.join_as_cwd_relative_path(self)
    }
}

// endregion

// endregion
