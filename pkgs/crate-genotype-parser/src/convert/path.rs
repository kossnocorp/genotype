use crate::prelude::internal::*;

pub fn parse_path_with_name(
    token: GtToken<'_>,
    module_id: &GtModuleId,
) -> GtNodeParseResult<(GtPath, (GtSpan, String))> {
    let span = token.span;
    let span_start = span.0;
    let str = token.as_str().to_string();
    let else_err = || {
        GtParseError::Internal(
            (span.0, span.1).into(),
            GtNode::Path,
            "expected path with name segment",
        )
    };

    let name_index = str.rfind("/").ok_or_else(else_err)?;

    let path_str = str.get(..name_index).ok_or_else(else_err)?;
    let path_span = (span_start, span_start + name_index).into();
    let path = GtPath::parse(path_span, module_id, path_str)?;

    let name = str.get(name_index + 1..).ok_or_else(else_err)?;
    let name_span = (span_start + name_index + 1, span.1).into();

    Ok((path, (name_span, name.into())))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_split() {
        let mut tokens = GtParser::parse(GtTokenKind::InlineImport, "./hello/World").unwrap();
        let module_id: GtModuleId = "module".into();
        let result = parse_path_with_name(tokens.next().unwrap(), &module_id).unwrap();
        assert_eq!(
            result,
            (
                GtPath::parse((0, 7).into(), &module_id, "./hello").unwrap(),
                ((8, 13).into(), "World".into())
            )
        );
    }
}
#[cfg(test)]
mod normalization_tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut tokens = GtParser::parse(GtTokenKind::Path, "./hello/world/").unwrap();
        let module_id: GtModuleId = "module".into();
        assert_eq!(
            GtPath::parse((0, 14).into(), &module_id, "./hello/world/"),
            {
                let token = tokens.next().unwrap();
                GtPath::parse(token.span, &module_id, token.as_str())
            }
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
}
