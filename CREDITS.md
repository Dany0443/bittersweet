# Credits and provenance

`bittersweet` is a Linux/Proton compatibility layer built on the open
`klbf-lab` codebase. It is distributed under **GPL-3.0-or-later**, the
licence shared by every project whose code it contains.

This file records where the code came from and which licence applies. It is
kept in sync with the commit messages; when a fix is ported, the upstream
repository and pull request number are named there too.

## Provenance chain

| Project | Role in this repository | Licence |
|---|---|---|
| [PastiiJay/klbf-lab](https://github.com/PastiiJay/klbf-lab) | Base snapshot, commit `c8193ee` | GPL-3.0-or-later |
| [Marvisak/VIctory](https://github.com/Marvisak/VIctory) | Upstream of `klbf-lab`: loader, HLE runtime, D3D12 backend, shader recompiler | GPL-3.0-or-later |
| [fxceless/victory-fork](https://github.com/fxceless/victory-fork) | Upstream of `klbf-lab`: fork-specific fixes carried into the base | GPL-3.0-or-later |
| [pablogonz12/Scarlight](https://github.com/pablogonz12/Scarlight) | Ported fixes (AMD mesh-shader gate, capped early-pipeline list) | GPL-3.0-or-later |
| [myokex/victory-amd](https://github.com/myokex/victory-amd) | Fixes already incorporated into `klbf-lab` (pipeline gate, texture sweep) | GPL-3.0-or-later |

The base import intentionally drops upstream Git history. The original
copyright notices and the attribution above satisfy the source and notice
requirements of the GPL; the complete corresponding source is this
repository.

## Fixes ported on `linux-port`

| Local commit | Upstream | What was ported |
|---|---|---|
| `8ac2e69` | pablogonz12/Scarlight PR #80 | AMD family table; refuse mesh pipelines below RDNA2 and on Vega; record native wave width |
| `3945747` | pablogonz12/Scarlight PR #78 | Cap the early-pipeline key list, keep the first keys, count dropped entries |
| `9cec5e1` | Marvisak/VIctory PR #2579 | Treat an exported-but-empty `XDG_CACHE_HOME`/`HOME` as unset |
| `02d62b2` | Marvisak/VIctory PR #2578 | Ignore empty log-filter variables and fall through |
| `1fe8e03` | PastiiJay/klbf-lab PRs #7, #8 | Separate shader warm-up from pipeline pre-build; stop the input scan at its budget |
| `f11be15` | Marvisak/VIctory PR #2334 (approach) | Gate the Windows host surface with `cfg` instead of forking the runtime |
| `fd7beef` | PastiiJay/klbf-lab PR #12 | One latch per loop header for vkd3d-proton; larger pipeline worker stacks |

## Reference only, no code copied

| Project | Licence | Used for |
|---|---|---|
| [xcl-dev/xcl](https://github.com/xcl-dev/xcl) | GPL-2.0-only | Behavioural reference for the same title under another layer; fixes were written anew |
| [brcly/TuxBox](https://github.com/brcly/TuxBox) | Apache-2.0 | Cross-build and packaging reference |

No code from a GPL-2.0-only or Apache-2.0 project is present in this
repository, so neither licence is combined into the work. If that changes,
the combined licence must be revisited.

## Inherited credits

The `klbf-lab` snapshot already contains work ported from other projects
and test reports from their authors. Those contributions remain under
GPL-3.0-or-later as part of the snapshot; the original project READMEs and
commit messages are the authoritative record. Named there: Geckomacabre
(`vx-layer`, water readback), MiamiGame (`XTP`, XOR swizzle), JappertjeV
(DirectStorage page-list analysis), and the testers listed in the imported
`README.md` history.

## Third-party dependencies

Rust dependencies keep their own licences, recorded in `Cargo.lock` and in
each crate's manifest. They are linked, not copied into this repository.
