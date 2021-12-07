from collections import Counter
from pathlib import Path
from typing import List, Tuple


DATA_PATH = Path(__file__).resolve().parents[1] / "data"


def read_input(path: Path) -> Tuple[List[int], List[List[List[int]]]]:
    raw_numbers, *raw_boards = path.read_text().strip("\n").split("\n\n")
    numbers = [int(n) for n in raw_numbers.split(",")]
    boards = []
    for b in raw_boards:
        boards.append([[int(elem) for elem in row.split()] for row in b.split("\n")])

    return numbers, boards


def is_winning(board: List[List[int]], drawn_numbers: List[int]) -> bool:
    """Check whether or not a board is winning, depending on the drawn numbers"""
    cols, rows = [], []
    for i, row in enumerate(board):
        for j, elem in enumerate(row):
            if elem in drawn_numbers:
                cols.append(j)
                rows.append(i)

    row_counter = Counter(rows)
    cols_counter = Counter(cols)
    if len(row_counter) > 1 and row_counter.most_common(1)[0][1] == len(board):
        return True
    if len(cols_counter) > 1 and cols_counter.most_common(1)[0][1] == len(board[0]):
        return True

    return False


def score(board: List[List[int]], drawn_numbers: List[int]) -> int:
    """Compute the score of a winning board, using the last drawn number"""

    list_elem = []
    for row in board:
        for elem in row:
            if elem not in drawn_numbers:
                list_elem.append(elem)

    return sum(list_elem) * drawn_numbers[-1]


def part_1(numbers: List[int], boards: List[List[List[int]]]) -> int:
    """Compute the score of the first winning board"""
    drawn_numbers = []
    for number in numbers:
        drawn_numbers.append(number)
        for board in boards:
            winning = is_winning(board, drawn_numbers)
            if winning:
                return score(board, drawn_numbers)


def part_2(numbers: List[int], boards: List[List[List[int]]]) -> int:
    """Compute the score of the last winning board"""
    drawn_numbers = []
    index_not_winning = list(range(len(boards)))
    for number in numbers:
        drawn_numbers.append(number)
        for idx in index_not_winning:
            board = boards[idx]
            winning = is_winning(board, drawn_numbers)
            if winning:
                index_not_winning.remove(idx)

            if len(index_not_winning) == 0:
                return score(board, drawn_numbers)


def main(problem_number: int):
    numbers, boards = read_input(DATA_PATH / f"input_{problem_number}.txt")

    # Part 1
    print(part_1(numbers, boards))

    # Part 2
    print(part_2(numbers, boards))
