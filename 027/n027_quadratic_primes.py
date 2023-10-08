from time import time

start = time()

primes = {}
with open('additionals/primes.txt') as doc:
	for line in doc:
		primes[int(line)] = line.strip()

def f(n,a,b):
	# a = abs(a)
	# b = abs(b)
	return n**2 + a*n + b


best = (0,0)
streak = 0

for a in range(-999,1000):
	for b in range(max(2,a-1),1000): # b >= 2, a + b + 1 >= 2
		n = 0
		quad = f(n,a,b)
		current_streak = 0
		while(quad in primes):
			current_streak += 1
			n += 1
			quad = f(n,a,b)

		if current_streak > streak:
			best = (a,b)
			streak = current_streak


print('Streak:',streak,'\t Best:',best)
a,b = best
print('Answer:',a*b)
print('Time:',time()-start)