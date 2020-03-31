import dllist
import bstree
import random
import cProfile

def search_list_iter(dat, elem):
    low = 0
    data = list(sorted(dat))
    high = len(data)

    while low < high:
        mid = (high - low) // 2 + low
        mid_val = data[mid]

        if mid_val == elem:
            return elem, mid
        elif mid_val > elem:
            high = mid
        elif mid_val < elem:
            low = mid + 1

    return None, -1

def search_dllist(data, elem):
    dl = dllist.DoubleLinkedList()
    dat = sorted(data)
    for x in dat: 
        dl.push(x)

    low = 0
    high = dl.count()

    while low < high:
        mid = (high - low) // 2 + low
        mid_val = dl.get(mid)

        if mid_val == elem:
            return elem, mid
        elif mid_val > elem:
            high = mid
        elif mid_val < elem:
            low = mid + 1

    return None, -1
    

def search_btree(data, elem):
    """Unlike the other functions this doesn't expect sorted data."""
    tree = bstree.BSTree()
    for i, key in enumerate(data):
        # use value for index
        tree.set(key, i)
  
    index = tree.get(elem)
    return index != None and (elem, index) or (None, -1)

if __name__ == "__main__":
    data = []
    num = 0
    for i in range(2000):
        data.append(num)
        num += random.randint(-10, 15)
    search_btree(data, data[777])
    search_list_iter(data, data[777])
    search_dllist(data, data[777])
   


    
    
    