#!/usr/bin/env PYTHONPATH=.. python
from itertools import  cycle
from lib import *

def Input(filename):
  with open(filename, 'r') as fp:
    return [l.strip('\n') for l in fp.readlines()]

nums = [int(x) for x in Input('input.txt')]
#nums = [+7, +7, -2, -7, -4]
seen = set()
freq = 0
for n in cycle(nums):
  freq += n
  if freq in seen:
    print(freq)
    break
  seen.add(freq)

#print(sum([int(x) for x in Input('input.txt')]))

