from collections import Counter
from pathlib import Path
from typing import List


DATA_PATH = Path(__file__).resolve().parents[1] / "data"


def read_input(path: Path) -> List[int]:
    return list(map(int, path.read_text().strip("\n").split("\n")))


def main(problem_number: int):
    data = read_input(DATA_PATH / f"input_{problem_number}.txt")

    # Part 1
    counter = Counter(list(a < b for a, b in zip(data[:-1], data[1:])))
    print(counter[True])

    # Part 2
    counter = Counter(list(a < b for a, b in zip(data[:-3], data[3:])))
    print(counter[True])
