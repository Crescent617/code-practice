from collections import Counter
from functools import lru_cache


def nCr_factory(limit, mod):
    """ return a combinatorial number operator with upper bound """

    fact = [1] * (10**6 + 1)
    for i in range(1, len(fact)):
        fact[i] = i * fact[i-1] % mod

    ifact = [1] * (10**6 + 1)
    ifact[-1] = pow(fact[-1], mod-2, mod)

    for i in range(1, len(fact)-1)[::-1]:
        ifact[i] = ifact[i+1] * (i+1) % mod

    def calc(n, i):
        """ combinatorial number with modulo """
        if i < 0 or i > n:
            return 0
        return fact[n] * ifact[i] % mod * ifact[n-i] % mod
    return calc


def twos_in_factorial(n):
    """ return the number of factor 2 in target factorial """
    return n - bin(n).count('1')


def factorize_factory(limit):
    """ return a fast factorize function with upper bound """
    factor = list(range(limit+1))
    for p in range(2, limit+1):
        if p*p > limit:
            break
        if factor[p] == p:
            for i in range(p*p, limit+1, p):
                factor[i] = p

    def calc(x) -> list:
        """ fast factorize """
        res = []
        while x != 1:
            cnt = 0
            p = factor[x]
            while factor[x] == p:
                x //= p
                cnt += 1
            res.append((p, cnt))
        return res
    return calc


def factorize1(x):
    assert x > 0 and isinstance(x, int)
    res = Counter()
    _x, k = x, 2
    while k <= x and k <= round(_x**0.5) + 1:
        if x % k == 0:
            x //= k
            res[k] += 1
        else:
            k += 1
    return res if res else Counter({_x: 1})


def exgcd(a, b, x=0, y=0):
    """ solve equation ax + by = gcd(a, b) """
    if b == 0:
        return a, 1, 0
    d, x1, y1 = exgcd(b, a % b)
    x, y = y1, x1 - a//b*y1
    return d, x, y


def Y(f):
    """ combinator Y = lf. (lu. (u u)) (lx. f (x x)) """
    return (lambda u: u(u))(lambda x: f(lambda v: x(x)(v)))


def next_combo(comb):
    """ return the next bit combination """
    while True:
        yield comb
        # extract the last '1'
        tail = comb & -comb
        y = comb + tail
        comb = ((comb & ~y)//tail >> 1) | y
