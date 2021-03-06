#include <stdio.h>
#include <stdlib.h>

#define GRID_WIDTH 80
#define GRID_HEIGHT 80

#define N_NEIGHBOURS 3

#define INF 2000000000

enum directions {SOUTH=0, EAST=1, NORTH=2, WEST=3};

typedef int int_t;

typedef struct vertex_t vertex_t;
struct vertex_t {
  int weight;
  vertex_t** neighbours;

  // dijkstra
  vertex_t* parent; // pi
  int estimate; // d
  int idx;
};

typedef struct ll_node_t ll_node_t;
struct ll_node_t {
  vertex_t* data;
  ll_node_t* left;
  ll_node_t* right;
};


int_t* createGridFromFile(char *input) {
  int_t* grid = malloc(GRID_HEIGHT * GRID_WIDTH * sizeof(int_t));

  FILE* fp = fopen(input, "r");

  int_t weight;
  for (int i = 0; i < GRID_HEIGHT; ++i) {
    for (int j = 0; j < GRID_WIDTH; ++j) {
      fscanf(fp, "%d", &weight);
      grid[i * GRID_HEIGHT + j] = weight;
    }
  }
  fclose(fp);

  return grid;
}

void printGrid(int* grid) {
  for (int i = 0; i < GRID_HEIGHT; ++i) {
    for (int j = 0; j < GRID_WIDTH; ++j) {
      printf("%d ", grid[i * GRID_HEIGHT + j]);
    }
    printf("\n");
  }
}



vertex_t* createGraphFromGrid(int* grid) {

  vertex_t* graph = malloc(GRID_HEIGHT * GRID_WIDTH * sizeof(vertex_t));


  // First, create all vertices
  for (int i = 0; i < GRID_HEIGHT; ++i) {
    for (int j = 0; j < GRID_WIDTH; ++j) {
      vertex_t *newVertex = malloc(sizeof(vertex_t));
      int_t weight = grid[i * GRID_HEIGHT + j];
      newVertex->weight = weight;
      newVertex->neighbours = calloc(N_NEIGHBOURS, sizeof(vertex_t*));
      newVertex->parent = NULL;
      newVertex->estimate = INF;
      newVertex->idx = i*GRID_HEIGHT + j;

      graph[i * GRID_HEIGHT + j] = *newVertex;
    }
  }

  // Second, set up edges between all adjacent vertices
  for (int i = 0; i < GRID_HEIGHT; ++i) {
    for (int j = 0; j < GRID_WIDTH; ++j) {
      vertex_t rootVertex = graph[i * GRID_HEIGHT + j];

      // South
      if (i+1 < GRID_HEIGHT) {
        vertex_t *southVertex = &graph[(i+1) * GRID_HEIGHT + j];
        rootVertex.neighbours[SOUTH] = southVertex;
      }

      // East
      if (j+1 < GRID_WIDTH) {
        vertex_t *eastVertex = &graph[i * GRID_HEIGHT + (j+1)];
        rootVertex.neighbours[EAST] = eastVertex;
      }

      // North
      #if N_NEIGHBOURS > 2
      if (i-1 >= 0) {
        vertex_t *northVertex = &graph[(i-1) * GRID_HEIGHT + j];
        rootVertex.neighbours[NORTH] = northVertex;
      }
      #endif

      // West
      #if N_NEIGHBOURS > 3
      if (j-1 >= 0) {
        vertex_t *westVertex = &graph[i * GRID_HEIGHT + j-1];
        rootVertex.neighbours[WEST] = westVertex;
      }
      #endif

    }
  }

  return graph;
}

vertex_t* extractMin(ll_node_t** vertices, int *size) {
  ll_node_t *minimum = *vertices;
  ll_node_t *iter = *vertices;

  while(iter != NULL) {
    if(iter->data->estimate < minimum->data->estimate) {
      minimum = iter;
    }
    // printf("weight=%d, ", iter->data->weight);
    // if(iter->left) {
    //   printf("left=%d, ", iter->left->data->weight);
    // } else {
    //   printf("left=NULL, ");
    // }
    // if (iter->right) {
    //   printf("right=%d\n", iter->right->data->weight);
    // } else {
    //   printf("right=NULL\n");
    // }
    iter = iter->right;
  }
  // printf("========================================\n");
  // printf("%d\n", minimum->data->weight);

  // remove node from vertices
  if(minimum->left) {
    minimum->left->right = minimum->right;
    if (minimum->right) {
      minimum->right->left = minimum->left;
    }
  } else {
    *vertices = (*vertices)->right;
    if (*vertices) {
      (*vertices)->left = NULL;
    }
  }
  (*size)--;


  return minimum->data;
}

