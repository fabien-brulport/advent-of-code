from pathlib import Path
from typing import List, Tuple, Set


DATA_PATH = Path(__file__).resolve().parents[1] / "data"


def read_input(path: Path) -> Tuple[Set[Tuple[int, int]], List[Tuple[str, int]]]:
    dots, instructions = path.read_text().strip("\n").split("\n\n")
    dots = set(tuple(map(int, d.split(","))) for d in dots.split("\n"))
    instructions = [tuple(i[11:].split("=")) for i in instructions.split("\n")]
    instructions = [(d, int(pos)) for d, pos in instructions]

    return dots, instructions


def transform_dot(
    dot: Tuple[int, int], instruction: Tuple[str, int]
) -> Tuple[int, int]:
    d, pos = instruction
    x, y = dot
    if d == "x" and x > pos:
        dot = (pos * 2 - x, y)
    elif d == "y" and y > pos:
        dot = (x, pos * 2 - y)
    return dot


def print_grid(dots: Set[Tuple[int, int]]):
    xx, yy = zip(*dots)
    for y in range(max(yy) + 1):
        line = []
        for x in range(max(xx) + 1):
            line.append("#" if (x, y) in dots else ".")
        print("".join(line))


def main(problem_number: int):
    dots, instructions = read_input(DATA_PATH / f"input_{problem_number}.txt")

    # Part 1
    print(len(set(transform_dot(d, instructions[0]) for d in dots)))

    # Part 2
    for instruction in instructions:
        dots = set(transform_dot(d, instruction) for d in dots)
    print_grid(dots)
