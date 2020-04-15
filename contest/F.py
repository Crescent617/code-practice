import sys
from collections import Counter
from itertools import count
from heapq import heapify, heappop, heappush

sys.setrecursionlimit(10**6)
# input = sys.stdin.buffer.readline

MAX_INT = 2**62-1
MOD = 10**9 + 7


if __name__ == "__main__":
    test_cases = int(input())
    for _ in range(test_cases):
        m, n = int(input())
        direction = {'U': (-1, 0), 'D': (1, 0), 'L': (0, -1), 'R': (0, 1)}

        clr_gird = [input().strip() for _ in range(m)]
        dir_gird = [input().strip() for _ in range(m)]

        components = []
        cycle_len = []
        visited = [[False] * n for _ in range(m)]
        counter = count()

        for i in range(m):
            for j in range(n):
                if visited[i][j] is None:
                    path = {}
                    r, c = i, j
                    while True:
                        dr, dc = direction[clr_gird[r][c]]
                        r, c = r + dr, c + dc
                        if visited[r][c]:
                            ...
                        visited[r][c] = True

                        
