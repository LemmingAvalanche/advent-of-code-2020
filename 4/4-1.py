#!/usr/bin/env python3

import re;

from pathlib import Path

def parse_records(i):
    return i.split("\n\n")

def parse_to_passwords(record):
    fields = re.split(r'[ \n]', record)
    return dict(map(parse_pair, list(filter(None, fields))))

def parse_pair(pair):
    p = pair.split(":")
    return (p[0], p[1])

def is_mandatory_key(f):
    return f == "byr" or f == "iyr" or f == "eyr" or f == "hgt" or f == "hcl" \
        or f == "ecl" or f == "pid"

def count_mandatory(record):
    return len(list(filter(is_mandatory_key, record.keys())))

def count_valid_passports(passports):
    return len(list(filter(lambda x: count_mandatory(x) == 7, passports)))

input = Path('input.txt').read_text()
records = parse_records(input)
passports = map(parse_to_passwords, records)
print(count_valid_passports(passports))
