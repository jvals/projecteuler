# -*- coding: utf-8 -*-
'''
Problem #6
The sum of the squares of the first ten natural numbers is,
12 + 22 + ... + 102 = 385

The square of the sum of the first ten natural numbers is,
(1 + 2 + ... + 10)2 = 552 = 3025

Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
'''
#Working implementation by JV

def sum_of_squares(n):
    return (n*(n+1)*(2*n+1))/6

def sum_of_integers(n):
    return (n*(n+1))/2


def square_sum_of_integers(n):
    return (sum_of_integers(n))**2

def difference(n):
    return square_sum_of_integers(n)-sum_of_squares(n)

#print sum_of_squares(10)
#print square_sum_of_integers(10)
#print difference(10)
 
print "Final answer: ",difference(100)


