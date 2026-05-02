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
    from module import Status

    status = Status(status="success", code=200)

    assert status.model_dump() == {"status": "success", "code": 200}
    assert json.loads(status.model_dump_json()) == {"status": "success", "code": 200}

    status.status = "failure"
    status.code = 500

    assert status.model_dump() == {"status": "failure", "code": 500}
    assert json.loads(status.model_dump_json()) == {"status": "failure", "code": 500}

    print("🟢 OK: literal annotations match expected runtime values")


if __name__ == "__main__":
    main()
