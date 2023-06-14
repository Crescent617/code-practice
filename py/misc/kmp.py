from typing import Iterable


def match(text: Iterable, pat: Iterable, is_none_true=True):
    if not pat:
        return is_none_true
    fail = create_fail_array(pat)
    end = len(pat)
    cur = -1
    for ch in text:
        while cur >= 0 and ch != pat[cur + 1]:
            cur = fail[cur]
        if ch == pat[cur + 1]:
            cur += 1
        if cur + 1 == end:
            return True
    return False


def create_fail_array(pat: Iterable):
    """
    >>> create_fail_array('abcabc')
    [-1, -1, -1, 0, 1, 2]
    >>> create_fail_array('abcacb')
    [-1, -1, -1, 0, -1, -1]
    """
    m = len(pat)
    fail = [-1] * m

    for i in range(1, m):
        k = fail[i - 1]
        while k >= 0 and pat[i] != pat[k + 1]:
            k = fail[k - 1]
        if pat[i] == pat[k + 1]:
            k += 1
        fail[i] = k
    return fail

