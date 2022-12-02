import typing

import pytest

from main import WrappingDimensionFromFileParser, PackageDimension
from ports import InputPort


class TestInput(InputPort):
    def __init__(self, init_list: typing.List[str]):
        self.list = init_list
        self.idx = 0

    def __iter__(self):
        return self

    def __next__(self) -> str:
        try:
            result = self.list[self.idx]
            self.idx += 1
            return result
        except IndexError:
            raise StopIteration


@pytest.mark.parametrize(
    "test_input, expected_result",
    [
        ([], []),
        (
            ["12x34x54", "12x23534x45"],
            [
                PackageDimension(12, 34, 54),
                PackageDimension(12, 23534, 45),
            ],
        ),
        (
            ["1x3x5\n", "2x23x5 "],
            [
                PackageDimension(1, 3, 5),
                PackageDimension(2, 23, 5),
            ],
        ),
    ],
)
def test_wrapping_paper_parser_success(test_input, expected_result):
    result = WrappingDimensionFromFileParser.parse(
        input_reader=TestInput(init_list=test_input)
    )

    assert expected_result == result


@pytest.mark.parametrize(
    "test_input, expected_exception",
    [
        (["12x123"], ValueError),
        (["something"], ValueError),
        (["12x123x532x432x234"], ValueError),
        (["WhateverxBullshitxYouknow"], ValueError),
    ],
)
def test_wrapping_paper_parser_exception(test_input, expected_exception):
    with pytest.raises(expected_exception):
        WrappingDimensionFromFileParser.parse(
            input_reader=TestInput(init_list=test_input)
        )


@pytest.mark.parametrize(
    "dimension, expected_area",
    [
        (PackageDimension(1, 1, 1), 6),
        (PackageDimension(1, 2, 3), 12),
        (PackageDimension(29, 10, 100), 278),
    ],
)
def test_wrapping_paper_dimensions_no_slack(
    dimension: PackageDimension, expected_area: int
):
    assert expected_area == dimension.calculate_wrapping_area()


@pytest.mark.parametrize(
    "dimension, expected_area",
    [
        (PackageDimension(1, 1, 1), 7),
        (PackageDimension(1, 2, 3), 13),
        (PackageDimension(6, 6, 4), 36),
    ],
)
def test_wrapping_paper_dimension_with_slack(
    dimension: PackageDimension, expected_area: int
):
    assert expected_area == dimension.calculate_wrapping_area_with_slack()
