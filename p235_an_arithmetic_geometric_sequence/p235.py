def s(r: float) -> float:
    return sum((900 - 3*k)*r ** (k - 1) for k in range(1, 5001))

def goal(r: float) -> float:
    return abs(s(r) - (-600_000_000_000))

# I got to this point by wiggling the input around by hand
# before getting tired of it.
r = 1.002322110000

step = 0.0001

diff = goal(r)
while step > 1e-14:
    if goal(r + step) < diff:
        r += step
    elif goal(r - step) < diff:
        r -= step
    
    step /= 2
    diff = goal(r)

print(f"{r:0.12f}")