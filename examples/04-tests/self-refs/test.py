import sys
import types


def install_genotype_runtime_stub() -> None:
    genotype = types.ModuleType("genotype")

    class Model:
        pass

    genotype.Model = Model
    sys.modules["genotype"] = genotype


def main() -> None:
    install_genotype_runtime_stub()
    import module.model  # noqa: F401

    print("🟢 OK: imported generated module")


if __name__ == "__main__":
    main()
