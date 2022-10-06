import time
import math
import functools
from fractions import Fraction

N = 1000
BILLION = 10**9
TOTAL_AMOUNT_OF_POSSIBILITES = 2**N

@functools.lru_cache(maxsize=1000)
def binom_coeff(n, k):
    return math.factorial(n)//math.factorial(k)//math.factorial(n-k)

@functools.lru_cache()
def billionaire_prob(f):
    prob = Fraction(0,1)
    for k in range(1001):
        money = (1+2*f)**(1000-k) * (1-f)**k
        if money < BILLION:
            break
        prob += binom_coeff(1000, k)
    return prob / TOTAL_AMOUNT_OF_POSSIBILITES

start = time.time()

f = Fraction(1,2)
step = Fraction(1,4)

# do binary search until no further change visible
old_f = Fraction(0,1)
while abs( billionaire_prob(old_f) - billionaire_prob(f)) > 1e-12:
    old_f = f
    if billionaire_prob(f+step) > billionaire_prob(f-step):
        f += step
    else:
        f -= step
    step /= 2

prob = float( billionaire_prob(f) )
print("{:.12}".format(prob) )

end = time.time()
print("time needed: {:.0f}ms".format( (end-start)*1000) )
