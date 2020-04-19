#include <stdio.h>
#include <stdlib.h>

#define GRID_WIDTH 5
#define GRID_HEIGHT 5

#define N_NEIGHBOURS 4

#define INF 2000000000

#define IDX(X, Y) (Y*GRID_HEIGHT+X)

enum directions {SOUTH=0, EAST=1, NORTH=2, WEST=3};


int* createGridFromFile(char *input) {
  int* grid = malloc(GRID_HEIGHT * GRID_WIDTH * sizeof(int));

  FILE* fp = fopen(input, "r");

  int weight;
  for (int i = 0; i < GRID_HEIGHT; ++i) {
    for (int j = 0; j < GRID_WIDTH; ++j) {
      fscanf(fp, "%d", &weight);
      grid[i * GRID_HEIGHT + j] = weight;
    }
  }
  fclose(fp);

  return grid;
}


int main(int argc, char *argv[]) {
  if (argc != 2) {
    printf("Usage: %s input.txt\n", argv[0]);
    return 1;
  }

  int estimate[GRID_HEIGHT*GRID_WIDTH];
  for (int i = 0; i < GRID_HEIGHT*GRID_WIDTH; i++) {
    estimate[i] = INF;
  }

  int *grid = createGridFromFile(argv[1]);

  int start = grid[0];
  grid[0] = -1;
  for (int i = 0; i < GRID_HEIGHT; i++) {
    for (int j = 0; j < GRID_WIDTH; j++) {
      if(i==0 && j==0) {
        continue;
      }

      // North
      if (i-1 >= 0) {
        if (estimate[IDX(j,i-1)] > estimate[IDX(j,i)] + grid[IDX(j,i)] ) {
          estimate[IDX(j,i-1)] = estimate[IDX(j,i)] + grid[IDX(j,i)];
        }
      }

      // South
      if (i+1 < GRID_HEIGHT) {
        if (estimate[IDX(j,i+1)] > estimate[IDX(j,i)] + grid[IDX(j,i)] ) {
          estimate[IDX(j,i+1)] = estimate[IDX(j,i)] + grid[IDX(j,i)];
        }
      }

      // East
      if (j+1 < GRID_WIDTH) {
        if (estimate[IDX(j+1,i)] > estimate[IDX(j,i)] + grid[IDX(j,i)] ) {
          estimate[IDX(j+1,i)] = estimate[IDX(j,i)] + grid[IDX(j,i)];
        }
      }

      // West
      if (j-1 >= 0) {
        if (estimate[IDX(j-1,i)] > estimate[IDX(j,i)] + grid[IDX(j,i)] ) {
          estimate[IDX(j-1,i)] = estimate[IDX(j,i)] + grid[IDX(j,i)];
        }
      }

      int candidate = INF;
      for (int k = 0; k < GRID_HEIGHT*GRID_WIDTH; k++) {
        if(grid[k] != -1 && grid[k] < candidate) {
          candidate = grid[k];
        }
      }
      start = candidate;
    }
  }
  printf("%d\n", estimate[GRID_HEIGHT*GRID_WIDTH-1]);

}
