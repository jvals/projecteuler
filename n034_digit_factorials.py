from math import factorial as f

def digit_factorial(n):

	digits = str(n)
	digits = [f(int(x)) for x in digits]
	return n==sum(digits)


def main():
	l = []
	for i in range(3,41000):
		if(digit_factorial(i)):
			l.append(i)

	print(l)
	print(sum(l))

main()