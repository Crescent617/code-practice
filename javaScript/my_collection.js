class PriorityQueue {
    #data = []

    constructor(func) {
        this._compare = typeof func == "function" ? func : (a, b) => a > b
        Object.freeze(this)
    }

    get length() {
        return this.#data.length
    }

    [Symbol.iterator]() {
        return this.#data[Symbol.iterator]()
    }

    push(item) {
        this.#data.push(item)
        this.shiftUp(this.#data.length - 1)
    }

    top() {
        return this.#data[0]
    }

    pop() {
        let val = this.#data[0]
        this.#data[0] = this.#data[this.#data.length - 1]
        this.#data.pop()
        this.shiftDown(0)
        return val
    }

    shiftUp(childIdx) {
        if (childIdx <= 0) return
        let parIdx = Math.floor((childIdx - 1) / 2)
        this._shift(childIdx, parIdx, () => this.shiftUp(parIdx))
    }

    _shift(childIdx, parIdx, callback) {
        let child = this.#data[childIdx],
            par = this.#data[parIdx]
        if (this._compare(par, child)) {
            ;[this.#data[childIdx], this.#data[parIdx]] = [par, child]
            callback()
        }
    }

    shiftDown(parIdx) {
        if (parIdx >= this.length) return
        let childIndices = [parIdx * 2 + 1, parIdx * 2 + 2]
        for (const childIdx of childIndices.filter(
            (v) => v < this.#data.length
        )) {
            this._shift(childIdx, parIdx, () => this.shiftDown(childIdx))
        }
    }
}

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

class LFUCache {
    /**
     * @param {number} capacity
     */
    constructor(capacity) {
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
    get(key) {
        let node = this.lookUpNode.get(key)
        return node ? this.increase(node) : -1
    }

    increase(node) {
        if (node.freq > 0) {
            if (node.prev == node.next) {
                this.freqToLinkedList.delete(node.freq)
                if (this.minFreq == node.freq) this.minFreq++
            }
            LinkedListNode.detach(node)
        }
        node.freq++
        let f = node.freq
        this.minFreq = Math.min(this.minFreq, f)
        let tail
        if (this.freqToLinkedList.has(f)) tail = this.freqToLinkedList.get(f)
        else {
            tail = createLinkedList()
            this.freqToLinkedList.set(f, tail)
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
    put(key, value) {
        let node
        if (this.lookUpNode.has(key)) {
            node = this.lookUpNode.get(key)
            node.val = value
        } else {
            if (this.size >= this.capacity) {
                let poped = this.pop()
                if (!poped) return
            }
            node = new LinkedListNode(key, value)
            this.lookUpNode.set(key, node)
            this.size++
        }
        this.increase(node)
    }

    pop() {
        let head = this.freqToLinkedList.get(this.minFreq)
        if (!head) return

        let node = head.next
        if (node.prev == node.next) this.freqToLinkedList.delete(this.minFreq++)
        LinkedListNode.detach(node)
        this.lookUpNode.delete(node.key)
        this.size--
        return node
    }
}

/**
 * Your LFUCache object will be instantiated and called as such:
 * var obj = new LFUCache(capacity)
 * var param_1 = obj.get(key)
 * obj.put(key,value)
 */

let _need_export = [PriorityQueue, LFUCache]
for (const item of _need_export) {
    module.exports[item.name] = item
}
