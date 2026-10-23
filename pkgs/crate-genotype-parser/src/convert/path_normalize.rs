use crate::prelude::internal::*;

impl GtParsePath for GtPath {
    fn parse(span: GtSpan, module_id: &GtModuleId, path: &str) -> GtNodeParseResult<Self> {
        match normalize_path(path) {
            Ok(path) => Ok(GtPath {
                span,
                id: GtPathModuleId::new(span, module_id.clone()),
                path: path.into(),
            }),

            Err(_) => Err(GtParseError::Internal(
                span,
                GtNode::Path,
                "failed to normalize path",
            )),
        }
    }
}

fn normalize_path(path: &str) -> Result<String, OsString> {
    let mut result = PathBuf::new();
    let mut components = Path::new(path).components().peekable();

    let mut leading = Vec::new();
    while let Some(&component) = components.peek() {
        match component {
            Component::CurDir | Component::ParentDir => {
                leading.push(component.as_os_str().to_owned());
                components.next();
            }
            _ => break,
        }
    }

    for component in components {
        match component {
                Component::CurDir => {
                    // Skip redundant .
                }

                Component::ParentDir
                    // Pop the last component if possible, else push ..
                    if !result.pop() => {
                        result.push("..");
                    }

                Component::Normal(c) => {
                    result.push(c);
                }

                _ => {}
            }
    }

    let mut normalized = PathBuf::new();
    for component in leading {
        normalized.push(component);
    }
    normalized.push(result);

    normalized.into_os_string().into_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_path() {
        assert_eq!(
            GtPath::parse((0, 0).into(), &"module".into(), "./path/to/../module")
                .unwrap()
                .source_str(),
            "./path/module"
        );
        assert_eq!(
            GtPath::parse((0, 0).into(), &"module".into(), "./path/./to/./module")
                .unwrap()
                .source_str(),
            "./path/to/module"
        );
        assert_eq!(
            GtPath::parse((0, 0).into(), &"module".into(), "./path/./to/./module/")
                .unwrap()
                .source_str(),
            "./path/to/module"
        );
        assert_eq!(
            GtPath::parse(
                (0, 0).into(),
                &"module".into(),
                "path/./to/./module/../module"
            )
            .unwrap()
            .source_str(),
            "path/to/module"
        );
        assert_eq!(
            GtPath::parse(
                (0, 0).into(),
                &"module".into(),
                "./././path/./to/./module/../module"
            )
            .unwrap()
            .source_str(),
            "./path/to/module"
        );
        assert_eq!(
            GtPath::parse(
                (0, 0).into(),
                &"module".into(),
                "../../../path/./to/./module/../module"
            )
            .unwrap()
            .source_str(),
            "../../../path/to/module"
        );
    }
}
