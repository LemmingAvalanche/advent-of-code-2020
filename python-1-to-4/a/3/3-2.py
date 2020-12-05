#!/usr/bin/env python3

def read_file_and_work(filename):
    with open(filename) as f:
        lines = []
        for l in f:
            lines.append(l)
        print(work(lines, 1, 1) * work(lines, 3, 1) * work(lines, 5, 1) * work(lines, 7, 1) * work(lines, 1, 2))

def work(lst, right, down):
    i = down
    total = 0
    progress = 0
    while i < len(lst):
        line = lst[i]
        length = len(line)
        progress = (progress + right) % (length - 1)
        if line[progress] == '#':
            total += 1
        i += down
    print("total: ", total)
    return total

read_file_and_work("input.txt")
