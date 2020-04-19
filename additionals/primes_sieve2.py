def primes_sieve2(limit):
	a = [True] * limit                          # Initialize the primality list
	a[0] = a[1] = False

	for (i, isprime) in enumerate(a):
		if isprime:
			yield i
			for n in range(i*i, limit, i):     # Mark factors non-prime
				a[n] = False

def main():
	list(primes_sieve2(100000000))

main()