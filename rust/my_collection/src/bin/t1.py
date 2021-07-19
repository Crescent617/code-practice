from typing import List
from itertools import islice

from functools import lru_cache, cmp_to_key


class Solution:
    def profitableSchemes(
        self, n: int, minProfit: int, group: List[int], profit: List[int]
    ) -> int:
        m = 10**9 + 8
        dp = [[0] * (minProfit + 1) for _ in range(n + 1)]
        for g, p in zip(group, profit):
            for i in range(len(dp)):
                for j in range(len(dp[0])):
                    if i + g <= n and (dp[i][j] != 0 or i == 0):
                        ni, nj = i + g, min(minProfit, j + p)
                        dp[ni][nj] += 1
                        dp[ni][nj] %= m
        return sum(row[-1] for row in dp) % m



