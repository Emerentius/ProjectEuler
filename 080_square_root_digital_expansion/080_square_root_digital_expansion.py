import time
from itertools import islice

def isqrt(n):
    x = n
    y = (x + 1) // 2
    while y < x:
        x = y
        y = (x + n // x) // 2
    return x

def digits(num):
    while num != 0:
        yield num % 10
        num //= 10

def non_squares(num):
    squares = [x*x for x in range(1, isqrt(num)+1)]
    yield from filter(lambda n: n not in squares, range(1, num+1))

start = time.time()

dig_sum = 0
for num in non_squares(100):
    root = isqrt(num * 10**200 )
    dig_sum +=  sum( islice( digits(root), 1, None) )

print(dig_sum)

end = time.time()

print("elapsed time: {:.2}s".format(end-start) )
