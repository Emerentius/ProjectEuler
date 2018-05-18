# Formula for F(x) = Sum_1.. (x^n * f(n)), where f(n) are fibonacci coefficients, f(1) = f(2) = 1
# F(x) = x / (1 - x - x²)
#      = ab / (b²-ab-a²), with x = a/b
#
#     N = x / (1 - x - x²)
#  => N - (N+1)x - Nx² = 0
#  => x² + (N+1)/N x - 1 = 0
#     p = (N+1)/N
#     q = -1
#     x = -p/2 ± sqrt(p²/4 - q)
#     x is rational if the sqrt is rational
#
#     sqrt(p²/4 - q) = 5N² + 2N + 1 / 4N²
#       rational if that's sqrt(a²/b²)
#       4N² is always square so numerator is the only hurdle
#       => 5N² + 2N + 1 = a² must hold for some integer a
#
#     Analysis of the first few solutions have shown that consecutive ones
#     are roughly apart by a factor of 6.8541
#     => jump ahead if a solution is found, otherwise do linear search for integer a

n = 1
sols = []
while len(sols) < 15:
    n += 1
    a2 = (5*n*n + 2*n + 1)
    a = int(a2**0.5)
    if a*a == a2:
        sols.append(n)
        print(n, a2/4/n/n)

        n = int(n * 6.8541) - 2
