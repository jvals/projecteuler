'''Permuted multiples

It can be seen that the number, 125874, and its double, 251748, contain exactly the same digits, but in a different order.

Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same digits.'''

def check2x3x4x5x5x6x(N):
	digits = {int(x) for x in str(N)}
	x2 = {int(x) for x in str(2*N)}
	x3 = {int(x) for x in str(3*N)}
	x4 = {int(x) for x in str(4*N)}
	x5 = {int(x) for x in str(5*N)}
	x6 = {int(x) for x in str(6*N)}
	return digits == x2 == x3 == x4 == x5 == x6

i = 1
while True:
	if check2x3x4x5x5x6x(i):
		print(i)
		break
	else:
		i += 1
