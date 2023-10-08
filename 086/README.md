This was a fun problem to solve!

The first major insight for solving this problem is to unfold the cuboid into the "net of a cuboid".
On this 2D surface we can find the shortest path by the Pythagorean theorem.

Through trial, error and guesswork we can find the three candidate paths described in the problem.

S_1² = A² + (B+C)²

S_2² = (A+C)² + B²

S_3² = (A+B)² + (C)²


Because we are ignoring rotations, we can say that A <= B <= C. Therefore, S_3 will always be the shortest path.

With this information we arrive at the solution in cuboids.py. The code works for small M like 99 and 100, but it is very slow.
I did a manual binary search by running cuboids.py and adjusting M until I found the answer, but this process took hours.

The second major insight comes from the fact that all sides must be integers, including the shortest path. Pythagorean triples has this property!
In cuboids2.py, we generate a bunch of pythagorean triples and count them to find the answer for a given M. There is additional code for decomposing the terms of the triples into side lengths (and checking them against M).

cuboids2.py finds the answer fairly quick, but do note that the code in cuboids2.py is ugly and makes use of globals.

There are much quicker, more elegant solutions out there!
