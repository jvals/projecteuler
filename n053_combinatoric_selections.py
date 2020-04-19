# https://projecteuler.net/problem=53
# Combinatoric selections
# Problem 53

from math import factorial as f

def nCr(n,r):
	return f(n) / ( f(r) * f(n-r) )


distinct_over_1m = 0
for n in range(1,101):
	for r in range(1,n):
		if nCr(n,r)>1000000:
			distinct_over_1m += 1

print(distinct_over_1m)
