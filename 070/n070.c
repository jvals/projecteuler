#include <stdint.h>
#include <stdio.h>
#include <stdbool.h>

#define LIMIT 10000000

struct Compare { double val; uint64_t index; };
#pragma omp declare reduction(minimum : struct Compare : omp_out = omp_in.val < omp_out.val ? omp_in : omp_out)


uint64_t phi(uint64_t n) {
  uint64_t result = n;

  for (uint64_t p = 2; p*p <= n; ++p) {
    if (n % p == 0) {
      while (n % p == 0) {
        n /= p;
      }
      result -= result / p;
    }
  }

  if (n > 1) {
    result -= result / n;
  }

  return result;
}

bool permuation(uint64_t n, uint64_t candidate) {
  // https://stackoverflow.com/questions/3219112/checking-whether-two-numbers-are-permutation-of-each-other
  /* Each digit 0-9 should appear an equal number of times in both n and candidate 
   for them to be permuations of each other */
  
  int digits[10] = {0};

  while(n != 0) {
    digits[n%10]++;
    n /= 10;
  }

  while(candidate != 0) {
    digits[candidate%10]--;
    candidate /= 10;
  }

  for (int i = 0; i < 10; ++i) {
    if (digits[i] != 0) {
      return false;
    }
  }

  return true;
}


int main()
{
  uint64_t minimum_n = 2;
  double minimum_ratio = 40000;

  for (uint64_t n = 2; n < LIMIT; ++n) {
    /* printf("n=%lld\r", n); */
    uint64_t candidate = phi(n);
    if (permuation(n, candidate)) {
      if (n/(double)candidate < minimum_ratio) {
        minimum_n = n;
        minimum_ratio = n/(double)candidate;
      }
    }
  }

  printf("Minimum ratio: %f for n=%lld\n", minimum_ratio, minimum_n);

  return 0;
}

/* int main() */
/* { */
/*   struct Compare min; */
  
/*   min.val = 2; */
/*   min.index = 40000; */

/* #pragma omp parallel for reduction(minimum:min) */
/*   for (uint64_t n = 2; n < LIMIT; ++n) { */
/*     /\* printf("n=%lld\r", n); *\/ */
/*     uint64_t candidate = phi(n); */
/*     if (permuation(n, candidate)) { */
/*       if (n/(double)candidate < min.val) { */
/*         min.index = n; */
/*         min.val = n/(double)candidate; */
/*       } */
/*     } */
/*   } */

/*   printf("Minimum ratio: %f for n=%lld\n", min.val, min.index); */

/*   return 0; */
/* } */
