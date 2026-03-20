use crate::prelude::internal::*;

impl GTObject {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();

        let name = context.name_object(span.clone())?;

        // It is an explicitly named object, so we need to add an anonymous parent so following
        // children don't get the object name.
        let named = matches!(name, GTObjectName::Named(_));
        if named {
            context.enter_parent(GTContextParent::Anonymous);
        }

        let mut object = GTObject {
            span: span.clone(),
            doc: None,
            attributes: vec![],
            name,
            extensions: vec![],
            properties: vec![],
        };

        if let Some(properties_pair) = pair.into_inner().next() {
            for pair in properties_pair.into_inner() {
                // Extract property rule
                let property_pair = pair
                    .into_inner()
                    .next()
                    .ok_or_else(|| GTParseError::UnexpectedEnd(span.clone(), GTNode::Object))?;

                match property_pair.as_rule() {
                    Rule::required_property | Rule::optional_property => {
                        object
                            .properties
                            .push(GTProperty::parse(property_pair, context)?);
                    }

                    Rule::extension_property => {
                        object
                            .extensions
                            .push(GTExtension::parse(property_pair, context)?);
                    }

                    rule => {
                        return Err(GTParseError::UnexpectedRule(
                            property_pair.as_span().into(),
                            GTNode::Object,
                            rule,
                        ));
                    }
                }
            }
        }

        if named {
            context.exit_parent(span, GTNode::Object)?;
        }

        Ok(object)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::object, "{ hello: string }").unwrap();
        let mut context = GTContext {
            module_id: "module".into(),
            resolve: GTModuleResolve::new(),
            parents: vec![GTContextParent::Alias(GTIdentifier::new(
                (0, 5).into(),
                "Hello".into(),
            ))],
            claimed_names: Default::default(),
            annotation: None,
        };
        assert_ron_snapshot!(
            GTObject::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        GTObject(
          span: GTSpan(0, 17),
          doc: None,
          attributes: [],
          name: Named(GTIdentifier(GTSpan(0, 5), "Hello")),
          extensions: [],
          properties: [
            GTProperty(
              span: GTSpan(2, 15),
              doc: None,
              attributes: [],
              name: GTKey(GTSpan(2, 7), "hello"),
              descriptor: Primitive(GTPrimitive(
                span: GTSpan(9, 15),
                kind: String,
                doc: None,
                attributes: [],
              )),
              required: true,
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_parse_deps_base() {
        let source_code = NamedSource::new(
            "module.type",
            r#"Order: {
                book: book/Book,
                user: ./misc/user/User
            }"#
            .into(),
        );
        let parse = GTModule::parse("module".into(), source_code).unwrap();
        assert_ron_snapshot!(
            parse.resolve.deps,
            @r#"
        [
          GTPath(GTSpan(31, 35), Unresolved, "book"),
          GTPath(GTSpan(64, 75), Unresolved, "./misc/user"),
        ]
        "#
        );
    }

    #[test]
    fn test_parse_deps_normalize() {
        let source_code = NamedSource::new(
            "module.type",
            r#"Order: {
                book: book/Book,
                user: ./misc/../misc/./user/User
            }"#
            .into(),
        );
        let parse = GTModule::parse("module".into(), source_code).unwrap();
        assert_ron_snapshot!(
            parse.resolve.deps,
            @r#"
        [
          GTPath(GTSpan(31, 35), Unresolved, "book"),
          GTPath(GTSpan(64, 85), Unresolved, "./misc/user"),
        ]
        "#
        );
    }

    #[test]
    fn test_parse_name() {
        let mut pairs = GenotypeParser::parse(Rule::object, "{ hello: string }").unwrap();
        let mut context = GTContext {
            module_id: "module".into(),
            resolve: GTModuleResolve::new(),
            parents: vec![
                GTContextParent::Alias(GTIdentifier::new((0, 5).into(), "Hello".into())),
                GTContextParent::Anonymous,
            ],
            claimed_names: Default::default(),
            annotation: None,
        };
        assert_ron_snapshot!(
            GTObject::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        GTObject(
          span: GTSpan(0, 17),
          doc: None,
          attributes: [],
          name: Alias(GTIdentifier(GTSpan(0, 17), "HelloObj"), Alias(GTIdentifier(GTSpan(0, 5), "Hello"))),
          extensions: [],
          properties: [
            GTProperty(
              span: GTSpan(2, 15),
              doc: None,
              attributes: [],
              name: GTKey(GTSpan(2, 7), "hello"),
              descriptor: Primitive(GTPrimitive(
                span: GTSpan(9, 15),
                kind: String,
                doc: None,
                attributes: [],
              )),
              required: true,
            ),
          ],
        )
        "#
        );
    }
}
