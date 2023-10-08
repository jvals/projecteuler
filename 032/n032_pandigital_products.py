def is_pandigital(number):
	if '0' in str(number):
		return False
	n = len(str(number))
	if n != 9:
		return False
	number_list = [int(x) for x in str(number)]
	number_list.sort()
	for i in range(0,len(number_list)):
		if i+1 != number_list[i]:
			return False
	return True

	


pandigitals = set()
for a in range(10000):
	for b in range(100):
		number = int(str(a)+str(b)+str(a*b))
		if is_pandigital(number):
			pandigitals.add(a*b)

print(sum(pandigitals))



