from module import Hello
from module.types import Hello as HelloFromTypes


def main() -> None:
    assert Hello is HelloFromTypes
    assert "Hello" in str(Hello)
    print("🟢 Python import: OK")


if __name__ == "__main__":
    main()
