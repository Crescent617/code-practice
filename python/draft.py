class Solution:
    def minDistance(self, houses: List[int], k: int) -> int:
        from functools import lru_cache

        n = len(houses)
        houses.sort()
        prefix = [0]
        for h in houses:
            prefix.append(prefix[-1] + h)

        def rangeSum(i, j):
            m = (i + j) // 2
            return sum(abs(houses[k] - houses[m]) for k in range(i, j + 1))

        @lru_cache(None)
        def dfs(i, j, num):
            if num == 1:
                return rangeSum(i, j)
            ans = float('inf')
            for k in range(i, j):
                ans = min(ans, dfs(i, k, num - 1) + dfs(k + 1, j, 1))
            return ans

        return dfs(0, len(houses) - 1, k)
