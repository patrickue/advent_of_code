import typing
from dataclasses import dataclass
from pprint import pprint

from db_reader_adapter import DBReaderAdapter
from ports import InputPort


# BUSINESS MODELS / TYPES
@dataclass(frozen=True)
class PackageDimension:
    l: int
    w: int
    h: int

    def calculate_wrapping_area_with_slack(self) -> int:
        return self.calculate_wrapping_area() + min(
            self.l * self.w, self.w * self.h, self.h * self.l
        )

    def calculate_wrapping_area(self) -> int:
        return 2 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l

    def calculate_ribbon_length(self) -> int:
        dimensions = sorted([self.l, self.w, self.h])

        return 2 * (dimensions[0] + dimensions[1]) + self.h * self.w * self.l


# ADAPTERS
class FileReaderAdapter(InputPort):
    file: typing.IO

    def __init__(self, filename: typing.Any):
        self.file = open(filename, "r")

    def __iter__(self) -> typing.Iterator[typing.Any]:
        return self

    def __next__(self) -> typing.Any:
        return self.file.__next__()


class WrappingDimensionFromFileParser:
    @classmethod
    def parse(cls, input_reader: InputPort) -> typing.List[PackageDimension]:
        dimensions = list()
        for line in input_reader:
            dimensions.append(cls.parse_line(line))
        return dimensions

    @staticmethod
    def parse_line(line: str) -> PackageDimension:
        length_width_height = line.split("x")
        if len(length_width_height) != 3:
            raise ValueError
        return PackageDimension(
            int(length_width_height[0]),
            int(length_width_height[1]),
            int(length_width_height[2]),
        )


class WrappingDimensionFromDatabaseParser:
    @classmethod
    def parse(cls, input_reader: InputPort) -> typing.List[PackageDimension]:
        dimensions = list()
        for line in input_reader:
            if line is not None:
                dimensions.append(cls.parse_line(line))
            else:
                break
        return dimensions

    @staticmethod
    def parse_line(line: str) -> PackageDimension:
        if len(line) != 3:
            raise ValueError
        return PackageDimension(
            int(line[0]),
            int(line[1]),
            int(line[2]),
        )


def main() -> None:

    # dimensions = WrappingDimensionFromFileParser.parse(
    # input_reader=FileReaderAdapter("my_input.txt")
    dimensions = WrappingDimensionFromDatabaseParser.parse(
        input_reader=DBReaderAdapter("packages.sqlite3")
    )
    all_packing_paper_area = sum(
        dimension.calculate_wrapping_area_with_slack() for dimension in dimensions
    )
    pprint(f"The packing paper should be {all_packing_paper_area} sqft-retard-units")
    all_ribbon_length = sum(
        dimension.calculate_ribbon_length() for dimension in dimensions
    )
    pprint(f"The ribbon should be {all_ribbon_length} feet-retard-units")


if __name__ == "__main__":
    main()
