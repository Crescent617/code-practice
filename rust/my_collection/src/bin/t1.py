from typing import List
from itertools import islice

class Solution:
    def removeDuplicates(self, S: str) -> str:
        stk = ['$']
        for ch in S:
            if ch == stk[-1]:
                stk.pop()
            else:
                stk.append(ch)
        return ''.join(islice(stk, 1, len(stk)))