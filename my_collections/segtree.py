

class SegmentTree:

    def __init__(self, items, interval_info, init_val=0):
        # assert callable(interval_info)

        raw_len, n = len(items), 1
        while n < raw_len:
            n *= 2

        items = [0]*n + items + [0]*(n-raw_len)
        for i in range(n)[::-1]:
            items[i] = interval_info(items[2*i], items[2*i+1])

        self.interval_info = interval_info
        self.items, self.offset = items, n
        self.raw_len = raw_len

    def update(self, i, val):
        assert 0 <= i < self.raw_len
        i = self.offset + i
        self.items[i] = val

        func, items = self.interval_info, self.items
        while i > 0:
            i //= 2
            items[i] = func(items[2*i], items[2*i+1])

    def query(self, left, right, inclusive=True):
        if not inclusive:
            right -= 1

        assert 0 <= left <= right < self.raw_len

        left, right = left + self.offset, right + self.offset
        func, items = self.interval_info, self.items

        # as func has two argument, add left fist
        ans = self.items[left]
        left += 1

        while left <= right:
            if left % 2:
                ans = func(items[left], ans)
                left += 1
            if right % 2 == 0:
                ans = func(items[right], ans)
                right -= 1
            left //= 2
            right //= 2
        return ans

    def print(self):
        i = 1
        while i <= self.offset:
            print(*self.items[i:2*i])
            i *= 2
