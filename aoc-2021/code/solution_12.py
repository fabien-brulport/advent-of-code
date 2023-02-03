from pathlib import Path
from typing import Dict, List


DATA_PATH = Path(__file__).resolve().parents[1] / "data"


class Node:
    def __init__(self, letter):
        self.letter = letter
        self.large = letter.upper() == letter
        self.children = []

    def __str__(self) -> str:
        return f"Large cave: {self.large}, {self.letter} -> "
        "{[c.letter for c in self.children]}"


def read_input(path: Path) -> Dict[str, Node]:
    graph = dict()
    for line in path.read_text().strip("\n").split("\n"):
        c1, c2 = line.split("-")
        n1 = graph.get(c1, Node(c1))
        n2 = graph.get(c2, Node(c2))
        n1.children.append(n2)
        n2.children.append(n1)
        graph[c1] = n1
        graph[c2] = n2
    return graph


def bfs(
    graph: Dict[str, Node],
    node: Node,
    current_path: List[str],
    paths: List[List[str]],
    small_cave_visited: bool,
):
    current_path.append(node.letter)
    # Stop the path
    if node.letter == "end":
        paths.append(current_path)
        return

    for n in node.children:
        if n.large or n.letter not in current_path:
            # Continue the search
            bfs(graph, n, list(current_path), paths, small_cave_visited)
        else:
            if not small_cave_visited and n.letter not in ["start", "end"]:
                # Continue the search but mark the small cave as already visited
                bfs(graph, n, list(current_path), paths, True)

    paths.append(current_path)


def main(problem_number: int):
    data = read_input(DATA_PATH / f"input_{problem_number}.txt")

    # Part 1
    paths = []
    bfs(data, data["start"], [], paths, True)
    print(len([p for p in paths if p[-1] == "end"]))

    # Part 2
    paths = []
    bfs(data, data["start"], [], paths, False)
    print(len([p for p in paths if p[-1] == "end"]))
