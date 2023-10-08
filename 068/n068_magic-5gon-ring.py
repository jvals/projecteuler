from itertools import permutations


def checkLineSumsShort(A, B, C, D, E, F):
    line_sums = [A + B + C, D + C + E, F + D + B]
    return len(set(line_sums)) == 1


def checkLineSums(A, B, C, D, E, F, G, H, I, J):
    line_sums = [A + B + C, D + C + E, F + E + G, H + G + I, J + I + B]
    return len(set(line_sums)) == 1


def findStartShort(lines):
    start_line_idx = 0
    minimal_vertex = 10**9
    magic_pattern = (0, 2, 0)
    for i, line in enumerate(lines):
        magic_idx = magic_pattern[i]
        if line[magic_idx] < minimal_vertex:
            minimal_vertex = line[magic_idx]
            start_line_idx = i
    return start_line_idx


def concatTuple(numeric_tuple):
    concatTuple = ""
    for number in numeric_tuple:
        concatTuple += str(number)
    return concatTuple


def verifyStructureShort(A, E, F):
    if A > E or A > F:
        return False
    return True


def verifyStructure(A, D, F, H, J):
    if A > D or A > F or A > H or A > J:
        return False
    return True


def main():
    final_number = 0
    # Go through all permutations of 1..10
    for permutation in permutations([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]):
        A, B, C, D, E, F, G, H, I, J = permutation
        # Verify that all lines have equal sums
        if not checkLineSums(A, B, C, D, E, F, G, H, I, J):
            continue
        # Verify that A is the smallest of the outer vertices
        if not verifyStructure(A, D, F, H, J):
            continue

        # Possible solution found, concatinate the numbers
        lines = ((A, B, C), (D, C, E), (F, E, G), (H, G, I), (J, I, B))
        concat_number = ""
        t = 0
        while t < len(lines):
            line = concatTuple(lines[t])
            concat_number += line
            t += 1

        # The solution has 16 digits, as specified by the problem
        if len(concat_number) == 16 and int(concat_number) > final_number:
            final_number = int(concat_number)

    print(final_number)


if __name__ == '__main__':
    main()
