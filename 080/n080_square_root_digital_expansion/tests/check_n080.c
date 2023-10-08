// http://www.ccs.neu.edu/home/skotthe/classes/cs5600/fall/2015/labs/intro-check.html

#include <check.h>
#include <stdlib.h>
#include "../src/n080.h"


START_TEST(test_digital_sum_of_root) {
	int answer = digital_sum_of_root(2);
	ck_assert_int_eq(answer, 475);
} END_TEST

Suite *n080_suite(void) {
  Suite *s;
  TCase *tc_core;

  s = suite_create("n080");
  tc_core = tcase_create("Core");

  tcase_add_test(tc_core, test_digital_sum_of_root);
  suite_add_tcase(s, tc_core);

  return s;
}

int main(void)
{
  int no_failed = 0;
  Suite *s;
  SRunner *runner;

  s = n080_suite();
  runner = srunner_create(s);
  srunner_run_all(runner, CK_NORMAL);
  no_failed = srunner_ntests_failed(runner);
  srunner_free(runner);
  
  return (no_failed == 0) ? EXIT_SUCCESS : EXIT_FAILURE;
}
