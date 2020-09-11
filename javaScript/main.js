class LinkedListNode {
    constructor(key, val) {
        this.key = key
        this.val = val
        this.freq = 0
        this.prev = this.next = null
    }

    static detach(node) {
        let [prev, next] = [node.prev, node.next]
        node.prev = node.next = null
        if (prev) prev.next = next
        if (next) next.prev = prev
    }

    static link(node1, node2) {
        node1.next = node2
        node2.prev = node1
    }
}

function createLinkedList() {
    let node = new LinkedListNode(null)
    node.prev = node.next = node
    return node
}

/**
 * @param {number} capacity
 */
var LFUCache = function (capacity) {
    this.minFreq = 1
    this.capacity = capacity
    this.size = 0
    this.lookUpNode = new Map()
    this.freqToLinkedList = new Map()
}

/**
 * @param {number} key
 * @return {number}
 */
LFUCache.prototype.get = function (key) {
    let node = this.lookUpNode.get(key)
    return node ? this.increase(node) : -1
}

LFUCache.prototype.increase = function (node) {
    if (node.freq > 0) {
        if (node.prev == node.next) {
            this.freqToLinkedList.delete(node.freq)
            if (this.minFreq == node.freq) this.minFreq++
        }
        LinkedListNode.detach(node)
    }
    node.freq++
    this.minFreq = min(this.minFreq, node.freq)
    let tail
    if (this.freqToLinkedList.has(node.freq))
        tail = this.freqToLinkedList.get(node.freq)
    else {
        tail = createLinkedList()
        this.freqToLinkedList.set(node.freq, tail)
    }
    LinkedListNode.link(tail.prev, node)
    LinkedListNode.link(node, tail)
    return node.val
}

/**
 * @param {number} key
 * @param {number} value
 * @return {void}
 */
LFUCache.prototype.put = function (key, value) {
    let node
    if (this.lookUpNode.has(key)) {
        node = this.lookUpNode.get(key)
        node.val = value
    } else {
        if (this.size >= this.capacity) {
            poped = this.pop()
            if (!poped) return
        }
        node = new LinkedListNode(key, value)
        this.lookUpNode.set(key, node)
        this.size++
    }
    this.increase(node)
}

LFUCache.prototype.pop = function () {
    let head = this.freqToLinkedList.get(this.minFreq)
    if (!head) return

    let node = head.next
    if (node.prev == node.next) this.freqToLinkedList.delete(this.minFreq++)
    LinkedListNode.detach(node)
    this.lookUpNode.delete(node.key)
    this.size--
    return node
}

function min(a, b) {
    return a > b ? b : a
}
/**
 * Your LFUCache object will be instantiated and called as such:
 * var obj = new LFUCache(capacity)
 * var param_1 = obj.get(key)
 * obj.put(key,value)
 */
