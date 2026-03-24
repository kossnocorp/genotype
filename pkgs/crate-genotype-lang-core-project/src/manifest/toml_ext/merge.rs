use crate::prelude::internal::*;

pub trait TomlMerge {
    fn merge(&mut self, other: &Self) -> Result<()>
    where
        Self: Sized;
}

impl TomlMerge for DocumentMut {
    fn merge(&mut self, other: &Self) -> Result<()>
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

impl TomlMerge for Item {
    fn merge(&mut self, other: &Self) -> Result<()>
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

impl TomlMerge for Value {
    fn merge(&mut self, other: &Self) -> Result<()>
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

impl TomlMerge for Array {
    fn merge(&mut self, other: &Self) -> Result<()>
    where
        Self: Sized,
    {
        self.extend(other);
        Ok(())
    }
}

impl TomlMerge for Table {
    fn merge(&mut self, other: &Self) -> Result<()>
    where
        Self: Sized,
    {
        merge_tables(self, other)
    }
}

impl TomlMerge for InlineTable {
    fn merge(&mut self, other: &Self) -> Result<()>
    where
        Self: Sized,
    {
        merge_tables(self, other)
    }
}

impl TomlMerge for ArrayOfTables {
    fn merge(&mut self, other: &Self) -> Result<()>
    where
        Self: Sized,
    {
        self.extend(other.clone());
        Ok(())
    }
}

fn merge_tables<Type: TableLike>(self_table: &mut Type, other_table: &Type) -> Result<()> {
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
    fn test_merge_empty() {
        let mut doc = DocumentMut::new();
        let other = DocumentMut::new();
        doc.merge(&other).unwrap();
        assert_str_eq!(doc.to_string(), "");
    }

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
    fn test_merge_objs_with_primitives() {
        let mut doc = DocumentMut::from_str(
            r#"qwe = [1, 2]
asd = { a = 1, b = 2 }
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
            r#"qwe = [1, 2]
asd = 3
zxc = 4
"#
        );
    }

    #[test]
    fn test_merge_primitives_with_objs() {
        let mut doc = DocumentMut::from_str(
            r#"qwe = 1
asd = 2
zxc = 3
"#,
        )
        .unwrap();
        let other = DocumentMut::from_str(
            r#"asd = [3]
zxc = { a = 4 }
"#,
        )
        .unwrap();
        doc.merge(&other).unwrap();
        assert_str_eq!(
            doc.to_string(),
            r#"qwe = 1
asd = [3]
zxc = { a = 4 }
"#
        );
    }

    #[test]
    fn test_merge_arrays() {
        let mut doc = DocumentMut::from_str(
            r#"qwe = [1, 2]
zxc = [4, 5]
"#,
        )
        .unwrap();
        let other = DocumentMut::from_str(
            r#"qwe = [3, 4]
asd = [1, 2]
zxc = []
"#,
        )
        .unwrap();
        doc.merge(&other).unwrap();
        assert_str_eq!(
            doc.to_string(),
            r#"qwe = [1, 2,3, 4]
zxc = [4, 5]
asd = [1, 2]
"#,
        );
    }

    #[test]
    fn test_merge_array_of_tables() {
        let mut doc = DocumentMut::from_str(
            r#"[[qwe]]
a = 1
[[qwe]]
b = 2
[[zxc]]
c = 4
[[zxc]]
d = 5
"#,
        )
        .unwrap();
        let other = DocumentMut::from_str(
            r#"[[qwe]]
c = 3
[[qwe]]
d = 4
[[asd]]
a = 1
[[asd]]
b = 2
[[zxc]]
"#,
        )
        .unwrap();
        doc.merge(&other).unwrap();
        assert_str_eq!(
            doc.to_string(),
            r#"[[qwe]]
a = 1
[[qwe]]
c = 3
[[qwe]]
b = 2
[[qwe]]
d = 4
[[zxc]]
c = 4
[[asd]]
a = 1
[[zxc]]
d = 5
[[asd]]
b = 2
[[zxc]]
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

    #[test]
    fn test_merge_inline_tables() {
        let mut doc = DocumentMut::from_str(
            r#"qwe = { a = 1, b = 2 }
asd = {}
zxc = { a = 4 }
"#,
        )
        .unwrap();
        let other = DocumentMut::from_str(
            r#"qwe = { a = 0, c = 3 }
asd = { a = 6 }
zxc = {}
"#,
        )
        .unwrap();
        doc.merge(&other).unwrap();
        assert_str_eq!(
            doc.to_string(),
            r#"qwe = { a = 0, b = 2 , c = 3 }
asd = { a = 6 }
zxc = { a = 4 }
"#,
        );
    }

    #[test]
    fn test_merge_none() {
        let mut doc = DocumentMut::from_str(
            r#"[[qwe]]
[asd]
a = 1
        "#,
        )
        .unwrap();
        let other = DocumentMut::from_str(
            r#"[qwe]
c = 3
[[zxc]]
                            "#,
        )
        .unwrap();
        doc.merge(&other).unwrap();
        assert_str_eq!(
            doc.to_string(),
            r#"[qwe]
c = 3
[asd]
a = 1
[[zxc]]
        "#,
        );
    }

    #[test]
    fn test_merge_different_types() {
        let mut doc = DocumentMut::from_str(
            r#"value = 1
obj = { b = 2 }
arr = [3]
[[arrtbl]]
d = 4
    "#,
        )
        .unwrap();
        let other = DocumentMut::from_str(
            r#"obj = [6, 7]
arr = { j = 8 }
arrtbl = 9
[[value]]
e = 5
    "#,
        )
        .unwrap();
        doc.merge(&other).unwrap();
        assert_str_eq!(
            doc.to_string(),
            r#"obj = [6, 7]
arr = { j = 8 }
arrtbl= 9
[[value ]]
e = 5
    "#,
        );
    }
}
