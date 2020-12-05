#!/usr/bin/env python3

# Copied from 2-1.py and modified

from itertools import compress

def parse(filename):
    with open(filename) as f:
        # Ok, this is not great
        print(sum(bool(y) for y in [check_psw(x) for x in f]))

def check_psw(line):
    split_on_ws = line.split(" ");
    range = parse_range(split_on_ws[0])
    pos_1 = range[0]
    pos_2 = range[1]
    ch = parse_char(split_on_ws[1])
    psw = split_on_ws[2]
    return (psw[pos_1 - 1] == ch) ^ (psw[pos_2 - 1] == ch)

def parse_range(range):
    r = range.split("-")
    return (int(r[0]), int(r[1]))

def parse_char(c):
    return c[0]

s = parse("input.txt")
