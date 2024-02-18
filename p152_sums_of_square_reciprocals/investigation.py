from fractions import Fraction
from typing import Iterable

N = 19
combs = list(range(0, 2**N))


def comb_to_numbers(comb: int) -> Iterable[int]:
    for pos in range(N):
        if (1 << pos) & comb != 0:
            yield pos + 3


def combination_sum(comb: int) -> Fraction:
    return sum(
        (Fraction(1, num * num) for num in comb_to_numbers(comb)), start=Fraction()
    )


sums = [combination_sum(comb) for comb in combs]
distinct_sums = set(sums)

print(
    f"{len(sums)}, {len(distinct_sums)} distinct, {len(distinct_sums) / len(sums) * 100:.04}%"
)
