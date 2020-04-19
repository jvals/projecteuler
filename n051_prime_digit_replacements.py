'''
By replacing the 1st digit of the 2-digit number *3, it turns out that six of the nine possible values: 13, 23, 43, 53, 73, and 83, are all prime.

By replacing the 3rd and 4th digits of 56**3 with the same digit, this 5-digit number 
is the first example having seven primes among the ten generated numbers, yielding 
the family: 56003, 56113, 56333, 56443, 56663, 56773, and 56993. Consequently 56003, 
being the first member of this family, is the smallest prime with this property.

Find the smallest prime which, by replacing part of the number 
(not necessarily adjacent digits) with the same digit, is part of an eight prime value family.
'''

def primes_sieve2(limit):
	a = [True] * limit                          # Initialize the primality list
	a[0] = a[1] = False

	for (i, isprime) in enumerate(a):
		if isprime:
			yield i
			for n in range(i*i, limit, i):     # Mark factors non-prime
				a[n] = False

def listToInt(numList):         # [1,2,3]
	s = map(str, numList)   # ['1','2','3']
	s = ''.join(s)          # '123'
	s = int(s)              # 123
	return s


max_prime = 10**7

primes = set(primes_sieve2(max_prime))


def mask1(N,number_to_test): # Number will be tested with one mask
	for i in range(len(number_to_test)):
		answer = []
		temp = list(number_to_test)
		for j in range(10):
			temp[i] = str(j)
			number = listToInt(temp)
			if number in primes and len(str(number)) == len(number_to_test):
				answer.append(number)
		if len(answer) >= N:
			print(answer)
			return True

def mask2(N,number_to_test): # Number will be tested with two masks
	for i in range(len(number_to_test)-1):
		for j in range(i+1,len(number_to_test)):
			answer = []
			temp = list(number_to_test)
			for k in range(10):
				temp[i] = str(k)
				temp[j] = str(k)
				number = listToInt(temp)
				if number in primes and len(str(number)) == len(number_to_test):
					answer.append(number)
			if len(answer) >= N:
				print(answer)
				return True


def mask3(N,number_to_test): # Number will be tested with three masks
	for i in range(len(number_to_test)-2):
		for j in range(i+1,len(number_to_test)-1):
			for k in range(j+1,len(number_to_test)):
				answer = []
				temp = list(number_to_test)
				for l in range(10):
					temp[i] = str(l)
					temp[j] = str(l)
					temp[k] = str(l)
					number = listToInt(temp)
					if number in primes and len(str(number)) == len(number_to_test):
						answer.append(number)
				if len(answer) >= N:
					print(answer)
					return True

def mask4(N,number_to_test): # Number will be tested with four masks
	for i in range(len(number_to_test)-3):
		for j in range(i+1,len(number_to_test)-2):
			for k in range(j+1,len(number_to_test)-1):
				for l in range(k+1,len(number_to_test)):
					answer = []
					temp = list(number_to_test)
					for m in range(10):
						temp[i] = str(m)
						temp[j] = str(m)
						temp[k] = str(m)
						temp[l] = str(m)
						number = listToInt(temp)
						if number in primes and len(str(number)) == len(number_to_test):
							answer.append(number)
					if len(answer) >= N:
						print(answer)
						return True




def main():
	limit = 8
	for prime in primes_sieve2(max_prime):
		if mask1(limit,str(prime)) or mask2(limit,str(prime)) or mask3(limit,str(prime)): #or mask4(limit,str(prime)):
			return True
	print("No result")

main()












