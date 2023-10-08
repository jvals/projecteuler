'''
Triangle, pentagonal, and hexagonal numbers are generated by the following formulae:
Triangle 	  	Tn=n(n+1)/2 	  	1, 3, 6, 10, 15, ...
Pentagonal 	  	Pn=n(3n−1)/2 	  	1, 5, 12, 22, 35, ...
Hexagonal 	  	Hn=n(2n−1) 	  	1, 6, 15, 28, 45, ...

It can be verified that T285 = P165 = H143 = 40755.

Find the next triangle number that is also pentagonal and hexagonal.
'''

from math import sqrt

T = lambda n: int((n*(n+1))/2)
P = lambda n: int((n*(3*n-1))/2)
H = lambda n: int(n*(2*n-1))

invP = lambda n: float((1/6) * (1 + sqrt(24*n+1)))
invH = lambda n: float ((1/4) * (1 + sqrt(8*n+1)))


n=286
while(True):
	Triangle = T(n)
	Pentagonal = invP(Triangle)
	Hexagonal = invH(Triangle)
	if Pentagonal.is_integer() and Hexagonal.is_integer():
		print(Triangle,n)
		break
	n += 1

