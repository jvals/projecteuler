from fractions import gcd
from math import cos, pi, log, e

# Slow
def phi(n):
	return sum(gcd(k, n) * cos(2*pi*(k/n))  for k in range(1, n+1))


# Slow
def phi2(n):
	result = 1
	for i in range(2, n):
		if gcd(i, n) == 1:
			result += 1
	return result

# Fast, http://www.geeksforgeeks.org/eulers-totient-function/
def phi3(n):
	# Initialize result as n
	result = n

	# Consider every number p, where p is between 2 and √n
	for p in range(2, n):
		if p*p > n:
			break

		# If p divides n
		if n % p == 0:
			# Subtract all multiples of p from 1 to n
			while n % p == 0:
				n /= p
			result -= result / p

	# If the reduced n is more than 1, then remove all multiples of n
	if n > 1:
		result -= result / n
	return result


def main():
	ans = 0
	result = 0
	for n in range(1, 10**6 + 1):
		print(n, end="\r") # Print the number we are currently computing
		candidate = n/phi3(n)
		if candidate > ans:
			result = n
			ans = candidate
	print("\n", result, sep="")


if __name__ == '__main__':
	main()