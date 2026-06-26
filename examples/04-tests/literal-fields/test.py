import sys
import types
from typing import Literal, get_type_hints, Union
import json
from pydantic import BaseModel, ConfigDict


def install_genotype_runtime_stub() -> None:
    genotype = types.ModuleType("genotype")

    class Model(BaseModel):
        model_config = ConfigDict(populate_by_name=True)

        def model_dump(self, **kwargs):
            kwargs.setdefault("by_alias", True)
            return super().model_dump(**kwargs)

        def model_dump_json(self, **kwargs):
            kwargs.setdefault("by_alias", True)
            return super().model_dump_json(**kwargs)

    genotype.Model = Model
    sys.modules["genotype"] = genotype


def main() -> None:
    install_genotype_runtime_stub()
    from module import (
        Executor,
        FormatterShell,
        LiteralBag,
        RemoveFileRequest,
        Response,
        ResponseFailure,
        ResponseSuccess,
    )

    response_success = ResponseSuccess(status="success", value="it works!")

    assert response_success.model_dump() == {"status": "success", "value": "it works!"}
    assert json.loads(response_success.model_dump_json()) == {"status": "success", "value": "it works!"}

    response_failure = ResponseFailure(status="failure", error="something went wrong")

    assert response_failure.model_dump() == {"status": "failure", "error": "something went wrong"}
    assert json.loads(response_failure.model_dump_json()) == {"status": "failure", "error": "something went wrong"}

    bag = LiteralBag(
        kind="demo",
        enabled=True,
        code=200,
        empty=None,
    )

    assert bag.model_dump() == {
        "kind": "demo",
        "enabled": True,
        "code": 200,
        "empty": None,
    }
    assert json.loads(bag.model_dump_json()) == {
        "kind": "demo",
        "enabled": True,
        "code": 200,
        "empty": None,
    }

    remove_file_request = RemoveFileRequest(
        requestType="remove-file",
        request_kind="file-operation",
        filePath="src/main.type",
        retry_count=2,
    )

    assert remove_file_request.model_dump() == {
        "requestType": "remove-file",
        "request_kind": "file-operation",
        "filePath": "src/main.type",
        "retry_count": 2,
    }
    assert json.loads(remove_file_request.model_dump_json()) == {
        "requestType": "remove-file",
        "request_kind": "file-operation",
        "filePath": "src/main.type",
        "retry_count": 2,
    }

    executor = Executor(kind="pnpm", cmd="prettier")
    assert executor.model_dump() == {"kind": "pnpm", "cmd": "prettier"}
    assert json.loads(executor.model_dump_json()) == {"kind": "pnpm", "cmd": "prettier"}

    cargo_executor = Executor(kind="cargo", cmd="fmt")
    assert cargo_executor.model_dump() == {"kind": "cargo", "cmd": "fmt"}
    assert json.loads(cargo_executor.model_dump_json()) == {"kind": "cargo", "cmd": "fmt"}

    formatter_shell = FormatterShell(kind="shell", cmd="npm run format")
    assert formatter_shell.model_dump() == {"kind": "shell", "cmd": "npm run format"}
    assert json.loads(formatter_shell.model_dump_json()) == {
        "kind": "shell",
        "cmd": "npm run format",
    }

    print("🟢 OK: literal annotations match expected runtime values")


if __name__ == "__main__":
    main()
