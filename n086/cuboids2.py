from functools import reduce
# code for finding pythagorean triples from http://rosettacode.org/wiki/Pythagorean_triples
from sys import setrecursionlimit
setrecursionlimit(2000) 

all_triples = []
M = 0

def triples(lim, a = 3, b = 4, c = 5):
    global all_triples
    all_triples.append((a,b,c))
    l = a + b + c
    if l>lim:
        return (0, 0)
    return reduce(lambda x, y: (x[0] + y[0], x[1] + y[1]), [
        (1, lim // l),
        triples(lim,  a - 2*b + 2*c,  2*a - b + 2*c,  2*a - 2*b + 3*c),
        triples(lim,  a + 2*b + 2*c,  2*a + b + 2*c,  2*a + 2*b + 3*c),
        triples(lim, -a + 2*b + 2*c, -2*a + b + 2*c, -2*a + 2*b + 3*c) ])

def count_valid_compositions(K, beta, delta, M):
    comps = []
    for a in range(1,(K//2)+1):
        if a > M or a > beta:
            break
        b =  K - a
        if b > M or b > beta:
            continue
        comps.append((a,b,beta,delta))
            
    return comps

def scale_triples(M):
    global all_triples
    scaled_triples = []
    for a,b,c in all_triples:
        factor = 2
        while a*factor < 2*M and b*factor<2*M:
            scaled_triples.append((a*factor, b*factor, c*factor))
            factor +=1
    all_triples += scaled_triples
            
def program():
    triples(10000)
    scale_triples(M)

    count = 0
    for x,y,z in all_triples:

        alpha,beta = (x,y)
        delta = z
        if beta > M:
            continue
        comp1 = count_valid_compositions(alpha, beta, delta, M)

        count += len(comp1)


    for x,y,z in all_triples:

        alpha,beta = (y,x)
        delta = z
        if beta > M:
            continue
        comp1 = count_valid_compositions(alpha, beta, delta, M)

        count += len(comp1)
 
    return count

def main():
    global M
    global all_triples

    low = 100
    high = 3000
    current = ((high-low)/2)+low
    while True:
        all_triples = []
        M = current
        result = program()
        print(current, result)
        if result > 1000000:
            high = current
        else:
            low = current
        if abs(high-low)<=1:
            print(current)
            break
        current = ((high-low)//2)+low


main()
