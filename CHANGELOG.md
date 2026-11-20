# Changelog

All notable changes to this extension. Versions follow semver, independent of the Nabi toolchain.

## 0.1.0 (unreleased)

First release.

- tree-sitter highlighting, bracket matching, auto-indent, outline, Vim text
  objects and comment toggling for `.nabi`, from the nabi-syntax grammars.
- NABION for `.nabion` files and `nabi.lock`.
- The `nabils` language server over LSP: diagnostics as you type, hover,
  completion, go to definition, find references, rename, document symbols,
  formatting, semantic highlighting, signature help and quick fixes
  (including `nabi add` for a missing package).
- `nabils` found on the login shell's `PATH`, never downloaded; settings
  under `lsp.nabils.binary`.
- Installed as a dev extension until it is in the Zed extension registry.
  Needs Nabi 0.1 or later for `nabils`; highlighting works without it.
