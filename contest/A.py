import heapq
import math
import random
import sys
from bisect import bisect_left, bisect_right
from collections import Counter, defaultdict, deque, namedtuple, UserDict
from functools import cmp_to_key, lru_cache, reduce
from itertools import (chain, combinations, combinations_with_replacement,
                       permutations, product)

sys.setrecursionlimit(10**6+1)
write = sys.stdout.write
# input = sys.stdin.buffer.readline

MOD = 10**9 + 7
INT_MAX = (1 << 63) - 1


def twos_in_factorial(n):
    return n - bin(n).count('1')


def parity_of_nCr(n, i):
    """ if odd, return False. """
    f = twos_in_factorial
    return f(n) - f(n-i) - f(i) > 0


if __name__ == "__main__":
    n = int(input())
    nums = list(map(lambda x: int(x) - 1, input().strip()))
    no_one = False
    if 1 not in nums:
        nums = [num // 2 for num in nums]
        no_one = True

    is_odd = False
    for i, num in enumerate(nums):
        if not parity_of_nCr(n-1, i) and num & 1:
            is_odd ^= 1

    if not is_odd:
        print(0)
    else:
        print(1 << no_one)

