from functools import reduce
import operator
from pathlib import Path
from typing import List, Tuple, Set


DATA_PATH = Path(__file__).resolve().parents[1] / "data"


def read_input(path: Path) -> List[List[int]]:
    grid = []
    for line in path.read_text().strip("\n").split("\n"):
        grid.append([int(element) for element in line])

    return grid


def get_height(grid: List[List[int]], coordinates: Tuple[int, int]) -> int:
    """Return the height of a point, returns 9 for out of bound coordinates"""
    i, j = coordinates
    if i < 0 or j < 0:
        return 9
    try:
        return grid[i][j]
    except IndexError:
        return 9


def get_neighbors(
    grid: List[List[int]], coordinates: Tuple[int, int]
) -> List[Tuple[Tuple[int, int], int]]:
    """Returns the neighbors coordinates with their associated height"""
    i, j = coordinates
    return [
        ((i - 1, j), get_height(grid, (i - 1, j))),
        ((i + 1, j), get_height(grid, (i + 1, j))),
        ((i, j - 1), get_height(grid, (i, j - 1))),
        ((i, j + 1), get_height(grid, (i, j + 1))),
    ]


def is_low_point(grid: List[List[int]], coordinates: Tuple[int, int]) -> bool:
    """Return True if coordinates is a low point"""
    point = get_height(grid, coordinates)
    _, neighbors_value = zip(*get_neighbors(grid, coordinates))
    return all(v > point for v in neighbors_value)


def get_basin(
    grid: List[List[int]], low_point: Tuple[int, int]
) -> Set[Tuple[int, int]]:
    """Return the basin of a low point, using all the assumptions"""
    basin = {low_point}
    points_to_check = [low_point]

    while points_to_check:
        coordinates = points_to_check.pop(0)
        for coord, v in get_neighbors(grid, coordinates):
            if v != 9 and coord not in basin:
                points_to_check.append(coord)
                basin.add(coord)
    return basin


def main(problem_number: int):
    data = read_input(DATA_PATH / f"input_{problem_number}.txt")

    # Part 1
    result = 0
    low_points = []
    for i in range(len(data)):
        for j in range(len(data[0])):
            if is_low_point(data, (i, j)):
                low_points.append((i, j))
                result += 1 + get_height(data, (i, j))

    print(result)

    # Part 2
    basin_list = [get_basin(data, point) for point in low_points]
    print(reduce(operator.mul, sorted([len(s) for s in basin_list])[-3:]))
