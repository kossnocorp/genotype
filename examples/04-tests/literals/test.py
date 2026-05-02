import sys
import types
from typing import Literal, get_type_hints


def install_genotype_runtime_stub() -> None:
    genotype = types.ModuleType("genotype")

    class Model:
        pass

    genotype.Model = Model
    sys.modules["genotype"] = genotype


def main() -> None:
    install_genotype_runtime_stub()
    import module.model as model

    status_hints = get_type_hints(model.RuntimeResponseSuccess)
    assert status_hints["status"] == Literal["success"]

    failure_hints = get_type_hints(model.RuntimeResponseFailure)
    assert failure_hints["status"] == Literal["failure"]

    bag_hints = get_type_hints(model.LiteralBag)
    assert bag_hints["kind"] == Literal["demo"]
    assert bag_hints["enabled"] == Literal[True]
    assert bag_hints["code"] == Literal[200]
    assert bag_hints["empty"] == Literal[None]

    print("🟢 OK: literal annotations match expected runtime values")


if __name__ == "__main__":
    main()
