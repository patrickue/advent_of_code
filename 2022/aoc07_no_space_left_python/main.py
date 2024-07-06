import re
from pprint import pprint


def parse_input(file, name) -> (int, int):
    size = 0
    small_folder_size = 0
    while True:
        line = file.readline()
        if line == "":
            return small_folder_size, size
        extracted_filesize = re.search(r"(\d+) [a-zA-Z.]+", line)
        if extracted_filesize is not None:
            size += int(extracted_filesize.group(1))
        elif line.startswith("$ cd .."):
            if size >= 8_729_145:
                pprint(f"Closed folder {name}:{size}")
            return small_folder_size, size
        elif line.startswith("$ cd"):
            (subsmall_folder_size, subfolder_size) = parse_input(file, name+'/'+line.strip()[5:])
            size += subfolder_size
            small_folder_size += subsmall_folder_size
            if subfolder_size <= 100_000:
                small_folder_size += subfolder_size

        elif line.startswith("$ ls"):
            pass
        elif line.startswith("dir"):
            pass


file = open("input.txt", "r")

pprint(f"Total {parse_input(file,'/')}")

