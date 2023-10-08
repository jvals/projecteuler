'''
Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing over five-thousand first names, begin by sorting it into alphabetical order. 
Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a name score.

For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list.
So, COLIN would obtain a score of 938 × 53 = 49714.

What is the total of all the name scores in the file?
'''

# Opens a text file with names and puts them in a string
with open('additionals/names.txt') as doc:
	nameString = ''
	for line in doc:
		nameString += line

# Split the string into a list
nameList = nameString.split(',')

# Remove quotation marks
for i in range(len(nameList)):
	nameList[i] = nameList[i].replace('"','')

# Sort the list
nameList.sort()

# A string containing the alphabet, each position represents a numerical value
stringAlpha = '0abcdefghijklmnopqrstuvwxyz'

# Method to determine the alphabetical value
def determineAlphaValue(name):
	name = name.lower()
	score = 0
	for c in name:
		score += stringAlpha.find(c)
	return score

# Method to determine a score
def determineScore(name,pos):
	return determineAlphaValue(name)*pos

# Empty highscore dictinary
highScore = {}

# Iterate the list, determine each score and put it in the dictionary
for name in nameList:
	highScore[name] = determineScore(name,nameList.index(name)+1)

# Iterate the values in the dictionary and calculate the total value
total = 0
for value in highScore.values():
	total += value

print(total)