use crate::error::{TomlExtError, TomlExtResult};
use toml::{Table, Value};
use toml_edit::{DocumentMut, Item};

pub trait TomlExtDocDrill {
    fn drill_table_mut<'a>(&'a mut self, path: &str) -> TomlExtResult<&'a mut toml_edit::Table>
    where
        Self: Sized;

    fn drill_item_mut<'a>(&'a mut self, path: &str) -> TomlExtResult<&'a mut toml_edit::Item>
    where
        Self: Sized;
}

impl TomlExtDocDrill for DocumentMut {
    fn drill_table_mut<'a>(&'a mut self, path: &str) -> TomlExtResult<&'a mut toml_edit::Table>
    where
        Self: Sized,
    {
        let mut table = self.as_table_mut();
        let mut keys = path.split('.');

        for key in keys {
            if !table.contains_key(key) {
                table[key] = Item::Table(toml_edit::Table::new());
            }
            let type_name = table[key].type_name();
            table = table[key].as_table_mut().ok_or_else(|| {
                TomlExtError::FailedCast(format!("Failed to access mutable {type_name} as table",))
            })?;
        }

        Ok(table)
    }

    fn drill_item_mut<'a>(&'a mut self, path: &str) -> TomlExtResult<&'a mut toml_edit::Item>
    where
        Self: Sized,
    {
        let table_path_arr: Vec<&str> = path.split(".").collect();
        let entry_key = table_path_arr
            .iter()
            .last()
            .ok_or_else(|| TomlExtError::InvalidPath(path.to_string()))?;
        let table_path = &table_path_arr[..table_path_arr.len() - 1].join(".");
        let table = self.drill_table_mut(table_path)?;
        let item = table.entry(entry_key).or_insert(Item::None);
        Ok(item)
    }
}

pub trait TomlExtTableDrill {
    fn drill_table_mut<'a>(&'a mut self, path: &str) -> TomlExtResult<&'a mut Table>
    where
        Self: Sized;

    fn drill_str_mut<'a>(&'a mut self, path: &str) -> TomlExtResult<&'a mut toml::Value>
    where
        Self: Sized;
}

impl TomlExtTableDrill for Table {
    fn drill_table_mut<'a>(&'a mut self, path: &str) -> TomlExtResult<&'a mut Table>
    where
        Self: Sized,
    {
        let mut current = self;

        if path.is_empty() {
            return Ok(current);
        }

        for key in path.split('.') {
            if !current.contains_key(key) {
                current.insert(key.into(), Value::Table(Table::new()));
            }

            let type_name = current
                .get(key)
                .map(|value| value.type_str())
                .unwrap_or("none");

            current = current
                .get_mut(key)
                .and_then(|value| value.as_table_mut())
                .ok_or_else(|| {
                    TomlExtError::FailedCast(format!(
                        "Failed to access mutable {type_name} as table",
                    ))
                })?;
        }

        Ok(current)
    }

    fn drill_str_mut<'a>(&'a mut self, path: &str) -> TomlExtResult<&'a mut toml::Value>
    where
        Self: Sized,
    {
        let table_path_arr: Vec<&str> = path.split(".").collect();
        let entry_key = table_path_arr
            .iter()
            .last()
            .ok_or_else(|| TomlExtError::InvalidPath(path.to_string()))?;
        let table_path = &table_path_arr[..table_path_arr.len() - 1].join(".");
        let table = self.drill_table_mut(table_path)?;
        let item = table.entry(*entry_key).or_insert(Value::String("".into()));
        Ok(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use std::str::FromStr;

    #[test]
    fn test_doc_drill_table_empty() {
        let mut doc = DocumentMut::new();

        TomlExtDocDrill::drill_table_mut(&mut doc, "qwe").unwrap();

        assert_snapshot!(
            doc.to_string(),
            @"[qwe]"
        );
    }

    #[test]
    fn test_doc_drill_table_existing() {
        let mut doc = DocumentMut::from_str(
            "[qwe.asd.zxc]
a = 1
",
        )
        .unwrap();

        TomlExtDocDrill::drill_table_mut(&mut doc, "qwe.asd.zxc").unwrap();

        assert_snapshot!(
            doc.to_string(),
            @"
        [qwe.asd.zxc]
        a = 1
        "
        );
    }

    #[test]
    fn test_table_drill_table_empty() {
        let mut table = toml::Table::new();

        let nested = TomlExtTableDrill::drill_table_mut(&mut table, "tool.poetry").unwrap();
        nested.insert("version".into(), Value::String("0.1.0".into()));

        assert_snapshot!(
            table.to_string(),
            @r#"
        [tool.poetry]
        version = "0.1.0"
        "#
        );
    }

    #[test]
    fn test_table_drill_table_existing() {
        let mut table = toml::from_str::<Table>(
            r#"[tool.poetry]
version = "0.1.0"
"#,
        )
        .unwrap();

        let nested = TomlExtTableDrill::drill_table_mut(&mut table, "tool.poetry").unwrap();
        nested.insert("version".into(), Value::String("0.2.0".into()));

        assert_snapshot!(
            table.to_string(),
            @r#"
        [tool.poetry]
        version = "0.2.0"
        "#
        );
    }

    #[test]
    fn test_table_drill_table_self() {
        let mut table = toml::Table::new();

        let nested = TomlExtTableDrill::drill_table_mut(&mut table, "").unwrap();
        nested.insert("version".into(), Value::String("0.2.0".into()));

        assert_snapshot!(
            table.to_string(),
            @r#"version = "0.2.0""#
        );
    }

    #[test]
    fn test_doc_drill_item_empty() {
        let mut doc = DocumentMut::new();

        let item = TomlExtDocDrill::drill_item_mut(&mut doc, "qwe.asd.zxc").unwrap();
        *item = Item::Value("0.1.0".into());

        assert_snapshot!(
            doc.to_string(),
            @r#"
        [qwe]

        [qwe.asd]
        zxc = "0.1.0"
        "#
        );
    }

    #[test]
    fn test_doc_drill_item_existing() {
        let mut doc = DocumentMut::from_str(
            r#"[ts.manifest]
version = "0.1.0"
"#,
        )
        .unwrap();

        let item = TomlExtDocDrill::drill_item_mut(&mut doc, "ts.manifest.version").unwrap();
        *item = Item::Value("0.2.0".into());

        assert_snapshot!(
            doc.to_string(),
            @r#"
        [ts.manifest]
        version = "0.2.0"
        "#
        );
    }

    #[test]
    fn test_table_drill_str_empty() {
        let mut table = toml::Table::new();
        let item = TomlExtTableDrill::drill_str_mut(&mut table, "ts.manifest.version").unwrap();
        *item = toml::Value::String("0.1.0".into());

        assert_snapshot!(
            table.to_string(),
            @r#"
        [ts.manifest]
        version = "0.1.0"
        "#
        );
    }

    #[test]
    fn test_table_drill_str_existing() {
        let mut table = toml::from_str::<Table>(
            r#"[ts.manifest]
version = "0.1.0"
"#,
        )
        .unwrap();

        let value = TomlExtTableDrill::drill_str_mut(&mut table, "ts.manifest.version").unwrap();
        *value = toml::Value::String("0.2.0".into());

        assert_snapshot!(
            table.to_string(),
            @r#"
        [ts.manifest]
        version = "0.2.0"
        "#
        );
    }
}
