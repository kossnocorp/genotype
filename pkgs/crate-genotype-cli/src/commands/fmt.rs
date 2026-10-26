use crate::prelude::internal::*;

#[derive(Args)]
pub struct GtFmtCommand {}

impl GtFmtCommand {
    pub fn run(_args: &GtFmtCommand) -> Result<()> {
        let cwd = std::env::current_dir().into_diagnostic()?;
        let count = Self::format_directory(&cwd)?;
        println!("Formatted {count} file(s)");
        Ok(())
    }

    fn format_directory(root: &Path) -> Result<usize> {
        let mut paths = vec![];
        for entry in ignore::WalkBuilder::new(root).require_git(false).build() {
            let entry = entry.into_diagnostic()?;
            if entry.file_type().is_some_and(|kind| kind.is_file())
                && entry.path().extension().is_some_and(|ext| ext == "type")
            {
                paths.push(entry.into_path());
            }
        }
        paths.sort();

        // Parse every input before writing, so syntax errors cannot leave a partially
        // formatted project. Formatting needs no config, import resolution, or build.
        let mut changes = vec![];
        for path in paths {
            let source = std::fs::read_to_string(&path)
                .into_diagnostic()
                .wrap_err_with(|| format!("Failed to read {}", path.display()))?;
            let formatted = genotype_fmt::GtfmtFormatter::format(&source).map_err(|error| {
                miette::Report::new(error).with_source_code(miette::NamedSource::new(
                    path.display().to_string(),
                    source.clone(),
                ))
            })?;
            if source != formatted {
                changes.push((path, formatted));
            }
        }
        for (path, formatted) in &changes {
            std::fs::write(path, formatted)
                .into_diagnostic()
                .wrap_err_with(|| format!("Failed to write {}", path.display()))?;
        }
        Ok(changes.len())
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn formats_type_files_recursively_and_respects_ignores() {
        let root = tempfile::tempdir().unwrap();
        std::fs::create_dir(root.path().join("src")).unwrap();
        std::fs::write(root.path().join(".gitignore"), "ignored.type\n").unwrap();
        std::fs::write(root.path().join("ignored.type"), "invalid").unwrap();
        std::fs::write(root.path().join("src/a.type"), "A:{x:string}").unwrap();
        std::fs::write(root.path().join("other.txt"), "untouched").unwrap();
        assert_eq!(GtFmtCommand::format_directory(root.path()).unwrap(), 1);
        assert_snapshot!(
            std::fs::read_to_string(root.path().join("src/a.type")).unwrap(),
            @"A: { x: string, }\n"
        );
        assert_snapshot!(
            std::fs::read_to_string(root.path().join("ignored.type")).unwrap(),
            @"invalid"
        );
        assert_snapshot!(
            std::fs::read_to_string(root.path().join("other.txt")).unwrap(),
            @"untouched"
        );
        assert_eq!(GtFmtCommand::format_directory(root.path()).unwrap(), 0);
    }

    #[test]
    fn syntax_errors_do_not_modify_files() {
        let root = tempfile::tempdir().unwrap();
        std::fs::write(root.path().join("a.type"), "A:string").unwrap();
        std::fs::write(root.path().join("b.type"), "B:").unwrap();
        assert!(GtFmtCommand::format_directory(root.path()).is_err());
        assert_snapshot!(
            std::fs::read_to_string(root.path().join("a.type")).unwrap(),
            @"A:string"
        );
        assert_snapshot!(
            std::fs::read_to_string(root.path().join("b.type")).unwrap(),
            @"B:"
        );
    }
}
