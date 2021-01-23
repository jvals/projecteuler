# slow, but finds the answer under 1m

from functools import reduce
from copy import deepcopy

def prodOfTerms(terms):
    return reduce((lambda x, y: x * y), terms)

def prodIsSum(terms):
    return sum(terms) == prodOfTerms(terms)

def twoTerms(limit, solutions):
    for x0 in range(2, limit+1):
        for x1 in range(2, x0+1):
            productOfTerms = x0 * x1
            sumOfTerms = x0 + x1
            k = 2 + (productOfTerms - sumOfTerms)
            # if k == 5:
            #     print(x0, x1)
            if k in solutions:
                solutions[k] = min(solutions[k], productOfTerms)
            else:
                solutions[k] = productOfTerms

def nTerms(terms, paramIndex, paramLimit, solutions, lim):
#    print("terms: ", terms)
    productOfTerms = prodOfTerms(terms)
    sumOfTerms = sum(terms)
    if productOfTerms > 2*lim:
        return True
    k = len(terms) + (productOfTerms - sumOfTerms)
    if k in solutions:
        solutions[k] = min(solutions[k], productOfTerms)
    else:
        solutions[k] = productOfTerms


    if paramIndex < len(terms):
        while terms[paramIndex] < paramLimit:
            terms[paramIndex] += 1
            if nTerms(deepcopy(terms), paramIndex + 1, paramLimit, solutions, lim):
                break


def n088(lim):
    solutions = {}
    for k in range(2,17):
        nTerms([2]*k, 0, lim/2, solutions, lim)

    results = set()
    for i in range(2,lim):
        results.add(solutions[i])
    print(sum(results))


def main():
    n088(12001)

main()
