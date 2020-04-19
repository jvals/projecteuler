from time import time

start = time()

prime_list = []
prime_set = set()

with open("additionals/primes.txt") as doc:
	for line in doc:
		prime_list.append(int(line.strip()))
		prime_set.add(int(line.strip()))

def is_truncatable(number):
	length = len(str(number))
	for i in range(length):
		tempnumber = str(number)[:len(str(number))-i]
		tempnumber = int(tempnumber)

		if tempnumber not in prime_set:
			return False

	for i in range(length):
		tempnumber = str(number)[i:]
		tempnumber = int(tempnumber)

		if tempnumber not in prime_set:
			return False

	return True


p = []

for prime in prime_list[4:]:
	if is_truncatable(prime):
		p.append(prime)

print(len(p))
print(sum(p))
