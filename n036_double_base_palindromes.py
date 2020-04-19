from time import time

start = time()

def is_palindrome(number):
	return str(number) == str(number)[::-1]

mysum = 0
for i in range(10**6):
	binary = int(bin(i)[2:])
	if is_palindrome(i) and is_palindrome(binary):
		mysum += i

print(mysum)
print('Time:',time()-start)