import sys
import types
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
    from module.signature_output import SignatureOutput

    parsed_by_alias = SignatureOutput(type="json", schema='{"kind":"object"}')
    parsed_by_name = SignatureOutput(type="json", schema_='{"kind":"array"}')

    assert parsed_by_alias.schema_ == '{"kind":"object"}'
    assert parsed_by_name.schema_ == '{"kind":"array"}'

    assert parsed_by_alias.model_dump() == {
        "type": "json",
        "schema": '{"kind":"object"}',
    }
    assert parsed_by_name.model_dump() == {
        "type": "json",
        "schema": '{"kind":"array"}',
    }

    assert json.loads(parsed_by_alias.model_dump_json()) == {
        "type": "json",
        "schema": '{"kind":"object"}',
    }
    assert json.loads(parsed_by_name.model_dump_json()) == {
        "type": "json",
        "schema": '{"kind":"array"}',
    }

    print("🟢 OK: schema field parses and serializes")


if __name__ == "__main__":
    main()
