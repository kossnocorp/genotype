use crate::prelude::internal::*;

#[derive(Error, Debug, PartialEq)]
pub enum TomlDrillError {
    #[error("Failed to cast {0} to table")]
    FailedCast(String),
}

pub trait TomlDrill {
    fn drill_table_mut<'a>(&'a mut self, path: &str) -> Result<&'a mut Table>
    where
        Self: Sized;
}

impl TomlDrill for DocumentMut {
    fn drill_table_mut<'a>(&'a mut self, path: &str) -> Result<&'a mut Table>
    where
        Self: Sized,
    {
        let mut table = self.as_table_mut();
        let mut keys = path.split('.');

        while let Some(key) = keys.next() {
            if !table.contains_key(key) {
                table[key] = Item::Table(Table::new());
            }
            let type_name = table[key].type_name();
            table = table[key]
                .as_table_mut()
                .ok_or_else(|| {
                    TomlDrillError::FailedCast(format!(
                        "Failed to access mutable {type_name} as table",
                    ))
                })
                .into_diagnostic()?;
        }

        Ok(table)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_str_eq;
    use std::str::FromStr;

    #[test]
    fn test_drill_empty() {
        let mut doc = DocumentMut::new();
        let table = doc.drill_table_mut("qwe").unwrap();
        assert_str_eq!(table.to_string(), "");
        assert_str_eq!(doc.to_string(), "[qwe]\n");
    }

    #[test]
    fn test_drill_existing() {
        let mut doc = DocumentMut::from_str(
            "[qwe.asd.zxc]
a = 1
",
        )
        .unwrap();
        let table = doc.drill_table_mut("qwe.asd.zxc").unwrap();
        assert_str_eq!(table.to_string(), "a = 1\n");
        assert_str_eq!(
            doc.to_string(),
            r#"[qwe.asd.zxc]
a = 1
"#
        );
    }

    #[test]
    fn test_drill_none() {
        let mut doc = DocumentMut::from_str(
            "[qwe.asd.zxc]
",
        )
        .unwrap();
        let table = doc.drill_table_mut("qwe.asd.zxc").unwrap();
        assert_str_eq!(table.to_string(), "");
        assert_str_eq!(doc.to_string(), "[qwe.asd.zxc]\n");
    }

    #[test]
    fn test_drill_partial() {
        let mut doc = DocumentMut::from_str(
            "[qwe.asd]
a = 1
",
        )
        .unwrap();
        let table = doc.drill_table_mut("qwe.asd.zxc").unwrap();
        assert_str_eq!(table.to_string(), "");
        assert_str_eq!(
            doc.to_string(),
            r#"[qwe.asd]
a = 1

[qwe.asd.zxc]
"#
        );
    }

    #[test]
    fn test_drill_mutate() {
        let mut doc = DocumentMut::from_str(
            r#"[qwe.asd.zxc]
a = 1
"#,
        )
        .unwrap();
        let table = doc.drill_table_mut("qwe.asd.zxc").unwrap();
        table["b"] = Item::Value(Value::from(2));
        assert_str_eq!(
            table.to_string(),
            r#"a = 1
b = 2
"#
        );
        assert_str_eq!(
            doc.to_string(),
            r#"[qwe.asd.zxc]
a = 1
b = 2
"#
        );
    }
}
