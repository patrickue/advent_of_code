import typing

import pytest

from main import check_order, parse_string_to_packet_data


@pytest.mark.parametrize(
    "test_input, expected_output",
    [
        (
            "[1,1,3,100,1]",
            [1, 1, 3, 100, 1],
        ),
        (
            "[3,[1],[1009],[[[],[]]]]",
            [3, [1], [1009], [[[], []]]],
        ),
        ("[1,[2,[3,[432,[5,6,7]]]],8,90]", [1, [2, [3, [432, [5, 6, 7]]]], 8, 90]),
        ("[1,[2,[3,[4,[5,6,0]]]],8,99]", [1, [2, [3, [4, [5, 6, 0]]]], 8, 99]),
    ],
)
def test_parsing(test_input: str, expected_output: typing.List[int]) -> None:
    _, real_output = parse_string_to_packet_data(test_input[1:-1])
    assert real_output == expected_output


@pytest.mark.parametrize(
    "first_line, second_line, expected",
    [
        ([1, 1, 3, 1, 1], [1, 1, 5, 1, 1], True),
        ([[1], [2, 3, 4]], [[1], 4], True),
        ([9], [[8, 7, 6]], False),
        ([[4, 4], 4, 4], [[4, 4], 4, 4, 4], True),
        ([7, 7, 7, 7], [7, 7, 7], False),
        ([], [3], True),
        ([[[]]], [[]], False),
        (
            [1, [2, [3, [4, [5, 6, 7]]]], 8, 9],
            [1, [2, [3, [4, [5, 6, 0]]]], 8, 9],
            False,
        ),
    ],
)
def test_something(first_line: str, second_line: str, expected: bool) -> None:
    result = check_order(first_line, second_line)
    assert result == expected
