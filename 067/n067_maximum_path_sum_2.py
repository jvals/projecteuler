from sys import stdin


def main():
    pyramid = []

    # Read input
    for line in stdin:
        row = line.strip().split(' ')
        row = list(map(int, row))
        pyramid.append(row)

    # Go through the pyramid, starting from the second to last row
    start_height = len(pyramid) - 2
    end_height = -1  # The loop includes the root node
    for i in range(start_height, end_height, -1):
        for j in range(len(pyramid[i])):
            child_height = i + 1
            left_idx = j
            right_idx = left_idx + 1
            left_child = pyramid[child_height][left_idx]
            right_child = pyramid[child_height][right_idx]
            pyramid[i][j] += max(left_child, right_child)

    # Print the maximum path sum
    print(pyramid[0][0])


if __name__ == '__main__':
    main()
