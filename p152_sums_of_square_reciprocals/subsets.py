import itertools
import math
from fractions import Fraction
from typing import Iterable

MAX_N = 80

primes = [
    p
    for p in [
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
        43,
        47,
        53,
        59,
        61,
        67,
        71,
        73,
        79,
    ]
    if p <= MAX_N
]


def gen_combinations(nums: list[int]) -> Iterable[list[int]]:
    for comb in range(2 ** len(nums)):
        combination = []
        idx = 0
        while comb != 0:
            if comb & 1 == 1:
                combination.append(nums[idx])
            comb >>= 1
            idx += 1
        yield combination


def valid_combinations(P: int, impossible_candidates: set[int] = set()) -> list[list[int]]:
    multiples_of_P = [n for n in range(P, MAX_N + 1, P) if n not in impossible_candidates]
    all_combinations = sorted(gen_combinations(multiples_of_P))

    def prime_multiplicity(num: int, prime: int) -> int:
        multiplicity = 0
        while num % prime == 0:
            num //= prime
            multiplicity += 1

        return multiplicity

    def is_valid_combination(prime: int, factors: list[int]) -> bool:
        mult_corr = -1 if prime == 2 else 0
        # multiplicity = 2 * sum(prime_multiplicity(d, prime) for d in factors)
        lcm = math.lcm(*[d**2 for d in factors])
        lcm_multiplicity = prime_multiplicity(lcm, prime)

        # multiplicity == multiplity - lcm_multiplicity + multiplicity_of_sum
        # for 2:
        # multiplicity - 2 = multiplicity - lcm_multiplicity + multiplcity_of_sum
        #

        return sum(lcm // d**2 for d in factors) % prime ** (lcm_multiplicity + mult_corr) == 0

    valid_combs = sorted([comb for comb in all_combinations if is_valid_combination(P, comb)])
    # print(len(all_combinations))
    return valid_combs
    # print(len(valid_combs))
    # print(valid_combs)


candidates = set(range(2, MAX_N + 1))
# consider all multiples of purely 2 and 3 a valid combination as we can't check them
# like the others
valid_twos = [n for n in [1, 2, 4, 8, 16, 32, 64] if n <= MAX_N]
valid_threes = [n for n in [1, 3, 9, 27] if n <= MAX_N]
numbers_in_any_valid_combination_2_and_3 = {a * b for a, b in itertools.product(valid_twos, valid_threes) if a * b <= MAX_N} - {1}
impossibles: set[int] = set()

numbers_in_any_valid_combination_above_3 = set()
for prime in primes[2:]:
    valid_combs = valid_combinations(prime)
    for comb in valid_combs:
        numbers_in_any_valid_combination_above_3.update(comb)

    if len(valid_combs) == 1:
        impossibles.update(range(prime, MAX_N, prime))
    # numbers_in_any_valid_combination.union

    # print(len(valid_combs))

numbers_in_any_valid_combination = numbers_in_any_valid_combination_2_and_3 | numbers_in_any_valid_combination_above_3
print(
    f"numbers in any combination, n = {len(numbers_in_any_valid_combination)}",
    numbers_in_any_valid_combination,
)

possible_combs_by_prime = [(prime, valid_combinations(prime)) for prime in primes[2:]]


Combination = list[int]

prime_factors: dict[int, set[int]] = {n: set() for n in range(2, MAX_N + 1)}
for p in primes:
    for multiple in range(p, MAX_N + 1, p):
        prime_factors[multiple].add(p)

# 2 must always be included or else you can't reach 0.5
pure_powers = [n for n in [4, 8, 16, 32, 64] if n <= MAX_N]
pure_power_combs = list(gen_combinations(pure_powers))
pure_power_combs = [[2] + comb for comb in pure_power_combs]


def reciprocal_sum(factors: Iterable[int]) -> Fraction:
    return sum((Fraction(1, n * n) for n in factors), start=Fraction())


n_combs = 0


def search_combinations(
    combs: list[tuple[int, list[Combination]]],
    output: list[Combination],
    combination: Combination,
    prime_idx: int,
    remaining_candidates: set[int],
) -> None:
    global n_combs
    if prime_idx == -1:
        # multiples_of_two = [n for n in combination if n % 2 == 0]
        # same procedure as for higher primes, but there are too many
        # possible combinations for powers of two.
        # so we flip the order of operations and let the higher prime combinations
        # filter out most sets and then we check is_valid_combination for the combinations 2
        # that can be reached.
        for pr_comb in pure_power_combs:
            # higher prime multiples are impossible by construction
            new_comb = set(combination).union(pr_comb)
            n_combs += 1
            sum_ = reciprocal_sum(new_comb)
            # print(sorted(new_comb))
            if sum_ == 0.5:
                output.append(list(new_comb))
        return

    prime, combinations = combs[prime_idx]
    remaining_candidates = remaining_candidates.difference(range(p, MAX_N + 1, p))
    for pr_comb in combinations:
        higher_prime_multiples = set(n for n in pr_comb if max(prime_factors[n]) > prime)
        comb_set = set(combination)
        if higher_prime_multiples & comb_set != higher_prime_multiples:
            continue

        new_comb = comb_set.union(pr_comb)
        next_remaining_candidates = remaining_candidates.union(pr_comb)

        if reciprocal_sum(next_remaining_candidates) < 0.5:
            continue

        search_combinations(combs, output, list(new_comb), prime_idx - 1, next_remaining_candidates)


candidates = numbers_in_any_valid_combination - impossibles
# print("2s", sum(1 for n in candidates if n % 2 == 0))
# print("3s", sum(1 for n in candidates if n % 3 == 0))
# print(sorted(candidates))
impossibles = set(range(2, MAX_N + 1)) - candidates

possible_combs_by_prime.insert(0, (3, valid_combinations(3, impossibles)))

numbers_in_any_valid_combination = set().union(numbers_in_any_valid_combination_above_3)
for comb in possible_combs_by_prime[0][1]:
    numbers_in_any_valid_combination.update(comb)
numbers_in_any_valid_combination.update(valid_twos)
numbers_in_any_valid_combination.remove(1)

print(
    f"numbers in any combination, n = {len(numbers_in_any_valid_combination)}",
    numbers_in_any_valid_combination,
)
# for prime in primes[1:]:
#     valid_combs = valid_combinations(prime)
#     for comb in valid_combs:
#         numbers_in_any_valid_combination_above_3.update(comb)

candidates = numbers_in_any_valid_combination - impossibles
# print("2s", sum(1 for n in candidates if n % 2 == 0))
# print("3s", sum(1 for n in candidates if n % 3 == 0))
# print(sorted(candidates))
impossibles = set(range(2, MAX_N + 1)) - candidates


# print(len(valid_combinations(3, set(range(2, MAX_N + 1)) - candidates)))

output: list[Combination] = []
search_combinations(possible_combs_by_prime, output, [], len(possible_combs_by_prime) - 1, candidates)

# the solutions are corect, but there are duplicates
# see result_check for filteringg
print([sorted(comb) for comb in output])
print(len(output))
