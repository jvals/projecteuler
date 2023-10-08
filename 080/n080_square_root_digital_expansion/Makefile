http://www.ccs.neu.edu/home/skotthe/classes/cs5600/fall/2015/labs/intro-check.html

CC = gcc-8
CFLAGS = -Wall -Wextra -Wpedantic -O2 -c -std=c11
TST_LIBS = -lcheck
SRC_DIR = src
TST_DIR = tests
SRC_FILES = $(addprefix $(SRC_DIR)/, *.c)
TST_FILES = $(addprefix $(TST_DIR)/, *.c)

TARGETS = check_n080_tests n080
all: ${TARGETS}

n080.o: $(SRC_FILES) #$(addprefix $(SRC_DIR)/, n080.h)
	$(CC) $(CFLAGS) $^

check_n080.o: $(TST_FILES)
	$(CC) $(CFLAGS) $^

check_n080_tests: n080.o check_n080.o
	$(CC) $^ $(TST_LIBS) -o $@

n080: main.o n080.o
	$(CC) $^ -o $@

.PHONY: clean run tests

run: n080
	./$<

tests: check_n080_tests
	./$<

clean:
	rm -rf check_n080_tests n080 *.o
