use toml::{Table, Value};

pub trait TomlExtTableGetters {
    fn get_table(&self, key: &str) -> Option<&Table>;

    fn get_str(&self, key: &str) -> Option<&str>;

    fn get_path(&self, path: &str) -> Option<&Value>;
}

impl TomlExtTableGetters for Table {
    fn get_table(&self, key: &str) -> Option<&Table> {
        self.get(key).and_then(|value| value.as_table())
    }

    fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key).and_then(|value| value.as_str())
    }

    fn get_path(&self, path: &str) -> Option<&Value> {
        let mut path = path.split('.');
        let first = path.next()?;
        let mut value = self.get(first)?;

        for key in path {
            value = value.as_table()?.get(key)?;
        }

        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_table_get_table() {
        let table = toml::from_str::<Table>(
            r#"[tool.poetry]
version = "0.1.0"
"#,
        )
        .unwrap();

        let tool = table.get_table("tool").unwrap();
        let poetry = tool.get_table("poetry").unwrap();

        assert_eq!(poetry.get_str("version"), Some("0.1.0"));
    }

    #[test]
    fn test_table_get_str() {
        let table = toml::from_str::<Table>(
            r#"[package]
name = "demo"
version = "0.2.0"
"#,
        )
        .unwrap();

        let package = table.get_table("package").unwrap();
        assert_eq!(package.get_str("name"), Some("demo"));
        assert_eq!(package.get_str("version"), Some("0.2.0"));
        assert_eq!(package.get_str("missing"), None);
    }

    #[test]
    fn test_table_get_path() {
        let table = toml::from_str::<Table>(
            r#"[tool.poetry]
version = "0.1.0"
"#,
        )
        .unwrap();

        assert_eq!(
            table
                .get_path("tool.poetry.version")
                .and_then(|value| value.as_str()),
            Some("0.1.0")
        );

        assert_eq!(table.get_path("tool.poetry.missing"), None);
    }

    #[test]
    fn test_table_get_path_non_table_intermediate() {
        let table = toml::from_str::<Table>(
            r#"[tool]
poetry = "0.1.0"
"#,
        )
        .unwrap();

        assert_eq!(table.get_path("tool.poetry.version"), None);
    }
}
