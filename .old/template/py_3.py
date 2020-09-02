import math
import sys
from bisect import bisect_left, bisect_right
from collections import Counter, defaultdict, deque
from functools import cmp_to_key, lru_cache, reduce
from heapq import heapify, heappop, heappush
from itertools import chain, combinations, permutations, product
from typing import List

# import numpy as np

sys.setrecursionlimit(10**5 + 1)
input = sys.stdin.readline
write = sys.stdout.write


def read_n_int() -> list:
    return [int(s) for s in input().split()]


def list2d(row, col, init=0) -> List[list]:
    return [[init] * col for _ in range(row)]


# MOD = 10**9 + 7
MOD = 998244353


if __name__ == "__main__":
    ...