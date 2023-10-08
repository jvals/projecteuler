'''
A perfect number is a number for which the sum of its proper divisors is exactly equal to the number. 
For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.

A number n is called deficient if the sum of its proper divisors is less than n and it is called abundant if this sum exceeds n.

As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be written as the sum of two abundant numbers is 24.
By mathematical analysis, it can be shown that all integers greater than 28123 can be written as the sum of two abundant numbers. 
However, this upper limit cannot be reduced any further by analysis even though it is known that the greatest number that cannot be expressed as the sum of two abundant numbers is less than this limit.

Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.
'''

from time import time as t
from math import sqrt

start = t()

def Divisors(n):

	return [x for x in range(1,n) if n%x == 0]

def Divisors2(n):
	l = set()
	for i in range(2,int(sqrt(n))+1):
		if not n%i:
			l.add(i)
			l.add(int(n/i))
	l.add(1)
	return l


def isAbundant(n):
	if n < sum(Divisors2(n)):
		return True
	else:
		return False



abundantNumbers = [x for x in range(1,30000) if isAbundant(x)]
sumsOfTwoAbundantNumbers = set()
for i in range(len(abundantNumbers)):
	for j in range(i,len(abundantNumbers)):
		sumsOfTwoAbundantNumbers.add(abundantNumbers[i]+abundantNumbers[j])

print('Time:',t()-start,'Result:',sum([x for x in range(1,28123) if x not in sumsOfTwoAbundantNumbers]))