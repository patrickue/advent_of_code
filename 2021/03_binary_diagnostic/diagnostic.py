import typing
from dataclasses import dataclass
from enum import Enum

import pandas as pd
import numpy as np


def read_input(filename: str) -> typing.List[str]:
    with open(filename) as f:
        lines = f.readlines()
        return [line.strip() for line in lines]


def transform_to_array(lines: typing.List[str]) -> typing.Dict[int, typing.List[int]]:
    binary_dict = {}
    for line in lines:
        for idx, char in enumerate(line):
            binary_dict.setdefault(idx, []).append(int(char))
    return binary_dict


def calculate_gamma(transposed_dict: typing.Dict[int, typing.List[int]]) -> typing.List[int]:
    idx = 0
    gamma_raw = []
    while transposed_dict.get(idx) is not None:
        char_list = transposed_dict[idx]
        gamma_raw.append(int(round(sum(char_list)/len(char_list))))
        idx += 1
    return gamma_raw

submarine_commands = read_input("input")
print(f"{submarine_commands=}")
binary_array = transform_to_array(submarine_commands)
print(f"{binary_array=}")
something = calculate_gamma(binary_array)
print(f"{something=}")
gamma = int("".join(str(x) for x in something), 2)
print(f"{gamma=}")
epsilon = int("".join(str(0 if x == 1 else 1) for x in something), 2)
print(f"{epsilon=}")
print(f"{epsilon*gamma=}")
