class Solution:
    def jobScheduling(
        self, startTime: List[int], endTime: List[int], profit: List[int]
    ) -> int:

        n = len(startTime)
        startTime, endTime, profit = zip(sorted(zip(startTime, endTime, profit)))

        pres = [-1] * n
        events: list = [(t, 1, i) for i, t in enumerate(startTime)] + [
            (t, 0, i) for i, t in enumerate(endTime)
        ]
        events.sort()
        pre = -1
        for _, isStart, i in events:
            if not isStart:
                pre = i
            else:
                pres[i] = pre

        dp = [0] * n
        for i in range(n):
            dp[i] = max(dp[i-1], dp[pres[i]] + profit[i])
        return dp[-1]

