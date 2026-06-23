import sys
import types


def install_genotype_runtime_stub() -> None:
    genotype = types.ModuleType("genotype")

    class Model:
        def __init__(self, **kwargs):
            for key, value in kwargs.items():
                setattr(self, key, value)

    genotype.Model = Model
    sys.modules["genotype"] = genotype


def main() -> None:
    install_genotype_runtime_stub()
    from module.generics import ResponseFailure, ResponsePair, ResponseString, ResponseSuccess
    from module.pair import Pair

    success = ResponseSuccess[str](status="success", value="ok")
    failure = ResponseFailure(status="failure", error="bad")
    pair = Pair[str, float](left="left", right=1.5)

    assert success.value == "ok"
    assert failure.error == "bad"
    assert pair.right == 1.5
    assert ResponseString is not None
    assert ResponsePair is not None

    print("🟢 OK: imported generated Python module")


if __name__ == "__main__":
    main()
