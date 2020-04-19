#2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without remainder. What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
#By Jv

from fractions import gcd


def lcm(a,b):
    return (a*b)/gcd(a,b)

def findMultiple(max):
    n=1
    for i in range(1,max+1):
        n = lcm(n,i)
    return n

print findMultiple(20)
