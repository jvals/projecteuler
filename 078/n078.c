#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define P(k, n) ((n)+ARRAYSIZE*(k))
#define LIMIT 60000L
#define MAXIMUM 100000

typedef uint64_t int_t;

int_t partition(int_t n) {
  if(n==0 || n==1) {
    return 1;
  }

  int p = 0;

  int m1 = 1;
  int m2 = 1;
  int i = 0;

  while (m1 >= 0 && m2 >=0) {
    i++;
    m1 = n - i * (3 * i - 1) / 2;
    m2 = n - i * (3 * i + 1) / 2;
    int s = 1;
    if (i % 2 == 0) {
      s = -1;
    }
    if (m1 >= 0) {
      p += s * partition(m1);
    }
    if (m2 >= 0) {
      p += s * partition(m2);
    }
  }
  return p;

}

int main()
{
  for (int n = 0; n < 100000; ++n) {
    int_t candidate = partition(n);
    printf("%d %"PRIu64"\n", n, candidate);
    if (candidate % 1000000 == 0) {
      printf("ANSWER: %d\n", n);
      break;
    }
  }

  return 0;
}
