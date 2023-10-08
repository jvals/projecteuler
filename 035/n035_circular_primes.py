'''
The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.

There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.

How many circular primes are there below one million?
'''

def isPrime(n):
	if n in primes:
		return True
	else:
		return False


def rotations_left(n):
	rotations = []
	stringDigit = str(n)
	lenght = len(stringDigit)
	for i in range(len(str(n))):
		end = stringDigit[lenght-1]
		stringDigit = end + stringDigit[:lenght-1]
		rotations.append(int(stringDigit))
	return(rotations)

primes = {}
with open('additionals/primes.txt') as doc:
	for line in doc:
		# if int(line)>100:
		# 	break
		primes[int(line)] = line.strip()
		# primes.append(int(line))


counter = 0
for prime in primes:
	rotations = rotations_left(prime)
	# print(prime,rotations, end=',')
	rotations = [isPrime(x) for x in rotations ]
	# print(rotations)
	if(all(rotations)):
		counter += 1


print(counter)
