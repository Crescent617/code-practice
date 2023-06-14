def fib(n):
    result = [0]
    if n > 0:
        result.append(1)
        for i in range(2,n+1):
            result.append(result[i-1] + result[i-2])
    return result


print(fib(10))
