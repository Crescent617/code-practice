# import heapq
import math
import random
import sys
# from bisect import bisect_left, bisect_right
from collections import Counter, defaultdict, deque, namedtuple, UserDict
from functools import cmp_to_key, lru_cache, reduce
# from itertools import (chain, combinations, combinations_with_replacement,
#                        permutations, product)

sys.setrecursionlimit(10**5+1)
write = sys.stdout.write
input = sys.stdin.buffer.readline

MOD = 10**9 + 7


if __name__ == "__main__":
    test_case_num = int(input())

    for _ in range(test_case_num):
        n, a, b = map(int, input().split())
        ans = []
        unit = []
        for i in range(a):
            unit.append(chr(ord('a')+i%b))
        ans = unit * (n//a) + unit[:n%a]
        print(''.join(ans))
        

