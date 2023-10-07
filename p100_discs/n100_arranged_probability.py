from math import sqrt

M1 = 1
N1 = 1
MAX_Y = 10 ** 12


def y(n):
    """Solves the following equation for y: (x/y) * ((x-1)/(y-1)) = 1/2."""
    return (1 + sqrt(2 * n ** 2 - 1)) / 2


def x(y):
    """Solves the following equation for x: (x/y) * ((x-1)/(y-1)) = 1/2."""
    return (1 + sqrt(2 * y ** 2 - 2 * y + 1)) / 2


def next_m(m_k_2, n_k_2):
    """Calculates the next m using the recurrence relation of the negative Pell equation: m^2-2n^2=-1."""
    return m_k_2 * M1 ** 2 + 2 * m_k_2 * N1 ** 2 + 4 * n_k_2 * N1 * M1


def next_n(m_k_2, n_k_2):
    """Calculates the next n using the recurrence relation of the negative Pell equation: m^2-2n^2=-1."""
    return n_k_2 * M1 ** 2 + 2 * n_k_2 * N1 ** 2 + 2 * m_k_2 * N1 * M1


def main():
    # Initial values for m and n sequences
    # m_k is the kth iteration of m, m_k_1 is the kth iteration minus one,
    # and m_k_2 is the kth iteration minus two.
    # The same pattern applies to n.
    m_k, n_k = 7, 5
    m_k_1, n_k_1 = 3, 2
    m_k_2, n_k_2 = 1, 1

    while True:
        y_n = y(n_k)
        x_n = x(y_n)

        print(f'y={y_n}, x={x_n}')

        if y_n > MAX_Y:
            break

        temp_m_k = next_m(m_k_2, n_k_2)
        temp_n_k = next_n(m_k_2, n_k_2)
        m_k_2, n_k_2 = m_k_1, n_k_1
        m_k_1, n_k_1 = m_k, n_k
        m_k, n_k = temp_m_k, temp_n_k


if __name__ == '__main__':
    main()
