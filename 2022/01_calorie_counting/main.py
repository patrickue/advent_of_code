import typing

from dataclasses import dataclass
from pprint import pprint

from calorie_inventory import ElfInventoryParser
from ports import InputPort
from rock_paper_scissors import (
    RockPaperScissorsStrategyParser,
    RockPaperScissorsScoreCalculator,
)


# ADAPTERS
class FileReaderAdapter(InputPort):
    file: typing.IO

    def __init__(self, filename: typing.Any):
        self.file = open(filename, "r")

    def __iter__(self) -> typing.Iterator[typing.Any]:
        return self

    def __next__(self) -> typing.Any:
        return self.file.__next__()


def main_01_calorie_counting() -> None:

    calorie_inventory = ElfInventoryParser.parse(
        input_reader=FileReaderAdapter("01_calorie_counting.txt")
    )
    # part 1
    max_calories = max(calorie_inventory, key=lambda x: x.get_total_calories())
    print(
        f"Elf with the biggest inventory carries {max_calories.get_total_calories()} calories."
    )

    # part2
    sorted_calorie_inventories = sorted(
        calorie_inventory, key=lambda x: x.get_total_calories()
    )
    top_three_elves_calories = sum(
        inventory.get_total_calories() for inventory in sorted_calorie_inventories[-3:]
    )
    print(f"Top three elves carry {top_three_elves_calories} calories.")


def main_02_rock_paper_scissors() -> None:

    rps_strategy = RockPaperScissorsStrategyParser.parse(
        input_reader=FileReaderAdapter("02_paper_rock_scissors.txt")
    )
    # for rps_round in rps_strategy:
    #    round_score = RockPaperScissorsScoreCalculator.calculate_score(rps_round)
    #    pprint(f"Round: {rps_round}, scored: {round_score}")

    # part 2
    total_strategy_score = sum(
        RockPaperScissorsScoreCalculator.calculate_score(rps_round)
        for rps_round in rps_strategy
    )
    print(f"Total strategy score: {total_strategy_score}")


if __name__ == "__main__":
    main_01_calorie_counting()
    main_02_rock_paper_scissors()
