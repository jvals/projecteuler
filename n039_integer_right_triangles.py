'''
If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there are exactly three solutions for p = 120.

{20,48,52}, {24,45,51}, {30,40,50}

For which value of p ≤ 1000, is the number of solutions maximised?
'''

# The triangle is right, so we can utilize pythagoras.
from collections import Counter

solutions = Counter()

for a in range(1,1000):
	for b in range(a,1000):
		c = ((a**2+b**2)**0.5)
		p = a + b + c
		# if p == 120.0:
		# 	print('a',a,'b',b,'c',c)
		if p > 1000 or p != int(p):
			continue
		else:
			solutions.update([p])

print(solutions.most_common(1))
