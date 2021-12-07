from collections import Counter
from dataclasses import dataclass
from pathlib import Path
from typing import List


DATA_PATH = Path(__file__).resolve().parents[1] / "data"


@dataclass
class Point:
    x: int
    y: int

    def __hash__(self) -> int:
        return hash((self.x, self.y))


@dataclass
class Line:
    p1: Point
    p2: Point

    def is_straight(self) -> bool:
        return (self.p1.x == self.p2.x) or (self.p1.y == self.p2.y)

    def points_covered(self, all_directions: bool) -> List[Point]:
        if self.p1.x == self.p2.x:
            y_direction = max(min(self.p2.y - self.p1.y, 1), -1)
            return [
                Point(x=self.p1.x, y=y)
                for y in range(self.p1.y, self.p2.y + y_direction, y_direction)
            ]
        elif self.p1.y == self.p2.y:
            x_direction = max(min(self.p2.x - self.p1.x, 1), -1)
            return [
                Point(x=x, y=self.p1.y)
                for x in range(self.p1.x, self.p2.x + x_direction, x_direction)
            ]
        if not all_directions:
            return []

        x_direction = max(min(self.p2.x - self.p1.x, 1), -1)
        y_direction = max(min(self.p2.y - self.p1.y, 1), -1)
        return [
            Point(x=x, y=y)
            for x, y in zip(
                range(self.p1.x, self.p2.x + x_direction, x_direction),
                range(self.p1.y, self.p2.y + y_direction, y_direction),
            )
        ]


def read_input(path: Path) -> List[Line]:
    lines = []
    for line in path.read_text().strip("\n").split("\n"):
        coord1, coord2 = line.split(" -> ")
        x1, y1 = tuple(int(elem) for elem in coord1.split(","))
        x2, y2 = tuple(int(elem) for elem in coord2.split(","))
        lines.append(Line(p1=Point(x=x1, y=y1), p2=Point(x=x2, y=y2)))
    return lines


def main(problem_number: int):
    lines = read_input(DATA_PATH / f"input_{problem_number}.txt")

    # Part 1
    all_points = []
    for line in lines:
        all_points.extend(line.points_covered(all_directions=False))
    counter = Counter(all_points)
    print(sum(v > 1 for v in counter.values()))

    # Part 2
    all_points = []
    for line in lines:
        all_points.extend(line.points_covered(all_directions=True))
    counter = Counter(all_points)
    print(sum(v > 1 for v in counter.values()))
