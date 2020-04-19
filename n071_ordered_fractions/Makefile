all: main.c
	gcc-7 -std=c11 -Wall -Wextra -Wpedantic -Ofast -fopenmp -I /usr/local/Cellar/gmp/6.1.2_1/include -L /usr/local/Cellar/gmp/6.1.2_1/lib -lgmp main.c

run: all
	./a.out
