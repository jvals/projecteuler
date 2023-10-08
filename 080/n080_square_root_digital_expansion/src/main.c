#include <stdio.h>
#include <math.h>
#include "n080.h"

int main()
{
  int result = 0;
  for (int i = 2; i < 100; ++i) {
    if (sqrt((double)i) != floor(sqrt((double)i))) {
      result += digital_sum_of_root(i);
    }
  }

  printf("%d\n", result);
  return 0;
}
