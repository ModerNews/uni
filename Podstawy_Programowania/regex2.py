#!/usr/bin/env python
# -*- coding: utf-8 -*-
import re
import sys
import getopt

# ===========
# Alternatywne rozwiązania w komentarzach
# Usunięte z powodu ograniczenia wbudowanego silnika regex
# ===========

optlist, args = getopt.getopt(sys.argv[1:], 'i:abcde', ["input="])

text = ""
for opt, arg in optlist:
    if opt in ('-i', '--input'):
        with open(arg, 'r') as f:
            text = f.read()

if text == "":
    text = sys.stdin.read()

# 1. All incorrect phone numbers
# Alternative solution using only regex: ^(?!.*(?<=\s|^)(\+\d{2}|)([\s-]?\d){9}(?=\s|$)).*$
regex = re.compile(
    r'(?<=\s)(\+\d{2}|)([\s-]?\d){9}(?=\s)|(?<=^)(\+\d{2}|)([\s-]?\d){9}(?=\s)|(?<=^)(\+1)([\s-]?\d){10}(?=\s|$)|(?<=\s)(\+1)([\s-]?\d){10}(?=\s|$)',
    re.MULTILINE)
correct = [tmp.group() for tmp in regex.finditer(text)]


def incorrect_phone_numbers():
    incorrect = []
    i, j = 0, 0
    lines = text.split('\n')
# print(correct)
    while j < len(correct):
        if correct[j] in lines[i]:
            i += 1
            j += 1
        else:
            incorrect.append(lines[i])
            i += 1
    print(incorrect)

# 2. All phone numbers without country code
# Alternative solution using only regex: (?<!\+\d{2}[\s-]?)\b(?:\d[\s-]?){9}(?![\s-]?\d)


def no_country_code():
    no_country_code = []
    for phone in correct:
        # Special case for US phone numbers
        if re.match(r'^(\+\d{2})|^(\+1)', phone) is None:
            no_country_code += [phone]
    print(no_country_code)

# 3. All polish phone numbers
# Alternative solution using only regex: (?<=\s|^)((\+\d{2}|)([\s-]?\d){9})(?=\s|$)


def polish_phone_numbers():
    polish = []
    for phone in correct:
        if re.match(r'^(\+48|\d)', phone) is not None:
            polish += [phone]
    print(polish)

# 4. All non-polish phone numbers
# Alternative solution using only regex: ^(((\+(?!48)\d{2}[- ]?))(\d[\s-]?){9})$


def non_polish_phone_numbers():
    non_polish = []
    for phone in correct:
        if re.match(r'^(\+48|\d)', phone) is None:
            non_polish += [phone]
    print(non_polish)

# 5. Re-structure all phone numbers to unified format


def restructure_phone_numbers():
    phone_numbers = [phone.replace(
        '-', '').replace(' ', '') for phone in correct]
    new_phone_numbers = []
    for phone in phone_numbers:
        new_phone = ""
        # Special case for US phone numbers
        if (tmp := re.match(r'^(\+1)', phone)) is not None:
            new_phone += tmp.group() + " " + \
                phone[2:5] + " " + phone[5:8] + " " + phone[8:]
            new_phone_numbers.append("<" + new_phone + ">")
            break

        elif (tmp := re.match(r'^(\+\d{2})', phone)) is not None:
            new_phone += tmp.group() + " "
            phone = phone[3:]

        if (tmp := re.match(r'^[34]0|5[0137]|6[069]|7[02389]|8[08]|90', phone)) is not None:
            new_phone += " ".join([phone[i:i+3]
                                  for i in range(0, len(phone), 3)])
        else:
            new_phone += phone[:2] + " " + phone[2:5] + " " + phone[5:]
        new_phone_numbers.append("<" + new_phone + ">")
    print(new_phone_numbers)


for opt, arg in optlist:
    if opt in ('-a'):
        incorrect_phone_numbers()
    elif opt in ('-b'):
        no_country_code()
    elif opt in ('-c'):
        polish_phone_numbers()
    elif opt in ('-d'):
        non_polish_phone_numbers()
    elif opt in ('-e'):
        restructure_phone_numbers()
