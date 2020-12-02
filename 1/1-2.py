#!/usr/bin/env python3

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
            result = bin_duck(dsints, first, second, 2020)
            if result == 0:
                return

# Shameless: https://careerkarma.com/blog/binary-search-python/
def bin_duck(lst, first, second, goal):
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

# With time(1) A little slower than horse and a lot slower than duck!
def manatee():
    goal = 2020
    ints = read_integers("input.txt")
    # Still: Really?
    dsints = list(OrderedDict.fromkeys(sorted(ints)))
    for first in dsints:
        for second in dsints:
            # prune
            index = bin_manatee(dsints, [first], second)
            if (dsints[index] == goal):
                dsints = dsints[start, index-1]
    for first in dsints:
        for second in dsints:
            # final search
            result = bin_manatee(dsints, [first, second], 2020)
            if result != -1:
                print(first * second * dsints[result])
                return

# Shameless: https://careerkarma.com/blog/binary-search-python/
# Returns index
def bin_manatee(lst, list_for_sum, goal):
    low = 0
    high = len(lst) - 1
    while low <= high:
        middle = low + (high - low) // 2
        current = lst[middle]
        result = sum(list_for_sum) + current
        if result == goal:
            return middle
        elif result > goal:
            high = middle - 1
        else:
            low = middle + 1
    return -1

manatee()
