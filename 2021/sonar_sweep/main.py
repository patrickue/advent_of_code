import typing
import pandas as pd
import numpy as np


def read_input(filename: str) -> typing.List[int]:
    with open(filename) as f:
        lines = f.readlines()
        return [int(line.strip()) for line in lines]


def count_increasing(lines: typing.List[int]) -> int:
    previous = None
    count = 0
    np_lines = np.array(lines)
    for line in np_lines:
        if previous:
            if previous < line:
                count += 1
        previous = line
    return count


def sonar_rolling_window(lines: typing.List[int]) -> typing.List[int]:
    return pd.DataFrame(lines).rolling(window=3, min_periods=3).sum()


sonar_results = read_input("input")
print(f"{sonar_results=}")
inc_count = count_increasing(sonar_results)
print(f"{inc_count=}")
rolling_windows = sonar_rolling_window(sonar_results)
print(f"{rolling_windows=}")
inc_count = count_increasing(rolling_windows)
print(f"{inc_count=}")
