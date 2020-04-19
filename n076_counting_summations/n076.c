#include <inttypes.h>
#include <stdio.h>

typedef uint64_t int_t;

int_t partition(int_t k, int_t n) {
  // printf("%"PRIu64", %"PRIu64"\n", k, n);
  if (k > n) {
    return 0;
  } else if (k == n) {
    return 1;
  } else if (k == 1) {
    return 1;
  } else {
    return partition(k-1, n-1) + partition(k, n-k);
  }
}

int main()
{
  // result is the number of ways 'one hundred' can be written as a
  // sum of at least two positive integers
  int_t result = 0;
  for (int_t i = 2; i < 150; ++i) {
    result += partition(i, 100);
  }
  printf("%"PRIu64"\n", result);
  

  return 0;
}
