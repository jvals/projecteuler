import sympy as sy
import sys

sys.setrecursionlimit(1500)


def main():
    number_of_fractions = 0
    for i in range(0, 1001):
        exp = get_expansion(i)
        expansions[i] = exp
        fraction = sy.nsimplify(1 + exp, tolerance=1000)
        # print(i, fraction)
        numerator, denominator = str(fraction).split('/')
        if len(numerator) > len(denominator):
            number_of_fractions += 1
    print(number_of_fractions)

expansions = {}


def get_expansion(index):
    if index == 0:
        return sy.nsimplify(1/2, tolerance=1000)
    elif index in expansions:
        return expansions[index]
    else:
        return sy.nsimplify(1/(2+get_expansion(index-1)), tolerance=1000)

main()
