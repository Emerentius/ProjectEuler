import math
from typing import Iterator, Tuple
import time

def cont_frac_sqrt(num: int) -> Iterator[int]:
    m, d = 0, 1
    a_0 = int( math.sqrt(num) )
    a = a_0

    while (True):
        yield a
        m = d*a - m
        d = (num - m*m)//d
        a = (a_0 + m)//d

def fractions(num: int) -> Iterator[Tuple[int, int]]:
    coeffs = cont_frac_sqrt(num)
    numer_n_1, denom_n_1 = 1, 0
    numer_n_2, denom_n_2 = 0, 1

    for c_n in coeffs:
        numer_next = c_n * numer_n_1 + numer_n_2
        denom_next = c_n * denom_n_1 + denom_n_2

        yield numer_next, denom_next

        numer_n_2, numer_n_1 = numer_n_1, numer_next
        denom_n_2, denom_n_1 = denom_n_1, denom_next

start = time.clock()
squares = set( map( lambda x: x*x, range(1, 33) ) )

max_x = 1000
max_x_num = 0

for D in filter(lambda x: x not in squares, range(1, 1001)):
    frac_iter = fractions(D)
    solutions = filter(lambda tup: tup[0]**2 - D * tup[1]**2 == 1, frac_iter)
    numer, denom = next(solutions)
    if numer > max_x:
        max_x = numer
        max_x_num = D
end = time.clock()

print(max_x_num, ": ", max_x)
print("Execution time: ", end - start)
