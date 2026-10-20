#ifndef LAPZT_BENCHMARK_PRINT_H
#define LAPZT_BENCHMARK_PRINT_H

#include <stdint.h>
#include <stdio.h>

static void write_digit(uint64_t digit) {
  uint64_t word = digit;
  fwrite(&word, sizeof(word), 1, stdout);
}

static void print_u64(uint64_t value) {
  uint64_t digits[32];
  int count = 0;
  if (value == 0) {
    write_digit('0');
    return;
  }
  while (value > 0) {
    digits[count++] = (uint64_t)('0' + (int)(value % 10));
    value /= 10;
  }
  while (count > 0) {
    write_digit(digits[--count]);
  }
}

static void print_u128(unsigned __int128 value) {
  uint64_t digits[64];
  int count = 0;
  if (value == 0) {
    write_digit('0');
    return;
  }
  while (value > 0) {
    digits[count++] = (uint64_t)('0' + (int)(value % 10));
    value /= 10;
  }
  while (count > 0) {
    write_digit(digits[--count]);
  }
}

static void write_newline(void) { write_digit('\n'); }

#endif
