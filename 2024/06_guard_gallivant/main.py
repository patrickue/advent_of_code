import copy
from collections import defaultdict
from dataclasses import dataclass
from enum import Enum, auto
from typing import List


class Direction(Enum):
    UP = auto()
    DOWN = auto()
    LEFT = auto()
    RIGHT = auto()


@dataclass(frozen=True)
class Point:
    x: int
    y: int


@dataclass
class GuardState:
    pos: Point
    direction: Direction

    ROTATION_MAP = {
        Direction.UP: Direction.RIGHT,
        Direction.RIGHT: Direction.DOWN,
        Direction.DOWN: Direction.LEFT,
        Direction.LEFT: Direction.UP,
    }

    def rotate(self):
        self.direction = self.ROTATION_MAP[self.direction]

    def predict_step_forward(self):
        if self.direction == Direction.UP:
            pos = Point(x=self.pos.x, y=self.pos.y - 1)
        if self.direction == Direction.DOWN:
            pos = Point(x=self.pos.x, y=self.pos.y + 1)
        if self.direction == Direction.LEFT:
            pos = Point(x=self.pos.x - 1, y=self.pos.y)
        if self.direction == Direction.RIGHT:
            pos = Point(x=self.pos.x + 1, y=self.pos.y)
        return GuardState(pos=pos, direction=self.direction)


file = open("input.txt")

matrix = [list(line.strip()) for line in file]


def find_guard(matrix: List[List[chr]]) -> GuardState:
    for y in range(len(matrix)):
        x_pos_guard = "".join(matrix[y]).find("^")
        if x_pos_guard != -1:
            return GuardState(pos=Point(x=x_pos_guard, y=y), direction=Direction.UP)


def is_on_edge(guard: GuardState, x_size: int, y_size: int):
    return (
        guard.pos.x == 0
        or guard.pos.y == 0
        or guard.pos.x == x_size - 1
        or guard.pos.y == y_size - 1
    )


def mark_guard_path(
    input_map: List[List[chr]], start_guard: GuardState
) -> (bool, List[List[chr]]):

    guard_state = copy.deepcopy(start_guard)
    map = copy.deepcopy(input_map)

    walked_ways = defaultdict(list)
    walked_ways[guard_state.pos].append(guard_state.direction)

    ends_in_loop = False
    while not is_on_edge(guard_state, len(map[0]), len(map)) and not ends_in_loop:
        next_guard = guard_state.predict_step_forward()
        if map[next_guard.pos.y][next_guard.pos.x] == "#":
            guard_state.rotate()
        else:
            map[guard_state.pos.y][guard_state.pos.x] = "X"
            guard_state = next_guard
            if guard_state.direction in walked_ways[guard_state.pos]:
                # print("Loop detected")
                ends_in_loop = True
            else:
                walked_ways[guard_state.pos].append(guard_state.direction)

    # mark final position on the edge
    map[guard_state.pos.y][guard_state.pos.x] = "X"

    return ends_in_loop, map


def main():
    [print(line) for line in matrix]
    guard = find_guard(matrix)
    print(guard)
    ends_in_loop, marked_map = mark_guard_path(matrix, guard)
    assert ends_in_loop == False
    [print("".join(line)) for line in marked_map]
    covered_fields = sum(["".join(line).count("X") for line in marked_map])
    print(f"Fields covered by Guard: {covered_fields}")

    possible_positions_for_loop = 0
    for x in range(len(marked_map[0])):
        for y in range(len(marked_map)):
            if marked_map[y][x] == "X":
                marked_map[y][x] = "#"
                ends_in_loop, _ = mark_guard_path(marked_map, guard)
                if ends_in_loop:
                    possible_positions_for_loop += 1
                marked_map[y][x] = "X"
            else:
                # ignore fields which the guard never reaches
                pass

    print(
        f"Possible positions for a blocker which end in a loop: {possible_positions_for_loop}"
    )


main()
