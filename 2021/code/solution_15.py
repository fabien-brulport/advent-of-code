import heapq
import math
from pathlib import Path
from typing import List


DATA_PATH = Path(__file__).resolve().parents[1] / "data"


def read_input(path: Path) -> List[List[int]]:
    lines = []
    for line in path.read_text().strip("\n").split("\n"):
        lines.append([int(elem) for elem in line])
    return lines


def grid_to_graph(grid: List[List[int]]):
    max_x = len(grid[0])
    max_y = len(grid)
    graph = dict()
    for x in range(max_x):
        for y in range(max_y):
            children = dict()
            if x > 0:
                children[(x - 1, y)] = grid[y][x - 1]
            if y > 0:
                children[(x, y - 1)] = grid[y - 1][x]
            if x < max_x - 1:
                children[(x + 1, y)] = grid[y][x + 1]
            if y < max_y - 1:
                children[(x, y + 1)] = grid[y + 1][x]
            graph[(x, y)] = children

    return graph


def dijkstra(graph, end) -> int:
    """
    Dijkstra using priority queue, see
    https://docs.python.org/3/library/heapq.html#priority-queue-implementation-notes.
    """
    # Initialization
    heap = []
    distances = {(0, 0): 0}
    # One item is [dist, coords, valid]
    item = [0, (0, 0), True]
    heapq.heappush(heap, item)
    # State is used to update the item in the heap, knowing the coords.
    # /!\ we can't update the distance, as it will break the heap !
    state = {(0, 0): item}

    while heap:
        current_dist, current, valid = heapq.heappop(heap)
        if not valid:
            # Skip non valid element
            continue
        if current == end:
            return current_dist

        for pos, cost in graph[current].items():
            next_cost = cost + distances[current]
            if next_cost < distances.get(pos, math.inf):
                distances[pos] = next_cost
                # As we can't update the distance of the elem in the heap (it
                # will break the heap), we use the boolean to mark
                # that the item is removed.
                if pos in state:
                    state[pos][-1] = False

                item = [next_cost, (pos), True]
                state[pos] = item
                heapq.heappush(heap, item)

    return distances[end]


def main(problem_number: int):
    data = read_input(DATA_PATH / f"input_{problem_number}.txt")

    # Part 1
    max_x = len(data[0])
    max_y = len(data)
    graph = grid_to_graph(data)
    print(dijkstra(graph, (max_x - 1, max_y - 1)))

    # Part 2
    new_grid = []
    for j in range(5):
        for y in range(max_y):
            line = []
            for i in range(5):
                for x in range(max_x):
                    line.append((data[y][x] + i + j - 1) % 9 + 1)
            new_grid.append(line)

    graph = grid_to_graph(new_grid)
    print(dijkstra(graph, (max_x * 5 - 1, max_y * 5 - 1)))
