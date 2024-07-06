from __future__ import annotations

import functools
import operator
import re
import typing
from enum import Enum
from pprint import pprint

from pydantic import BaseModel


class InvalidOperationExpressionError(Exception):
    ...


class Operation(str, Enum):
    ADD = "+"
    MULTIPLY = "*"
    SQUARE = "^2"

    @classmethod
    def parse_from_expression(
        cls, expression: str
    ) -> (Operation, typing.Optional[int]):
        if not expression.startswith("new ="):
            raise InvalidOperationExpressionError()
        re_groups = re.search(r"new = old (.) (\w+)", expression)
        if re_groups.group(1) == "*" and re_groups.group(2) == "old":
            return cls.SQUARE, None
        elif re_groups.group(1) == "+":
            operation = Operation.ADD
        elif re_groups.group(1) == "*":
            operation = Operation.MULTIPLY
        else:
            raise InvalidOperationExpressionError()
        operand = int(re_groups.group(2))
        return operation, operand


class Monkey(BaseModel):
    id: int
    items: typing.List[int]
    operation: Operation
    operand: typing.Optional[int]
    test_divisible_by: int
    target_on_true: int
    target_on_false: int
    inspection_counter: int = 0
    worry_level_divider: int = 1

    class Config:
        use_enum_values = True

    @staticmethod
    def parse_from_text(monkey_text: typing.List[str]):
        monkey_dict = dict()
        for line in monkey_text:
            if line.startswith("Monkey"):
                pass
                id_text = re.search(r"Monkey (\d+):", line)
                monkey_dict["id"] = int(id_text.group(1))
            elif line.startswith("Starting items"):
                monkey_dict["items"] = [
                    int(item.group(0)) for item in re.finditer(r"\d+", line)
                ]
            elif line.startswith("Operation"):
                operation_text = re.search(r"^Operation: (.+)$", line)
                (
                    monkey_dict["operation"],
                    monkey_dict["operand"],
                ) = Operation.parse_from_expression(operation_text.group(1))
            elif line.startswith("Test: divisible by "):
                test_operand = re.search(r"^Test: divisible by (\d+)$", line)
                monkey_dict["test_divisible_by"] = int(test_operand.group(1))
            elif line.startswith("If true:"):
                target_monkey = re.search(r"^If true: throw to monkey (\d+)$", line)
                monkey_dict["target_on_true"] = int(target_monkey.group(1))
            elif line.startswith("If false:"):
                target_monkey = re.search(r"^If false: throw to monkey (\d+)$", line)
                monkey_dict["target_on_false"] = int(target_monkey.group(1))
        return Monkey(**monkey_dict)

    def inspect_and_throw(self, monkeys: typing.Dict[int, Monkey]):
        while len(self.items) != 0:
            item = self.items.pop()
            self.inspection_counter += 1
            new_item = self.decrease_my_worry_level(item, self.worry_level_divider)
            new_item = self.calculate_worry_level(
                new_item, self.operation, self.operand
            )
            target_monkey_id = self.decide_where_to_throw_item(new_item)
            monkeys[target_monkey_id].catch_item(new_item)

    def catch_item(self, item: int) -> None:
        self.items.append(item)

    @staticmethod
    def calculate_worry_level(
        item: int, operation: Operation, operand: typing.Optional[int]
    ) -> int:
        if operation == Operation.ADD:
            return item + operand
        elif operation == Operation.MULTIPLY:
            return item * operand
        elif operation == Operation.SQUARE:
            return item * item

    @staticmethod
    def decrease_my_worry_level(previous_worry_level: int, test_divisor: int) -> int:
        return previous_worry_level % test_divisor

    def decide_where_to_throw_item(self, item: int) -> int:
        remainder_for_test = item % self.test_divisible_by
        if remainder_for_test == 0:
            return self.target_on_true
        else:
            return self.target_on_false


if __name__ == "__main__":

    file = open("11_monkey_middle.txt")
    monkeys: typing.Dict[id, Monkey] = dict()
    monkey_lines = list()
    for line in file:
        if line.strip() != "":
            monkey_lines.append(line.strip())
        else:
            monkey = Monkey.parse_from_text(monkey_lines)
            monkey_lines = list()
            monkeys[monkey.id] = monkey

    monkey_test_divisors = [monkey.test_divisible_by for monkey in monkeys.values()]
    smallest_common_multiple = functools.reduce(operator.mul, monkey_test_divisors)
    for monkey in monkeys.values():
        monkey.worry_level_divider = smallest_common_multiple

    for i in range(0, 10_000):
        for monkey in monkeys.values():
            monkey.inspect_and_throw(monkeys)

    # pprint(monkeys)

    monkey_list = monkeys.values()
    sorted_monkeys = sorted(monkey_list, key=lambda m: m.inspection_counter)
    pprint(
        (
            f"Monkey crazyness: "
            f"{sorted_monkeys[-1].inspection_counter * sorted_monkeys[-2].inspection_counter}"
        )
    )
