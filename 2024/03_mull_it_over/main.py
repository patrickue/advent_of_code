import re

file = open('input.txt')

input_line = ''
for line in file:
    input_line += line.strip()

split_line = input_line.split('do')

do_parts = [part for part in split_line if not part.startswith('n\'t')]
do_line = ''.join(do_parts)

mul_list = list()
mul_match_iter = re.finditer(r'mul\((\d{1,3}),(\d{1,3})\)', do_line)
for match in mul_match_iter:
    mul_list.append((int(match.group(1)), int(match.group(2))))

sum_of_mul = sum([x*y for x, y in mul_list])
print(f"Sum of all mul commands is: {sum_of_mul}")