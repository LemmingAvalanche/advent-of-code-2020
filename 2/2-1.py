#!/usr/bin/env python3

def parse(filename):
    with open(filename) as f:
        for x in f:
            parse_line(x)


def parse_line(line):
    split_on_ws = line.split(" ");
    range = parse_range(split_on_ws[0])
    ch = parse_char(split_on_ws[1])
    psw = split_on_ws[2]
    print(range)
    print(ch)
    print(psw)

def parse_range(range):
    r = range.split("-")
    return (r[0], r[1])

def parse_char(c):
    return c[0]

s = parse("input.txt")
