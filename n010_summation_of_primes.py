'''
The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.

'''

from math import fsum

def count_primes(n):
    prime_list = []
    for i in range(n):
        if(prime(i)):
            prime_list.append(i)
    return fsum(prime_list)

def prime(n):
    '''check if integer n is a prime'''

    # make sure n is a positive integer
    n = abs(int(n))

    # 0 and 1 are not primes                                                                                                                                                                     
    if n < 2:
        return False

    # 2 is the only even prime number
    if n == 2:
        return True

    # all other even numbers are not primes
    if not n & 1:
        return False

    # range starts with 3 and only needs to go up the squareroot of n for all odd numbers
    for x in range(3, int(n**0.5)+1, 2):
        if n % x == 0:
            return False
    return True


print(count_primes(10))
print(count_primes(2*10**6))
