'''
In the 20×20 grid below, four numbers along a diagonal line have been marked in red.

08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48

The product of these numbers is 26 × 63 × 78 × 14 = 1788696.

What is the greatest product of four adjacent numbers in the same direction (up, down, left, right, or diagonally) in the 20×20 grid?
'''

#1 Split the string into a list
#2 Split the list into a matrix, remember to parse all elements to int
#3 Columns and diagonals(left and right) must be made into lists for iteration. Rows are already iterable
#4 Make a function which takes a list and an integer n, the function should calculate the largest product of series by size n.
#5 Use the function on all the iterable lists (i.e rows, columns, diagonals) and return the largest result

import numpy as np

series = "08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08 49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00 81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65 52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91 22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80 24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50 32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70 67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21 24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72 21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95 78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92 16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57 86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58 19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40 04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66 88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69 04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36 20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16 20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54 01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48"

# Split the string into a list
list_series = series.split(" ")

# Iterate over the list and parse all elements to int
for i in range(len(list_series)):
	list_series[i] = int(list_series[i])

# Make empty grid
grid = []
# We want to convert the list into a 20x20 matrix, the loop iterates from 0 to 400, 20 steps at a time
for a in range(0,400,20):
	# We cut all elements from a to a+20 and add them to the grid
	grid.append(list_series[a:a+20])


# We make a transposed grid using a nested listcomprehension, this gives us lists of the columns
transposed_grid = [[row[i] for row in grid] for i in range(20)]

# We use numpy matrix, because it has a built in diagonal function
ma = np.array(grid)
diagonal_list = []
# There are 20+20-1 = 39 (or 20-(-19) = 39) diagonal lines from left to right (downwards)
for i in range(-19,20):
	# The diagonal function takes i as a parameter, which is the offset from base (0). In this case from -19 to 20
	diagonal_list.append(ma.diagonal(i))

# We reverse the grid in order to find diagonals from left to right (upwards)
reversed_grid = grid[::-1]
ma2 = np.array(reversed_grid)
diagonal_list2 = []
for i in range(-19,20):
	diagonal_list2.append(ma2.diagonal(i))


# The method for finding the maximum product of a series by size n
def findMax(list,n):
	if(len(list)<n):
		return 0
	# mx is the maximum we return in the end, it's updated in every iteration of the outer loop
	mx = 0 
	# nxt is the temporary product of series, it's updated in every iteration of the inner loop
	nxt = 1
	# Outer loop, a iterates from 0 to the size of the list
	for a in range(len(list)):
		mx = max(mx, nxt)
		nxt = 1
		# Inner loop, iterates from a to n+a. 
		for b in range(a,n+a):
			# if n+a > len(list), then the series is smaller than n, and is not worth checking
			if(n+a > len(list)):
				break
			nxt *= list[b]

	return mx

holy_sum = 0

for o in grid:
	holy_sum = max(holy_sum,findMax(o,4))

for o in transposed_grid:
	holy_sum = max(holy_sum,findMax(o,4))

for o in diagonal_list:
	holy_sum = max(holy_sum,findMax(o,4))

for o in diagonal_list2:
	holy_sum = max(holy_sum,findMax(o,4))


print(holy_sum)


