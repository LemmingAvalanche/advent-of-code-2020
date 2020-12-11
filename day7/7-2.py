#!/usr/bin/env python3

def parse(filename):
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
                    # ignore “bag(s)(,)
                    lst.append((int(words[i]), f"{words[i + 1]} {words[i + 2]}"))
                    if i + 3 < len(words) and \
                            f"{words[i]} {words[i + 1]} {words[i + 2]} {words[i + 3]}".endswith("."):
                        break
                i += 4
            d[name] = lst
        return d

def count_number_of_bags_within_shiny_gold(dictionary):
    total = 0
    for tup in dictionary["shiny gold"]:
        c = tup[0]
        b = tup[1]
        total += c + c * count(c, b, dictionary)
    print(total)

def count(c, b, dictionary):
    bags = dictionary[b]
    total = 0
    for tup in bags:
        c2 = tup[0]
        b2 = tup[1]
        total += c2 + (c2 * count(c2, b2, dictionary))
    return total

d = parse("input.txt")
count_number_of_bags_within_shiny_gold(d)
