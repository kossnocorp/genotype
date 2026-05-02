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
    from module import Response, ResponseFailure, ResponseSuccess, LiteralBag

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

    print("🟢 OK: literal annotations match expected runtime values")


if __name__ == "__main__":
    main()
