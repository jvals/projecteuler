# Program that sums even values in the Fibonacci sequence up to 4M
# Working implementation by JV

max = 4*10**6


old = 1
new = 2

def main():
    global old
    global new

    s = 0
    nextOld = 0
    while(new < max):
        if(iseven(new)):
            s+=new
        nextOld = new
        new = old + new
        old = nextOld
#        print(new, end='\t')
    return s

def iseven(n):
    if(n%2 == 0):
        return True
    else:
        return False

main()
