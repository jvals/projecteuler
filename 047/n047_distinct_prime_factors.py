'''
The first two consecutive numbers to have two distinct prime factors are:

14 = 2 × 7
15 = 3 × 5

The first three consecutive numbers to have three distinct prime factors are:

644 = 2² × 7 × 23
645 = 3 × 5 × 43
646 = 2 × 17 × 19.

Find the first four consecutive integers to have four distinct prime factors. What is the first of these numbers?
'''

from math import ceil
from time import time

start = time()
with open('additionals/primes.txt') as doc:
	prime_set = set()
	prime_list = []
	for line in doc:
		prime_set.add(int(line))
		prime_list.append(int(line))
end = time()

print('Time spent reading large_primes.txt:',end-start)

dists = {}


def find_primefactors(n,divisors = None):

	if divisors is None:
		divisors = set()
	if n in prime_set:
		divisors.add(n)
		return divisors

	for i, prime in enumerate(prime_list):
		if i > ceil(n**0.5):
			break
		if n%prime == 0:
			divisors.add(prime)
			find_primefactors(n//prime,divisors)
			break
	return divisors
	




def four_distinct(n):
	global dists

	if n in dists:
		return dists[n]

	prime_factors = find_primefactors(n)
	# print('n: {} \t divisors: {}'.format(n,prime_factors))

	if len(prime_factors) >= 4:
		dists[n] = True
		return True
	else:
		dists[n] = False
		return False



n=1
while(True):
	if n in prime_set:
		dists[n] = False
		n+=1
		continue
	try:
		if four_distinct(n) and four_distinct(n+1) and four_distinct(n+2) and four_distinct(n+3):
			print(n)
			break
	except:
		print('Error occured at n=',n)
	n+=1

