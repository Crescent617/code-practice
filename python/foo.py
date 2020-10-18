class Solution:
    def arrayPairSum(self, nums: List[int]) -> int:
        from collections import defaultdict

        def radixSort(nums, base):
            if not base:
                return nums
            tmp = defaultdict(list)
            for num in nums:
                tmp[num//base].append(num)
            res = []
            for x in range(10):
                res.extend(radixSort(tmp[x], base//10))
            return res



