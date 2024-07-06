import re
import typing
from collections import deque
from dataclasses import dataclass
from pprint import pprint
from typing import Iterable


@dataclass(frozen=True)
class StackMovement:
    source: int
    target: int
    amount: int


class StorageState(dict):
    pass


def apply_movements_to_storage_state(state: StorageState, movements: typing.List[StackMovement]):
    for movement in movements:
        crane_storage = deque()
        for x in range(0, movement.amount):
            crate = state[movement.source].pop()
            crane_storage.append(crate)
        for x in range(0, movement.amount):
            state[movement.target].append(crane_storage.pop())
    return state


def print_top_stacks(new_state):
    top_crates = str()
    for idx in range(1, 10):
        top_crates += (new_state[idx].pop())
    print(f"Top crates are: {str(top_crates)}")


def read_input_file(file: Iterable[str]) -> (StorageState, typing.List[StackMovement]):
    storage_state = read_storage_configuration(file)
    movements=read_stack_movements(file)
    new_state = apply_movements_to_storage_state(state=storage_state, movements=movements)
    pprint(new_state)
    print_top_stacks(new_state)


def read_storage_configuration(input: Iterable[str]) -> StorageState:
    stack_positions = []
    storage_configuration = list()
    for line in input:
        if not line.startswith(' 1 '):
            storage_configuration.append(line)
        else:
            stack_positions = find_stack_positions_in_string(line)
            break
    pprint(storage_configuration)
    stacks = StorageState()
    for idx, position in enumerate(stack_positions):
        stacks[idx+1] = deque()
    for storage_line in storage_configuration:
        for idx, position in enumerate(stack_positions):
            crate_letter = storage_line[position]
            if crate_letter != ' ':
                stacks[idx+1].appendleft(crate_letter)
    pprint(stacks)
    return stacks


def find_stack_positions_in_string(line: str) -> typing.List[int]:
    index_list = list()
    for i in range(1, 12):
        position = line.find(str(i))
        if position != -1:
            index_list.append(position)
        else:
            return index_list


def read_stack_movements(input: Iterable[str]) -> typing.List[StackMovement]:
    movements = list()
    for line in input:
        if line.startswith("move"):
            trimmed_line = line.strip()
            movement_substring = re.split(r"\s?[a-z]+\s?", trimmed_line)
            amount = int(movement_substring[1])
            source = int(movement_substring[2])
            target = int(movement_substring[3])
            movements.append(StackMovement(amount=amount, source=source, target=target))
    pprint(movements)
    return movements


def main():
    input_file = open("05_supply_stacks.txt", 'r')
    read_input_file(input_file)


if __name__ == '__main__':
    main()