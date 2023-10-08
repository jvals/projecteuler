def fib(mx):
	f1 = 1
	f2 = 1
	next = 0
	term = 3
	while(True):
		next = f1 + f2
		f1 = f2
		f2 = next
		if(len(str(next))==mx):
			return term
		else:
			term += 1

print(fib(1000))