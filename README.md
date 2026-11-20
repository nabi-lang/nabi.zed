<img src="https://raw.githubusercontent.com/nabi-lang/nabi/main/assets/icons/icon-512-transparent.png" alt="Nabi" width="96">

# Nabi for Zed

[![CI](https://github.com/nabi-lang/nabi.zed/actions/workflows/ci.yml/badge.svg)](https://github.com/nabi-lang/nabi.zed/actions/workflows/ci.yml) [![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)

Nabi language support for [Zed](https://zed.dev): tree-sitter highlighting
for `.nabi` and `.nabion`, and the Nabi language server `nabils`.

## Features

- Highlighting, bracket matching, auto-indent, outline, Vim text objects
  and comment toggling, from the
  [nabi-syntax](https://github.com/nabi-lang/nabi-syntax) tree-sitter grammars.
- NABION for `.nabion` files and `nabi.lock`.
- From `nabils`, over LSP:
  - diagnostics as you type
  - hover
  - completion
  - go to definition
  - find references
  - rename
  - document symbols
  - formatting
  - semantic highlighting of macros, user-defined literal suffixes,
    keywords and operators
  - signature help
  - quick fixes, including `nabi add` for a missing package

## Install

The extension is not in the Zed extension registry yet. Until it is,
install it as a dev extension:

1. Install Rust with [rustup](https://rustup.rs); Zed builds the
   extension with it.
2. Clone this repository.
3. In Zed, run `zed: install dev extension` from the command palette (or
   **Install Dev Extension** in the Extensions page) and pick the
   checkout.

Zed fetches and builds the grammars at the commit in `extension.toml`.

## Language server

Install Nabi 0.1 or later so `nabils` is on your `PATH`.

Linux and macOS:

```sh
curl -fsSL https://nabi.sh | sh
```

Windows (PowerShell):

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://nabi.sh/install.ps1 | iex"
```

More at <https://nabi-lang.org/download>. The extension finds `nabils` on
the `PATH` of your login shell and never downloads it. Without `nabils`
you still get highlighting.

## Settings

In Zed's `settings.json`:

```json
{
  "lsp": {
    "nabils": {
      "binary": {
        "path": "/path/to/nabils",
        "arguments": [],
        "env": {}
      }
    }
  },
  "languages": {
    "Nabi": {
      "semantic_tokens": "combined"
    }
  }
}
```

| Setting                          | Description                                                 |
| -------------------------------- | ----------------------------------------------------------- |
| `lsp.nabils.binary.path`         | Path to `nabils`, if it is not on `PATH`.                   |
| `lsp.nabils.binary.arguments`    | Arguments for `nabils`. Default: none.                      |
| `lsp.nabils.binary.env`          | Extra environment for `nabils`.                             |
| `languages.Nabi.semantic_tokens` | `"combined"` layers `nabils` semantic tokens on tree-sitter. |

To turn the language server off:

```json
{
  "languages": {
    "Nabi": { "language_servers": ["!nabils", "..."] }
  }
}
```

## Grammars

`extension.toml` pins the `nabi` and `nabion` grammars to the nabi-syntax
commit in `nabi-syntax.rev`. `highlights.scm` and `injections.scm` in
`languages/` are generated from nabi-syntax's queries with Zed's capture
names; the other queries are Zed's own. Change the grammar and its
queries in nabi-syntax, then sync:

```sh
scripts/sync --from ../nabi-syntax   # copy from a checkout, pin its HEAD
scripts/sync                         # fetch at the pinned commit
scripts/sync --check                 # what CI runs
```

More in [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Licensed under either of [MIT](LICENSE-MIT) or
[Apache-2.0](LICENSE-APACHE), at your option.
