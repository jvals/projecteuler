def triangle(n):
	return (n*(n+1))/2

def square(n):
	return n*n

def pentagonal(n):
	return (n*(3*n-1))/2

def hexagonal(n):
	return n*(2*n-1)

def heptagonal(n):
	return (n*(5*n-3))/2

def octagonal(n):
	return n*(3*n-2)



def counter(function):
	last = 0
	t = 0
	real_t = 0
	while(last < 10000):
		t += 1
		if(last >= 1000):
			real_t += 1
		last = function(t)

	print(function.__name__, real_t)

counter(triangle)
counter(square)
counter(pentagonal)
counter(hexagonal)
counter(heptagonal)
counter(octagonal)