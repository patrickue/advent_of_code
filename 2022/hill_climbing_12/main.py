import typing
from collections import deque
from pprint import pprint

import byte as byte


def parse_map(file: typing.IO) -> typing.List[bytearray]:
    topo_map = list()
    for line in file:
        b = bytearray()
        b.extend(line.strip().encode("ascii"))
        topo_map.append(b)
    return topo_map


def find_letters(
    topo_map: typing.List[bytearray], letter: byte
) -> typing.List[typing.Tuple[int, int]]:
    found_positions = list()
    for i, line in enumerate(topo_map):
        pos = line.find(letter)
        if pos != -1:
            found_positions.append((pos, i))
    return found_positions


def find_shortest_path(
    topological_map: typing.List[bytearray],
    start_pos: typing.Tuple[int, int],
    end_pos: typing.Tuple[int, int],
):
    distance_list: typing.List[typing.List[typing.Optional[int]]] = [
        [None] * len(topological_map[0]) for x in topological_map
    ]
    work_list = deque()
    start_pos_x, start_pos_y = start_pos
    distance_list[start_pos_y][start_pos_x] = 0
    work_list.append(start_pos)

    while len(work_list) > 0:
        pos_x, pos_y = work_list.popleft()
        step_count = distance_list[pos_y][pos_x]

        next_possible_fields = [
            (pos_x + 1, pos_y),
            (pos_x, pos_y + 1),
            (pos_x - 1, pos_y),
            (pos_x, pos_y - 1),
        ]
        for next_field in next_possible_fields:
            x, y = next_field
            if 0 <= x < len(topological_map[0]) and 0 <= y < len(topological_map):
                if topological_map[y][x] <= topological_map[pos_y][pos_x] + 1:
                    if (
                        distance_list[y][x] is None
                        or step_count + 1 < distance_list[y][x]
                    ):
                        distance_list[y][x] = step_count + 1
                        work_list.append(next_field)

    end_pos_x, end_pos_y = end_pos
    return distance_list[end_pos_y][end_pos_x]


if __name__ == "__main__":
    file = open("12_input_liuba.txt", "r")
    topological_map = parse_map(file)

    pprint(topological_map)

    start_pos = find_letters(topological_map, b"S")[0]
    end_pos = find_letters(topological_map, b"E")[0]
    all_a_pos = find_letters(topological_map, b"a")

    pprint(f"Task, go from: {start_pos} to {end_pos}.")

    start_pos_x, start_pos_y = start_pos
    end_pos_x, end_pos_y = end_pos
    topological_map[start_pos_y][start_pos_x] = ord("a")
    topological_map[end_pos_y][end_pos_x] = ord("z")

    # for a_pos in all_a_pos:
    shortest_distance = find_shortest_path(topological_map, start_pos, end_pos)
    pprint(f"Shortest distance between S and E is: {shortest_distance}")
