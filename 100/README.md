# Arranged Probability

## Problem Description

If a box contains twenty-one coloured discs, composed of fifteen blue discs and six red discs, and two discs were taken at random, it can be seen that the probability of taking two blue discs, $P(\text{BB}) = (15/21) \times (14/20) = 1/2$.

The next such arrangement, for which there is exactly $50\%$ chance of taking two blue discs at random, is a box containing eighty-five blue discs and thirty-five red discs.

By finding the first arrangement to contain over $10^{12} = 1\,000\,000\,000\,000$ discs in total, determine the number of blue discs that the box would contain.

## Solution steps
The probability of taking two blue discs is $\frac{1}{2}$, so the number of blue discs must be a solution to the equations: $\frac{x}{y} * \frac{x-1}{y-1} = \frac{1}{2}$ and $y \geq 10^{12}$, where $x$ is the number of blue discs and $y$ is the total number of discs.

The equation can be rewritten as: $2x^2 - 2x - y^2 + y = 0$

Solving for $x$, we get $x = \frac{1 + \sqrt{2y^2 - 2y + 1}}{2}$. At this point, we could start iterating over $y$ and checking if $x$ is an integer. However, the search space is too large. We need to find a way to reduce the search space.

Since $x$ is an integer, $2y^2 - 2y + 1$ must be a perfect square.

Let $n^2 = 2y^2 - 2y + 1$. Solving for $y$, we get $y = \frac{1 + \sqrt{2n^2 - 1}}{2}$. Since $y$ is an integer, $2n^2 - 1$ must be a perfect square.

Let $m^2 = 2n^2 - 1$. Rearranging, we get $m^2-2n^2 = -1$. This is a negative Pell equation.

According to https://en.wikipedia.org/wiki/Pell%27s_equation#The_negative_Pell's_equation, the fundamental solutions can be found by iterating over the continued fraction of $\sqrt{2}$. We use $\sqrt{2}$ because $2$ is the coefficient of $n^2$ in the equation.

The continued fraction of $\sqrt{2}$ is $[1; 2, 2, 2, 2, ...]$.

The first convergent is $1 = \frac{1}{1}$

The second convergent is $1 + \frac{1}{2} = \frac{3}{2}$

The third convergent is $1 + \frac{1}{2 + \frac{1}{2}} = \frac{7}{5}$
From the link above, we find the recurrence relations for the negative Pell equation are:

$$m_k = m_{k-2} * m_0^2 + 2 * m_{k-2} * n_0^2 + 4 * n_{k-2} * n_0 * m_0$$ 

and

$$n_k = n_{k-2} * m_0^2 + 2 * n_{k-2} * n_0^2 + 2 * m_{k-2} * n_0 * m_0$$

where $m_k$ is the kth iteration of $m$, $m_{k-1}$ is the kth iteration minus one and $m_{k-2}$ is the kth iteration minus two. The same applies for $n_k$. 

The initial values are $m_0 = 1, m_1 = 3, m_2 = 7, n_0 = 1, n_1 = 2, n_2 = 5$.

We iterate over the recurrence relations. For each $n$, we compute $y$ by plugging $n$ into the equation $y = \frac{1 + \sqrt{2n^2 - 1}}{2}$.

If $y$ is an integer, we compute $x$ by plugging $y$ into the equation $x = \frac{1 + \sqrt{2y^2 - 2y + 1}}{2}$.

If $x$ is an integer, we check if $y$ is greater than $10^{12}$. If it is, we have found our solution. We print $x$ and break out of the loop.

Steps 9 to 12 are implemented in n100_arranged_probability.py
