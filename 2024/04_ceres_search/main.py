from typing import List
import unittest


def find_xmas(matrix: List[str]) -> int:

    amount = count_xmas_in_lines(matrix)
    amount += count_xmas_in_lines(transpose_matrix(matrix))
    amount += count_xmas_in_lines(tilt_matrix(matrix, True))
    amount += count_xmas_in_lines(tilt_matrix(matrix, False))

    return amount


def count_xmas_in_lines(matrix: List[str]) -> int:

    xmas_count = lambda x: x.count("XMAS") + x.count("SAMX")

    return sum([xmas_count(row) for row in matrix])


def tilt_matrix(string_matrix: List[str], clockwise: bool) -> List[str]:
    """
    Tilts a matrix by 45° clockwise or counterclockwise

    :param string_matrix: assumes to have all str in this list equal length
    :return: matrix of
    """
    y_len = len(string_matrix)

    head = 0
    tail = y_len - 1

    shifted_matrix = []
    for y in range(y_len):
        if clockwise:
            shifted_matrix.append(" " * head + string_matrix[y] + " " * tail)
        else:
            shifted_matrix.append(" " * tail + string_matrix[y] + " " * head)
        head += 1
        tail -= 1
    return transpose_matrix(shifted_matrix)


def transpose_matrix(matrix: List[str]) -> List[str]:
    x_len = len(matrix[0])
    y_len = len(matrix)

    transposed = [[" " for _ in range(y_len)] for x in range(0, x_len)]
    for y in range(y_len):
        for x in range(x_len):
            transposed[x][y] = matrix[y][x]
    return ["".join(transposed[y]) for y in range(len(transposed))]


### Part 2


def find_crossing_mas(matrix: List[str]) -> int:
    x_len = len(matrix)
    y_len = len(matrix[0])

    count = 0
    for x in range(x_len):
        for y in range(y_len):
            if matrix[y][x] == "A" and 0 < x < x_len - 1 and 0 < y < y_len - 1:
                if matrix[y + 1][x + 1] + matrix[y][x] + matrix[y - 1][x - 1] in [
                    "SAM",
                    "MAS",
                ] and matrix[y - 1][x + 1] + matrix[y][x] + matrix[y + 1][x - 1] in [
                    "SAM",
                    "MAS",
                ]:
                    count += 1
    return count


class TestMatrix(unittest.TestCase):

    TESTS_MATRIX = ["1234", "5678", "9ABC"]

    def test_rotate_cw(self):
        result = tilt_matrix(self.TESTS_MATRIX, clockwise=True)
        print(result)
        self.assertEqual(["1  ", "25 ", "369", "47A", " 8B", "  C"], result)
        # should be self.assertEqual, but who cares

    def test_rotate_ccw(self):
        result = tilt_matrix(self.TESTS_MATRIX, clockwise=False)
        print(result)
        self.assertEqual(["  9", " 5A", "16B", "27C", "38 ", "4  "], result)
        # should be self.assertEqual, but who cares

    def test_transpose(self):
        result = transpose_matrix(self.TESTS_MATRIX)
        print(result)
        self.assertEqual(["159", "26A", "37B", "48C"], result)


if __name__ == "__main__":
    f = open("04_ceres_search/input.txt")
    matrix = list()
    for line in f:
        matrix.append(line.strip())

    # unittest.main()

    print(f"Found 'XMAS' in all directions: {find_xmas(matrix)}")
    print(f"Found 'MAS' in X form: {find_crossing_mas(matrix)}")
