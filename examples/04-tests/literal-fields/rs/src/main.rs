use genotype_test_literal_fields_types::{
    Executor, ExecutorKind, ExecutorKindNode, ExecutorKindPython, Formatter, RemoveFileRequest,
    Response, ResponseFailure, ResponseSuccess,
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

    #[test]
    fn mixed_executor_kind_node_roundtrip() {
        let decoded: ExecutorKind =
            from_value(json!("pnpm")).expect("deserialize node executor kind");
        assert_eq!(decoded, ExecutorKind::Node(ExecutorKindNode::Pnpm));
        assert_eq!(
            to_value(&decoded).expect("serialize node executor kind"),
            json!("pnpm")
        );
    }

    #[test]
    fn mixed_executor_kind_cargo_roundtrip() {
        let decoded: ExecutorKind =
            from_value(json!("cargo")).expect("deserialize cargo executor kind");
        assert_eq!(decoded, ExecutorKind::Cargo);
        assert_eq!(
            to_value(&decoded).expect("serialize cargo executor kind"),
            json!("cargo")
        );
    }

    #[test]
    fn mixed_executor_kind_python_roundtrip() {
        let decoded: ExecutorKind =
            from_value(json!("uv")).expect("deserialize python executor kind");
        assert_eq!(decoded, ExecutorKind::Python(ExecutorKindPython::Uv));
        assert_eq!(
            to_value(&decoded).expect("serialize python executor kind"),
            json!("uv")
        );
    }

    #[test]
    fn mixed_executor_roundtrip() {
        let decoded: Executor = from_value(json!({
            "kind": "pnpm",
            "cmd": "prettier",
        }))
        .expect("deserialize executor");

        assert_eq!(decoded.kind, ExecutorKind::Node(ExecutorKindNode::Pnpm));
        assert_eq!(
            to_value(&decoded).expect("serialize executor"),
            json!({
                "kind": "pnpm",
                "cmd": "prettier",
            })
        );
    }

    #[test]
    fn mixed_formatter_executor_roundtrip() {
        let decoded: Formatter = from_value(json!({
            "kind": "pnpm",
            "cmd": "prettier",
        }))
        .expect("deserialize formatter executor");

        assert_eq!(
            to_value(&decoded).expect("serialize formatter executor"),
            json!({
                "kind": "pnpm",
                "cmd": "prettier",
            })
        );
    }

    #[test]
    fn mixed_formatter_shell_roundtrip() {
        let decoded: Formatter = from_value(json!({
            "kind": "shell",
            "cmd": "npm run format",
        }))
        .expect("deserialize formatter shell");

        assert_eq!(
            to_value(&decoded).expect("serialize formatter shell"),
            json!({
                "kind": "shell",
                "cmd": "npm run format",
            })
        );
    }
}
