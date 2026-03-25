use crate::error::TomlExtResult;
use toml_edit::*;

pub trait TomlExtMerge {
    fn merge(&mut self, other: &Self) -> TomlExtResult<()>
    where
        Self: Sized;
}

impl TomlExtMerge for DocumentMut {
    fn merge(&mut self, other: &Self) -> TomlExtResult<()>
    where
        Self: Sized,
    {
        for (key, other_item) in other.iter() {
            if let Some(item) = self.get_mut(key) {
                item.merge(other_item)?;
            } else {
                self.insert(key, other_item.clone());
            }
        }
        Ok(())
    }
}

impl TomlExtMerge for Item {
    fn merge(&mut self, other: &Self) -> TomlExtResult<()>
    where
        Self: Sized,
    {
        match other {
            Item::Value(other_value) => {
                if let Some(value) = self.as_value_mut() {
                    value.merge(other_value)?;
                } else {
                    *self = other.clone();
                }
            }

            Item::Table(other_table) => {
                if let Some(table) = self.as_table_mut() {
                    table.merge(other_table)?;
                } else {
                    *self = other.clone();
                }
            }

            Item::ArrayOfTables(other_array) => {
                if let Some(array) = self.as_array_of_tables_mut() {
                    array.merge(other_array)?;
                } else {
                    *self = other.clone();
                }
            }

            Item::None => {
                *self = other.clone();
            }
        }

        Ok(())
    }
}

impl TomlExtMerge for Value {
    fn merge(&mut self, other: &Self) -> TomlExtResult<()>
    where
        Self: Sized,
    {
        match other {
            Value::Boolean(_)
            | Value::Datetime(_)
            | Value::Float(_)
            | Value::Integer(_)
            | Value::String(_) => {
                *self = other.clone();
            }

            Value::Array(other_array) => {
                if let Some(array) = self.as_array_mut() {
                    array.merge(other_array)?;
                } else {
                    *self = other.clone();
                }
            }

            Value::InlineTable(other_table) => {
                if let Some(table) = self.as_inline_table_mut() {
                    table.merge(other_table)?;
                } else {
                    *self = other.clone();
                }
            }
        }

        Ok(())
    }
}

impl TomlExtMerge for Array {
    fn merge(&mut self, other: &Self) -> TomlExtResult<()>
    where
        Self: Sized,
    {
        self.extend(other);
        Ok(())
    }
}

impl TomlExtMerge for Table {
    fn merge(&mut self, other: &Self) -> TomlExtResult<()>
    where
        Self: Sized,
    {
        merge_tables(self, other)
    }
}

impl TomlExtMerge for InlineTable {
    fn merge(&mut self, other: &Self) -> TomlExtResult<()>
    where
        Self: Sized,
    {
        merge_tables(self, other)
    }
}

impl TomlExtMerge for ArrayOfTables {
    fn merge(&mut self, other: &Self) -> TomlExtResult<()>
    where
        Self: Sized,
    {
        self.extend(other.clone());
        Ok(())
    }
}

fn merge_tables<Type: TableLike>(self_table: &mut Type, other_table: &Type) -> TomlExtResult<()> {
    for (key, other_item) in other_table.iter() {
        if let Some(item) = self_table.get_mut(key) {
            item.merge(other_item)?;
        } else {
            self_table.insert(key, other_item.clone());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_merge_primitives() {
        let mut doc = DocumentMut::from_str(
            r#"qwe = 1
asd = 2
"#,
        )
        .unwrap();
        let other = DocumentMut::from_str(
            r#"asd = 3
zxc = 4
"#,
        )
        .unwrap();
        doc.merge(&other).unwrap();
        assert_str_eq!(
            doc.to_string(),
            r#"qwe = 1
asd = 3
zxc = 4
"#
        );
    }

    #[test]
    fn test_merge_tables() {
        let mut doc = DocumentMut::from_str(
            r#"[qwe]
a = 1
b = 2
[asd]
[zxc]
a = 4
        "#,
        )
        .unwrap();
        let other = DocumentMut::from_str(
            r#"[qwe]
a = 0
c = 3
[asd]
a = 6
[zxc]
                            "#,
        )
        .unwrap();
        doc.merge(&other).unwrap();
        assert_str_eq!(
            doc.to_string(),
            r#"[qwe]
a = 0
b = 2
c = 3
[asd]
a = 6
[zxc]
a = 4
        "#,
        );
    }
}
