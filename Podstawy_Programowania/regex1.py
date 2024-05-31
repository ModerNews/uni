#!/usr/bin/env python
# -*- coding: utf-8 -*-
import re

with open('tekst.utf8.txt', 'r') as f:
    text = f.read()

# 1. words starting with [aAoO]
regex = re.compile(r'\b[aoAO]\w+')
ao_match = regex.findall(text)
print(ao_match)

# 2. words ending with PL characters
regex = re.compile(r'\b\w+[ąęóżźćń]\b')
pl_match = regex.findall(text)
print(pl_match)

# 3. words ending the sentence
regex = re.compile(r'\w+(?=[.!?])')
sentence_end = regex.findall(text)
print(sentence_end)

# 4. All words prepped with two spaces
regex = re.compile(r'\b(?<=  )\w+')
two_spaces = regex.findall(text)
print(two_spaces)

# 5. All words with exectly 3 vowels
regex = re.compile(r'\b\w*[aeiouyAEIOUY]\w*[aeiouyAEIOUY]\w*[aeiouyAEIOUY]\w*\b')
three_vowels = regex.findall(text)
print(three_vowels)
