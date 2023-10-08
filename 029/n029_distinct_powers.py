def main(minimum,maximum):

	powers = set()

	for a in range(minimum,maximum+1):
		for b in range(minimum,maximum+1):
			powers.add(a**b)

	return len(powers)

print(main(2,10**5))