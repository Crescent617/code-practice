from fractions import Fraction
from functools import lru_cache
from itertools import permutations
from operator import add, mul, sub
from typing import List


class Solution:

    def judgePoint24(self, nums: List[int]) -> bool:
        @lru_cache(None)
        def dfs(nums, expr):
            if len(nums) == 1:
                return nums[0] == 24, expr

            (a, b), nums = nums[:2], nums[2:]
            ops = (mul, add, sub) + (b != 0)*(Fraction,)
            symbols = 'x+-/'

            for idx, op in enumerate(ops):
                for i in range(len(nums)+1):
                    ans = op(a, b)
                    nxt_expr = f'{expr} -> {a} {symbols[idx]} {b} = {ans}'
                    done, path = dfs(nums[:i] + (ans,) + nums[i:], nxt_expr)
                    if done:
                        return done, path
            return False, ''

        for permu in permutations(nums, 4):
            done, path = dfs(permu, f'{permu}')
            if done:
                return path
        return 'No solution!'
