#include <stdio.h>
#include <inttypes.h>
#include <stdbool.h>

typedef uint64_t int_t;

int_t factorial(int_t x) {
  int_t result = 1;
  for (int i = 2; i <= x; ++i) {
    result *= i;
  }

  return result;
}

int_t digit_factorial_sum(int_t x) {
  int_t digit_sum = 0;
  int_t digit = x;
  while(digit != 0) {
    digit_sum += factorial(digit%10);
    digit /= 10;
  }

  return digit_sum;
}

bool inArray(int_t value, int_t* array) {
  for (int_t i = 0; i < 60; ++i) {
    if (array[i] == value) {
      return true;
    }
  }

  return false;
}

bool chain60(int_t starting_number) {
  int_t chain_length = 0;
  int_t chain[60] = {0};
  int_t next = starting_number;
  while(!inArray(next, chain)) {
    chain[chain_length] = next;
    int_t current = next;
    next = digit_factorial_sum(current);
    chain_length++;
    if (chain_length > 60) {
      return false;
    }
  }
  
  return chain_length == 60;
}

int main()
{
  // Number of chains containing sixty non-repeating terms
  int_t chains = 0;
  // For every starting number
  for (int_t i = 12; i < 1000000; ++i) {
    if (chain60(i))
      chains++;
  }

  printf("%"PRIu64"\n", chains);

  return 0;
}
