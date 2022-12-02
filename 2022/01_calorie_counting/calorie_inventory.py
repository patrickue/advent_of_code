import typing
from dataclasses import dataclass

from ports import InputPort


@dataclass(frozen=True)
class ElfInventory:
    id: int
    calories: typing.List[int]

    def get_total_calories(self) -> int:
        return sum(self.calories)


class ElfInventoryParser:
    @classmethod
    def parse(cls, input_reader: InputPort) -> typing.List[ElfInventory]:
        elf_inventories: typing.List[ElfInventory] = list()
        elf_id = 0
        calories = list()
        for line in input_reader:
            line_stripped = line.strip()
            if line_stripped != '':
                calories.append(int(line_stripped.strip()))
            else:
                elf_inventories.append(ElfInventory(id=elf_id, calories=calories))
                calories = list()

        return elf_inventories
