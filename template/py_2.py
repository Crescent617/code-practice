import math
import sys
from bisect import bisect_left, bisect_right
from collections import Counter, defaultdict, deque
from functools import lru_cache, reduce
from heapq import heapify, heappop, heappush
from itertools import chain, combinations, permutations, product
from typing import List, Dict

# import numpy as np

# sys.setrecursionlimit(10**5 + 1)
input = sys.stdin.readline
write = sys.stdout.write


def read_int() -> int:
    return int(input())


def read_n_int() -> list:
    return [int(s) for s in input().split()]


def list2d(row, col, init=0) -> List[list]:
    return [[init] * col for _ in range(row)]


MOD = 10**9 + 7
MOD = 163577857

fact = [1] * (10**5 + 1)
for i in range(1, len(fact)):
    fact[i] = i * fact[i-1] % MOD

ifact = [1] * (10**5 + 1)
ifact[-1] = pow(fact[-1], MOD-2, MOD)

for i in range(1, len(fact)-1)[::-1]:
    ifact[i] = ifact[i+1] * (i+1) % MOD


def nCr(n, i):
    """ combinatorial number modulo 10**9 + 7
    >>> nCr(10, 2)
    45
    >>> nCr(1, 0)
    1
    """
    if i < 0 or i > n:
        return 0
    return fact[n] * ifact[i] % MOD * ifact[n-i] % MOD


if __name__ == "__main__":

    T = read_int()
    for _ in range(T):
        N = read_int()
        arr = read_n_int()

        cnt = Counter(arr)
        zeros = pow(2, cnt[0], MOD)
        ans = [0] * (2*N+1)
        one, n_one = cnt[1], cnt[-1]
        for i in range(2*N+1):
            k = i - N
            ans[i] = nCr(one+n_one, one-k) * zeros % MOD
        ans[N] -= 1
        print(*ans, sep=' ')
