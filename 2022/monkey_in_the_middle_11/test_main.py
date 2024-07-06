import pytest as pytest

from main import Monkey, Operation

TEST_MONKEY_TEXT = [
    "Monkey 0:",
    "Starting items: 83, 62, 93",
    "Operation: new = old * 17",
    "Test: divisible by 2",
    "If true: throw to monkey 1",
    "If false: throw to monkey 6",
]


TEST_MONKEY_TEXT_2 = [
    "Monkey 4:",
    "Starting items: 98, 92, 99, 51",
    "Operation: new = old * old",
    "Test: divisible by 5",
    "If true: throw to monkey 0",
    "If false: throw to monkey 1",
]


@pytest.mark.parametrize(
    "monkey_text, expected_monkey",
    [
        (
            TEST_MONKEY_TEXT,
            Monkey(
                id=0,
                items=[83, 62, 93],
                operation=Operation.MULTIPLY,
                operand=17,
                test_divisible_by=2,
                target_on_true=1,
                target_on_false=6,
            ),
        ),
        (
            TEST_MONKEY_TEXT_2,
            Monkey(
                id=4,
                items=[98, 92, 99, 51],
                operation=Operation.SQUARE,
                operand=None,
                test_divisible_by=5,
                target_on_true=0,
                target_on_false=1,
            ),
        ),
    ],
)
def test_monkey_parsing(monkey_text, expected_monkey):
    monkey = Monkey.parse_from_text(monkey_text)

    assert monkey == expected_monkey
