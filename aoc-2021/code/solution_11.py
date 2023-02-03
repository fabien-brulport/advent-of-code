from pathlib import Path
from typing import List, Tuple


DATA_PATH = Path(__file__).resolve().parents[1] / "data"


def read_input(path: Path) -> List[List[int]]:
    lines = []
    for line in path.read_text().strip("\n").split("\n"):
        lines.append([int(elem) for elem in line])
    return lines


def get_neighbors(
    grid: List[List[int]], coords: Tuple[int, int]
) -> List[Tuple[int, int]]:
    """Return the list of the neighbors (coordinates) of a point"""
    ii, jj = coords
    neighbors = []
    for i in range(-1, 2):
        for j in range(-1, 2):
            if i == j == 0:
                continue
            if ii + i < 0 or jj + j < 0:
                continue
            if ii + i >= len(grid) or jj + j >= len(grid[0]):
                continue
            neighbors.append((ii + i, jj + j))

    return neighbors


def do_step(grid: List[List[int]]) -> int:
    """Perform a step and retun the number of flashing octopuses at this step"""
    # First increment
    flashing = []
    for i in range(len(grid)):
        for j in range(len(grid[0])):
            grid[i][j] = grid[i][j] + 1
            if grid[i][j] == 10:
                flashing.append((i, j))
                grid[i][j] = 0

    # Increment neighbors
    to_flash = list(flashing)
    while to_flash:
        coords = to_flash.pop(0)
        neighbors = get_neighbors(grid, coords)
        for i, j in neighbors:
            # Flashin octopuse always restart at 0
            if (i, j) in flashing:
                continue
            grid[i][j] = grid[i][j] + 1
            if grid[i][j] == 10:
                to_flash.append((i, j))
                flashing.append((i, j))
                grid[i][j] = 0

    return len(flashing)


def main(problem_number: int):

    # Part 1
    data = read_input(DATA_PATH / f"input_{problem_number}.txt")
    print(sum(do_step(data) for _ in range(100)))

    # Part 2
    data = read_input(DATA_PATH / f"input_{problem_number}.txt")
    step = 0
    while True:
        step += 1
        n_flash = do_step(data)
        if n_flash == len(data) * len(data[0]):
            break

    print(step)
