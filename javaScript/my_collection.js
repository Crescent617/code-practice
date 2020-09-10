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
    for (const childIdx of childIndices.filter((v) => v < this.#data.length)) {
      this._shift(childIdx, parIdx, () => this.shiftDown(childIdx))
    }
  }
}

let _need_export = [PriorityQueue]
for (const item of _need_export) {
  module.exports[item.name] = item
}

