from time import time

start = time()

def pandigital(number):
	ideal_number = '0123456789'
	ideal_number = ideal_number[:len(str(number))]
	return not ideal_number.strip(str(number))
	

def check_num(number):
	if len(str(number)) != 10:
		return False
	if not pandigital(number):
		return False
	x1 = int(str(number)[1:4])#; print(x1)
	x2 = int(str(number)[2:5])#; print(x2)
	x3 = int(str(number)[3:6])#; print(x3)
	x4 = int(str(number)[4:7])#; print(x4)
	x5 = int(str(number)[5:8])#; print(x5)
	x6 = int(str(number)[6:9])#; print(x6)
	x7 = int(str(number)[7:10])#; print(x7)

	return not(x1%2 or x2%3 or x3%5 or x4%7 or x5%11 or x6%13 or x7%17)
l= []
for a in range(10):
	for b in range(10):
		if(b not in {a}):
			for c in range(10): 
				if(c not in {a,b}):
					for d in range(10):
						if d not in {a,b,c}:
							for e in range(10):
								if e not in {a,b,c,d}:
									for f in range(10):
										if f not in {a,b,c,d,e}:
											for g in range(10):
												if g not in {a,b,c,d,e,f}:
													for h in range(10):
														if h not in {a,b,c,d,e,f,g}:
															for i in range(10):
																if i not in {a,b,c,d,e,f,g,h}:
																	for j in range(10):
																		if j not in {a,b,c,d,e,f,g,h,i}:
																			l.append(str(a)+str(b)+str(c)+str(d)+str(e)+str(f)+str(g)+str(h)+str(i)+str(j))

# print(check_num(int('1406357289')))

s=0
for digit in l:
	if check_num(int(digit)):
		s+= int(digit)

print(s)
print('Time:',time()-start)