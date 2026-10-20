#include "print.h"

static uint64_t fnv(uint64_t limit) {
    uint64_t hash = 14695981039346656037ULL;
    for (uint64_t n = 0; n < limit; n++) {
        hash ^= n;
        hash *= 1099511628211ULL;
    }
    return hash;
}

int main(void) {
    uint64_t extra = 0;
    fread(&extra, sizeof(extra), 1, stdin);
    uint64_t limit = 1000000000 + extra;
    uint64_t result = fnv(limit);
    print_u64(result);
    write_newline();
    return 0;
}
