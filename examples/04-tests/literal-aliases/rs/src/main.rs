use genotype_test_literal_aliases_types::{Status, StatusCode};

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_value, json, to_value};

    #[test]
    fn runtime_status_roundtrip() {
        let parsed: Status = from_value(json!("success")).expect("deserialize status");
        let value = to_value(&parsed).expect("serialize status");
        assert_eq!(value, json!("success"));
    }

    #[test]
    fn runtime_code_roundtrip() {
        let parsed: StatusCode = from_value(json!(200)).expect("deserialize code");
        let value = to_value(&parsed).expect("serialize code");
        assert_eq!(value, json!(200));
    }
}
