from heapq import heappop, heappush
class Solution:
    def minRefuelStops(self, target: int, startFuel: int, stations: List[List[int]]) -> int:
        furthest = startFuel
        cnt = 0
        pq = [0]
        while furthest < target and pq:
            while stations and furthest >= stations[0][0]:
                heappush(pq, -stations.pop(0)[1])
            furthest -= heappop(pq)
        return furthest >= target



