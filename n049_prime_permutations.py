'''
The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330, is unusual in two ways: (i) each of the three terms are prime, and,
(ii) each of the 4-digit numbers are permutations of one another.

There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes, exhibiting this property, but there is one other 4-digit increasing sequence.

What 12-digit number do you form by concatenating the three terms in this sequence?
'''

from itertools import permutations
from collections import Counter

with open('additionals/primes.txt') as doc:
	primes = set()
	for i,line in enumerate(doc):
		if int(line)>9999: break
		primes.add(int(line))

def has_n_prime_permutations(number):
	permutation_generator = permutations(str(number),4)
	perm_list = []
	for perm in permutation_generator:
		perm = ''.join(perm)
		perm = int(perm)

		if perm in primes and len(str(perm))==4:
			perm_list.append(perm)

	c = Counter()

	perm_list.sort()


	results = {}


	for i in range( len(perm_list)-1, 0 , -1 ):
		for j in range( i-1, -1, -1  ):
			if not perm_list[i]-perm_list[j] in results:
				results[perm_list[i]-perm_list[j]] = {perm_list[i],perm_list[j]}
			else:
				results[perm_list[i]-perm_list[j]].add(perm_list[i])
				results[perm_list[i]-perm_list[j]].add(perm_list[j])

			c.update([perm_list[i]-perm_list[j]])

	if not c:
		return

	largest = c.most_common()[0][1]
	for element in c.most_common():
		if element[1] < largest:
			del c[element[0]]

	for key in results:
		if key in c and len(results[key]) == 3 and number in results[key] and key != 0:
			print(number,'\t',results[key],'\t',key)


# has_n_prime_permutations(1487)
for i in range(1000,10000):
	has_n_prime_permutations(i)

