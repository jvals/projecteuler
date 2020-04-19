# Cubic Permuations

cubes = dict()
t = 1
while(t < 10**6):
    cube = t*t*t
    cube = sorted(str(cube))
    cube = ''.join(cube)

    if cube in cubes:
        cubes[cube][0] += 1
    else:
        cubes[cube] = [1,t]

    t += 1

smallest = float('inf')
for key in cubes:
    if cubes[key][0] == 5 and cubes[key][1] < smallest:
        smallest = cubes[key][1]

print(smallest**3)
