from collections import Counter
from pathlib import Path
from typing import Tuple, Dict


DATA_PATH = Path(__file__).resolve().parents[1] / "data"


def read_input(path: Path) -> Tuple[Counter, Dict[str, str], str]:
    state, rules = path.read_text().strip("\n").split("\n\n")
    last_char = state[-1]
    state = Counter(state[i : i + 2] for i in range(len(state) - 1))
    rules = dict(r.split(" -> ") for r in rules.split("\n"))
    return state, rules, last_char


def step(state: Counter, rules: Dict[str, str]) -> Counter:
    new_state = Counter(state)
    for chunk, number in state.items():
        if chunk in rules:
            # Remove all the chunk
            new_state[chunk] -= number
            if new_state[chunk] == 0:
                del new_state[chunk]

            # Create the new chunks, based on the insertion of the letter
            new_state[f"{chunk[0]}{rules[chunk]}"] += number
            new_state[f"{rules[chunk]}{chunk[1]}"] += number

    return new_state


def solve(state: Counter, rules: Dict[str, str], last_char: str, n_steps: int) -> int:
    for i in range(n_steps):
        state = step(state, rules)
    counter = Counter()
    for chunk, number in state.items():
        counter[chunk[0]] = counter.get(chunk[0], 0) + number
    # Add the last char
    counter[last_char] += 1

    most_common = counter.most_common()
    return most_common[0][1] - most_common[-1][1]


def main(problem_number: int):
    state, rules, last_char = read_input(DATA_PATH / f"input_{problem_number}.txt")

    # Part 1
    print(solve(state, rules, last_char, 10))

    # Part 2
    print(solve(state, rules, last_char, 40))
