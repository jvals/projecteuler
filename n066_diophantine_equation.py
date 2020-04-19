'''
https://en.wikipedia.org/wiki/Pell%27s_equation

Consider quadratic Diophantine equations of the form:

x2 – Dy2 = 1

For example, when D=13, the minimal solution in x is 6492 – 13×1802 =
1.

It can be assumed that there are no solutions in positive integers
when D is square.

By finding minimal solutions in x for D = {2, 3, 5, 6, 7}, we obtain
the following:

3^2 – 2×22 = 1
2^2 – 3×12 = 1
9^2 – 5×42 = 1
5^2 – 6×22 = 1
8^2 – 7×32 = 1

Hence, by considering minimal solutions in x for D ≤ 7, the largest x
is obtained when D=5.

Find the value of D ≤ 1000 in minimal solutions of x for which the
largest value of x is obtained.
'''

from math import sqrt, floor
import sys
from decimal import *

# def cofrac(r, limit, a=[]):
#     i = floor(r)
#     a.append(i)

#     if(limit == 0):
#         return a

#     f = r-i
#     return cofrac(1/f, limit-1, a)

# memo_co = {}

def cofrac_iter(r, n):
    # if (r,n) in memo_co:
    #     return memo_co[(r, n)]

    # rational = r
    i = 0
    for t in range(n+1):
        i = floor(r)
        # memo_co[(rational, t)] = i
        f = Decimal(r)-Decimal(i)
        r = Decimal(1)/Decimal(f)
    return i

def a(r, n):
    return cofrac_iter(r, n)

# memo_h = {}

# def h(r, n):
#     if (r,n) in memo_h:
#         return memo_h[(r, n)]

#     if n == -1:
#         return 1
#     if n == -2:
#         return 0

#     # return a(r, n)*h(r, n-1) + h(r, n-2)
#     memo_h[(r,n)] = a(r, n)*h(r, n-1) + h(r, n-2)
#     return memo_h[(r,n)]

def h_iter(r, n):
    u = 0
    v = 1
    for i in range(n+1):
        u, v = v, (u + v * a(r, i))

    return v


# memo_k = {}

# def k(r, n):
#     if (r,n) in memo_k:
#         return memo_k[(r, n)]

#     if n == -1:
#         return 0
#     if n == -2:
#         return 1

#     # return a(r, n)*k(r, n-1)+k(r, n-2)
#     memo_k[(r,n)] = a(r, n)*k(r, n-1)+k(r, n-2)
#     return memo_k[(r,n)]

def k_iter(r, n):
    u = 1
    v = 0
    for i in range(n+1):
        u, v = v, (u + v * a(r, i))

    return v

# print()
# print(cofrac(sqrt(19), 20))
# print(cofrac_iter(sqrt(19), 20))
# print(memo)

# print(k(sqrt(7), 3))
def diophantine_eq(D):
    rhs = 0
    n = 0
    x = 0
    y = 0
    while(rhs != 1):
        x = h_iter(Decimal(D).sqrt(), n)
        y = k_iter(Decimal(D).sqrt(), n)
        rhs = x*x - D * y*y
        n += 1
        # if n > 1000:
        #     return -1

    return x

def main():
    large_x = 0
    large_D = 0
    for D in range(1,1001):
        if sqrt(D) == int(sqrt(D)):
            continue
        # x = diophantine_eq(D)
        # print(D, end=" ")
        sys.stdout.flush()
        if x > large_x:
            large_x = x
            large_D = D
        # memo_co = {}
        # memo_h = {}
        # memo_k = {}
    print("\nResult ",large_x, large_D)


main()
