from pathlib import Path

DIRECTORY = "/Users/lucasvanderwielen/Desktop/Programming/Rust/emu/asm/tests"
LOCATION = "/Users/lucasvanderwielen/Desktop/Programming/Rust/emu/emulator/src/tests/"


def search_dir(path: Path) -> list[Path]:
    items = list(path.iterdir())
    full_list: list[Path] = []
    full_list.extend(items)
    for item in items:
        if item.is_file():
            pass
        else:
            full_list.extend(search_dir(item.absolute()))

    return full_list


def create_test(path: Path) -> str:
    with open(LOCATION + "test_outline.txt", "r") as file:
        if path.name.endswith(".asm"):
            name = path.name.removesuffix(".asm")
            full_path = str(path.resolve())
            content = file.read()
            content = content.replace("<%0>", name)
            content = content.replace("<%1>", full_path)
            return content
        return ""


def insert_tests(tests: list[str]):
    start_label = "//---- Python Start ----"
    final_str: str = ""
    with open(LOCATION + "test_default.txt", "r") as file:
        for line in file:
            final_str += line
            if line.strip() == start_label:
                # insert all tests here
                for test in tests:
                    final_str += test
    with open(LOCATION + "tests.rs", "w") as file:
        _res: int = file.write(final_str)


files = search_dir(Path(DIRECTORY))
test_list: list[str] = []
for file in files:
    test_list.append(create_test(file))
insert_tests(test_list)
