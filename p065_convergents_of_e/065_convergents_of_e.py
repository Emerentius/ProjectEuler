from typing import Iterator, Tuple
import time
import itertools

def cont_frac_of_e() -> Iterator[int]:
    yield 2
    yield 1

    for N in itertools.count(2, 2):
        yield N
        yield 1
        yield 1

def fractions() -> Iterator[Tuple[int, int]]:
    coeffs = cont_frac_of_e()
    numer_n_1, denom_n_1 = 1, 0
    numer_n_2, denom_n_2 = 0, 1

    for c_n in coeffs:
        numer_next = c_n * numer_n_1 + numer_n_2
        denom_next = c_n * denom_n_1 + denom_n_2

        yield numer_next, denom_next

        numer_n_2, numer_n_1 = numer_n_1, numer_next
        denom_n_2, denom_n_1 = denom_n_1, denom_next

# little endian
def digits(num: int) -> Iterator[int]:
    while num != 0:
        yield num % 10
        num //= 10

start = time.clock()
convergent_100_numer, _ = next(itertools.islice(fractions(), 99, 100))
print( sum( digits(convergent_100_numer) ) )
end = time.clock()

print("Execution time: ", end - start)
