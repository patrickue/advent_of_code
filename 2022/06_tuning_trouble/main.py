import re


def has_double_characters(word: str):
    idx = 1
    for char in word[1:]:
        if char in word[0:idx]:
            return True
        idx += 1
    return False


tuning = open("input.txt", 'r')

tuning_line = ""

for line in tuning:
    tuning_line += line

previous_chars = tuning_line[0:14]
for idx, character in enumerate(tuning_line):
    if not has_double_characters(previous_chars):
        print("Finished" + str(idx))
    previous_chars = previous_chars[1:14]+character
