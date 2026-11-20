# Releasing

Zed installs extensions from the [zed-industries/extensions](https://github.com/zed-industries/extensions) registry, which holds each extension as a git submodule. A release of this extension is a pull request there. Zed's CI builds `src/` to WebAssembly and the grammars from the revs in `extension.toml`; nothing is uploaded from here.

Rules from Zed's [publishing guide](https://zed.dev/docs/extensions/publishing/publishing-guide), [prerequisites](https://zed.dev/docs/extensions/publishing/prerequisites) and [license requirements](https://zed.dev/docs/extensions/publishing/license-requirements), checked 2026-11-21:

- the id (`nabi`) is kebab-case, names the language and has no `zed` or `extension` in it;
- a `LICENSE*` file at the extension root under an accepted license (`LICENSE-MIT` and `LICENSE-APACHE`: MIT and Apache 2.0 are both accepted);
- the repository is public, the submodule URL is `https://`, not `git@github.com:`, and the commit is on a branch;
- `version` in the registry entry equals `version` in `extension.toml` at that commit;
- one extension per pull request, at most three open; answer review within 3 weeks or the PR is closed.

## Before the first submission

1. nabi-lang/nabi.zed and nabi-lang/nabi-syntax are public, and the grammar `rev`s in `extension.toml` are commits on nabi-syntax `main` (`scripts/sync --check` passes).
2. `extension.toml` and `Cargo.toml` carry the version to ship, and `CHANGELOG.md` has its section.
3. The commit to submit is on `main` here, CI (`ci.yml`) passed on it, and it is tested in Zed: `zed: install dev extension` on a checkout of that commit, then open a `.nabi` and a `.nabion` file (highlighting, indent, outline) with `nabils` on `PATH` (hover, diagnostics).
4. Tag it: `git tag v<version> && git push origin v<version>`.

## First submission

From a fork of zed-industries/extensions under a personal account (Zed asks for that, so its staff can push to the PR):

```sh
git clone https://github.com/<you>/extensions
cd extensions
git submodule init
git submodule update
git checkout -b nabi
git submodule add https://github.com/nabi-lang/nabi.zed.git extensions/nabi
git -C extensions/nabi checkout v0.1.0
git add extensions/nabi
```

Add to `extensions.toml` (no `path`: the extension is the repository root):

```toml
[nabi]
submodule = "extensions/nabi"
version = "0.1.0"
```

Then sort, commit and open the PR:

```sh
pnpm install
pnpm sort-extensions
git commit -am "Add Nabi extension"
git push -u origin nabi
```

PR title `Add Nabi extension`; body: one line on what it gives (tree-sitter highlighting for `.nabi` and `.nabion`, the `nabils` language server) and a link to this repository.

## Updates

Bump the version here (`extension.toml`, `Cargo.toml`, `CHANGELOG.md`), merge, tag `v<version>`, then in the fork:

```sh
git checkout main && git pull https://github.com/zed-industries/extensions main
git checkout -b nabi-<version>
git -C extensions/nabi fetch --tags && git -C extensions/nabi checkout v<version>
# set version = "<version>" under [nabi] in extensions.toml
git commit -am "Update Nabi extension to <version>"
```

and open a PR as above. A nabi-syntax release reaches Zed only this way: `scripts/sync` moves the grammar revs, then this extension is released.
