#!/usr/bin/env /usr/bin/python

# Thanks to Rudolph: https://codereview.stackexchange.com/a/12449
def read_integers(filename):
    with open(filename) as f:
        return [int(x) for x in f]

read_integers("input.txt")
