import typing
from dataclasses import dataclass


@dataclass(frozen=True)
class Assignment:
    first_elf: range
    second_elf: range

    def check_complete_overlap(self):
        overlap = range(
            max(self.first_elf[0], self.second_elf[0]),
            min(self.first_elf[-1], self.second_elf[-1]) + 1,
        )
        return overlap.start < overlap.stop


def parse_input_to_assignment(input: typing.List[str]) -> typing.List[Assignment]:
    assignments = list()
    for line in input:
        comma_separate_line = line.replace("-", ",")
        range_array = comma_separate_line.split(",")
        first = range(int(range_array[0]), int(range_array[1]) + 1)
        second = range(int(range_array[2]), int(range_array[3]) + 1)
        assignments.append(Assignment(first_elf=first, second_elf=second))
    return assignments


def main():
    input = open("04_camp_cleanup.txt", "r")

    input_list = [line.strip() for line in input]

    assignment_list = parse_input_to_assignment(input_list)

    complete_overlap_list = [x.check_complete_overlap() for x in assignment_list]

    print(
        f"We have a complete overlap on {complete_overlap_list.count(True)} assignments."
    )


if __name__ == "__main__":
    main()
