'''
We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once. For example, 2143 is a 4-digit pandigital and is also prime.

What is the largest n-digit pandigital prime that exists?
'''

def pandigital(number):
	ideal_number = '123456789'
	ideal_number = ideal_number[:len(str(number))]
	return not ideal_number.strip(str(number))



with open('additionals/large_primes.txt') as doc:
	largest_prime = 0
	for prime in doc:
		if pandigital(int(prime)):
			largest_prime = int(prime)
		if len(prime) > 9:
			break

print(largest_prime)
