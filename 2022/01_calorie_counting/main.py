import typing

# from dataclasses import dataclass
from pprint import pprint

# from calorie_inventory import ElfInventoryParser
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


def main() -> None:

    rps_strategy = RockPaperScissorsStrategyParser.parse(
        input_reader=FileReaderAdapter("02_paper_rock_scissors.txt")
    )
    for rps_round in rps_strategy:
        round_score = RockPaperScissorsScoreCalculator.calculate_score(rps_round)
        pprint(f"Round: {rps_round}, scored: {round_score}")

    total_strategy_score = sum(
        RockPaperScissorsScoreCalculator.calculate_score(rps_round)
        for rps_round in rps_strategy
    )
    pprint(f"Total strategy score: {total_strategy_score}")


if __name__ == "__main__":
    main()
