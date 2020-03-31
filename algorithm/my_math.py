
def exgcd(a, b, x=0, y=0):
    if b == 0:
        return a, 1, 0
    d, x1, y1 = exgcd(b, a%b)
    x, y = y1, x1 - a//b*y1
    return d, x, y

