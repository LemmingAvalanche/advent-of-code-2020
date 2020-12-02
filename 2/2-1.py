#!/usr/bin/env python3

from itertools import compress

def parse(filename):
    with open(filename) as f:
        # Ok, this is not great
        print(sum(bool(y) for y in [check_psw(x) for x in f]))

def check_psw(line):
    split_on_ws = line.split(" ");
    range = parse_range(split_on_ws[0])
    at_least = range[0]
    at_most = range[1]
    ch = parse_char(split_on_ws[1])
    psw = split_on_ws[2]
    count = psw.count(ch)
    if count >= at_least and count <= at_most:
        return True
    else:
        return False

def parse_range(range):
    r = range.split("-")
    return (int(r[0]), int(r[1]))

def parse_char(c):
    return c[0]

s = parse("input.txt")
