function deepClone(src) {
  if (typeof src != "object") return src
  let target = Object.create(src.__proto__)

  Reflect.ownKeys(src).forEach((key) => {
    target[key] = deepClone(src[key])
  })

  return target
}

let _need_export = [deepClone]
for (const item of _need_export) {
  module.exports[item.name] = item
}
