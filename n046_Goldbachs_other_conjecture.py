'''
It was proposed by Christian Goldbach that every odd composite number can be written as the sum of a prime and twice a square.

9 = 7 + 2×12
15 = 7 + 2×22
21 = 3 + 2×32
25 = 7 + 2×32
27 = 19 + 2×22
33 = 31 + 2×12

It turns out that the conjecture was false.

What is the smallest odd composite that cannot be written as the sum of a prime and twice a square?
'''

with open('additionals/primes.txt') as doc:
	prime_set = set()
	prime_list = []
	for line in doc:
		prime_set.add(int(line))
		prime_list.append((int(line)))


def check_number(n):
	j = 0
	while prime_list[j] < n:
		i = 0
		while(prime_list[j]+i**2 <= n):
			if prime_list[j] + 2*i**2 == n:
				return True
			else:
				i+=1
		j+=1

for i in range(9,1000000,2):
	if i in prime_set:
		continue
	else:
		if not check_number(i):
			print(i)
			break



		