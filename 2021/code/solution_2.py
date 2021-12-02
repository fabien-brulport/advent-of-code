from pathlib import Path
from typing import List


DATA_PATH = Path(__file__).resolve().parents[1] / "data"


def read_input(path: Path) -> List[int]:
    data = [elem.split(" ") for elem in path.read_text().strip("\n").split("\n")]
    return [(d, int(l)) for d, l in data]


def main(problem_number: int):
    data = read_input(DATA_PATH / f"input_{problem_number}.txt")

    # Part 1
    depth, position = 0, 0
    for direction, length in data:
        if direction == "forward":
            position += length
        elif direction == "up":
            depth -= length
        elif direction == "down":
            depth += length
        else:
            raise ValueError(direction)

    print(depth * position)

    # Part 2
    depth, position, aim = 0, 0, 0
    for direction, length in data:
        if direction == "forward":
            position += length
            depth += aim * length
        elif direction == "up":
            aim -= length
        elif direction == "down":
            aim += length
        else:
            raise ValueError(direction)

    print(depth * position)
