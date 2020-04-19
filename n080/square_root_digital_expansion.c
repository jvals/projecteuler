//https://en.wikipedia.org/wiki/Shifting_nth_root_algorithm
#include <math.h>
#include <stdio.h>
#include <stdint.h>
#include <gmp.h>
#include <stdbool.h>

#define ipow (uint64_t)pow
#define N 2

mpz_t radicand;
mpz_t r;
mpz_t y;
mpz_t x;
mpz_t alpha;
mpz_t beta;
mpz_t B;
mpz_t n;
mpz_t y_next;
mpz_t r_next;

// for computing beta
mpz_t By;
mpz_t By_candbeta;
mpz_t By_candbeta_expo_n;

mpz_t B_to_n;
mpz_t y_to_n;
mpz_t Bnyn;

mpz_t By_candbeta_expo_n_minus_Bnyn; // beta LHS

mpz_t B_to_n_times_r;
mpz_t B_to_n_times_r_plus_alpha; // beta RHS

mpz_t candidate_beta;

// for computing r prime
mpz_t By_plus_beta;
mpz_t By_plus_beta_to_n;
mpz_t By_plus_beta_to_n_minus_Bnyn;


void compute_By_candbeta_expo_n_minus_Bnyn() {
  mpz_mul(By, B, y);
  mpz_add(By_candbeta, By, candidate_beta);
  mpz_pow_ui(By_candbeta_expo_n, By_candbeta, N);

  mpz_pow_ui(B_to_n, B, N);
  mpz_pow_ui(y_to_n, y, N);
  mpz_mul(Bnyn, B_to_n, y_to_n);

  mpz_sub(By_candbeta_expo_n_minus_Bnyn, By_candbeta_expo_n, Bnyn);
}

void compute_B_to_n_times_r_plus_alpha() {
  // B to n is already computed
  mpz_mul(B_to_n_times_r, B_to_n, r);
  mpz_add(B_to_n_times_r_plus_alpha, B_to_n_times_r, alpha);
}

bool isLHSLessThanOrEqualToRHS() {
  if (mpz_cmp(By_candbeta_expo_n_minus_Bnyn, B_to_n_times_r_plus_alpha) <= 0) {
    return true;
  } else {
    return false;
  }
}

int main() {

  int candidates[] = {2, 3, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20, 21, 22, 23, 24, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99};

  mpz_t hundred_digital_sums;
  mpz_init(hundred_digital_sums);

  for (int j = 0; j < 90; ++j) {
    int candidate = candidates[j];


    mpz_init_set_ui(r, 0);
    mpz_init_set_ui(y, 0);
    mpz_init_set_ui(x, 0);
    mpz_init_set_ui(alpha, 0);
    mpz_init_set_ui(beta, 0);
    mpz_init_set_ui(B, 10);
    mpz_init_set_ui(n, 2);
    mpz_init_set_ui(y_next, 0);
    mpz_init_set_ui(r_next, 0);

    mpz_init_set_ui(radicand, candidate);
    mpz_init_set(alpha, radicand);

    mpz_init_set_ui(candidate_beta, 0);

    mpz_init_set_ui(By, 0);
    mpz_init_set_ui(By_candbeta, 0);
    mpz_init_set_ui(By_candbeta_expo_n, 0);

    mpz_init_set_ui(B_to_n, 0);
    mpz_init_set_ui(y_to_n, 0);
    mpz_init_set_ui(Bnyn, 0);

    mpz_init_set_ui(By_candbeta_expo_n_minus_Bnyn, 0); // beta LHS

    mpz_init_set_ui(B_to_n_times_r, 0);
    mpz_init_set_ui(B_to_n_times_r_plus_alpha, 0); // beta RHS

    mpz_init_set_ui(By_plus_beta, 0);

    mpz_init_set_ui(By_plus_beta_to_n, 0);
    mpz_init_set_ui(By_plus_beta_to_n_minus_Bnyn, 0);


    mpz_t digital_sum;
    mpz_init(digital_sum);
    for (int i = 0; i < 100; ++i) {
      // uint64_t candidate_beta = 0;
      // mpz_set_ui(r, 0);
      // mpz_set_ui(y, 0);
      mpz_set_ui(candidate_beta, 0);
      // printf("beta=%llu r=%llu alpha=%llu y=%llu\n", beta, r, alpha, y);
      // printf("\n");

      mpz_set_ui(By, 0);
      mpz_set_ui(By_candbeta, 0);
      mpz_set_ui(By_candbeta_expo_n, 0);

      mpz_set_ui(B_to_n, 0);
      mpz_set_ui(y_to_n, 0);
      mpz_set_ui(Bnyn, 0);

      mpz_set_ui(By_candbeta_expo_n_minus_Bnyn, 0);

      mpz_set_ui(B_to_n_times_r, 0);
      mpz_set_ui(B_to_n_times_r_plus_alpha, 0);
      mpz_set_ui(candidate_beta, 0);

      compute_By_candbeta_expo_n_minus_Bnyn();
      compute_B_to_n_times_r_plus_alpha();

      while(isLHSLessThanOrEqualToRHS()) {
        mpz_set(beta, candidate_beta);
        mpz_add_ui(candidate_beta, candidate_beta, 1);
        compute_By_candbeta_expo_n_minus_Bnyn();
        compute_B_to_n_times_r_plus_alpha();
      }


      mpz_add(By_plus_beta, By, beta);
      mpz_set(y_next, By_plus_beta);

      mpz_pow_ui(By_plus_beta_to_n, By_plus_beta, N);
      mpz_sub(By_plus_beta_to_n_minus_Bnyn, By_plus_beta_to_n, Bnyn);

      mpz_sub(r_next, B_to_n_times_r_plus_alpha, By_plus_beta_to_n_minus_Bnyn);

      mpz_set(y, y_next);
      mpz_set(r, r_next);
      mpz_set_ui(alpha, 0);

      // mpz_out_str(stdout, 10, By_candbeta_expo_n_minus_Bnyn);
      // mpz_out_str(stdout, 10, beta);

      mpz_add(digital_sum, digital_sum, beta);

    }

    mpz_add(hundred_digital_sums, hundred_digital_sums, digital_sum);

  }

  mpz_out_str(stdout, 10, hundred_digital_sums);

  printf("\n");


}
