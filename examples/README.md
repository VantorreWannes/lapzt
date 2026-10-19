# Examples

## Squares

`squares.lap` computes the sum of squares from 0 to 10,000,000,000 in `u128` and prints it in decimal. It reads one word from stdin so the compiler cannot constant-fold the computation.

Build and run:

```sh
cargo build --release
target/release/lapzt std > /tmp/std.lap
target/release/lapzt compile /tmp/std.lap examples/squares.lap > /tmp/squares.zig
zig 0.17.0 build-exe /tmp/squares.zig -O ReleaseFast -femit-bin=/tmp/squares.exe
/tmp/squares.exe < /dev/null
```

Benchmark:

```sh
hyperfine --warmup 1 --runs 5 --shell=bash '/tmp/squares.exe < /dev/null > /dev/null'
```

## Fibonacci

`fibonacci.lap` computes the 50th Fibonacci number and prints it in decimal.

Build and run it the same way, replacing `squares.lap` with `fibonacci.lap`.
