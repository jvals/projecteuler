'''
http://web.math.princeton.edu/mathlab/jr02fall/Periodicity/mariusjp.pdf
'''
from math import floor, sqrt

def period_length(n):
    a0 = floor(sqrt(n))
    b = a0
    b0 = a0
    c = n-a0**2
    c0 = c
    result = 0
    
    while True:
        a = floor((a0 + b)/c)
        b = a*c-b
        c = (n-b**2)/c
        result += 1
        if (b==b0 and c==c0):
            break
    return result

t = 0
for i in range(2,int(1e4)):
    try:
        p = period_length(i)
        if p%2==1:
            t+=1
    except:
        pass
print(t)
