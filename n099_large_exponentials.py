from math import log10

mx = 0
with open('additionals/p099_base_exp.txt') as doc:
	for lineNumber,line in enumerate(doc,1):
		line = line.split(',')
		x = int(line[1])*log10(int(line[0]))
		if x>mx:
			mx = x
			largestLineNumber = lineNumber

print(largestLineNumber)