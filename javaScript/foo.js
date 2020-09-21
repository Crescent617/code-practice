const { run } = require("jest")
const R = require("ramda")

const xyzLens = R.lensProp("x.y.z")
let obj = {}
R.set(xyzLens, 111, obj)
console.log(obj)

const sortByFirstItem = R.sortBy(R.prop(0));
const pairs = [[-1, 1], [-2, 2], [-3, 3]];
sortByFirstItem(pairs); //=> [[-3, 3], [-2, 2], [-1, 1]]

const sortByNameCaseInsensitive = R.sortBy(R.compose(R.toLower, R.prop('name')));
const alice = {
  name: 'ALICE',
  age: 101
};
const bob = {
  name: 'Bob',
  age: -10
};
const clara = {
  name: 'clara',
  age: 314.159
};
const people = [clara, bob, alice];
R.compose(console.log, sortByNameCaseInsensitive)(people); //=> [alice, bob, clara])