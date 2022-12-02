import typing
from dataclasses import dataclass
from enum import Enum, auto

from ports import InputPort


class Symbol(Enum):
    Rock = auto()
    Paper = auto()
    Scissors = auto()


class Outcome(Enum):
    Loose = 0
    Draw = 3
    Win = 6


@dataclass(frozen=True)
class RockPaperScissorsRoundStrategy:
    opponents_choice: Symbol
    outcome: Outcome


OPPONENT_MAPPING = {"A": Symbol.Rock, "B": Symbol.Paper, "C": Symbol.Scissors}
OUTCOME_MAPPING = {"X": Outcome.Loose, "Y": Outcome.Draw, "Z": Outcome.Win}


class RockPaperScissorsStrategyParser:
    @classmethod
    def parse(
        cls, input_reader: InputPort
    ) -> typing.List[RockPaperScissorsRoundStrategy]:
        strategy = list()
        for line in input_reader:
            strategy.append(cls.parse_line(line.strip()))
        return strategy

    @staticmethod
    def parse_line(line: str) -> RockPaperScissorsRoundStrategy:
        strategy_line = line.split(" ")
        if len(strategy_line) != 2:
            raise ValueError
        opponent_symbol = OPPONENT_MAPPING[strategy_line[0]]
        desired_outcome = OUTCOME_MAPPING[strategy_line[1]]
        return RockPaperScissorsRoundStrategy(
            opponents_choice=opponent_symbol, outcome=desired_outcome
        )


class RockPaperScissorsScoreCalculator:
    @classmethod
    def calculate_score(cls, rps_round: RockPaperScissorsRoundStrategy) -> int:
        my_symbol = cls._determine_my_symbol(rps_round)
        return cls._game_points(
            me=my_symbol, opponent=rps_round.opponents_choice
        ) + cls._symbol_points(my_symbol)

    WINNING_MAPPING = {
        Symbol.Rock: Symbol.Paper,
        Symbol.Paper: Symbol.Scissors,
        Symbol.Scissors: Symbol.Rock,
    }
    LOOSING_MAPPING = {v: k for k, v in WINNING_MAPPING.items()}

    @classmethod
    def _game_points(cls, me: Symbol, opponent: Symbol) -> int:
        if opponent == me:
            # draw
            return 3
        elif me == cls.WINNING_MAPPING[opponent]:
            # I won!
            return 6
        else:
            # I lost
            return 0

    SYMBOL_POINT_MAPPING = {Symbol.Rock: 1, Symbol.Paper: 2, Symbol.Scissors: 3}

    @classmethod
    def _symbol_points(cls, me: Symbol) -> int:
        return cls.SYMBOL_POINT_MAPPING[me]

    @classmethod
    def _determine_my_symbol(cls, rps_round: RockPaperScissorsRoundStrategy) -> Symbol:
        if rps_round.outcome == Outcome.Draw:
            return rps_round.opponents_choice
        elif rps_round.outcome == Outcome.Win:
            return cls.WINNING_MAPPING[rps_round.opponents_choice]
        else:
            return cls.LOOSING_MAPPING[rps_round.opponents_choice]
