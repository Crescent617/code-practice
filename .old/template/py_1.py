import math
import sys
from bisect import bisect_left, bisect_right
from collections import Counter, defaultdict, deque
from functools import lru_cache, reduce
from heapq import heapify, heappop, heappush
from itertools import chain, combinations, permutations, product
from typing import List, Dict

# import numpy as np

sys.setrecursionlimit(10**6 + 1)
input = sys.stdin.readline
write = sys.stdout.write


def read_item(cls=int):
    return cls(input())


def read_n_item(cls=int) -> list:
    return [cls(s) for s in input().split()]


def list2d(row, col, init_val=0) -> List[list]:
    return [[init_val] * col for _ in range(row)]


MOD = 10**9 + 7








