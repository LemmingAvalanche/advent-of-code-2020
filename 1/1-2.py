#!/usr/bin/env /usr/bin/python3

# Copied over from 1-1.py
# ---
# Thanks to Rudolph: https://codereview.stackexchange.com/a/12449
def read_integers(filename):
    with open(filename) as f:
        return [int(x) for x in f]

def horse():
    goal = 2020
    ints = read_integers("input.txt")
    for first in ints:
        for second in ints:
            for third in ints:
                result = first + second + third
                if result == 2020:
                    print(first*second*third)
                    return

horse()
