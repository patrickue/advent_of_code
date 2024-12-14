import typing
from collections import Counter

start = [30, 71441, 3784, 580926, 2, 8122942, 0, 291]
start_counter = Counter(start)

# 3x25 -> 75 Blinks like requested
TOTAL_BLINKS = 25
MAX_DEPTH = 3


def calculate_counter_for_25_cycles(
    marking: int,
) -> typing.Dict[int, Counter[int | typing.Any]]:

    process_list = [marking]
    next_list = list()
    for blink_nr in range(TOTAL_BLINKS):
        next_list = list()
        for pebble in process_list:
            if pebble == 0:
                next_list.append(1)
            elif len(str(pebble)) % 2 == 0:
                pebble_str = str(pebble)
                split_pos = int(len(pebble_str) / 2)
                next_list.append(int(pebble_str[:split_pos]))
                next_list.append(int(pebble_str[split_pos:]))
            else:
                next_list.append(pebble * 2024)
        process_list = next_list
    counter = Counter(next_list)
    return {marking: counter}


def calculate_total_number(pebble_counter: Counter[int | int], depth: int) -> int:

    global global_hash_map  # Dedicated to my colleagues Armin and Ciro
    if depth == MAX_DEPTH:
        return sum(pebble_counter.values())
    else:
        result = 0
        for key, amount in pebble_counter.items():
            if key not in global_hash_map:
                global_hash_map = global_hash_map | calculate_counter_for_25_cycles(key)
            result += amount * calculate_total_number(
                global_hash_map[key], depth=depth + 1
            )
        return result


global_hash_map = dict()

total_sum = calculate_total_number(start_counter, depth=0)
print(f"Sum is: {total_sum}")
