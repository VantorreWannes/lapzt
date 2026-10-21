# Examples

The examples are tested with Zig 0.17.0.

Install `lapzt` once with `cargo install --path .`, then generate the standard library:

```sh
lapzt std > std.lap
```

Compile and run `tictactoe.lap`:

```sh
lapzt compile std.lap examples/tictactoe.lap > tictactoe.zig
zig build-exe tictactoe.zig -O ReleaseFast
./tictactoe
```

`tictactoe.lap` is a two-player game. Enter a cell number from 1 to 9.

Benchmarks live in [benchmarks](../benchmarks).
