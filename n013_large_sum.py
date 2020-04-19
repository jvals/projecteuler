'''
Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.
'''


with open("additionals/large_number.txt") as doc:
	sum = 0
	for lines in doc:
		sum += int(lines)

str_sum = str(sum)
print(str_sum[0:10])
