#!/usr/bin/env /usr/bin/python3

# Copied over from 1-1.py
# ---

# This must demand a greedy algorithm, maybe dynamic programming...
# but who can supply it?

from collections import OrderedDict
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

# Experimental, try to use binary search in the inner loop.
# Better complexity than O(n↑3)?  I dunno.  It’s just twice as fast
# as `horse()` in my “benchmarks” (time(1)).  But I don’t know how
# to benchmark so who knows.
def duck():
    goal = 2020
    ints = read_integers("input.txt")
    # Really?
    dsints = list(OrderedDict.fromkeys(sorted(ints)))
    for first in dsints:
        for second in dsints:
            result = bin(dsints, first, second, 2020)
            if result == 0:
                return

# Shameless: https://careerkarma.com/blog/binary-search-python/
def bin(lst, first, second, goal):
    low = 0
    high = len(lst) - 1
    while low <= high:
        middle = low + (high - low) // 2
        current = lst[middle]
        result = first + second + current
        if result == goal:
            print(first * second * current)
            return 0
        elif result > goal:
            high = middle - 1
        else:
            low = middle + 1
    return -1

duck()
