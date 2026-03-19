use crate::prelude::internal::*;

impl GTReference {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> Self {
        let span: GTSpan = pair.as_span().into();
        let identifier: GTIdentifier = pair.into();
        context.resolve.references.insert(identifier.clone());
        GTReference {
            span: span.clone(),
            id: GTReferenceId(context.module_id.clone(), span),
            definition_id: GTReferenceDefinitionId::Unresolved,
            identifier,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use insta::assert_ron_snapshot;
    use miette::NamedSource;

    #[test]
    fn test_parse_references() {
        let parse = GTModule::parse(
            "module".into(),
            NamedSource::new(
                "module.type",
                r#"use user/User

            Author: {
              name: Name,
              user: User
            }

            Name: string"#
                    .into(),
            ),
        )
        .unwrap();
        assert_ron_snapshot!(
            parse.resolve.references,
            @r#"
        [
          GTIdentifier(GTSpan(57, 61), "Name"),
          GTIdentifier(GTSpan(83, 87), "User"),
        ]
        "#
        );
    }
}
