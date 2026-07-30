import json
import sys
import types

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
    from module import Address, AddressId, User

    address_id = AddressId("home")
    user = User(addresses={address_id: Address(street="Main Street")})
    expected = {"addresses": {"home": {"street": "Main Street"}}}

    assert user.model_dump() == expected
    assert json.loads(user.model_dump_json()) == expected


if __name__ == "__main__":
    main()
