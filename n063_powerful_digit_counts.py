def number_of_digits(d):
    return len(str(d))

def power_check(d):
    n = 1
    print()
    print('d\tn\td**n\tlen')
    while(n == number_of_digits(d**n)):
        print(d,n,d**n,number_of_digits(d**n),sep='\t')
        n += 1

    return(n-1)
        
def main():
    total = 0
    for i in range(1,10):
        total += power_check(i)
    print('Total: ', total)

main()

'''
Example of numbers that fit the description

d	n	d**n	len
1	1	1	1

2	1	2	1

3	1	3	1

4	1	4	1
4	2	16	2

5	1	5	1
5	2	25	2
5	3	125	3

'''
