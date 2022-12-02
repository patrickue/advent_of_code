import typing
from dataclasses import dataclass
from enum import Enum

import pandas as pd
import numpy as np


@dataclass(frozen=True)
class Direction(Enum):
    FORWARD = 1
    DOWN = 2
    UP = 3


def parse_direction(direction_str: str) -> Direction:
    if direction_str == "forward":
        return Direction.FORWARD
    elif direction_str == "down":
        return Direction.DOWN
    elif direction_str == "up":
        return Direction.UP
    else:
        raise Exception()


@dataclass(frozen=True)
class Command:
    direction: Direction
    steps: int


def read_input(filename: str) -> typing.List[str]:
    with open(filename) as f:
        lines = f.readlines()
        return [line.strip() for line in lines]


def transform_to_commands(lines: typing.List[str]) -> typing.List[Command]:
    commands = []
    for line in lines:
        linesplit = line.split()
        commands.append(
            Command(direction=parse_direction(linesplit[0]), steps=int(linesplit[1]))
        )
    return commands


def sum_for_new_position(
    forward_pos: int, depth: int, commands: typing.List[Command]
) -> (int, int):
    x_pos = forward_pos
    down_pos = depth
    for command in commands:
        if command.direction is Direction.FORWARD:
            x_pos += command.steps
        elif command.direction is Direction.DOWN:
            down_pos += command.steps
        elif command.direction is Direction.UP:
            down_pos -= command.steps
    return x_pos, down_pos


def advanced_course(aim: int, depth: int, commands: typing.List[Command]) -> (int, int):
    x_pos = 0
    down_pos = depth
    for command in commands:
        if command.direction is Direction.FORWARD:
            x_pos += command.steps
            down_pos += aim * command.steps
        elif command.direction is Direction.DOWN:
            aim += command.steps
        elif command.direction is Direction.UP:
            aim -= command.steps
    return x_pos, down_pos


submarine_commands = read_input("input")
print(f"{submarine_commands=}")
commands = transform_to_commands(submarine_commands)
print(f"{commands=}")
final_pos = sum_for_new_position(forward_pos=0, depth=0, commands=commands)
print(f"{final_pos=}")
print(f"{final_pos[0]*final_pos[1]}")
advanced_pos = advanced_course(aim=0, depth=0, commands=commands)
print(f"{advanced_pos=}")
print(f"{advanced_pos[0]*advanced_pos[1]}")
