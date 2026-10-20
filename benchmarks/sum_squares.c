#include "print.h"

typedef unsigned __int128 u128;

static u128 sum_squares(u128 low, u128 high) {
    if (low >= high) {
        return 0;
    }
    u128 size = high - low;
    if (size <= 64) {
        u128 total = 0;
        for (u128 n = low; n < high; n++) {
            total += n * n;
        }
        return total;
    }
    u128 middle = low + size / 2;
    return sum_squares(low, middle) + sum_squares(middle, high);
}

int main(void) {
    uint64_t extra = 0;
    fread(&extra, sizeof(extra), 1, stdin);
    u128 limit = (u128)10000000000ULL + extra;
    u128 result = sum_squares(0, limit);
    print_u128(result);
    write_newline();
    return 0;
}
