from copy import deepcopy

def triangle(n):
	return int((n*(n+1))/2)

def square(n):
	return int(n*n)

def pentagonal(n):
	return int((n*(3*n-1))/2)

def hexagonal(n):
	return int(n*(2*n-1))

def heptagonal(n):
	return int((n*(5*n-3))/2)

def octagonal(n):
	return int(n*(3*n-2))

def build_list(function):
	inner_list = []
	last = 1
	t = 2
	while(last < 10000):
		if(last >= 1000):
			inner_list.append(last)
		last = function(t)
		t += 1

	return inner_list


all_types = []

all_types.append(build_list(triangle))
all_types.append(build_list(square))
all_types.append(build_list(pentagonal))
all_types.append(build_list(hexagonal))
all_types.append(build_list(heptagonal))
all_types.append(build_list(octagonal))

#print(all_types)

def match(n1,n2):
	if str(n1)[-2:] == str(n2)[:2]:
		return True
	else:
		return False

# Algorithm

for type1 in range(len(all_types)):
	for n1 in all_types[type1]:

		type2_loop = deepcopy(all_types)
		del type2_loop[type1]

		for type2 in range(len(type2_loop)):
			for n2 in type2_loop[type2]:

				if not match(n1, n2):
					continue

				type3_loop = deepcopy(type2_loop)
				del type3_loop[type2]

				for type3 in range(len(type3_loop)):
					for n3 in type3_loop[type3]:
						
						if not match(n2, n3):
							continue
						
						type4_loop = deepcopy(type3_loop)
						del type4_loop[type3]

						for type4 in range(len(type4_loop)):
							for n4 in type4_loop[type4]:

								if not match(n3, n4):
									continue

								type5_loop = deepcopy(type4_loop)
								del type5_loop[type4]

								for type5 in range(len(type5_loop)):
									for n5 in type5_loop[type5]:

										if not match(n4, n5):
											continue

										type6_loop = deepcopy(type5_loop)
										del type6_loop[type5]

										for type6 in range(len(type6_loop)):
											for n6 in type6_loop[type6]:

												if not match(n5, n6):
													continue

												if match(n6, n1):
													print(n1,n2,n3,n4,n5,n6, end='')
													print(" Sum = ", n1+n2+n3+n4+n5+n6)




