'''
A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4. 
If all of the permutations are listed numerically or alphabetically, we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:

012   021   102   120   201   210

What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
'''
from time import time

start = time()

l=[]
for a in range(10):
	for b in range(10):
		if(b not in [a]):
			for c in range(10): 
				if(c not in [a,b]):
					for d in range(10):
						if d not in [a,b,c]:
							for e in range(10):
								if e not in [a,b,c,d]:
									for f in range(10):
										if f not in [a,b,c,d,e]:
											for g in range(10):
												if g not in [a,b,c,d,e,f]:
													for h in range(10):
														if h not in [a,b,c,d,e,f,g]:
															for i in range(10):
																if i not in [a,b,c,d,e,f,g,h]:
																	for j in range(10):
																		if j not in [a,b,c,d,e,f,g,h,i]:
																			l.append([a,b,c,d,e,f,g,h,i,j])
					
l.sort()


print(l[1000000-1])
print('Time:',time()-start)
