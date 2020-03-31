from dllist import DoubleLinkedList

def bubble_sort(numbers):
    while True:
        is_sorted = True
        node = numbers.begin.next
        while node:
            if node.prev.value > node.value:
                node.prev.value, node.value = node.value, node.prev.value
                is_sorted = False
            node = node.next
        if is_sorted:
            break

def merge_sort(numbers):

    if numbers.begin.next is None:
        return numbers

    mid = numbers.count() // 2
    left = DoubleLinkedList()
    right = DoubleLinkedList()

    scanner = numbers.begin
    for i in range(0, mid-1):
        scanner = scanner.next

    left.begin = numbers.begin
    left.end = scanner
    right.end = numbers.end
    right.begin = scanner.next

    scanner.next.prev = None
    scanner.next = None

    left = merge_sort(left)
    right = merge_sort(right)

    return merge(left, right)

def merge(left, right):
    result = DoubleLinkedList()

    if left.begin is None:
        return right
    if right.begin is None:
        return left
    
    while left.begin and right.begin:
        if left.begin.value < right.begin.value:
            result.push(left.begin.value)
            left.begin = left.begin.next
        else:
            result.push(right.begin.value)
            right.begin = right.begin.next        

    return result
'''
def node_at(numbers, i):
    count = 1
    node = numbers.begin
    while node and count < i:
        node = node.next
        count += 1
    return node

def quick_sort(numbers, lo, hi):
    if lo < hi:
        p = quick_sort_partition(numbers, lo, hi)
        quick_sort(numbers, lo, p - 1)
        quick_sort(numbers, p + 1, hi)

def quick_sort_partition(numbers, lo, hi):
    pivot = node_at(numbers, hi)
    index = lo

    for j in range(lo, hi):
        node_j = node_at(numbers, j)
        if node_j.value < pivot.value:
            if index != j:
                node_i = node_at(numbers, index)
                node_i.value, node_j.value = node_j.value, node_i.value
            index += 1

    node_hi = node_at(numbers, hi)
    node_i = node_at(numbers, index)
    node_hi.value, node_i.value = node_i.value, node_hi.value

    return index
'''
def quick_sort(numbers, lo, hi):
    if lo < hi:
        p = quick_sort_partition(numbers, lo, hi)
        quick_sort(numbers, lo, p - 1)
        quick_sort(numbers, p + 1, hi)

def quick_sort_partition(numbers, lo, hi):
    pivot = numbers[hi]
    index = lo

    for j in range(lo, hi):
        if numbers[j] < pivot:
            if index != j:
                numbers[index], numbers[j] = numbers[j], numbers[index]
            index += 1
    numbers[index], numbers[hi] = numbers[hi], numbers[index]
    return index
