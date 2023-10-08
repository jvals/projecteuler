diagonals = [1]
space_limit = 1
current_space = 0
four_list = []

PROBLEM_SIZE = 1001 ######

for i in range(2,PROBLEM_SIZE**2+1):
	if(len(four_list)==4):
		diagonals.extend(four_list)
		four_list = []
		space_limit += 2

	if(current_space == space_limit):
		four_list.append(i)
		current_space = 0

	elif(current_space < space_limit):
		current_space += 1

diagonals.extend(four_list)

print(sum(diagonals))