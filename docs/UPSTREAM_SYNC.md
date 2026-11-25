# Upstream tracking and sync

`bittersweet` follows the public ecosystem around the target title. The
tracking tools are read-only against upstream: they fetch pages and query
the GitHub API, and they never push, comment or open issues.

## Running an update check

```bash
python3 scripts/scrape_standings.py            # refresh tracking/standings.json
python3 scripts/sync_upstream.py               # discover, sync, triage
python3 scripts/sync_upstream.py --offline-page saved.html
```

`scrape_standings.py` parses the standings table, the "also watched" list
and the "no published work" list. It writes `tracking/standings.json` and
does not contact GitHub.

`sync_upstream.py` compares the page with `UPSTREAM_REPOS.json`, then asks
GitHub for the default branch, licence, archived flag, latest commits and
open pull requests of every tracked repository. It writes:

| File | Contents |
|---|---|
| `UPSTREAM_REPOS.json` | The manifest: one record per repository |
| `tracking/sync-report.json` | Machine-readable triage, per repository and in total |
| `tracking/CHANGELOG.md` | The same report as Markdown |

## Manifest

The manifest is schema `1`. The top level records the source URL, the
generation time and `repo_count`. Each entry in `repos` has:

| Field | Meaning |
|---|---|
| `id` | `owner/name`, lowercased for comparison |
| `url`, `name` | Canonical URL and display name |
| `family`, `role` | Ecosystem grouping and the project's role in it |
| `status` | `active`, `archived`, `offline`, `renamed` or `takedown` |
| `renamed_to` | New `owner/name` when `status` is `renamed` |
| `base` | True only for the snapshot this repository is built on |
| `first_seen`, `last_sync_at` | Discovery and last successful sync |
| `last_sync_sha` | Last commit seen at the time of the sync |
| `on_page` | Whether the standings page still lists it |
| `license`, `default_branch`, `archived`, `stars` | GitHub metadata |
| `notes` | Why the repository is tracked |

A case-only rename is compared by slug and is not reported as a new
project. A repository that disappears from the page is marked `offline`
unless it was already known to be renamed or taken down; the record is kept
so the history stays visible.

## Discovery

Every check registers public repositories that the page mentions but the
manifest does not contain. The first run found six: `cblvictoryvi`,
`Holy-Interoperability`, `xbox-emulator`, `AnyPS5`, `ScarlettX` and
`scarlett-layer`. `ZSlayerHQ/victory-world-runtime` was detected as renamed
to `ZSlayerHQ/zslayer-pc-port-runtime`; the old id remains in the report so
the rename is auditable.

## Triage buckets

Every commit and pull request is scored by keywords in its title and body
and placed in one of five buckets. Ties go to the bucket listed first; a
zero score becomes `unclassified`.

| Bucket | Meaning |
|---|---|
| `critical-linux` | Required for a Linux/Proton-GE run, or a hard blocker |
| `frame-time` | CPU/GPU bottleneck, copy, upload or batching wins |
| `stability` | Crashes, device loss, memory growth, hangs |
| `rendering` | Correctness and completeness of the picture or gameplay |
| `experimental` | Opt-in, off by default, or not merged; hold |

An entry that mentions a proprietary component, the Windows SDK,
"not merged" or "experimental" is pushed towards `experimental` even when
it also promises speed. A hold list records entries that cannot be ported
without proprietary material.

## Porting rules

- Keep the upstream repository, PR number and, where available, the commit
  SHA in the local commit message.
- Port behaviour, not files: adapt to this tree's naming and `cfg` layout.
- Add a test for the ported behaviour where the upstream change had one.
- Never copy game assets, retail binaries, console SDK libraries or keys.
- Record the provenance in `CREDITS.md` and run `cargo test -p xtask`
  before committing.
