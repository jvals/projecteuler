def numWordBuild(n=0):
	oneDict ={	0:"",
				1:"one",
				2:"two",
				3:"three",
				4:"four",
				5:"five",
				6:"six",
				7:"seven",
				8:"eight",
				9:"nine",

				10:"ten",
				11:"eleven",
				12:"twelve",
				13:"thirteen",
				14:"fourteen",
				15:"fifteen",
				16:"sixteen",
				17:"seventeen",
				18:"eighteen",
				19:"nineteen"}

	tenDict ={	0:"",
				2:"twenty",
				3:"thirty",
				4:"forty",
				5:"fifty",
				6:"sixty",
				7:"seventy",
				8:"eighty",
				9:"ninety"}


	if n<20:
		return oneDict.get(n)


	size = len(str(n))

	one = int(str(n)[-1])
	ten = int(str(n)[-2])

	if(one!=0 and ten>1):
		word = tenDict.get(ten) +'-'+ oneDict.get(one)
	elif(int(str(ten)+str(one))<20):
		word = oneDict.get(int(str(ten)+str(one)))
	else:
		word = tenDict.get(ten) + oneDict.get(one)

	if(size>2):
		hundred = int(str(n)[-3])
		if(ten == 0 and one == 0):
			word = oneDict.get(hundred) + " hundred"
		else:
			word = oneDict.get(hundred) + " hundred and " + word

	
	return word

wordList = {}
for i in range(1,1000):
	wordList[i] = numWordBuild(i)

wordList[1000] = "one thousand"

# #I used this to make myself a txt with all the numberwords
# with open("additionals/numbers.txt",'w') as doc:
# 	for value in wordList.values():
# 		print(value,file=doc)



sum = 0
for o in wordList.values():
	o = o.replace(" ","")
	o = o.replace("-","")
	print(o)
	sum += len(o)
print(sum)