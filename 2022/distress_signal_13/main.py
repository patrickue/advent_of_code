from __future__ import annotations
import itertools
from dataclasses import dataclass
from pprint import pprint
from typing import List, Optional, Union, IO, Tuple


@dataclass(frozen=True)
class PacketPair:
    index: int
    left_data: PacketData
    right_data: PacketData


PacketData = List[Union["PacketData", int]]


def check_order(first: PacketData, second: PacketData) -> bool:

    for left, right in itertools.zip_longest(first, second):
        if isinstance(left, int) and isinstance(right, int):
            if left < right:
                return True
            if left == right:
                pass
            if left > right:
                return False
        elif left is None and right is not None:
            return True
        elif left is not None and right is None:
            return False
        else:
            left = [left] if isinstance(left, int) else left
            right = [right] if isinstance(right, int) else right
            if check_order(left, right) is False:
                return False
    return True


def parse_string_to_packet_data(
    line: str,
    pos: int = 0,
) -> Tuple[int, PacketData]:
    collected_number: str = ""
    result: PacketData = list()
    i = pos
    while i < len(line):
        char = line[i]
        i += 1

        if char in "[],":
            parsed_number = parse_number(collected_number)
            if parsed_number is not None:
                result.append(parsed_number)
                collected_number = ""
            if char == "[":
                new_pos, output = parse_string_to_packet_data(line, i)
                i = new_pos
                result.append(output)
            if char == "]":
                return i, result
        if char in "0123456789":
            collected_number += char

    parsed_number = parse_number(collected_number)
    if parsed_number:
        result.append(parsed_number)
    return 0, result


def parse_number(num_str: str) -> Optional[int]:
    if num_str != "":
        return int(num_str)
    return None


def parse_pair_list(file: IO) -> List[PacketPair]:
    pair_list: List[PacketPair] = list()
    right_list = None
    left_list = None
    index = 1
    for line in file:
        if line.strip() == "":
            pass
        else:
            _, packet = parse_string_to_packet_data(line)
            if left_list is None:
                left_list = packet
            else:
                right_list = packet
                pair_list.append(PacketPair(index, left_list, right_list))
                left_list = None
                index += 1

    return pair_list


if __name__ == "__main__":
    file = open("input_13.txt", "r")

    pairs = parse_pair_list(file)

    pprint(pairs)

    index_sum = 0
    for pair in pairs:
        if check_order(pair.left_data, pair.right_data):
            index_sum += pair.index
    print(f"The sum of pair indexes which are in order is: {index_sum}")
