from collections import defaultdict
from itertools import takewhile
from typing import List


def Trie():
    return defaultdict(Trie)


class Solution:
    def suggestedProducts(
        self, products: List[str], searchWord: str
    ) -> List[List[str]]:
        trie = Trie()

        def search(t, prefix):
            if not t:
                return []
            res = []
            for i, r in enumerate(_search(t)):
                res.append(prefix + r)
                if i == 2:
                    break
            return res

        def _search(t, pre=''):
            for ch in sorted(t):
                if ch == '$':
                    yield pre + ch
                else:
                    yield from _search(t[ch], pre + ch)

        for p in products:
            temp = trie
            for ch in p + '$':
                temp = temp[ch]

        ans = []
        t = trie
        for i, ch in enumerate(searchWord):
            if ch in t:
                ans.append(search(t, searchWord[: i + 1]))
                t = t[ch]
            else:
                break

        for _ in range(len(searchWord) - len(ans)):
            ans.append([])

        return ans
