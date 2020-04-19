# Find the largest prime factor of a number
# Slow working implementation by JV

from math import sqrt
from time import time

number = 13
#number = 600851475143
#number = 13195
#number = 9009
#number = 220

def lpf(number):
	numbers = []
	largest = 0
	t=2
	while(True):
		if((number%t)==0):
			if(sqrt(t) in numbers):
				numbers.append(sqrt(t))
			else:
				numbers.append(t)
			largest = max(largest,t)
			if(product_of_list(numbers)==number): # To avoid horrible execution times
				break
		t+=1
	return largest

def product_of_list(some_list):
	p = 1
	for i in some_list:
		p *= i
	return p

start = time()
print(lpf(number))
print(time()-start)
# x = timeit.timeit(lpf,number = 1)
# print(x)
