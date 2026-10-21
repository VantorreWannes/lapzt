# Lap editor support

## Layout

- `tree-sitter-lap` at the repository root: the grammar and its queries.
- `editors/zed`: the Zed extension.

## Grammar

`tree-sitter-lap/src` is generated from `grammar.js`. Regenerate and test it after every grammar change:

```sh
cd tree-sitter-lap
npx tree-sitter-cli generate
npx tree-sitter-cli test
```

Commit `src/parser.c`, `src/grammar.json`, and `src/node-types.json`. Zed compiles the grammar from these files and does not run the generator.

## Zed

The extension references the grammar through `path = "tree-sitter-lap"`, so the grammar stays in this repository.

Install it as a dev extension:

1. Run `zed: install dev extension` from the command palette.
2. Select `editors/zed`.

Reinstall the dev extension after changing `editors/zed/src` or `extension.toml`.

Zed clones the grammar repository at `rev`. Uncommitted grammar changes are not visible until you commit them. Pin `rev` to a commit SHA before publishing. To test a local checkout, point `repository` at the checkout and set `rev` to the current commit:

```toml
[grammars.lap]
repository = "file:///<absolute-path-to-lapzt>"
rev = "<commit>"
path = "tree-sitter-lap"
```

## Language server

`lapzt lsp` provides hover and go-to-definition. The extension launches it from `PATH`.

Install it once:

```sh
cargo install --path .
```

Hover shows the comment block immediately above the binding, then the full binding body. Go to definition works on labels.

The server indexes every `.lap` file under the workspace root. It skips `target`, `.git`, `node_modules`, `grammars`, and `tree-sitter-lap`.

## Grammar notes

- Keywords match exactly. `BIT.ONE` is a label because the label token is longer than the keyword token.
- `Block` is `Statement*`. The trailing expression is syntactically a statement; the block-value rule is semantic.
- `[a, b] = ...` and `[a, b]` are ambiguous until `=`. The grammar declares GLR conflicts for the target and expression forms.

## Queries

Zed loads query files from the language directory, not from the grammar's `queries` directory. `editors/zed/languages/lap` holds the copies that Zed uses:

- `highlights.scm` maps grammar nodes to Zed captures. Labels without a dot are `@variable`, labels with a dot are `@property`, and binding targets are `@property` so definitions stand out from uses. Function definitions and calls are `@function`, parameters are `@variable.parameter`, and types are `@type`. Collection brackets are `@keyword` so collection literals and destructuring patterns are visible.
- `brackets.scm`, `indents.scm`, and `outline.scm` provide bracket matching, indentation, and the outline.

Keep `tree-sitter-lap/queries` and `editors/zed/languages/lap` in sync. The grammar copies serve other tree-sitter consumers.

## Other editors

VS Code, Sublime Text, and GitHub use TextMate grammars. A TextMate grammar is a separate file and does not share the tree-sitter queries.
