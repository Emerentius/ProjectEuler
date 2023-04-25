from __future__ import annotations

primes = [
    2,
    3,
    5,
    7,
    11,
    13,
    17,
    19,
    23,
    29,
    31,
    37,
    41,
    # 43,
    # 47,
]  # 53, 59, 61, 67, 71]
_n = 8
muls = [1] * _n + [0] * (len(primes) - _n)
max_depth = 10

nums: list[int] = []
target_muls: list[list[int]] = []


target_limit = 4_000_000
# target_limit = 100


def visit_nums(i: int, depth: int) -> None:
    if depth > max_depth:
        return
    n_div_ = n_solutions(muls)
    if n_div_ > target_limit:
        num_ = num(muls)
        nums.append(num_)
        target_muls.append(muls[:])
        return

    for next_i in range(i, len(primes)):
        muls[next_i] += 1
        visit_nums(next_i, depth + 1)
        muls[next_i] -= 1


def n_solutions(num: list[int]) -> int:
    return (n_divisors_of_square(num) + 1) // 2


def n_divisors_of_square(num: list[int]) -> int:
    prod = 1
    for occ, prime in zip(num, primes):
        prod *= occ * 2 + 1

    return prod


n_div = n_divisors_of_square


def compute_num(num: list[int]) -> int:
    prod = 1
    for occ, prime in zip(num, primes):
        prod *= prime**occ

    return prod


num = compute_num


def increase_occ_1(n_prime: int) -> tuple[int, float]:
    old_occ = muls[n_prime]
    old_factor = old_occ * 2 + 1
    new_factor = old_occ * 2 + 3

    return primes[n_prime], new_factor / old_factor


def next_most_efficient() -> int:
    it = (increase_occ_1(n_prime) for n_prime in range(len(primes)))
    efficiencies = [
        (n_div_increase - 1) / num_increase for num_increase, n_div_increase in it
    ]
    max_eff = max(efficiencies)
    return efficiencies.index(max_eff)


visit_nums(0, 0)
print(min(nums))
