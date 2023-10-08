// http://rosettacode.org/wiki/Pythagorean_triples#C

#include <inttypes.h>
#include <stdlib.h>
#include <stdio.h>

#define L 1500000

typedef uint64_t int_t;

int_t* wires;

int_t U[][9] =  {{-1,  2, 2, -2,  1, 2, -2,  2, 3},
                 { 1,  2, 2,  2,  1, 2,  2,  2, 3},
                 { 1, -2, 2,  2, -1, 2,  2, -2, 3}};

void vector_add(int_t* triple1, int_t* triple2) {
  for (int_t i = 0; i < 3; ++i) {
    triple1[i] += triple2[i];
  }
}

void generate_triples(int_t triple[3]) {
  if (triple[0] == 8)
    printf("%"PRIu64", %"PRIu64", %"PRIu64"\n", triple[0], triple[1], triple[2]);
  int_t length = triple[0] + triple[1] + triple[2];

  if (length > L) return;

  wires[length]++;

  // Add all multiples of triple
  // Copy of triple which will be multiplied
  int_t triple_x[3];
  for (int_t i = 0; i < 3; ++i) {
    triple_x[i] = triple[i];
  }

  vector_add(triple_x, triple);
  length = triple_x[0] + triple_x[1] + triple_x[2];
  while(length < L) {
    // printf("%"PRIu64", %"PRIu64", %"PRIu64"\n", triple_x[0], triple_x[1], triple_x[2]);
    wires[length]++;
    vector_add(triple_x, triple);
    length = triple_x[0] + triple_x[1] + triple_x[2];
  }

  // Generate next tier
  int_t next_triple[3] = {0};
  for (int_t i = 0; i < 3; ++i) {
    next_triple[0] = U[i][0] * triple[0] + U[i][1] * triple[1] + U[i][2] * triple[2];
    next_triple[1] = U[i][3] * triple[0] + U[i][4] * triple[1] + U[i][5] * triple[2];
    next_triple[2] = U[i][6] * triple[0] + U[i][7] * triple[1] + U[i][8] * triple[2];
    generate_triples(next_triple);
  }
}

int main()
{
  wires = (int_t*)calloc(L, sizeof(int_t));
  int_t seed[3] = {3, 4, 5};

  generate_triples(seed);

  int_t sum = 0;
  for (int_t i = 0; i < L; ++i) {
    // printf("%d  %"PRIu64"\n", i, wires[i]);
    if (wires[i] == 1) {
      sum++;
    }
  }
  printf("%"PRIu64"\n", sum);
  free(wires);
  
  return 0;
}
