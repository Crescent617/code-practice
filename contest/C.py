import sys
from collections import Counter
from heapq import heapify, heappop, heappush

sys.setrecursionlimit(10**6)
# input = sys.stdin.buffer.readline

MAX_INT = 2**62-1
MOD = 10**9 + 7


if __name__ == "__main__":
    test_cases = int(input())
    for _ in range(test_cases):
        grid = []
        for _ in range(9):
            grid.append([int(s) for s in input().strip()])

        to_change = [(0, 0), (3, 1), (6, 2), (1, 3), (4, 4),
                     (7, 5), (2, 6), (5, 7), (8, 8)]
        # print(grid)

        for i, j in to_change:
            grid[i][j] = grid[i-1][j]
        for row in grid:
            print(''.join(str(r) for r in row))
            
