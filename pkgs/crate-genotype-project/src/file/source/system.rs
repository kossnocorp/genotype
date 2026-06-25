use crate::prelude::internal::*;

use glob::glob;

/// System project file source. It provides file system interop for the project. It is the default
/// system project source used by the system project runtime.
pub trait GtpFileSourceSystem: GtpFileAccessSystem {}

impl<Type: GtpFileSourceSystem + ?Sized> GtpFileSource for Type {
    /// Globs files from the given path using the file system.
    fn glob_files(&self, path: &GtpCwdRelativePath) -> Result<Vec<GtpCwdRelativePath>> {
        let path_buf = path.to_path_buf();
        let path_str = path_buf
            .to_str()
            .ok_or_else(|| miette!("Failed to convert path '{}' to string", path_buf.display()))?;

        let paths = glob(path_str)
            .map_err(|err| {
                miette!(
                    labels = vec![LabeledSpan::at_offset(err.pos, "here")],
                    "Invalid glob pattern: {path_str}",
                )
                .with_source_code(path_str.to_string())
            })?
            .map(|file_result| {
                file_result
                    .map_err(|e| miette!(e))
                    .wrap_err("Failed to read file path from glob pattern")
                    .and_then(|file_path| {
                        RelativePathBuf::from_path(file_path)
                            .map_err(|e| miette!(e))
                            .wrap_err("Failed to convert file path from glob into relative path")
                    })
                    .map(|file_path| file_path.into())
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(paths)
    }

    /// Reads a file from the given path using the file system.
    fn read_file(&self, path: &GtpCwdRelativePath) -> Result<String> {
        let source = fs::read_to_string(self.resolve_path_buf(path))
            .map_err(|e| miette!(e))
            .wrap_err_with(|| {
                format!("File `{path}` doesn't exist or don't have permission to read it")
            })?;
        Ok(source)
    }

    /// Checks if the given path exists using the file system.
    fn file_exists(&self, path: &GtpCwdRelativePath) -> Result<bool> {
        Ok(self.resolve_path_buf(path).exists())
    }

    /// Checks if the given path is a file using the file system.
    fn is_file(&self, path: &GtpCwdRelativePath) -> Result<bool> {
        Ok(self.resolve_path_buf(path).is_file())
    }

    /// Searches for a file path using the file system starting from the base path.
    fn find_file(&self, file_name: &str) -> Result<GtpCwdRelativePath> {
        for parent in self.base_path().to_self_with_parents() {
            let file = parent.join_str(file_name);
            if file.to_path_buf().is_file() {
                return Ok(file);
            }
        }

        Err(miette!(
            "Reached the root directory without finding the file '{file_name}' starting from '{}'",
            self.base_path().display()
        ))
    }
}
