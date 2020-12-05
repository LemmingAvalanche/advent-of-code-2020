#!/usr/bin/env python3

def read_file_and_work(filename):
    with open(filename) as f:
        lines = []
        for l in f:
            lines.append(l)
        work(lines)

def work(lst):
    i = 1
    total = 0
    progress = 0
    while i < len(lst):
        line = lst[i]
        length = len(line)
        progress = (progress + 3) % (length - 1)
        if line[progress] == '#':
            total += 1
        i += 1
    print(total)

read_file_and_work("input.txt")
