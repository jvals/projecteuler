def move_right(x, y):
    return x+1, y


def move_down(x, y):
    return x, y-1


def move_left(x, y):
    return x-1, y


def move_up(x, y):
    return x, y+1


moves = [move_right, move_up, move_left, move_down]


def gen_points(end):
    from itertools import cycle
    _moves = cycle(moves)
    n = 1
    pos = 0, 0
    times_to_move = 1

    # yield (n, pos)

    while True:
        for _ in range(2):
            move = next(_moves)
            for _ in range(times_to_move):
                if n >= end:
                    return
                pos = move(*pos)
                n += 1
                if abs(pos[0]) == abs(pos[1]):
                    yield (n, pos)
                else:
                    continue

        times_to_move += 1

def isprime(n):
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
    # range starts with 3 and only needs to go up the squareroot of n
    # for all odd numbers
    for x in range(3, int(n**0.5)+1, 2):
        if n % x == 0:
            return False
    return True

def main():
    spiral = gen_points(10**10)
    diagonal_non_primes = 0
    diagonal_primes = 0
    counter = 0
    # v = []
    for value in spiral:
        # print(value)
        # v.append(value)
        if isprime(value[0]):
            diagonal_primes += 1
        else:
            diagonal_non_primes += 1
        counter += 1
        if counter % 4 == 0:
            # print(v)
            a = diagonal_primes/(diagonal_primes+diagonal_non_primes+1)
            print(counter/4*2+1, a)
            if float("{:.2f}".format(a)) < 0.1:
                print(counter/4*2+1)
                break


main()
