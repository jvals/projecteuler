
tree = []
with open('additionals/binary_tree.txt') as doc:
	for line in doc:
		tree.append(line.rstrip('\n').split(" "))

		# This part converts every node from string to int
for i in range(len(tree)):
	for j in range(len(tree[i])):
		tree[i][j] = int(tree[i][j])



def findMaxPath(list,level):
	for i in range(len(list[level])):
		list[level][i] +=  max(list[level+1][i],list[level+1][i+1])
	
	if len(list[level]) is 1:
		return list[level][0]
	else:
		return findMaxPath(list, level-1)
		

maxPath = findMaxPath(tree,len(tree)-2)
print(maxPath)