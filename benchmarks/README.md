# Benchmarks

Each problem is implemented in idiomatic Lap and idiomatic C. Lap uses recursion; C uses loops. Both share the decimal printer: `liblapzt/print.lap` and `print.h`.

| Problem       | Impl | Wall     | CPU      | User     | System  |
| ------------- | ---- | -------- | -------- | -------- | ------- |
| `sum_squares` | Lap  | 2.338 s  | 2.331 s  | 2.306 s  | 25.0 ms |
| `sum_squares` | C    | 2.839 s  | 2.812 s  | 2.781 s  | 31.2 ms |
| `primes`      | Lap  | 2.169 s  | 2.144 s  | 2.078 s  | 65.6 ms |
| `primes`      | C    | 2.143 s  | 2.131 s  | 2.100 s  | 31.2 ms |
| `fnv`         | Lap  | 815.4 ms | 812.5 ms | 784.4 ms | 28.1 ms |
| `fnv`         | C    | 822.4 ms | 815.6 ms | 793.7 ms | 21.9 ms |

Measured with `hyperfine --metrics all`, 5 runs after 1 warmup, on Windows x86_64. Hardware counters are unavailable on this platform. Both sides use LLVM.

Compile each `.lap` file as in [examples](../examples). Compile each `.c` file with `zig cc -O3 -o <name>_c benchmarks/<name>.c`. Then run:

```sh
hyperfine --warmup 1 --runs 5 --metrics all './<name>' './<name>_c'
```

Replace `<name>` with `sum_squares`, `primes`, or `fnv`.

Each program reads one word from stdin so the compiler cannot constant-fold the workload.
