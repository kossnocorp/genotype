# Parent-Relative Trait Sized Bound

Adding a generic parent-relative conversion trait introduced an object-safety related type-size error.

## Description

I introduced a new trait `GtpParentRelativePath` to convert parent-relative paths into cwd-relative paths, with:

- `type ParentDirPath: GtpCwdRelativeDirPathWrapper<Self>`
- `fn to_cwd_relative_path(&self, parent_dir_path: &Self::ParentDirPath) -> GtpCwdRelativePath`

I expected this to compile because all concrete path types in this hierarchy are sized structs.

What actually happened: compilation failed with `E0277` complaining that `Self` does not have a known size at compile time when used as `ChildPath` in `GtpCwdRelativeDirPathWrapper<Self>`.

## Analysis

Root cause: trait parameters are `?Sized` unless constrained. The associated type bound `GtpCwdRelativeDirPathWrapper<Self>` required `Self` to satisfy an implicit `Sized` bound in `GtpCwdRelativeDirPathWrapper<ChildPath>`, but `GtpParentRelativePath` itself did not require `Self: Sized`.

Fix applied: changed the trait declaration to `pub trait GtpParentRelativePath: GtpRelativePath + Sized`.

This matches the intended usage (concrete struct path types) and resolves the bound mismatch without relaxing `GtpCwdRelativeDirPathWrapper` to `?Sized`.
