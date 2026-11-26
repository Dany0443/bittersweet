# Linux status and roadmap

This page records the current, locally verified state of the Linux/Proton
effort. External project status is not tracked in this repository; agents
should research current upstream sources when a task depends on them.

## What works today

| Area | State |
|---|---|
| Workspace build | `cargo build --workspace` and `--release` succeed on Linux |
| Tests | `cargo test --workspace` passes; Windows-only self-image tests are gated |
| Host probe | `bittersweet doctor` reads sysfs, procfs and `/etc/os-release` only |
| Host plan | `bittersweet plan` writes cache paths, VRAM budget, wave policy, vendor flags |
| Setup | `setup.sh` prepares Proton-GE, prefix and `vm.max_map_count`; idempotent |
| Windows runtime | Windows-target release cross-builds on Linux; account/player prompt rendered under GE-Proton11-12 |
| First mission | Not verified; world draw submission is reached, but the main presentation plane remains nearly black |

Local execution on 2026-11-26 used a Ryzen 5 7600X, Radeon RX 7600 XT,
Mesa RADV 26.2.7 and the Series S profile. All imports bound, and the
account/player prompt rendered after the presentation fixes. Synthetic GPU
tests under Proton create both compositor and both depth-helper pipelines.
These observations do not establish mission compatibility or world frame times.

Further tests with GE-Proton11-14 (vkd3d-proton revision
`v3.1-83-gcae8353ed`) passed the mesh-conversion abort observed with
GE-Proton11-12. The GPU memory-comparison helper now compiles and passes
synthetic hardware readback for changed pieces and hash updates. A title
run used that path and reached world draw submission. A separate run
selected the main presentation plane and read it back directly: the
observed world frames remained nearly black, independently of overlay
composition. Shader translation still rejects descriptor-only data reads
and some compute programs. RAM and swap pressure also causes long frame
times on this 14 GiB host. Neither test verifies the first mission.

## What is missing

1. **Mission-level execution evidence.** Proton reaches the account/player
   prompt and publisher logo. The current copy passes the two source-range
   decompression checks that failed on the earlier copy. DirectStorage now
   waits for explicit submission of partial batches; all 16 Windows-target
   storage tests pass under Proton. Mission loading remains unverified.
2. **Wine memory semantics.** Title memory is not always mapped in place
   under Wine. Upstream builds copy it, and one class of copies must not be
   sparse buffers or the GPU resets when the world loads.
3. **Pipeline-library behaviour without host support.** vkd3d-proton can
   refuse the empty host library blob, which is why the early-pipeline list
   must still be written when no library exists.
4. **Shader-conversion recursion limits.** vkd3d-proton converts DXIL on
   the calling thread; large traversal programs need one latch per loop and
   a larger worker stack.
5. **World rendering and input validation.** Startup runs under Proton, but
   mission loading, world rendering, memory pressure and sustained input
   still need end-to-end verification.

## Ported here

| Fix | Upstream | Why it matters on Linux |
|---|---|---|
| AMD family table and mesh gate | Scarlight #80 | RADV does not expose mesh shaders below RDNA2 and aborts on Vega |
| Capped early-pipeline list | Scarlight #78 | Truncation is counted instead of losing keys silently |
| Empty environment paths | VIctory #2579 | An exported-but-empty `HOME`/`XDG_CACHE_HOME` is not a directory |
| Empty log filters | VIctory #2578 | An empty filter no longer hides the default one |
| Warm-up/pre-build conflict | klbf-lab #7, #8 | The two modes compete for the same workers and cache |
| `cfg` gates, not a fork | VIctory #2334 approach | Keeps one tree for Windows and Linux |
| One latch per loop | klbf-lab #12 | vkd3d-proton recurses forever on multiple back edges |
| Larger pipeline stacks | klbf-lab #12 | DXIL conversion overflows 2 MiB and hangs under Wine |
| DPP inactive-source handling, swizzle zeros, helper-lane reads, zero-stride raw buffers | Scarlight #103 / project-flamingo #684, #697, #686, #720 | Partial port; synthetic Linux lowering tests pass, host GPU execution unverified |
| Raw VCC scalar branches | Belobug #83 / VIctory #2406 | Makes the rule explicit even with an attached EXEC dependency |
| Parallel large-texture content hashes | klbf-lab #28 | Portable checksum-equivalence tests pass; Windows staging call sites and runtime speed unverified |

DexNom ports also cover uniform static-sampler selection, restored wave64
masks, source-less VOP1 and VOP3 CMPX decoding, pixel quad operations,
cube-gradient mip selection and optimizer analysis reuse. Synthetic Linux
tests pass and Windows shader tests type-check. GPU readback, frame-time
changes and title execution remain unverified.

## Pending, in priority order

| Item | Upstream | Status |
|---|---|---|
| Save the early list when there is no host library | victory-fork #259 | Not ported; Windows-only code, needs a Windows build to verify |
| Root signature made beside Wine-compiled HLSL | victory-fork #1011 | Compositor, depth helpers and indirect clamp ported; synthetic Proton GPU tests pass |
| Compile runtime HLSL with DXC when the system compiler refuses | victory-fork #20 | Not ported; Windows-only |
| Depth twin's root signature when the compiler drops it | VIctory #1872 | Not ported; Windows-only |
| Title memory copied, not in place, under Wine | Belobug #53 | Not ported; needs the Linux backend first |
| Non-sparse copies under Wine | Belobug #69 | Not ported; same |
| Empty host library from a null blob | Belobug #124 | Not ported; same |

Windows-target checks and Proton GPU tests are now available on this host.
Remaining ports need focused regressions and runtime evidence.

## Risk register

| Risk | Current handling |
|---|---|
| Proprietary material | No game files, executables or SDK material are included or linked |
| Upstream changes | Agents research current sources per task; no shared snapshot is maintained |
| Bit-rot in the base | The base snapshot is pinned; every port carries its upstream PR number |
| False confidence from a green build | This page and the README state that the runtime is not functional on Linux |

## Next milestones

1. Continue mission-loading validation against the current user-supplied files.
2. Fix observed shader, storage and GPU failures with focused regressions.
3. Verify mission rendering, input and memory use under Proton.
4. Validate cache behavior and longer sessions after mission startup works.
