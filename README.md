# bittersweet

A Linux/Proton compatibility layer for the Xbox Series X|S build of the
target title, built on the open [`klbf-lab`](https://github.com/PastiiJay/klbf-lab)
codebase.

`bittersweet` keeps the upstream loader, console-API layer, D3D12 backend
and shader translator, and adds a Linux host layer: a read-only machine
probe, a Proton-GE setup script, GPU-vendor policy, and a small CLI that
turns the probe into a runnable environment. It contains **no game files**
and never ships or downloads a retail executable. You supply your own
legally obtained files.

## Status

This is a work in progress. Be clear about what works today:

- **Builds and tests on Linux.** The whole workspace compiles on Linux,
  including the Windows-target runtime crates, after `cfg` gates. The
  portable half plus the new host layer passes `cargo test --workspace`.
- **Linux host tooling is usable now.** `setup.sh` prepares Proton-GE, a
  Wine prefix and the kernel tunables; `bittersweet doctor` and
  `bittersweet plan` inspect the machine and emit an environment file.
- **The title does not run on Linux yet.** The Windows runtime still talks
  to host D3D12 through Windows APIs. The gates make the code portable, not
  functional under Proton. Do not expect a playable frame rate from this
  tree; that is the next milestone.

The inherited runtime receives attributed shader and texture ports. Recent
changes correct helper-lane reads, inactive-lane swizzles, DPP fetch-inactive
handling and zero-stride raw-buffer accesses. Large texture-content checks
use parallel BLAKE3 with identical checksums. Synthetic tests run on Linux;
these checks do not establish title compatibility or frame-time gains.
DexNom ports add uniform static-sampler choices, restored wave64 masks,
source-less/CMPX decoding fixes, pixel quad operations, cube-gradient mip
selection and shader optimizer analysis reuse.

## Layout

| Path | What it is |
|---|---|
| `crates/bittersweet-host` | Dependency-free Linux host policy: probe, plan, GPU/wave/VRAM rules, cache and log paths |
| `apps/bittersweet` | `bittersweet` CLI: `doctor`, `plan`, `check`, `pipeline` |
| `crates/victory-*` | The upstream runtime: PE loader, console APIs, D3D12.X → host D3D12, shader translation, formats, pipeline cache |
| `apps/victory` | The Windows runtime front end (`run`, `inspect`, `shader`, and the Windows-only `translate`) |
| `setup.sh` | Idempotent Proton-GE/prefix/kernel setup |
| `docs/` | Architecture, beginner guide, asset policy and Linux status |

See [`docs/architecture.md`](docs/architecture.md) for the data flow and
crate responsibilities, and [`docs/STATUS.md`](docs/STATUS.md)
for what works, what is missing and the ported/pending Linux fixes.
AI coding agents should follow [`AGENTS.md`](AGENTS.md).

## Quick start (Linux)

Requirements: Rust (see `rust-toolchain.toml`), Git, and the usual build
tools.

```bash
git clone <this repository> bittersweet
cd bittersweet
cargo build --workspace
cargo test --workspace
```

The host CLI has no runtime dependency on Wine:

```bash
cargo run -p bittersweet -- doctor
cargo run -p bittersweet -- plan --write .runtime
cargo run -p bittersweet -- check --cache
```

`doctor` reads `/sys`, `/proc` and `/etc/os-release` only. On multi-GPU
hosts it plans for the DRM card with the most reported VRAM, so a small
integrated GPU does not hide a discrete card. This is a planning heuristic;
it does not select the rendering device in Proton. `plan` writes
`.runtime/bittersweet.env`, a shell-sourceable file with the cache paths,
VRAM budget, wave policy and vendor flags. Neither opens a game binary.

### Setup script

`setup.sh` is interactive by default and safe to re-run. It checks the host
tools, builds and runs the planner, can download the latest Proton-GE
release and verify its published SHA-512 (or accept an existing build),
writes the prefix environment, and offers to raise `vm.max_map_count`.

```bash
./setup.sh                       # guided
./setup.sh --non-interactive     # defaults, no prompts
./setup.sh --proton /path/to/GE  # existing build
./setup.sh --proton auto         # download latest GE
./setup.sh --init-prefix         # create the prefix with wineboot
```

It never searches for, copies or launches a title. The only optional launch
is `wineboot`, and only when you ask for it.

## Legal

- Licence: **GPL-3.0-or-later**. The base and every project code was taken
  from use the same licence. See [`LICENSE`](LICENSE) and
  [`CREDITS.md`](CREDITS.md).
- No game assets, retail binaries, console SDK libraries or keys are
  included. The asset policy is enforced by `cargo test -p xtask`.
- This is an independent project, not affiliated with or endorsed by the
  platform holder or the publisher. Names are used only to describe
  compatibility.
- Ported fixes keep their upstream repository and pull-request number in
  the commit message and in [`CREDITS.md`](CREDITS.md).
