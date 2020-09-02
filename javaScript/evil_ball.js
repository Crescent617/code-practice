let canvas = document.getElementById("my_canvas")
canvas.setAttribute("width", window.innerWidth)
canvas.setAttribute("height", window.innerHeight)
canvas.style.background = "black"

let ctx = canvas.getContext("2d")
let fps = 20

function randomColor() {
    const num = 255
    return `rgb(${randomInt(num)}, ${randomInt(num)}, ${randomInt(num)})`
}

function randomInt(hi, lo = 0) {
    return Math.round(lo + (hi - lo) * Math.random())
}

function clearCanvas() {
    ctx.fillStyle = "#000000"
    ctx.fillRect(0, 0, canvas.width, canvas.height)
}

function* range(start, end) {
    if (end == undefined) {
        end = start
        start = 0
    }
    yield* _range(start, end)
}

function* _range(start, end) {
    yield start
    if (start == end) return
    yield* range(start + 1, end)
}

class Shape {
    constructor(x, y, vx = 0, vy = 0) {
        this.x = x
        this.y = y
        this.velX = vx
        this.velY = vy
        this._exists = true
    }

    get exists() {
        return this._exists
    }

    update() {
        throw "Base class cannot draw"
    }

    draw() {
        throw "Base class cannot draw"
    }
}

class Ball extends Shape {
    constructor(x, y, vx, vy) {
        console.assert(arguments.length >= 2, "x, y must be set!")
        super(...arguments)
        this.color = randomColor()
        this.radius = randomInt(10, 30)
    }

    draw() {
        ctx.fillStyle = this.color
        ctx.beginPath()
        ctx.arc(this.x, this.y, this.radius, 0, 2 * Math.PI, false)
        ctx.closePath()
        ctx.fill()
    }

    update() {
        let dx = this.velX / fps,
            dy = this.velY / fps

        const temp_x = this.x + dx
        let temp_xs = [temp_x + this.radius, temp_x - this.radius]

        const temp_y = this.y + dy
        let temp_ys = [temp_y + this.radius, temp_y - this.radius]

        let max_ = canvas.width
        let isOut = function (item) {
            if (item < 0) return 1
            if (item > max_) return -1
            return 0
        }
        let is_x_out = isOut(temp_xs[0]) | isOut(temp_xs[1])

        max_ = canvas.height
        let is_y_out = isOut(temp_ys[0]) | isOut(temp_ys[1])

        this.changeDirection(is_x_out, is_y_out)
        this._update()
    }

    changeDirection(dirX, dirY) {
        this.velX = Ball._change(this.velX, dirX)
        this.velY = Ball._change(this.velY, dirY)
    }

    static _change(val, d) {
        let abs_val = Math.abs(val)
        return d == 0 ? val : abs_val * d
    }

    _update() {
        let dx = this.velX / fps,
            dy = this.velY / fps
        this.x += dx
        this.y += dy
    }
}

function createBall() {
    let width = window.innerWidth
    let height = window.innerHeight
    innerHeight
    return new Ball(
        randomInt(width),
        randomInt(height),
        randomInt(100),
        randomInt(100)
    )
}

/** @type {Ball[]} */
let balls = []
for (const _ of range(30)) {
    balls.push(createBall())
}

function main() {
    clearCanvas()
    balls.forEach((ball) => {
        if (ball.exists) {
            ball.draw()
            ball.update()
        }
    })
}


main()
window.setInterval("main()", 1000 / fps)
