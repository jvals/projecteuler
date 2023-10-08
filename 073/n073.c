#include <stdio.h>
#include <inttypes.h>

#define DLIMIT 12000

/* Depth first recursion search through a ternary tree, which nodes are coprimes */
uint64_t generate_coprimes(int m, int n) {
  if (m > DLIMIT) {
    return 0;
  }

  uint64_t total = 0;
  double fraction = n/(double)m;
  if (fraction > 1.0/3.0 && fraction < 1.0/2.0) {
    total = 1;
  }

  /* printf("%d/%d\n", n, m);  */
  
  uint64_t b1, b2, b3;
  b1 = generate_coprimes(2*m - n, m);
  b2 = generate_coprimes(2*m + n, m);
  b3 = generate_coprimes(m + 2*n, n);

  total += (b1 + b2 + b3);
  return total;
}

int main()
{
  uint64_t total = 0;
  total += generate_coprimes(2, 1); // even-odd and odd-even
  total += generate_coprimes(3, 1); // odd-odd

  printf("%"PRIu64"\n", total);
  return 0;
}


