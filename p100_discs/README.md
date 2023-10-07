# Arranged Probability

## Problem Description

If a box contains twenty-one coloured discs, composed of fifteen blue discs and six red discs, and two discs were taken at random, it can be seen that the probability of taking two blue discs, P(BB) = (15/21) * (14/20) = 1/2.

The next such arrangement, for which there is exactly 50% chance of taking two blue discs at random, is a box containing eighty-five blue discs and thirty-five red discs.

By finding the first arrangement to contain over 10^12 = 1,000,000,000,000 discs in total, determine the number of blue discs that the box would contain.

## Solution steps
1. The probability of taking two blue discs is 1/2, so the number of blue discs must be a solution to the equations: ((x/y) * ((x-1)/(y-1))) = 1/2 and y>=10^12, where x is the number of blue discs and y is the total number of discs.
2. The equation can be rewritten as: 2x^2 - 2x - y^2 + y = 0
3. Solving for x, we get x = (1 + sqrt(2y^2 - 2y + 1)) / 2. At this point, we could start iterating over y and checking if x is an integer. However, the search space is too large. We need to find a way to reduce the search space.
4. Since x is an integer, 2y^2 - 2y + 1 must be a perfect square. 
5. Let n^2 = 2y^2 - 2y + 1. Solving for y, we get y = (1 + sqrt(2n^2 - 1)) / 2. Since y is an integer, 2n^2 - 1 must be a perfect square.
6. Let m^2 = 2n^2 - 1. Rearranging, we get m^2-2n^2 = -1. This is a negative Pell equation.
7. According to https://en.wikipedia.org/wiki/Pell%27s_equation#The_negative_Pell's_equation, the fundamental solutions can be found by iterating over the continued fraction of sqrt(2). We use sqrt(2) because 2 is the coefficient of n^2 in the equation.
8. The continued fraction of sqrt(2) is [1; 2, 2, 2, 2, ...].
The first convergent is 1 = 1/1
The second convergent is 1 + 1/2 = 3/2. 
The third convergent is 1 + 1/(2 + 1/2) = 7/5. 
9. From the link above, we find the recurrence relations for the negative Pell equation are:
m_k = m_k_2 * M1 ** 2 + 2 * m_k_2 * N1 ** 2 + 4 * n_k_2 * N1 * M1
and
n_k = n_k_2 * M1 ** 2 + 2 * n_k_2 * N1 ** 2 + 2 * m_k_2 * N1 * M1
where m_k is the kth iteration of m, m_k_1 is the kth iteration minus one and m_k_2 is the kth iteration minus two. The same applies for n_k. M1 and N1 are the first convergent of the continued fraction of sqrt(2). In this case, M1=1 and N1=1.
10. The initial values are m_0 = 1, m_1 = 3, m_2 = 7, n_0 = 0, n_1 = 1, n_2 = 5.
11. We iterate over the recurrence relations. For each n, we compute y by plugging n into the equation y = (1 + sqrt(2n^2 - 1)) / 2. 
12. If y is an integer, we compute x by plugging y into the equation x = (1 + sqrt(2y^2 - 2y + 1)) / 2. 
13. If x is an integer, we check if y is greater than 10^12. If it is, we have found our solution. We print x and break out of the loop.

Steps 9 to 12 are implemented in n100_arranged_probability.py
