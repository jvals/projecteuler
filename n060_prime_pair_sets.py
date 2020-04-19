'''
The primes 3, 7, 109, and 673, are quite remarkable. 
By taking any two primes and concatenating them in any order the result will always be prime. 
For example, taking 7 and 109, both 7109 and 1097 are prime. 
The sum of these four primes, 792, represents the lowest sum for a set of four primes with this property.

Find the lowest sum for a set of five primes for which any two primes concatenate to produce another prime.
'''

from time import time

def primes_sieve2(limit):
	a = [True] * limit                          # Initialize the primality list
	a[0] = a[1] = False

	for (i, isprime) in enumerate(a):
		if isprime:
			yield i
			for n in range(i*i, limit, i):     # Mark factors non-prime
				a[n] = False

def main():
	primes_generator = primes_sieve2(10000)

	primes_list = list(primes_generator)
	print(primes_list[:-4])

	for a in primes_list:
		for b in primes_list:
			p = (a, b)
			if not concat_test(p):
				continue
			else:
				for c in primes_list:
					p = (a, b, c)
					if not concat_test(p):
						continue
					else:
						for d in primes_list:
							p = (a, b, c, d)
							if not concat_test(p):
								continue
							else:
								for e in primes_list:
									p = (a,b,c,d,e)
									if concat_test(p):
										print(p)
										return True
									else:
										continue
	print("Failed")

def concat_test(p):
	prime2 = p[-1]
	for a in range(len(p)-1):
		prime1 = p[a]
		if isprime(int(str(prime1)+str(prime2))) and isprime(int(str(prime2)+str(prime1))):
			continue
		else:
			return False
	return True

weird_primes = set()

def isprime(n):
	if n in weird_primes:
		return True
	#'''check if integer n is a prime'''
	# make sure n is a positive integer
	n = abs(int(n))
	# 0 and 1 are not primes
	if n < 2:
		return False
	# 2 is the only even prime number
	if n == 2:
		return True
	# all other even numbers are not primes
	if not n & 1:
		return False
	# range starts with 3 and only needs to go up the squareroot of n
	# for all odd numbers
	for x in range(3, int(n**0.5)+1, 2):
		if n % x == 0:
			return False
	weird_primes.add(n)
	return True

start = time()
main()
print("Timing:", time()-start)


