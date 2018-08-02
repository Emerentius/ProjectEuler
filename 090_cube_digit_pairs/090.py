import time
from itertools import product

def generate_cubes(n, cache):
    if n == 0:
        yield cache

    last = cache[-1] if cache else -1
    for i in range(last+1, 10):
        new_cache = cache + [i]
        yield from generate_cubes(n-1, new_cache)

def are_valid_sets(set1, set2):
    def clauses(set1, set2):
        yield 0 in set1 and 1 in set2
        yield 0 in set1 and 4 in set2
        yield 0 in set1 and (9 in set2 or 6 in set2)
        yield 1 in set1 and (6 in set2 or 9 in set2)
        yield 2 in set1 and 5 in set2
        yield 3 in set1 and (6 in set2 or 9 in set2)
        yield 4 in set1 and (6 in set2 or 9 in set2)
        yield (6 in set1 or 9 in set1) and 4 in set2
        yield 8 in set1 and 1 in set2

    checks1 = clauses(set1, set2)
    checks2 = clauses(set2, set1)

    return all(c1 | c2 for c1, c2 in zip(checks1, checks2))

cubes = list(generate_cubes(6,[]))
valid_combinations = sum(1 for s1, s2 in product(cubes, cubes) if s1 > s2 and are_valid_sets(s1, s2))

print(valid_combinations)
