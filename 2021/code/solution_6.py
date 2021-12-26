from collections import Counter
from pathlib import Path
from typing import List


DATA_PATH = Path(__file__).resolve().parents[1] / "data"


def read_input(path: Path) -> List[int]:
    return list(map(int, path.read_text().strip("\n").split(",")))


def update(state: dict) -> dict:
    """
    Update the state by doing one iteration (update timeres,
    create new fishes etc
    """

    new_state = dict()
    for i in range(7):
        new_state[(i - 1) % 7] = state.get(i, 0)
    new_state[6] = state.get(7, 0) + new_state[6]
    new_state[7] = state.get(8, 0)
    new_state[8] = state.get(0, 0)
    return new_state


def main(problem_number: int):
    # Part 1
    data = read_input(DATA_PATH / f"input_{problem_number}.txt")
    state = dict(Counter(data))
    for i in range(80):
        state = update(state)
    print(sum(state.values()))

    # Part 2
    data = read_input(DATA_PATH / f"input_{problem_number}.txt")
    state = dict(Counter(data))
    for i in range(256):
        state = update(state)
    print(sum(state.values()))
