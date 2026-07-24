use crate::prelude::internal::*;
use heck::ToSnakeCase;

impl RsConvert<RsFieldName> for GtKey {
    fn convert(&self, context: &mut RsConvertContext) -> RsConvertResult<RsFieldName> {
        let name = self.1.to_snake_case();
        let rust_name = match name.as_str() {
            "self" | "Self" | "super" | "crate" => format!("{name}_"),
            "as" | "async" | "await" | "become" | "box" | "break" | "const" | "continue" | "do"
            | "dyn" | "else" | "enum" | "extern" | "false" | "final" | "fn" | "for" | "gen"
            | "if" | "impl" | "in" | "let" | "loop" | "macro" | "match" | "mod" | "move"
            | "override" | "priv" | "pub" | "ref" | "return" | "static" | "struct" | "trait"
            | "true" | "try" | "type" | "typeof" | "union" | "unsafe" | "unsized" | "use"
            | "virtual" | "where" | "while" | "yield" => {
                format!("r#{name}")
            }
            _ => name,
        };

        // Add rename attribute in case of aliasing
        if rust_name.as_str() != self.1.as_ref() {
            context.attribute_field(format!(
                r#"serde(rename = "{original_name}")"#,
                original_name = self.1
            ));
        }

        Ok(RsFieldName(rust_name.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GtKey::new((0, 0).into(), "foo".into())
                .convert(&mut RsConvertContext::empty("module".into()))
                .unwrap(),
            @r#"RsFieldName("foo")"#
        );
    }

    #[test]
    fn test_convert_aliased() {
        let mut context = RsConvertContext::empty("module".into());
        assert_ron_snapshot!(
            GtKey::new((0, 0).into(), "fooBar".into())
                .convert(&mut context)
                .unwrap(),
            @r#"RsFieldName("foo_bar")"#
        );
        assert_ron_snapshot!(
            context.drain_field_attributes(),
            @r#"
        [
          RsAttribute("serde(rename = \"fooBar\")"),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_keyword() {
        let mut context = RsConvertContext::empty("module".into());
        assert_ron_snapshot!(
            GtKey::new((0, 0).into(), "type".into())
                .convert(&mut context)
                .unwrap(),
            @r#"RsFieldName("r#type")"#
        );
        assert_ron_snapshot!(
            context.drain_field_attributes(),
            @r#"
        [
          RsAttribute("serde(rename = \"type\")"),
        ]
        "#
        );
    }
}
