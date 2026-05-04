use crate::prelude::internal::*;

/// Unique module path reference. It defines a particular path reference in the source code.
#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtPath {
    /// Where the path is defined in the source code.
    pub span: GtSpan,
    /// Path id.
    pub id: GtPathModuleId,
    /// Literal path string how it was defined in the source code.
    pub path: Arc<str>,
}

impl GtPath {
    pub fn new(span: GtSpan, module_id: GtPathModuleId, path: Arc<str>) -> Self {
        Self {
            span,
            id: module_id,
            path,
        }
    }

    /// Returns the literal path string ref.
    pub fn source_str(&self) -> &str {
        self.path.as_ref()
    }

    pub fn kind(&self) -> GtPathKind {
        if self.path.starts_with('.') || self.path.starts_with("~") {
            GtPathKind::Local
        } else {
            GtPathKind::Package
        }
    }

    pub fn package_path(&self) -> Option<(String, Option<String>)> {
        if self.kind() == GtPathKind::Package {
            Some(match self.path.find("/") {
                Some(index) => (
                    self.path[..index].to_owned(),
                    Some(self.path[index + 1..].to_owned()),
                ),
                None => (self.path.to_string(), None),
            })
        } else {
            None
        }
    }
}

#[derive(PartialEq)]
pub enum GtPathKind {
    /// Local path (starts with `.`, `..` or `~/`).
    Local,
    /// Package path (have to prefix).
    Package,
}

impl GtPath {
    pub fn split_parse(
        pair: Pair<'_, Rule>,
        module_id: &GtModuleId,
    ) -> GtNodeParseResult<(GtPath, (GtSpan, String))> {
        let span = pair.as_span();
        let span_start = span.start();
        let str = pair.as_str().to_string();
        let else_err = || GtParseError::InternalLegacy(span.into(), GtNode::Path);

        let name_index = str.rfind("/").ok_or_else(else_err)?;

        let path_str = str.get(..name_index).ok_or_else(else_err)?;
        let path_span = (span_start, span_start + name_index).into();
        let path = GtPath::parse(path_span, module_id, path_str)?;

        let name = str.get(name_index + 1..).ok_or_else(else_err)?;
        let name_span = (span_start + name_index + 1, span.end()).into();

        Ok((path, (name_span, name.into())))
    }

    pub fn parse(span: GtSpan, module_id: &GtModuleId, path: &str) -> GtNodeParseResult<Self> {
        match Self::normalize_path(path) {
            Ok(path) => Ok(GtPath {
                span,
                id: GtPathModuleId::new(span, module_id.clone()),
                path: path.into(),
            }),

            Err(_) => Err(GtParseError::InternalLegacy(span, GtNode::Path)),
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

                Component::ParentDir => {
                    // Pop the last component if possible, else push ..
                    if !result.pop() {
                        result.push("..");
                    }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::path, "./hello/world/").unwrap();
        let module_id: GtModuleId = "module".into();
        assert_eq!(
            GtPath::parse((0, 14).into(), &module_id, "./hello/world/"),
            pairs.next().unwrap().try_into()
        );
    }

    #[test]
    fn test_split() {
        let mut pairs = GenotypeParser::parse(Rule::inline_import, "./hello/World").unwrap();
        let module_id: GtModuleId = "module".into();
        let result = GtPath::split_parse(pairs.next().unwrap(), &module_id).unwrap();
        assert_eq!(
            result,
            (
                GtPath::parse((0, 7).into(), &module_id, "./hello").unwrap(),
                ((8, 13).into(), "World".into())
            )
        );
    }

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

    #[test]
    fn test_normalize_source() {
        let source_code = r#"use author/./*
            use ../user/../user/User
            use ./././misc/order/{Order, SomethingElse}

            Order: {
                book: book/Book,
                user: ./misc/../misc/./user/User
            }"#
        .to_owned();
        let parse = GtModule::parse("module".into(), &source_code).unwrap();
        assert_ron_snapshot!(
            parse.module,
            @r#"
        GtModule(
          id: GtModuleId("module"),
          doc: None,
          imports: [
            GtImport(
              span: GtSpan(0, 14),
              path: GtPath(
                span: GtSpan(4, 12),
                id: GtPathModuleId(
                  span: GtSpan(4, 12),
                  module_id: GtModuleId("module"),
                ),
                path: "author",
              ),
              reference: Glob(GtSpan(13, 14)),
            ),
            GtImport(
              span: GtSpan(27, 51),
              path: GtPath(
                span: GtSpan(31, 46),
                id: GtPathModuleId(
                  span: GtSpan(31, 46),
                  module_id: GtModuleId("module"),
                ),
                path: "../user",
              ),
              reference: Name(GtSpan(47, 51), GtIdentifier(GtSpan(47, 51), "User")),
            ),
            GtImport(
              span: GtSpan(64, 107),
              path: GtPath(
                span: GtSpan(68, 84),
                id: GtPathModuleId(
                  span: GtSpan(68, 84),
                  module_id: GtModuleId("module"),
                ),
                path: "./misc/order",
              ),
              reference: Names(GtSpan(85, 107), [
                Name(GtSpan(86, 91), GtIdentifier(GtSpan(86, 91), "Order")),
                Name(GtSpan(93, 106), GtIdentifier(GtSpan(93, 106), "SomethingElse")),
              ]),
            ),
          ],
          aliases: [
            GtAlias(
              id: GtDefinitionId(GtModuleId("module"), "Order"),
              span: GtSpan(121, 225),
              doc: None,
              attributes: [],
              name: GtIdentifier(GtSpan(121, 126), "Order"),
              generics: [],
              descriptor: Object(GtObject(
                span: GtSpan(128, 225),
                doc: None,
                attributes: [],
                name: Named(GtIdentifier(GtSpan(121, 126), "Order")),
                extensions: [],
                properties: [
                  GtProperty(
                    span: GtSpan(146, 161),
                    doc: None,
                    attributes: [],
                    name: GtKey(GtSpan(146, 150), "book"),
                    descriptor: InlineImport(GtInlineImport(
                      span: GtSpan(152, 161),
                      doc: None,
                      attributes: [],
                      name: GtIdentifier(GtSpan(157, 161), "Book"),
                      arguments: [],
                      path: GtPath(
                        span: GtSpan(152, 157),
                        id: GtPathModuleId(
                          span: GtSpan(152, 157),
                          module_id: GtModuleId("module"),
                        ),
                        path: "book",
                      ),
                    )),
                    required: true,
                  ),
                  GtProperty(
                    span: GtSpan(179, 211),
                    doc: None,
                    attributes: [],
                    name: GtKey(GtSpan(179, 183), "user"),
                    descriptor: InlineImport(GtInlineImport(
                      span: GtSpan(185, 211),
                      doc: None,
                      attributes: [],
                      name: GtIdentifier(GtSpan(207, 211), "User"),
                      arguments: [],
                      path: GtPath(
                        span: GtSpan(185, 207),
                        id: GtPathModuleId(
                          span: GtSpan(185, 207),
                          module_id: GtModuleId("module"),
                        ),
                        path: "./misc/user",
                      ),
                    )),
                    required: true,
                  ),
                ],
              )),
            ),
          ],
        )
        "#
        );
    }

    impl TryFrom<Pair<'_, Rule>> for GtPath {
        type Error = GtParseError;

        fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
            let module_id: GtModuleId = "module".into();
            GtPath::parse(pair.as_span().into(), &module_id, pair.as_str())
        }
    }
}
