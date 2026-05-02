use genotype_test_literal_fields::{Response, ResponseFailure, ResponseSuccess};

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_value, json, to_value};

    #[test]
    fn runtime_response_success_roundtrip() {
        let success = Response::Success(ResponseSuccess { value: "ok".into() });

        let value = to_value(&success).expect("serialize success");
        assert_eq!(value, json!({"status": "success", "value": "ok"}));

        let decoded: Response =
            from_value(json!({"status": "success", "value": "ok"})).expect("deserialize success");
        assert_eq!(decoded, success);
    }

    #[test]
    fn runtime_response_failure_roundtrip() {
        let failure = Response::Failure(ResponseFailure {
            error: "boom".into(),
        });

        let value = to_value(&failure).expect("serialize failure");
        assert_eq!(value, json!({"status": "failure", "error": "boom"}));

        let decoded: Response =
            from_value(json!({"status": "failure", "error": "boom"})).expect("deserialize failure");
        assert_eq!(decoded, failure);
    }
}
