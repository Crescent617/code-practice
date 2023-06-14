const http = require("http")

const hostname = "localhost"
const port = 3000

const server = http.createServer((req, res) => {
    req.pipe(res)
})

server.listen(port, () => {
    console.log(`服务器运行在 http://${hostname}:${port}/`)
})
