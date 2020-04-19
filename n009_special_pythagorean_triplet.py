'''


A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
a^2 + b^2 = c^2

For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.

'''

def main():
    for a in range(1, 1000):
        #a<b<c, so b should be in range (a,1000)
        for b in range(a, 1000):
            #Looking for a + b + c = 1000
            c = 1000 - a - b
            if c > 0:
                # a^2 + b^2 = c^2
                if c**2 == a**2 + b**2:
                    print a*b*c
                    break


main()    
