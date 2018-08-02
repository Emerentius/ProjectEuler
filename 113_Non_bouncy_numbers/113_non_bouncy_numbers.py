# amount of monotonically increasing/decreasing numbers can be computed
# via lattice paths

from scipy.special import comb

n = 100 # length of a googol
print( comb(n+9, n, exact=True) + comb(n+10, n, exact=True) - 10*n - 2 )
