'''
The prime 41, can be written as the sum of six consecutive primes:
41 = 2 + 3 + 5 + 7 + 11 + 13

This is the longest sum of consecutive primes that adds to a prime below one-hundred.

The longest sum of consecutive primes below one-thousand that adds to a prime, contains 21 terms, and is equal to 953.

Which prime, below one-million, can be written as the sum of the most consecutive primes?
'''

from collections import OrderedDict
from copy import *
from time import time

start = time()
with open('additionals/primes.txt') as doc:
	list_of_primes = []
	set_of_primes = set()
	for i,line in enumerate(doc):
		set_of_primes.add(int(line))
		list_of_primes.append(int(line))



sequences = []

for i in range(len(list_of_primes)):
	sequence = [list_of_primes[i]]
	for j in range(i+1,len(list_of_primes)):
		if sum(sequence)+list_of_primes[j] >= 1000000:
			break

		sequence.append(list_of_primes[j])
		
		if sum(sequence) in set_of_primes:
			sequences.append(sequence.copy())

sequences.sort(key = len, reverse = True)


# for seq in sequences:
# 	print('Sum:',sum(seq),'\t',seq)

print('Sum:',sum(sequences[0]),'\t',sequences[0])

print(time()-start)
		
