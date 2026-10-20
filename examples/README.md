# Examples

Install `lapzt` once with `cargo install --path .`, then generate the standard library:

```sh
lapzt std > std.lap
```

Compile and run `fibonacci.lap`:

```sh
lapzt compile std.lap examples/fibonacci.lap > fibonacci.zig
zig build-exe fibonacci.zig -O ReleaseFast
./fibonacci
```

`fibonacci.lap` prints the 50th Fibonacci number.

Benchmarks live in [benchmarks](../benchmarks).
