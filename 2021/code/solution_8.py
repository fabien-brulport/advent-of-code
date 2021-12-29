from pathlib import Path
from typing import List, Set, Callable


DATA_PATH = Path(__file__).resolve().parents[1] / "data"


def read_input(path: Path):
    lines = []
    for line in path.read_text().strip("\n").split("\n"):
        signals, digits = line.split(" | ")
        signals = [set(s) for s in signals.split(" ")]
        digits = [set(d) for d in digits.split(" ")]
        lines.append((signals, digits))
    return lines


def digit_from_signal(mapping, signal):
    for d, s in mapping.items():
        if s == signal:
            return d
    raise ValueError(f"{signal} not found in {mapping}")


def find_mapping(
    signals: List[Set[str]], condition: Callable[[Set[str]], bool]
) -> Set[str]:
    possible_solution = [s for s in signals if condition(s)]
    assert len(possible_solution) == 1
    return possible_solution[0]


def solve(signals: List[Set[str]], digits: List[Set[str]]) -> int:
    mapping = {
        1: find_mapping(signals, lambda s: len(s) == 2),
        7: find_mapping(signals, lambda s: len(s) == 3),
        4: find_mapping(signals, lambda s: len(s) == 4),
        8: find_mapping(signals, lambda s: len(s) == 7),
    }

    # Find 0, 6 and 9
    mapping[9] = find_mapping(
        signals, lambda s: len(s) == 6 and s.intersection(mapping[4]) == mapping[4]
    )
    mapping[0] = find_mapping(
        signals,
        lambda s: len(s) == 6
        and s.intersection(mapping[1]) == mapping[1]
        and s != mapping[9],
    )
    mapping[6] = find_mapping(
        signals, lambda s: len(s) == 6 and s != mapping[9] and s != mapping[0]
    )

    # Find 2, 3 and 5
    mapping[5] = find_mapping(
        signals, lambda s: len(s) == 5 and mapping[6].intersection(s) == s
    )
    mapping[3] = find_mapping(
        signals, lambda s: len(s) == 5 and s.intersection(mapping[1]) == mapping[1]
    )
    mapping[2] = find_mapping(
        signals, lambda s: len(s) == 5 and s != mapping[5] and s != mapping[3]
    )

    list_digit = [digit_from_signal(mapping, d) for d in digits]
    return int("".join(str(d) for d in list_digit))


def main(problem_number: int):
    data = read_input(DATA_PATH / f"input_{problem_number}.txt")

    # Part 1
    _, all_digits = zip(*data)
    all_digits = [d for digits in all_digits for d in digits]
    length_unique_digits = [2, 3, 4, 7]
    print(sum(len(d) in length_unique_digits for d in all_digits))

    # Part 2
    print(sum([solve(*line) for line in data]))
