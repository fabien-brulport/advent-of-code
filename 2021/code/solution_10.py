from pathlib import Path
from typing import List


DATA_PATH = Path(__file__).resolve().parents[1] / "data"
MAPPING_CHARACTER = {
    "(": ")",
    "[": "]",
    "{": "}",
    "<": ">",
}
CORRUPTED_CHARACTER_TO_POINT = {
    ")": 3,
    "]": 57,
    "}": 1197,
    ">": 25137,
}
COMPLETION_CHARACTER_TO_POINT = {
    ")": 1,
    "]": 2,
    "}": 3,
    ">": 4,
}


def read_input(path: Path) -> List[str]:
    return path.read_text().strip("\n").split("\n")


def main(problem_number: int):
    data = read_input(DATA_PATH / f"input_{problem_number}.txt")

    corrupted_chars = []
    incomplete_sequence = []
    for line in data:
        open_chunk = []
        corrupted = False
        for elem in line:
            if elem in MAPPING_CHARACTER.keys():
                open_chunk.append(elem)
            else:
                if elem == MAPPING_CHARACTER[open_chunk[-1]]:
                    open_chunk.pop()
                else:
                    corrupted_chars.append(elem)
                    corrupted = True
                    break

        if not corrupted:
            incomplete_sequence.append(
                [MAPPING_CHARACTER[c] for c in reversed(open_chunk)]
            )

    # Part 1
    print(sum(CORRUPTED_CHARACTER_TO_POINT[c] for c in corrupted_chars))

    # Part 2
    list_scores = []
    for sequence in incomplete_sequence:
        score = 0
        for elem in sequence:
            score = score * 5 + COMPLETION_CHARACTER_TO_POINT[elem]
        list_scores.append(score)

    print(sorted(list_scores)[int(len(list_scores) / 2)])
