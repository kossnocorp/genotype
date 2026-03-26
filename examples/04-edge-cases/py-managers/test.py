import sys
import types


def install_genotype_runtime_stub() -> None:
    genotype = types.ModuleType("genotype")

    class Model:
        pass

    genotype.Model = Model
    sys.modules["genotype"] = genotype


def main() -> None:
    profile = sys.argv[1]

    install_genotype_runtime_stub()
    from module import Profile, Team

    author = Profile()
    author.name = "Sasha"

    team = Team()
    team.name = "Core"
    team.members = [author]

    assert team.members[0].name == "Sasha"
    print(f"🟢 OK: imports and interaction work for {profile}")


if __name__ == "__main__":
    main()
