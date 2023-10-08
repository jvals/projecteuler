#include <stdio.h>
#include <gmp.h>

#define MAX 1000000

struct Fraction {
  int a;
  int b;
  double f;
};

/* int gcd(int a, int b)
{
    while(b) b ^= a ^= b ^= a %= b;
    return a;
} */

int gcd(int a, int b)
{
  mpz_t x, y, z;
  mpz_set_ui(x, a);
  mpz_set_ui(y, b);
  
  mpz_gcd(z, x, y);
  return mpz_get_ui(z);
  /* while (a != b) { */
  /*   if (a > b) { */
  /*     a = a - b; */
  /*   } else { */
  /*     b = b - a; */
  /*   } */
  /* } */
  /* return a; */
}

int gcd_binary_2(int a, int b)
{
  /* https://hbfs.wordpress.com/2013/12/10/the-speed-of-gcd/ */
  if (a == 0) return b;
  if (b == 0) return a;
  
  int shift=__builtin_ctz(a|b);
  a>>=__builtin_ctz(a);
  
  do
   {
    b>>=__builtin_ctz(b);
  
    if (a>b){
      a ^= b;
      b ^= a;
      a ^= b;
      /* std::swap(a,b); */
    }
    b-=a;
   } while (b);
 
  return a << shift;
}

int main()
{
  struct Fraction best;
  best.a = 1;
  best.b = 1;
  best.f = 0;

/* #pragma omp parallel for schedule(guided) */
  for (int d = 1; d <= MAX; ++d) {
    printf("%d\r", d);
    for (int n = 1; n < d; ++n) {
      if (n/(double)d >= 3.0/7.0) {
        break;
      }
      if (n/(double)d > best.f) {
        /* #pragma omp critical */
        /* { */
        /* if (gcd_binary_2(n, d) == 1) { */
          best.a = n;
          best.b = d;
          best.f = n/(double)d;
        /* } */
        /* } */
      }
      
    }
  }
  printf("\n%d/%d\n", best.a, best.b);

  return 0;
}
