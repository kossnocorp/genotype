use toml_edit::{ArrayOfTables, InlineTable, Item, Table, Value};

pub trait TomlExtPrune {
    fn prune_defaults(&mut self, defaults: &Table, original: Option<&Table>);
}

impl TomlExtPrune for Table {
    fn prune_defaults(&mut self, defaults: &Table, original: Option<&Table>) {
        let keys: Vec<String> = self.iter().map(|(k, _)| k.to_string()).collect();

        for key in keys {
            let original_has_key = original.and_then(|table| table.get(&key)).is_some();
            let default_item = defaults.get(&key);
            let current_item = match self.get_mut(&key) {
                Some(item) => item,
                None => continue,
            };

            if !original_has_key {
                if let Some(default_item) = default_item {
                    if items_equal(current_item, default_item) {
                        self.remove(&key);
                        continue;
                    }
                }
            }

            if let Some(default_item) = default_item {
                match (current_item, default_item) {
                    (Item::Table(current_table), Item::Table(default_table)) => {
                        let original_table = original
                            .and_then(|table| table.get(&key))
                            .and_then(|item| item.as_table());
                        current_table.prune_defaults(default_table, original_table);
                        if !original_has_key && current_table.is_empty() {
                            self.remove(&key);
                        }
                    }
                    (
                        Item::Value(Value::InlineTable(current_table)),
                        Item::Value(Value::InlineTable(default_table)),
                    ) => {
                        let original_table = original
                            .and_then(|table| table.get(&key))
                            .and_then(|item| item.as_value())
                            .and_then(|value| value.as_inline_table());
                        prune_inline_defaults(current_table, default_table, original_table);
                        if !original_has_key && current_table.is_empty() {
                            self.remove(&key);
                        }
                    }
                    (Item::ArrayOfTables(current_array), Item::ArrayOfTables(default_array)) => {
                        if !original_has_key && array_tables_equal(current_array, default_array) {
                            self.remove(&key);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn prune_inline_defaults(
    current: &mut InlineTable,
    defaults: &InlineTable,
    original: Option<&InlineTable>,
) {
    let keys: Vec<String> = current.iter().map(|(k, _)| k.to_string()).collect();

    for key in keys {
        let original_has_key = original.and_then(|table| table.get(&key)).is_some();
        let default_item = defaults.get(&key);
        let current_item = match current.get_mut(&key) {
            Some(item) => item,
            None => continue,
        };

        if !original_has_key {
            if let Some(default_item) = default_item {
                if values_equal(current_item, default_item) {
                    current.remove(&key);
                    continue;
                }
            }
        }
    }
}

fn items_equal(left: &Item, right: &Item) -> bool {
    match (left, right) {
        (Item::Value(left), Item::Value(right)) => values_equal(left, right),
        (Item::Table(left), Item::Table(right)) => table_equal(left, right),
        (Item::ArrayOfTables(left), Item::ArrayOfTables(right)) => array_tables_equal(left, right),
        (Item::None, Item::None) => true,
        _ => false,
    }
}

fn values_equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::InlineTable(left), Value::InlineTable(right)) => table_like_equal(left, right),
        _ => left.to_string() == right.to_string(),
    }
}

fn table_equal(left: &Table, right: &Table) -> bool {
    table_like_equal(left, right)
}

fn table_like_equal<Type: toml_edit::TableLike>(left: &Type, right: &Type) -> bool {
    if left.len() != right.len() {
        return false;
    }

    for (key, left_item) in left.iter() {
        let Some(right_item) = right.get(key) else {
            return false;
        };

        if !items_equal(left_item, right_item) {
            return false;
        }
    }

    true
}

fn array_tables_equal(left: &ArrayOfTables, right: &ArrayOfTables) -> bool {
    if left.len() != right.len() {
        return false;
    }

    for (left_table, right_table) in left.iter().zip(right.iter()) {
        if !table_equal(left_table, right_table) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_str_eq;
    use std::str::FromStr;

    #[test]
    fn test_prune_defaults_removes_unset_defaults() {
        let mut current = toml_edit::DocumentMut::from_str(
            r#"name = "demo"
root = "."
"#,
        )
        .unwrap()
        .as_table()
        .clone();
        let defaults = toml_edit::DocumentMut::from_str(
            r#"root = "."
"#,
        )
        .unwrap()
        .as_table()
        .clone();
        let original = toml_edit::DocumentMut::from_str("name = \"demo\"\n")
            .unwrap()
            .as_table()
            .clone();

        current.prune_defaults(&defaults, Some(&original));

        assert_str_eq!(current.to_string(), "name = \"demo\"\n");
    }

    #[test]
    fn test_prune_defaults_keeps_original_keys() {
        let mut current = toml_edit::DocumentMut::from_str(
            r#"root = "."
"#,
        )
        .unwrap()
        .as_table()
        .clone();
        let defaults = toml_edit::DocumentMut::from_str(
            r#"root = "."
"#,
        )
        .unwrap()
        .as_table()
        .clone();
        let original = toml_edit::DocumentMut::from_str("root = \".\"\n")
            .unwrap()
            .as_table()
            .clone();

        current.prune_defaults(&defaults, Some(&original));

        assert_str_eq!(current.to_string(), "root = \".\"\n");
    }
}
