use crate::prelude::internal::*;

impl PyContextResolve for PyProperty {
    fn resolve(self, context: &mut PyConvertContext) -> Self {
        if self.name.0.as_ref() == "schema" {
            context.push_import(PyImport::new(PyDependencyIdent::Pydantic, "Field".into()));
        }

        if !self.required {
            context.push_import(PyImport::new(PyDependencyIdent::Typing, "Optional".into()));
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_resolve() {
        let mut context = PyConvertContext::default();
        let alias = PyProperty {
            doc: None,
            name: "foo".into(),
            descriptor: PyPrimitive::String.into(),
            required: true,
        };
        alias.resolve(&mut context);
        assert_ron_snapshot!(context.imports(), @"[]");
    }

    #[test]
    fn test_resolve_optional() {
        let mut context = PyConvertContext::default();
        let alias = PyProperty {
            doc: None,
            name: "foo".into(),
            descriptor: PyPrimitive::String.into(),
            required: false,
        };
        alias.resolve(&mut context);
        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          PyImport(
            dependency: Typing,
            reference: Named([
              Name(PyIdentifier("Optional")),
            ]),
          ),
        ]
        "#
        );
    }

    #[test]
    fn test_resolve_schema() {
        let mut context = PyConvertContext::default();
        let alias = PyProperty {
            doc: None,
            name: "schema".into(),
            descriptor: PyPrimitive::String.into(),
            required: true,
        };
        alias.resolve(&mut context);
        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          PyImport(
            dependency: Pydantic,
            reference: Named([
              Name(PyIdentifier("Field")),
            ]),
          ),
        ]
        "#
        );
    }

    #[test]
    fn test_resolve_schema_optional() {
        let mut context = PyConvertContext::default();
        let alias = PyProperty {
            doc: None,
            name: "schema".into(),
            descriptor: PyPrimitive::String.into(),
            required: false,
        };
        alias.resolve(&mut context);
        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          PyImport(
            dependency: Pydantic,
            reference: Named([
              Name(PyIdentifier("Field")),
            ]),
          ),
          PyImport(
            dependency: Typing,
            reference: Named([
              Name(PyIdentifier("Optional")),
            ]),
          ),
        ]
        "#
        );
    }
}
