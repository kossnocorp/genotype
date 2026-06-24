use genotype_test_literal_fields_types::{
    RemoveFileRequest, Response, ResponseFailure, ResponseSuccess,
};

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

    #[test]
    fn remove_file_request_roundtrip() {
        let request = RemoveFileRequest {
            file_path: "src/main.type".into(),
            retry_count: 2,
        };

        let value = to_value(&request).expect("serialize request");
        assert_eq!(
            value,
            json!({
                "requestType": "remove-file",
                "request_kind": "file-operation",
                "filePath": "src/main.type",
                "retry_count": 2,
            })
        );

        let decoded: RemoveFileRequest = from_value(json!({
            "requestType": "remove-file",
            "request_kind": "file-operation",
            "filePath": "src/main.type",
            "retry_count": 2,
        }))
        .expect("deserialize request");
        assert_eq!(decoded, request);
    }
}
