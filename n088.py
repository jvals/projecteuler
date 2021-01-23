# too slow for large numbers

from functools import reduce
from copy import deepcopy

def prodOfTerms(terms):
    return reduce((lambda x, y: x * y), terms)

def prodIsSum(terms):
    return sum(terms) == prodOfTerms(terms)


def iterateMinProdSum(terms, k, candidates, param=0):
    if prodIsSum(terms):
        candidates.append(sum(terms))

    while param < k and terms[param] < k and prodOfTerms(terms) < 2*k and 2**(param-1)<k and 2**param - param <= k:
        terms[param] += 1
        iterateMinProdSum(deepcopy(terms), k, candidates, param + 1)

    if candidates:
        return min(candidates)
    return 0

def minProdSumNum(k):
    terms = [1]*k

    # iterate
    return iterateMinProdSum(terms, k, [], 0)

def n088(lim):
    results = set()
    for k in range(2, lim+1):
        results.add(minProdSumNum(k))
    print(sum(results))

def main():
    n088(6)
    n088(12)
    n088(12001)

main()