void initializeWorkingSet(vertex_t* graph, ll_node_t* workingSet, int startNodeIdx) {
  graph[0].estimate = INF;
  graph[0].parent = NULL;
  workingSet->data = &graph[0];
  workingSet->left = NULL;

  ll_node_t *iter = workingSet;

  for (int i = 1; i < GRID_HEIGHT*GRID_WIDTH; ++i) {
    graph[i].estimate = INF;
    graph[i].parent = NULL;
    iter->right = malloc(sizeof(ll_node_t));
    iter->right->data = &graph[i];
    iter->right->left = iter;

    iter = iter->right;
  }
  graph[startNodeIdx].estimate = 0;
}

void relax(vertex_t *u, vertex_t *v) {
  if (v->estimate > u->estimate + v->weight) {
    v->estimate = u->estimate + v->weight;
    v->parent = u;
  }
}

void dijkstra(vertex_t* graph, int startNodeIdx) {
  // This function modifies 'graph' in-place
  //vertex_t **finishedSet = malloc(GRID_WIDTH * GRID_HEIGHT * sizeof(vertex_t*));
  //int finishedSetSize = 0;

  ll_node_t* workingSet = malloc(sizeof(ll_node_t));
  initializeWorkingSet(graph, workingSet, startNodeIdx);
  // workingSet->data = &graph[startNodeIdx];
  // workingSet->left = NULL;
  // workingSet->right = NULL;

  int workingSetSize = GRID_HEIGHT*GRID_WIDTH;

  while(workingSetSize > 0) {
    vertex_t *current = extractMin(&workingSet, &workingSetSize);
    //finishedSet[finishedSetSize++] = current;

    // Relax
    // South
    if (current->neighbours[SOUTH]) {
      relax(current, current->neighbours[SOUTH]);
    }
    // East
    if (current->neighbours[EAST]) {
      relax(current, current->neighbours[EAST]);
    }

    // North
    #if N_NEIGHBOURS > 2
    if (current->neighbours[NORTH]) {
      relax(current, current->neighbours[NORTH]);
    }
    #endif
  }

}

void printNeighbors(vertex_t *graph) {
  for (int i = 0; i < GRID_HEIGHT; ++i) {
    for (int j = 0; j < GRID_WIDTH; ++j) {
      vertex_t vertex = graph[i * GRID_HEIGHT + j];
      printf("i=%d, j=%d, weight=%d, neighbours: ", i, j, vertex.weight);
      if (i+1 < GRID_HEIGHT) {
        printf("%d ", vertex.neighbours[SOUTH]->weight);
      }

      if (j+1 < GRID_WIDTH) {
        printf("%d ", vertex.neighbours[EAST]->weight);
      }

      printf("\n");

    }
  }
}

int traceShortestPath(vertex_t goal, vertex_t start) {
  int pathSum = goal.weight;
  #ifdef DEBUG
  printf("%d->", goal.weight);
  #endif // DEBUG
  vertex_t *iter = &goal;
  while (iter->parent) {
    #ifdef DEBUG
    printf("%d->", iter->parent->weight);
    #endif // DEBUG
    pathSum += iter->parent->weight;
    iter = iter->parent;
  }
  if (iter->idx == start.idx) {
    #ifdef DEBUG
    printf("\npathSum=%d\n", pathSum);
    #endif // DEBUG
    return pathSum;
  } else {
    #ifdef DEBUG
    printf("\n");
    #endif // DEBUG
    return -1;
  }
}

int main(int argc, char *argv[])
{
  if (argc != 2) {
    printf("Usage: %s input.txt\n", argv[0]);
    return 1;
  }

  int_t *grid = createGridFromFile(argv[1]);

  #ifdef DEBUG
  printGrid(grid);
  #endif // DEBUG

  vertex_t *graph = createGraphFromGrid(grid);

  #ifdef DEBUG
  printNeighbors(graph);
  // puts("==========================");
  // graph[0].neighbours[EAST]->weight = 0;
  // printNeighbors(graph);
  #endif // DEBUG

  int minimalPathSum = INF;
  for (int i = 0; i < GRID_HEIGHT; i++) {
    //vertex_t *graph = createGraphFromGrid(grid);
    dijkstra(graph, i*GRID_HEIGHT);
    for (int j = 0; j < GRID_HEIGHT; j++) {
      int path = traceShortestPath(graph[j*GRID_HEIGHT+(GRID_WIDTH-1)], graph[i*GRID_HEIGHT]);
      if (path != -1 && path < minimalPathSum) {
        minimalPathSum = path;
      }
    }
  }

  printf("minimalPathSum = %d\n", minimalPathSum);

  free(grid);
  free(graph);
  return 0;
}
