# AI agent instructions

Use this file for repository work. Human-facing setup and legal guidance remain
in `README.md` and `docs/`; operational and repository-specific rules belong
here.

## Scope

This repository is a Linux/Proton compatibility fork of `victory`:

- `crates/bittersweet-host` contains the dependency-free Linux host probe,
  planning policy and environment logic.
- `apps/bittersweet` contains the host CLI.
- `crates/victory-*` and `apps/victory` contain the inherited Windows runtime.
- `docs/` contains architecture, status and human-facing guidance.

Keep Linux host changes separate from upstream runtime behavior unless the
request explicitly covers both.

## Before editing

1. Read `README.md`, `docs/architecture.md`, and the most relevant guide.
2. Inspect `git status` before changing files. Preserve unrelated work.
3. Search for existing helpers and tests instead of adding parallel utilities.
4. Read `docs/asset-policy.md` before adding files or fixtures.

## Asset and data policy

- Never add assets, retail binaries, executable-derived data, decompilation,
  disassembly, screenshots or proprietary SDK material to the repository.
- Synthetic test data is allowed. Build identities, small interoperability
  constants and byte-pattern signatures are allowed; preserve their narrow
  purpose and existing provenance.
- Real title files and caches belong outside the repository. Tests must read
  paths from the environment and skip when those paths are unavailable.
- Run `cargo test -p xtask` or `cargo xtask policy` after file additions.

## Upstream porting

- Research current upstream repositories and changes independently for each
  task. Do not rely on checked-in snapshots or a shared tracking manifest.
- Port behavior, not copied source trees or proprietary files.
- Adapt upstream behavior to this repository's naming and `cfg` layout.
- Keep the upstream repository, PR number and commit SHA in provenance,
  `CREDITS.md`, comments or commit context as applicable.
- Add or reuse a focused test for behavior introduced by the port.
- Never port proprietary components or game material. Run
  `cargo test -p xtask` before finalizing.

## Validation

Run the smallest relevant checks first:

```bash
cargo test -p xtask
cargo test -p bittersweet
cargo test --workspace
```

Use `cargo fmt --all -- --check` for Rust formatting changes.

Do not claim a Linux runtime or title run works unless it has been exercised
and verified. A successful build is not a successful run.

## Documentation

- Update `README.md` when a user-visible command or status changes.
- Update `docs/architecture.md` for crate or data-flow changes.
- Update `docs/STATUS.md` only for evidence-backed Linux status changes.
- Keep `CREDITS.md` and upstream PR/commit provenance current when porting
  behavior. Put provenance in code, comments or commit context as appropriate.

## Repository operations

Commit each completed, verified feature and significant change separately.
Use `type(scope): imperative description` for commit subjects, for example
`fix(probe): prefer the discrete GPU`. Use the maintainer identity already
configured in Git, work on `main`, and omit AI coauthor trailers. Do not
create another branch for this repository.

Do not push, force-push, or open or update a pull request unless the user
explicitly asks.

## Writing conventions

Keep code comments minimal. Explain only non-obvious behavior or necessary
provenance. Do not use em dashes in text you write, including code comments,
documentation, commit messages, and chat replies.
