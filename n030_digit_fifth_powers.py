def equality(number,power):
	power_sum = sum(int(x)**power for x in str(number))
	return number == power_sum

power_list = []
for i in range(2,531441): # 9 * 9^5
	if equality(i,5):
		power_list.append(i)


print(sum(power_list))

