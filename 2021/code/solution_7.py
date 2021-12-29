from pathlib import Path
from typing import List


DATA_PATH = Path(__file__).resolve().parents[1] / "data"


def read_input(path: Path) -> List[int]:
    return list(map(int, path.read_text().strip("\n").split(",")))


def solve(data: List[int], cost_per_dist: List[int]) -> int:
    min_pos, max_pos = min(data), max(data)
    best_cost = 1e10
    for pos in range(min_pos, max_pos + 1):
        cost = sum(cost_per_dist[abs(pos - crab)] for crab in data)
        if cost < best_cost:
            best_cost = cost

    return best_cost


def main(problem_number: int):
    data = read_input(DATA_PATH / f"input_{problem_number}.txt")
    min_pos, max_pos = min(data), max(data)

    # Part 1
    cost_per_dist = list(range(max_pos - min_pos + 1))
    print(solve(data, cost_per_dist))

    # Part 2
    cost_per_dist = [sum(range(i + 1)) for i in range(max_pos - min_pos + 1)]
    print(solve(data, cost_per_dist))
