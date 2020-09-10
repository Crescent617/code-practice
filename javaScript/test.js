const { random } = require("lodash")
const { assert } = require("console")
const { PriorityQueue } = require('./my_collection.js')

let pq = new PriorityQueue()

for (let i = 0; i < 20; i++) {
  pq.push(random(100))
  console.log(pq)
}

let pre = pq.pop()
while (pq.length) {
  let cur = pq.pop()
  assert(pre <= cur)
  pre = cur
}
