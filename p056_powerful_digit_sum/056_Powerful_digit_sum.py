import itertools

it = itertools.product(range(1, 100), range(1, 100))
it = (str(a**b) for (a, b) in it)
it = (sum(int(digit) for digit in num_string) for num_string in it)
print(max(it))

