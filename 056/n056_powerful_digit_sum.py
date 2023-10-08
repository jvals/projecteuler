# Powerful digit sum
# Problem 56

maxsum = 0
for a in range(100):
    for b in range(100):
        sumnum = sum(int(c) for c in str(a**b))
        maxsum = max(maxsum, sumnum)

print(maxsum)
