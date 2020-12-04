#!/usr/bin/env python3

import re;

from pathlib import Path

def parse_records(i):
    return i.split("\n\n")

def parse_to_passwords(record):
    fields = re.split(r'[ \n]', record)
    return list(map(parse_pair, list(filter(None, fields))))

def parse_pair(pair):
    p = pair.split(":")
    return (p[0], p[1])

def is_valid_key_pair(f):
    key = f[0]
    value = f[1]
    if key == "byr":
        i = int(value)
        return i >= 1920 and i <= 2002
    elif key == "iyr":
        i = int(value)
        return i >= 2010 and i <= 2020
    elif key == "eyr":
        i = int(value)
        return i >= 2020 and i <= 2030
    elif key == "hgt":
        if value.endswith("cm"):
            i = int(value[0: len(value)-2])
            return i >= 150 and i <= 193
        elif value.endswith("in"):
            i = int(value[0: len(value)-2])
            return i >= 59 and i <= 76
        else:
            return False;
    elif key == "hcl":
        return re.match("#[a-f0-9]{6}", value)
    elif key == "ecl":
        return value == "amb" or value == "blu" or value == "brn" \
            or value == "gry" or value == "grn" or value == "hzl" \
            or value == "oth"
    elif key == "pid":
        return re.match("[0-9]{9}", value)
    else:
        return False

def count_mandatory(record):
    m = dict(record)
    l = m.items()
    return len(list(filter(is_valid_key_pair, l)))

def count_valid_passports(passports):
    return len(list(filter(lambda x: count_mandatory(x) == 7, passports))) - 1

input = Path('input.txt').read_text()
records = parse_records(input)
passports = list(map(parse_to_passwords, records))
print(count_valid_passports(passports))
