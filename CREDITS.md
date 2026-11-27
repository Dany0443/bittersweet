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

## Adaptations on main

- [Belobug/VIctory-Belobug-AMD PR #70](https://github.com/Belobug/VIctory-Belobug-AMD/pull/70),
  head `98ec4ec1805bfd0428221dd133e1cabdd6ceaec1`: partial shader-translator
  cleanup in `emit/lower.rs`. Removed the f16 conversion arms already handled
  by `valu16`, unused loop-entry argument, and no-op bindings. Retained
  `Module::int_value`: unlike the PR's base, our memory lowering still calls it.
  The PR's integer-dot and half-input changes do not apply to this snapshot.
  GPL-3.0-or-later, with the upstream authors' notices retained.
- Belobug PR #7's console formats 189–191 and the synthetic layout test
  already exist in the inherited base; no duplicate port was applied.

### Shader and texture ports

- **pablogonz12/Scarlight PR #103**, head
  `dcf0723e38506a96b9d631e2a32a489fa9d511f6` (pablogonz12), partially
  adapted from **JMAN730/project-flamingo** (GPL-3.0-or-later):
  - #684, `19dda7a549c35e194ee8399e01fa6a1b23b649a2`: DPP fetch-inactive
    and bound-control handling.
  - #697, `7b3b668599ae9f97e4dc1d71ba5e0d26bd3892bd`: zero inactive
    `ds_swizzle_b32` source lanes.
  - #686, `22066e545810775d9e9f064f5159178838067964`: pixel helper lanes
    use their own value for first-lane reads.
  - #720, `7e956a90af57653800bd2a80eafc8e979ba81f9c` (axknxla): zero
    bound stride means raw byte addressing for indexed buffer operations.
  Local adaptation ballots the structurised EXEC guard, and keeps mesh quad
  activity checks quad-local. The pixel Valid-Mask/export rewrite is not part
  of this port. Synthetic shader tests run on Linux; GPU execution is unverified.
- **Marvisak/VIctory PR #2493**, merge
  `f3578e386ea0f06153445af10a698df029620aad` (head
  `6cd2489d552c765f7fd4b327f5bf78493f090589`): decode the inline float
  constants 240..=248 as half values in whole-wave `v_add_f16` simulation.
  They were read through the f32 inline table, whose low bits made some of
  them look like an additive zero, so a wave sum could be folded as if the
  step were an identity. A focused wave32 test covers the zero identity and
  all nine inline constants.
- **codec-d/cool_fox**, snapshot `cool_fox-main.zip` (SHA-256
  `79f5bfe598730ddabfc01a1bc02a2fd5a70f80eb2642a6982321c232242838de`, no
  Git history in the snapshot): include the per-instruction
  `lanes.guards[b][i]` in `resource::Tracer::descriptor()` and `live()` when
  pruning dead lane edges at a join. A join access guarded by EXEC could
  otherwise be traced through an arm the guarded lanes never ran, losing a
  descriptor that only the live arm builds. The regression test
  `a_guarded_access_excludes_dead_lane_edges_at_a_join` is the upstream one.
  GPL-3.0-or-later.
- **Belobug/VIctory-Belobug-AMD PR #83**, head
  `803b19b710a6731cf84d923728d8aa43b6c4f55f` (dutch2005), from
  **Marvisak/VIctory #2406** (makmatvey): test raw VCC independently of EXEC
  in scalar VCC branches. Our IR already omitted EXEC from those reads;
  an additional regression covers an explicit EXEC dependency.
- **PastiiJay/klbf-lab PR #28**, head
  `39ad8490478fa205f8f88a74acd237a1e517f97a` (spy0eye-beep): parallel
  BLAKE3 texture-content checks at 1 MiB and above, with serial checks below.
  The helper is portable and checksum-equivalence tests run on Linux; the
  four texture-runtime call sites remain Windows-target code. No title or
  frame-time measurement was performed. GPL-3.0-or-later.
- **JMAN730/project-flamingo #405**, commit
  `74c87f82d6dcbaaafc857aeb96d26790052d87c8`: GFX10 lowering for
  `v_mad_u32_u16`, `v_mad_i32_i16`, and `v_lerp_u8`. Adapted to bittersweet's
  integer operand decoding; unsupported modifiers remain rejected. This
  commit is also present in the high-rated Belobug AMD fork. Linux lowering
  tests cover translation, with runtime cases for Windows GPU/WARP. No RX
  hardware run was performed. GPL-3.0-or-later.
- **fxceless/victory-fork**, commit `06702fe86b91bacbf4a0a27662ae443435bf732e`:
  BC7 rotation handling from upstream VIctory #3111. Rotation control bits
  affect channel precision and endpoint order only in modes 4 and 5. Added a
  regression covering reserved bits in the other modes and actual channel
  swaps in modes 4 and 5. GPL-3.0-or-later.
- **fxceless/victory-fork**, commit `e4245f3c1b0490250d9cf0946a2113000be98687`,
  adapting upstream VIctory #3198: retain textures that fail host allocation
  with `E_OUTOFMEMORY` as retryable and ask again on a later use after a
  one-second delay. Other allocation failures remain refused. Added a portable
  classification test. GPL-3.0-or-later.

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

## DexNom adaptations

All entries below come from DexNom/VIctory (GPL-3.0-or-later).

- Uniform static-sampler selection, commit `ddb73326b09e600bd2405ee5c987fa611098aaa1`: Sample both constant bindings and select the texel using the original uniform branch edge. Preserve derivatives and retain unreachable-edge handling.

- Restored wave64 mask recognition, commit `76749ce58da23a43a3500454a8b69c53f05aa477`: Trace both scalar-copy chains back to the same known mask in their original dword order. Reject a duplicated or replaced high dword.

- Source-less VOP1 decoding, commit `496fc09e035b90dc77574a4e645f127ea3123cf8`, PR #30: Keep v_nop, v_pipeflush and v_clrexcp one dword even when unused fields resemble literal, SDWA or DPP extension markers.

- VOP3 CMPX destination decoding, commit `496fc09e035b90dc77574a4e645f127ea3123cf8`, PR #30: Decode v_cmpx destinations as EXEC rather than encoded VDST. Preserve ordinary compare destinations.

- Shader optimizer analysis reuse, commit `4c9a1532034e30db7c54e9ce06ae7c5dcc9e5f2c`, PR #25: Reuse function effects and dominator analysis between cleanup rounds, rebuilding the tree after control-flow pruning. The Windows memory-transfer changes are not included.

- Pixel quad reads and quad-local DPP8, commit `921d8d3b43b44f3a6ca4b44095edd3c4baf10a87`: Use native quad reads for pixel and mesh quad permutations, specialize broadcasts/exchanges, and reject DPP8 selections that cross pixel quads. Use quad-local EXEC guards for helper activity.

- Cube gradient mip selection, commit `921d8d3b43b44f3a6ca4b44095edd3c4baf10a87`: Use DXIL base-2 Log directly and honor the instruction minimum LOD before SampleLevel. Add mip-capable synthetic cube views without changing the existing single-mip view constructor.

- Sparse texture residency, commit `921d8d3b43b44f3a6ca4b44095edd3c4baf10a87`: Count committed interior mip regions even when the first and last pages are absent, while retaining the existing fast page-query paths for ordinary allocations. Add a reserved-memory regression test.

## cool_fox adaptations

Integer render-target blend normalization and its synthetic tests are adapted
from the user-supplied `codec-d/cool_fox` source archive (GPL-3.0-or-later),
`crates/victory-gpu/src/pipeline/host.rs`. Both graphics and mesh pipelines
preserve color blending while disabling integer-target blending. The archive
contains no Git commit identity; its SHA-256 is
`79f5bfe598730ddabfc01a1bc02a2fd5a70f80eb2642a6982321c232242838de`.
The source file SHA-256 is
`e81023c24ffbf003b2ef80419310a71c87c11b36df1fa387c6b23b3ade7e78f5`.

Retail presentation-plane stride follows `cool_fox`'s `RetailPlane` in
`crates/victory-gpu/src/queue.rs`, from the same archive. The adaptation
retains this tree's rectangle and layer composition handling and adds the
six missing option words. Source file SHA-256:
`b1bd87c602264b182ba5edd33a3dc9840ec86a0e71329355fb8be3b3e24d9713`.

Depth-bounds pipeline and command-list handling are adapted from
**DoubleGhosty/cool_fox**, commit
`c96b4f2b8365bf43d54ab074940923ef8267f4c6` (the depth-bounds portion of
upstream PR #82 by Kaivyyy). The port preserves the `DEPTH_STENCIL1` flag,
uses pipeline streams when the host supports depth bounds, and replays
dynamic bounds after command-list cuts. A focused test covers state parsing
and pipeline-key differentiation. GPL-3.0-or-later.

Normalized typed-buffer conversion follows **DoubleGhosty/cool_fox**, commit
`9a6c829d6818a0ca0f1029d417e1e5eef6714eae`. The port uses integer quotient
construction for correctly rounded 8-, 10-, and 16-bit UNORM/SNORM loads.
Hardware and WARP regression cases cover normalized values and code
round-trips. The port also marks dynamically selected typed integer buffer
views and preserves their 16-bit integer payloads during `d16` loads and
stores. Descriptor classification tests cover integer, signed, normalized,
raw, and structured views. GPU execution of the `d16` path remains
unverified. GPL-3.0-or-later.

## Wine helper pipeline adaptation

Explicit helper root-signature serialization follows
`fxceless/victory-fork` PR #1011, commit
`d9e68ebb9a58f2abf35def0cf2b5a3ab714068c5` (GPL-3.0-or-later).
This adaptation supplies the compositor's static sampler and SRV table and
the depth helpers' SRV table independently of embedded HLSL attributes.
The indirect clamp also follows that PR's Wine-compatible structured UAV
accesses, atomic functions and explicit root signature. This tree keeps
its existing per-record work budget and split-dispatch behavior. Synthetic
GPU readback under Proton verifies record preservation, split output and
budget accounting.

The card-memory comparison helper applies the same structured-UAV and
explicit-root-signature technique to Wine's unsupported raw UAV `Load4`
and `Store4` calls. Its existing hash algorithm and comparison modes are
preserved. Synthetic hardware readback under GE-Proton11-14 verifies
changed-piece detection, hash updates and result-offset boundaries.

DirectStorage partial batches now wait for the title's explicit submit,
following `codec-d/cool_fox` from the source archive identified above.
A full queue still submits when no queued or running reads can make room;
`VICTORY_DSTORAGE_HALF=1` retains the earlier behavior for comparison.
Synthetic tests cover both partial-batch ordering and full-queue progress.
Source file `crates/victory-hle/src/modules/dstorage.rs` SHA-256:
`ba5ca8e567b72c74f4c0aef5941541188deed762fd650eb67b92b763f3f09222`.

The bounded `VICTORY_PIPELINE_WORKERS` override follows `cool_fox`'s
`crates/victory-gpu/src/pipeline/make.rs`, from the same archive. This
adaptation preserves bittersweet's default of half the logical processors,
bounded to 2 through 8 workers, and its larger Proton pipeline stack.
Source file SHA-256:
`e329581b29607a54a8e5a076b323ab31382e88ebcbe67ce85894c8cc2d93b3ca`.

## Descriptor dimension queries

Dimension-only descriptor reads follow `codec-d/cool_fox`, from the source
archive identified above. The port recognizes 2D height extraction and
structured-buffer count queries with a known stride. It excludes the
source's fallback that assumes an unknown eight-dword view is 2D.
Synthetic translation tests and hardware readback under GE-Proton11-14
verify dimensions and record counts; ambiguous reads remain rejected.
Source `crates/victory-shader/src/emit/lower.rs` SHA-256:
`086faeb508aaf4d0a50d2bd3c8439402fbb212f51c26631fb10e4382afb16dfd`.
Source synthetic tests `crates/victory-shader/src/emit/size_tests.rs` SHA-256:
`feac08a488c9c782f2a1974b214ddf89c90a059662d54943667c89c6380de925`.

## ACCC compute containers

ACCC/FSPX parsing follows `Marvisak/VIctory` commit
`63bcca43c6a00a758510398fa43096ce7e4f7df0`, including its synthetic
parser fixtures. Runtime and offline translation select ACCC when a DXIL
part is absent. Tests cover resource metadata, wave sizes, malformed
records and complete synthetic container translation. This establishes
container support; remaining instruction gaps prevent a complete upscale.

Loaded buffer descriptors converted to byte addressing follow upstream
commit `0b35b7fe7d728c9a9c205cb3719a0f11744321ce`. Tracking recognizes the
complete stride-clearing, byte-count and bounds conversion for untyped
accesses. Other descriptor changes remain rejected. The synthetic
positive and negative test is adapted from the same upstream behavior.
With container support, seven of 20 captured compute programs create host
pipelines locally; the full console upscaler remains incomplete.

The existing simple scaler and RGB9E5 output helper use explicit root
signatures for Wine, following the helper-pipeline technique above.
The scaler's root-signature attribute uses one string literal because
Wine rejects adjacent literals there. Focused Proton tests reproduce
both pipeline-creation failures before the changes and pass afterward.

The memory-import probe applies that same Wine helper technique: explicit
UAV root serialization and structured-buffer accesses. Synthetic hardware
readback verifies that its shader writes reach the CPU-visible buffer.
This repairs the probe and does not itself establish title-memory imports.


## DirectStorage worker file handles

DirectStorage file reads now use a separate read handle per worker, following
`codec-d/cool_fox` commit `fbc183085801939cac7b469eb0ce5956d8e0377f`.
`VICTORY_DSTORAGE_IO=one` restores shared-handle reads for comparison. The
port keeps the existing seek-based read path and falls back to the original
handle if a worker handle cannot be reopened.

Optional `overlapped` and `direct` read modes adapt the corresponding paths
from the `codec-d/cool_fox` snapshot identified above. They use explicit
Windows overlapped reads, with sector-aligned buffering for direct I/O. The
high-offset synthetic read test passes under Proton; these modes did not
change the invalid stream observed in the current RX 5700 runtime.

## DirectStorage worker limit

The `VICTORY_DSTORAGE_WORKERS` override follows LoverCatys/ruff-lab commit
`bffe04b`. It lets users limit concurrent storage reads on rotational drives;
unset or invalid values preserve the existing logical-processor default.

## RAM pressure cache cleanup

System-memory caches now become eligible for cleanup sooner when available RAM
falls below 5 GiB and 3 GiB, following LoverCatys/ruff-lab commit `3f43861`.
This changes the existing idle timeouts from 10/5/2 seconds to 8/3.5/1.5
seconds, respectively.

The optional `VICTORY_COMMIT_RESERVE_MB` setting also pauses new indexed
texture copies and shortens idle-copy retention when Windows commit approaches
the configured reserve. This follows `ruff-lab` commit `bffe04b` and its
attribution to ka1qux/BRVIctory-ka1qux commit `8cac16489a`.

## Updated cool_fox shader arithmetic

Packed signed-byte dot accumulation follows `DoubleGhosty/cool_fox` main
commit `c3e5c375bad6447bbd99057c828eb14925b0e503`. The supplied archive
has SHA-256
`9c2d6976f01b5abe0c4ad3723967f18adffc16a9ea991b73a3364e8734f4ad39`.
The adaptation uses the existing integer lowering path and DXIL packed
dot intrinsic. Synthetic GPU readback under GE-Proton11-14 verifies signed
bytes, an existing accumulator and wrapping results. This does not yet
establish complete upscaler support or a title frame-rate improvement.

## Indexed write-watch coverage

Large submission coverage lookup follows unmerged
[`DoubleGhosty/cool_fox` PR 5](https://github.com/DoubleGhosty/cool_fox/pull/5),
head `bbcb014787a3454c4361ecce486de44816463ee8`. That PR is closed because
its upstream main branch adopted a larger concurrent journal refactor.
This adaptation retains bittersweet's journal and indexes only its queried
ranges. Overlapping or adjacent queries do not imply union coverage.
Seven focused tests pass under GE-Proton11-14, including exhaustive
comparison with the prior linear containment rule through promotion and
reset. A local optimized microbenchmark of 20,000 lookups over 1,024
ranges improves from 4.05 ms to 0.22 ms; title frame-rate gains are unverified.

## Guarded copies under Wine

The opt-in `VICTORY_DIRECT_COPY=1` path follows open
[`PastiiJay/klbf-lab` PR 34](https://github.com/PastiiJay/klbf-lab/pull/34),
head `ee53a8465ecbf1868d306511e4cff45ee63dbfe1`, including its tests.
That PR attributes the base copy routine to BitWuehler's
`fxceless/victory-fork` PR 154 and adds guard-page preservation.
This adaptation reuses bittersweet's write-watch handler and falls back
to the kernel copy if vectored-handler registration fails. The default
path is unchanged. Synthetic Proton checks cover guarded, reserved and
read-only pages, sparse remapping and write-watch visibility.

## Full-width packed byte dot products

`v_dot4_i32_i8` support follows the DXIL intrinsic approach in
`DoubleGhosty/cool_fox` main at
`636954118fae4a42fc75c84520836da147003073`. The adaptation uses the
full-width operand rules in the
[AMD RDNA2 ISA](https://docs.amd.com/v/u/en-US/rdna2-shader-instruction-set-architecture)
and [LLVM's RDNA2 instruction reference](https://rocmdocs.amd.com/projects/llvm-project/en/latest/LLVM/llvm/html/AMDGPU/AMDGPUAsmGFX1030.html).
It also handles unsigned byte products and signed/unsigned saturation
using Microsoft's documented DXIL packed-dot opcodes 163 and 164.
Synthetic hardware readback under GE-Proton11-14 covers negative products,
full-width accumulators, wrapping and both saturation bounds.

## Settings-aware prebuild resumption

Prebuild completion records reuse the runtime shader-cache fingerprint.
This follows the fingerprint-scoped cache bookkeeping approach discussed
in `DoubleGhosty/cool_fox` PR 54, head
`b20312654b049dda333dbfa317dd1357af5fc6a3` (closed and unmerged when
inspected). This tree's shader-pack prebuild has a different list format;
it checks both translation identity and retained library size. Synthetic
list tests cover changed settings, legacy records and library truncation.

Scalar condition-code operands follow `DoubleGhosty/cool_fox` main
`636954118fae4a42fc75c84520836da147003073`, which attributes the behavior
to VIctory commit `0b35b7f`. The existing scalar flag path supplies the
Boolean, converted to the instruction's integer operand. Synthetic GPU
readback checks both comparison results and arithmetic carry flags.

## Experimental Wine mapping server

The private Linux runtime experiment uses the mapping view-tree patch from
[DoubleGhosty/cool_fox PR 71](https://github.com/DoubleGhosty/cool_fox/pull/71),
head `856c6cae04539207fe6eb37ecb7806d0183f5c8f`, merged into upstream main.
The server is built outside this repository from GE-Proton11-14's Wine base
`a47c73f52fd7b2efafc1c560710f3a5099ae51e0`, its release patches and
wine-staging `6cc805ea57132eeaf44764e9213823c9b8d0d300`. The complete release
protocol is regenerated before testing. Synthetic mapping churn and GPU
readback pass; title frame times improve but remain far from playable.
No Wine sources or binaries are distributed in this tree by this experiment.
