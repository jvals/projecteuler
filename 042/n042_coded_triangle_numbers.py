'''
The nth term of the sequence of triangle numbers is given by, tn = ½n(n+1); so the first ten triangle numbers are:

1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...

By converting each letter in a word to a number corresponding to its alphabetical position and adding these values we form a word value. For example, the word value for SKY is 19 + 11 + 25 = 55 = t10. If the word value is a triangle number then we shall call the word a triangle word.

Using words.txt (right click and 'Save Link/Target As...'), a 16K text file containing nearly two-thousand common English words, how many are triangle words?
'''
from string import ascii_lowercase
from time import time

start = time()

def triangle(n):
	return int((1/2)*n*(n+1))

def triangle_converter(word):
	word_sum = 0
	for char in word.lower(): # .lower() uncapitalizes all characters
		# ascii_lowercase is the lowercase latin alphabet
		word_sum += ascii_lowercase.find(char) + 1
	return word_sum

triangle_numbers = set()
for i in range(26*14): # Worst Case of a fourteen letter word (administration)
	triangle_numbers.add(triangle(i))

with open('additionals/p042_words.txt') as doc:
	count = 0
	doc = doc.readline().split(',')
	for word in doc:
		word = word.strip('\"')
		if triangle_converter(word) in triangle_numbers:
			count += 1

print(count)

print('Time=',time()-start)




