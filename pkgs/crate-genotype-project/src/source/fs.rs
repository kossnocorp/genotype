use crate::prelude::internal::*;
use glob::glob;

/// File system project source. It provides file system interop for the project loader.
/// It is the default project source used by the system project runtime.
pub trait GtpSourceFs {
    /// Returns the base project directory to resolve relative file paths.
    fn base_path(&self) -> &GtpCwdRelativePath;

    /// Resolves full path from the given relative path using the file system.
    fn resolve_path_buf(&self, path: &GtpCwdRelativePath) -> PathBuf {
        path.to_path_buf()
    }
}

impl<Type: GtpSourceFs + ?Sized> GtpSource for Type {
    /// Globs files from the given path using the file system.
    fn glob(&self, path: &GtpCwdRelativePath) -> Result<Vec<GtpModulePath>> {
        let path_buf = path.to_path_buf();
        let path_str = path_buf
            .to_str()
            .ok_or_else(|| miette!("failed to convert path '{}' to string", path_buf.display()))?;

        let module_paths = glob(path_str)
            .map_err(|err| {
                miette!(
                    labels = vec![LabeledSpan::at_offset(err.pos, "here")],
                    "invalid glob pattern: {path_str}",
                )
                .with_source_code(path_str.to_string())
            })?
            .map(|file_result| {
                file_result
                    .map_err(|e| miette!(e))
                    .wrap_err("failed to read file path from glob pattern")
                    .and_then(|file_path| {
                        RelativePathBuf::from_path(file_path)
                            .map_err(|e| miette!(e))
                            .wrap_err("failed to convert file path from glob into relative path")
                    })
                    .map(|file_path| GtpModulePath::from_cwd_relative_path(file_path.into()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(module_paths)
    }

    /// Reads a file from the given path using the file system.
    fn read_file(&self, path: &GtpCwdRelativePath) -> Result<String> {
        let source = fs::read_to_string(self.resolve_path_buf(path))
            .map_err(|e| miette!(e))
            .wrap_err_with(|| format!("failed to read file '{}'", path.display()))?;
        Ok(source)
    }

    /// Checks if the given path is a file using the file system.
    fn is_file(&self, path: &GtpCwdRelativePath) -> bool {
        self.resolve_path_buf(path).is_file()
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
            "reached the root directory without finding the file '{file_name}' starting from '{}'",
            self.base_path().display()
        ))
    }
}
