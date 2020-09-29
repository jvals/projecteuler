from math import sqrt, isclose

def S1(A, B, C):
    return sqrt(A**2 + (B+C)**2)
    
def S2(A, B, C):
    return sqrt(B**2 + (A+C)**2)
    
def S3(A, B, C):
    return sqrt(C**2 + (B+A)**2)
#low  1817
#high 1818
M = 1818
solutions = set()
for i in range(1, M+1):
    print("Step: ", i, " count: ", len(solutions))
    for j in range(i, M+1):
        for k in range(j, M+1):
            if ((i,j,k) in solutions):
                continue
            shortest = S3(i,j,k)
            if isclose(int(shortest), shortest):
                #print(i+j,k,int(shortest))
                solutions.add((i,j,k))
                factor = 2
                while k*factor < M:
                    solutions.add(
                        (
                            i * factor,
                            j * factor,
                            k * factor
                            )
                        )
                    factor += 1
                    
                
print(len(solutions))
