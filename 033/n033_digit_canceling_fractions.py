from fractions import Fraction
from decimal import Decimal

def digit_cancels(numerator, denominator):
	if numerator < 10 or denominator < 10:
		return False
	elif numerator/denominator >= 1:
		return False
	nums = [str(numerator)[0],str(numerator)[1]]
	dens = [str(denominator)[0],str(denominator)[1]]

	if '0' in nums or '0' in dens:
		return False

	if nums[0] == dens[0]:
		del nums[0]
		del dens[0]
	elif nums[0] == dens[1]:
		del nums[0]
		del dens[1]
	elif nums[1] == dens[0]:
		del nums[1]
		del dens[0]
	elif nums[1] == dens[1]:
		del nums[1]
		del dens[1]
	else:
		return False

	if numerator/denominator == int(nums[0])/int(dens[0]):
		return True
	else:
		return False

fracs = []
for i in range(10,100):
	for j in range(10,100):
		if digit_cancels(i,j):
			fracs.append(Fraction(i,j))

print(fracs)
product = 1
for element in fracs:
	product *= element

print((product.denominator))


