from sys import stdout

n, q = map(int, input().split())
arr = [chr(ord('A') + i) for i in range(n)]


def is_gt(a, b):
    stdout.write('? ' + a + ' ' + b + '\n')
    stdout.flush()
    return '>' in input()


def merge_sort(arr):
    if len(arr) <= 1:
        return arr
    m = len(arr) // 2
    return merge(merge_sort(arr[:m]), merge_sort(arr[m:]))


def merge(arr1, arr2):
    ans = []
    i = j = 0
    m, n = len(arr1), len(arr2)

    while i < m or j < n:
        a = arr1[i] if i < m else None
        b = arr2[j] if j < n else None
        if None in (a, b):
            ans.append(a or b)
            i += 1
            j += 1
        elif is_gt(a, b):
            ans.append(b)
            j += 1
        else:
            ans.append(a)
            i += 1
    return ans


stdout.write('! ' + ''.join(merge_sort(arr)) + '\n')
stdout.flush()
