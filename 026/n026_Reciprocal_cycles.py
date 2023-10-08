
def findCycle(n):
	x = str(n)
	x = x[2:]
	x = [c for c in x]
	for i in range(len(x)-1,0,-1):
		x.insert(i,' ')
	string = ''
	for i in range(len(x)):
		string += x[i]

	import re

	if(len(string)==1):
		return string

	# regex = re.compile('(.+ .+)( \1)+')
	match = re.search(r'(.+ .+)( \1)+',string)
	# print(match.group(0))   # entire match
	return(match.group(1))    # repeating portion



print(findCycle(1/3))
# maxLength,maxCycle = 0,0
# for i in range(2,1000):
# 	try:
# 		cycle = findCycle(1/i)
# 		if len(cycle)>maxLength:
# 			maxLength = len(cycle)
# 			maxCycle = cycle
# 	except Exception:
# 		print(i)
# print(mx)