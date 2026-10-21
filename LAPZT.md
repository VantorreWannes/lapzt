# Lap Zig Transpiler

Lap Zig Transpiler (`lapzt`) compiles [Labels and Primitives](LAP.md) (Lap) source into Zig source.

## Scope

`lapzt` answers one question: how does a Lap program lower to Zig under fixed parser and runtime assumptions?

`lapzt` does not define the language, its primitives, or its grammar. See [LAP.md](LAP.md).

## Lowering

| Lap construct          | Zig representation          |
| ---------------------- | --------------------------- |
| `BIT` type             | `bool`                      |
| `NAND(BIT, NAND(...))` | `true`                      |
| `NAND(ONE, ONE)`       | `false`                     |
| `[]` type              | `struct {}`                 |
| `[T, ...]` type        | `struct { T, ... }`         |
| `[]` value             | `.{}`                       |
| `[v, ...]` value       | `.{ v, ... }`               |
| `*T` type              | `*T`                        |
| `label`                | `@"label"`                  |
| Global binding         | `pub const @"label" = ...;` |

## Assumptions

`LAP.md` leaves source layout, compilation units, determinacy checking, lowering, and the runtime interface to the compiler. `lapzt` makes the following assumptions.

### Compilation unit

- Input files are concatenated in the order of their CLI arguments, and the resulting source is compiled as a single `Program`.

### Source layout

- Statements in a `Program` or `Block` are separated by `\n`, with at most one statement or terminal expression per line.
- A statement continues across lines while its braces are unbalanced.
- Composite forms (`Type`, `Target`, `TypedIdentifier`, `Parameter`, `ParameterList`, `NandExpression`, `ExternExpression`, `CollectionExpression`, `CallExpression`, `BranchExpression`, `FunctionDefinition`, and `Binding`) split on their surface delimiter (`:`, `=`, `,`, `(`, `)`, `{`, `}`) without balancing nested delimiters.

### Determinacy

- `BitKeyword` carries an optional resolved boolean state (`Option<bool>`).
- Determinacy is decided by evaluating an expression under both boolean states of `BIT` (`false` and `true`). An expression is determinate when both evaluations produce the same `bool` tree.

### Erasure

- The compiler replaces certain defined labels with their scalar intrinsics. This process is called erasure.
- Erasure recognizes a label by its hash, and only when the label matches fully.
- Only labels ending in `.erased` are erased. A collection is never erased by its width.
- Unsigned types erase to `uN`, signed types to `iN`, and floats to `fN`.
- A collection of erased scalars packs into its unsigned word and bitcasts to the erased scalar.
- Integer and float operations with the `.erased` postfix erase to native Zig operations. The non-erased aliases resolve to them.
- Every `.erased` label has a Lap implementation. Compiling with `--no-erasure` emits those implementations instead of the scalar intrinsics.

### Entry point and `EXTERN`

- A standalone executable begins at `main`.
- `main` takes no parameters and returns `[]`.
- `EXTERN` takes an operation constant as its first argument.

### Recursion

- A function whose terminal expression is a self-call, or a branch of self-calls and values, is emitted as a loop. Its parameters become mutable locals, so self-tail recursion uses constant stack.
- Other recursion uses the native stack. Raise it with `zig build-exe ... --stack <bytes>` when a program recurses deeply without being self-tail-recursive.

## Runtime

`EXTERN` is the only exit from Lap. The runtime provides eleven operations and four handles. The runtime is emitted with the program, so the resulting Zig source is self-contained.

Eleven operations cover allocation, deallocation, reading, writing, termination, files, time, and randomness. Numbers, strings, collections, and control flow are library code.

A word is any value whose type is a collection of 64 `BIT`s. Words are little-endian. A handle is a word, not a reference. A memory handle remains valid until `OP.MEMORY.RELEASE`. A file handle remains valid until `OP.FILE.CLOSE`.

### Operations

| Operation           | Signature                                              | Result                            |
| ------------------- | ------------------------------------------------------ | --------------------------------- |
| `OP.MEMORY.ACQUIRE` | `(size: word)`                                         | `[BIT, word]`: status and handle  |
| `OP.MEMORY.RELEASE` | `(handle: word)`                                       | `[BIT]`: status                   |
| `OP.READ`           | `(handle: word, offset: word)`                         | `[BIT, word]`: status and word    |
| `OP.WRITE`          | `(handle: word, offset: word, value: word)`            | `[BIT]`: status                   |
| `OP.EXIT`           | `(code: word)`                                         | `[]`, never returns               |
| `OP.FILE.OPEN`      | `(path: word, offset: word, length: word, mode: word)` | `[BIT, word]`: status and handle  |
| `OP.FILE.CLOSE`     | `(handle: word)`                                       | `[BIT]`: status                   |
| `OP.FILE.SEEK`      | `(handle: word, offset: word, whence: word)`           | `[BIT]`: status                   |
| `OP.TIME.NOW`       | `()`                                                   | `word`: nanoseconds since epoch   |
| `OP.TIME.SLEEP`     | `(nanoseconds: word)`                                  | `[]`                              |
| `OP.RANDOM`         | `()`                                                   | `word`                            |

- `OP.MEMORY.ACQUIRE` returns a handle to a zeroed block of `size` bytes.
- `OP.MEMORY.RELEASE` invalidates a memory handle.
- `OP.READ` reads a word at `offset` from a memory handle, or the next word from a stream handle.
- `OP.WRITE` writes a word at `offset` to a memory handle, or the next word to a stream handle.
- `OP.EXIT` terminates the process with `code`.
- `OP.FILE.OPEN` opens the `length` bytes at `offset` in the memory handle `path` as a file. Modes are 0 read, 1 write, 2 append, and 3 read/write. Write truncates, append seeks to the end, and read fails when the file is missing.
- `OP.FILE.CLOSE` invalidates a file handle.
- `OP.FILE.SEEK` moves a file handle. Whence is 0 start, 1 current, and 2 end. `offset` is unsigned for start and a two's complement signed value for current and end.
- `OP.TIME.NOW` returns the wall clock in nanoseconds since the Unix epoch.
- `OP.TIME.SLEEP` waits for `nanoseconds` on the monotonic clock.
- `OP.RANDOM` returns a word from the operating system random source.
- Operations accept words by value. Any type that is a collection of 64 `BIT`s is a word, so a library may define its own word type.
- A status bit is true on success and false on failure or end of stream.
- Stream handles ignore `offset`.
- A stream read zero-pads the final word.

### Handles

| Handle      | Resource               |
| ----------- | ---------------------- |
| `IO.STDIN`  | standard input         |
| `IO.STDOUT` | standard output        |
| `IO.STDERR` | standard error         |
| `IO.ARGS`   | command-line arguments |

- `IO.ARGS` is a readable stream of zero-terminated argument bytes.
- `OP.FILE.OPEN` returns file handles. A file handle is a seekable stream, so `OP.READ` and `OP.WRITE` use the current position and ignore `offset`.
- The runtime provides the `OP.*` and `IO.*` labels. They are visible to every program.

## Ownership

- [LAP.md](LAP.md) owns the language primitives, rules, and grammar.
- This document owns the parser, lowering, and runtime assumptions of `lapzt`.
