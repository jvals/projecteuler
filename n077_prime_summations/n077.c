#include <inttypes.h>
#include <stdio.h>

#define LIMIT 300

typedef uint64_t int_t;

int_t primes[LIMIT] = {0};

void fill_primes() {
  for (int_t i = 2; i < LIMIT; ++i) {
    if (primes[i] == 0) {
      for (int_t j = i; (i*j)<LIMIT; ++j) {
        primes[(i*j)] = 1;
      }
    }
  }
}

int_t sopf(int_t n) {
  // Sum of primes
  int_t result = 0;
  for (int_t i = 0; i <= n; ++i) {
    if (primes[i] == 0 && (n%i)==0) {
      result += i;
    }
  }

  return result;

  
}

// int_t k(int_t n) {
//   if (n == 1) {
//     return 0;
//   }

//   int_t a = sopf(n);
//   int_t b = 0;
//   for (int_t j = 1; j < n; ++j) {
//     b += sopf(j) * k(n - j);
//   }

//   return (a+b)/n;

// }

int_t foo(int_t n, int_t k, int_t t) {
  if (k==n) {
    return 1;
  }
  if (k>n  || t==LIMIT) {
    return 0;
  }

  int_t total = 0;
  for (int_t i = t; i < LIMIT; ++i) {
    if (primes[i] == 0) {
      total += foo(n, k+i, i);
    }
  }

  return total;
}


int main()
{
  fill_primes();
  primes[0] = 1;
  primes[1] = 1;

  // for (int_t i = 1; i < 40; ++i) {
  //   printf("%"PRIu64"\n", sopf(i));
  // }

  // for (int_t i = 0; i < LIMIT; ++i) {
  //   if (primes[i] == 0) {
  //     printf("%"PRIu64"\n", i);
  //   }
  // }
  
  for (int_t i = 1; i < LIMIT; ++i) {
    // printf("%"PRIu64"\n", i);
    // int x = k(i);
    int_t x = foo(i, 0, 0);
    // printf("%"PRIu64" %"PRIu64"\n", i, x);
    if (x >= 5000) {
      printf("ANSWER: %"PRIu64"\n", i);
      break;
    }
  }

  
  return 0;
}

