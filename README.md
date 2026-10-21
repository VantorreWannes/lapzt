# lapzt

Lap Zig Transpiler. Compiles [Labels and Primitives](LAP.md) (Lap) source into Zig.

## Install

```sh
cargo install --path .
```

## Use

```sh
lapzt std > std.lap
lapzt compile std.lap main.lap > main.zig
zig build-exe main.zig -O ReleaseFast
./main
```

`lapzt compile` concatenates its inputs in order and writes Zig to stdout. `lapzt check` reports errors without emitting. `lapzt std` prints the standard library. `lapzt compile --no-erasure` emits the Lap implementations instead of the erased scalar operations.

## Example

Put this in `main.lap`:

```lap
main = () [] {
    liblapzt.print.u64(liblapzt.u64.one)
    liblapzt.print.newline()
    []
}
```

## Docs

- [LAP.md](LAP.md): the language.
- [LAPZT.md](LAPZT.md): compiler assumptions, erasure, and runtime.
- [editor-support](editor-support): tree-sitter grammar and Zed extension.
- [examples](examples): small programs.
- [benchmarks](benchmarks): Lap compared to C.
