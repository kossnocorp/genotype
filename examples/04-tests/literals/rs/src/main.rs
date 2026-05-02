use genotype_test_literals::{RuntimeResponse, RuntimeResponseFailure, RuntimeResponseSuccess};

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, from_value, to_value};

    #[test]
    fn runtime_response_success_roundtrip() {
        let success = RuntimeResponse::Success(RuntimeResponseSuccess {
            value: "ok".into(),
        });

        let value = to_value(&success).expect("serialize success");
        assert_eq!(value, json!({"status": "success", "value": "ok"}));

        let decoded: RuntimeResponse =
            from_value(json!({"status": "success", "value": "ok"})).expect("deserialize success");
        assert_eq!(decoded, success);
    }

    #[test]
    fn runtime_response_failure_roundtrip() {
        let failure = RuntimeResponse::Failure(RuntimeResponseFailure {
            error: "boom".into(),
        });

        let value = to_value(&failure).expect("serialize failure");
        assert_eq!(value, json!({"status": "failure", "error": "boom"}));

        let decoded: RuntimeResponse =
            from_value(json!({"status": "failure", "error": "boom"})).expect("deserialize failure");
        assert_eq!(decoded, failure);
    }
}
