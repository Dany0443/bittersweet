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
- `scripts/` and `tracking/` maintain read-only upstream discovery records.
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

## Upstream tracking and sync

The tracking scripts are read-only against upstream: never push, comment,
open issues or open pull requests through them. The normal update sequence is:

```bash
python3 scripts/scrape_standings.py
python3 scripts/fetch_portwatch.py
python3 scripts/sync_upstream.py
python3 scripts/sync_upstream.py --offline-page saved.html --offline-portwatch saved-tracker.json
```

- `scrape_standings.py` writes `tracking/standings.json` and does not query
  GitHub.
- `fetch_portwatch.py` merges vetted efforts and tools into
  `UPSTREAM_REPOS.json`, writes `tracking/PORTWATCH.md`, and never imports
  PortWatch `flagged` or `ignored` entries.
- `sync_upstream.py` queries repository metadata and PRs, then writes
  `tracking/sync-report.json`, `tracking/CHANGELOG.md`,
  `tracking/PORTWATCH.md`, and the manifest.
- Every update cycle must refresh both feeds and collect all pages of
  commits and open PRs for every public tracked repository. Automated
  buckets are screening, not source review or proof that a port is safe.
- Prioritize Linux/Proton forks, core aggregators, AMD forks, performance
  forks, then independent toolchains. Review timeout/device-loss holds
  before adapting code; keep proprietary/Windows-SDK work experimental.
- Treat PortWatch `candidates` as unreviewed. Preserve active, renamed,
  unreachable, nuked and restored history instead of replacing records.
- Triage commit and PR changes as `critical-linux`, `frame-time`, `stability`,
  `rendering` or `experimental`. Proprietary, Windows-SDK-only or explicitly
  experimental work belongs in `experimental`.

Do not run these external-source scripts as part of ordinary code validation.

## Upstream porting

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

Use `cargo fmt --all -- --check` for Rust formatting changes. Use
`python3 -m py_compile scripts/*.py` when changing tracking scripts.

Do not claim a Linux runtime or title run works unless it has been exercised
and verified. A successful build is not a successful run.

## Documentation

- Update `README.md` when a user-visible command or status changes.
- Update `docs/architecture.md` for crate or data-flow changes.
- Update `docs/STATUS.md` only for evidence-backed Linux status changes.
- Update `UPSTREAM_REPOS.json` and `tracking/` for manifest or sync behavior
  changes; do not create a separate human-facing sync instruction page.
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

Do not run the tracking scripts as a substitute for validation unless the
request is specifically to refresh upstream tracking. Those scripts contact
external sources and are not needed for ordinary code changes.
