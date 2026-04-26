# Hard Switch Missing Blanket Impl

Switching from `GtpCwdRelativeDirPathWrapper` to `GtpDirPath` caused a broad compile failure due to an unnoticed foundational trait impl change.

## Description

I performed a hard switch to option 3 by introducing `GtpDirPath`, adding `gtp_dir_path_wrapper_newtype!`, and updating parent-relative mappings.

I expected type-level parent/dir relationships to compile after replacing old dir-wrapper links.

What actually happened: compilation failed with many `trait bound ... GtpRelativePath is not satisfied` errors for wrapper-based path structs, plus missing method errors (`relative_path`, `display`, `new`) inside trait default methods.

## Analysis

Root cause was a combination of three issues:

1. The blanket implementation `impl<Type: GtpRelativePathWrapper> GtpRelativePath for Type` had been commented out. That removed `GtpRelativePath` behavior from all wrapper newtypes.
2. `gtp_relative_path_wrapper_trait!` did not require `GtpRelativePath` as a supertrait, so methods in `extra` blocks using `relative_path`/`display` became invalid when the blanket behavior was missing.
3. `GtpSrcDirPath` still used `gtp_relative_path_wrapper_newtype!(... marker: GtpCwdRelativePathWrapper)` instead of the new directory-specific macro, so it did not implement `GtpDirPath` while being used as a parent dir type.

Fix applied:

- Restored the blanket impl of `GtpRelativePath` for all `GtpRelativePathWrapper` types.
- Updated `gtp_relative_path_wrapper_trait!` to require `+ GtpRelativePath`.
- Switched `GtpSrcDirPath` to `gtp_dir_path_wrapper_newtype!`.

After these changes, `cargo check -p genotype_project_core` and `cargo check -p genotype_project` succeeded (warnings only).
