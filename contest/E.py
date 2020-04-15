import sys
from collections import Counter
from heapq import heapify, heappop, heappush

sys.setrecursionlimit(10**6)
input = sys.stdin.buffer.readline

MAX_INT = 2**62-1
MOD = 10**9 + 7


if __name__ == "__main__":
    test_cases = int(input())
    for _ in range(test_cases):
        n = int(input())
        nums = list(map(int, input().split()))
        dp = [0]
