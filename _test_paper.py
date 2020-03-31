# -*- coding: utf-8 -*-
import sys
from collections import defaultdict


MAX_INT = 2**62-1
sys.setrecursionlimit(10**6)

input = sys.stdin.readline


N = 2 * (10**5) + 1
MOD = 10**9 + 7

fact = [1] * N
fact_inv = [0] * N

for i in range(1, 2 * (10**5)+1):
    fact[i] = fact[i-1] * i % MOD
    fact_inv[i] = pow(fact[i], MOD-2, MOD)


def solve(g):
    def dfs(pre, cur):
        if (pre, cur) in memo:
            return memo[pre,cur]

        num, ways = 1, 1

        for nxt in g[cur]:
            if nxt == pre:
                continue
            n, w = dfs(cur, nxt)
            num += n
            ways = ways * w % m * fi[n] % m

        ways = ways * f[num-1] % m
        memo[pre, cur] = num, ways
        return num, ways

    m = MOD
    memo = {}
    f, fi = fact, fact_inv

    stack = [(None, 1)]
    n = len(g) - 1

    while stack:
        pre, cur = stack.pop()
        dfs(None, cur)

        for nxt in g[cur]:
            if nxt == pre:
                continue
            stack.append((cur, nxt))

    print(*[memo[None, i][1] for i in range(1, len(g))], sep='\n')


def main():
    n = int(input())
    g = [[] for _ in range(n+1)]

    for _ in range(n-1):
        a, b = map(int, input().split())
        g[a].append(b)
        g[b].append(a)

    solve(g)


if __name__ == "__main__":

    main()
