# Warning, this takes forever

one = 1
two = 2
five = 5
ten = 10
twenty = 20
fifty = 50
hundred = 100
twohundred = 200

i = 0

for a in range(201):
	print("a",a)
	for b in range(100+1):
		for c in range(40+1):
			for d in range(20+1):
				for e in range(10+1):
					for f in range(4+1):
						for g in range(2+1):
							for h in range(1+1):
								if(one*a + two * b + five * c + ten * d + twenty * e + fifty * f + hundred * g + twohundred * h == 200):
									i += 1

print(i)

