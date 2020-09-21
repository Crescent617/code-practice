class OpNode:
    name2op = {'add': operator.add, "mult": operator.mul}

    def __init__(self, lhs, rhs, opname) -> None:
        self.lhs = lhs
        self.rhs = rhs
        self.op = OpNode.name2op[opname]

    def __call__(self):
        return self.op(self.lhs(), self.rhs())


class ConstantNode:
    def __init__(self, val) -> None:
        self.val = val

    def __call__(self):
        return self.val

    def __repr__(self) -> str:
        return str(self.val)

def isConstant(s):
    try:
        int(s)
        return True
    except Exception as e:
        return False


def parseLisp(expr: str):
    ENV_STACK = []
    it = iter(expr.replace('(', ' ( ').replace(')', ' ) ').split())

    def parse():
        expr = next(it)
        if expr in OpNode.name2op:
            return parseOp(expr)
        elif expr == 'let':
            return parseLet(expr)
        else:
            # print(expr)
            return parseVar(expr)

    def parseOp(opname):
        args = []
        for e in it:
            temp_node = None
            if e == '(':
                temp_node = parse()
            elif e == ')':
                break
            else:
                print(opname, e, ENV_STACK)
                temp_node = parseVar(e)
            args.append(temp_node)
        return OpNode(*args, opname)

    def getVal(e):
        for env in reversed(ENV_STACK):
            if e in env:
                return env[e]
        raise Exception(f"{e} is not defined")

    def parseVar(expr: str):
        if isConstant(expr):
            node = ConstantNode(int(expr))
        else:
            node = getVal(expr)
        return node

    def parseLet(_):
        env = {}
        ENV_STACK.append(env)
        var = temp_node = last = None
        for e in it:
            if e == '(':
                temp_node = parse()
                last = None
            elif e == ')':
                break
            elif var:
                temp_node = parseVar(e)
            else:
                last = e

            if var is not None:
                env[var] = temp_node
                var = None
            elif e not in '()':
                var = e

        res = temp_node if not last else parseVar(last)
        ENV_STACK.pop()
        return res

    next(it)
    return parse()


class Solution:
    def evaluate(self, expression: str) -> int:
        node = parseLisp(expression)
        return node()
