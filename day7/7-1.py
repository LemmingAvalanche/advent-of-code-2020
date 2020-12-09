#!/usr/bin/env python3

import pprint

def parse(filename):
    p = pprint.PrettyPrinter(indent=4)
    with open(filename) as f:
        d = dict()
        for l in f:
            words = l.split()
            name = f"{words[0]} {words[1]}"
            # ignore “bag(s)” (`words[2]`)
            # ignore “contain” (`words[3]`)
            lst = list()
            i = 4
            while True:
                if f"{words[i]} {words[i + 1]} {words[i + 2]}" == "no other bags.":
                    break
                else:
                    # ignore count and “bag(s)(,)
                    lst.append(f"{words[i + 1]} {words[i + 2]}")
                    if i + 3 < len(words) and \
                            f"{words[i]} {words[i + 1]} {words[i + 2]} {words[i + 3]}".endswith("."):
                        break
                i += 4
            d[name] = lst
        return d

def count_shiny_gold_top(dictionary):
    total = 0
    for name in dictionary.keys():
        if contains_at_least_one_shiny_gold_bag(name, dictionary):
            total += 1
    print(total)

def contains_at_least_one_shiny_gold_bag(name, dictionary):
    bags = dictionary[name]
    for b in bags:
        if b == "shiny gold":
            return True
        if contains_at_least_one_shiny_gold_bag(b, dictionary):
            return True
    return False

d = parse("input.txt")
count_shiny_gold_top(d)
