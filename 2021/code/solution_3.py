from collections import Counter
from pathlib import Path
from typing import List


DATA_PATH = Path(__file__).resolve().parents[1] / "data"


def read_input(path: Path) -> List[str]:
    return path.read_text().strip("\n").split("\n")


def main(problem_number: int):
    data = read_input(DATA_PATH / f"input_{problem_number}.txt")

    # Part 1
    gamma, epsilon = "", ""
    for a in zip(*data):
        counter = Counter(a)
        most_common = counter.most_common(2)
        gamma += most_common[0][0]
        epsilon += most_common[1][0]
    print(int(gamma, 2) * int(epsilon, 2))

    # Part 2
    oxygen_idx, co2_idx = list(range(len(data))), list(range(len(data)))
    for a in zip(*data):
        # Oxygen
        if len(oxygen_idx) > 1:
            counter = Counter(a[idx] for idx in oxygen_idx)
            most_common, most_common_number = counter.most_common(1)[0]
            oxigen_value = (
                "1" if most_common_number == len(oxygen_idx) / 2 else most_common
            )
            oxygen_idx = [idx for idx in oxygen_idx if a[idx] == oxigen_value]

        # CO2
        if len(co2_idx) > 1:
            counter = Counter(a[idx] for idx in co2_idx)
            less_common, less_common_number = counter.most_common(2)[1]
            co2_value = "0" if less_common_number == len(co2_idx) / 2 else less_common
            co2_idx = [idx for idx in co2_idx if a[idx] == co2_value]

    assert len(oxygen_idx) == len(co2_idx) == 1
    print(int(data[oxygen_idx[0]], 2) * int(data[co2_idx[0]], 2))
