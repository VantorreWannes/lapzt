#include "print.h"

static int is_prime(uint64_t n, uint64_t divisor) {
  while (divisor * divisor <= n) {
    if (n % divisor == 0) {
      return 0;
    }
    divisor++;
  }
  return 1;
}

static uint64_t count_primes(uint64_t limit) {
  uint64_t count = 0;
  for (uint64_t n = 2; n < limit; n++) {
    if (is_prime(n, 2)) {
      count++;
    }
  }
  return count;
}

int main(void) {
  uint64_t extra = 0;
  fread(&extra, sizeof(extra), 1, stdin);
  uint64_t limit = 10000000 + extra;
  uint64_t result = count_primes(limit);
  print_u64(result);
  write_newline();
  return 0;
}
