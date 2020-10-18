const { Console } = require("console")
const R = require("ramda")

let a = R.when(R.lt(R.__, 100), R.always(16))(99)
console.log(a)

let e = document.createElement("a")
e.addEventListener("input", (e) => {
    e.preventDefault()
})

