# Linux status and roadmap

This page records the current, locally verified state of the Linux/Proton
effort. External project status is not tracked in this repository; agents
should research current upstream sources when a task depends on them.

## Latest validation: 2026-11-27

The RX 7600 reproduces the scalar min/max SCC error addressed by DexNom
PR #139. The corrected lowering passes GPU readback checks for signed and
unsigned min/max, zero, equal operands and selected values under Proton.

Shared texture-address tables from DexNom PR #118 retain byte-identical
addressing and pass concurrent sharing and texture round-trip tests. In a
synthetic 10,000-layout construction/clone benchmark, requested allocations
fell from 15.72 MB to 4.96 MB and construction time from 10.27 to 10.71 ms
to 2.25 to 2.49 ms across three runs per version. These are allocation and
construction measurements, not gameplay frame rates.

The newly fetched mesh fallback needed Linux adaptation: Wine's compiler
rejects raw UAV loads. Structured dword accesses and an explicit root
signature now pass five indirect-mesh tests under Proton, including indexed
and expanded output, count limits and 64-bit address carry. This tests the
converter on the RX 7600; RX 5700 world rendering remains unverified.

A controlled buffer-window reuse trial with the restricted draw lookup
reached world rendering at roughly 2 to 3 fps. The monitor stopped it after
99 seconds at 944 MiB available RAM and no free swap. No GPU fault was
observed before that stop. It establishes neither a performance gain nor
stability; reuse stays off by default. Stable 20 to 30 fps with complete
textures and objects remains unachieved.

After rebuilding these ports and warming the pipeline cache, a second trial
with window reuse disabled reached world rendering at 1.7 to 3.9 fps before
the monitor stopped it after 63 seconds, at 979 MiB available RAM and no
free swap. Starting memory and swap conditions differed, so run length and
frame rates are not a controlled comparison with the reuse trial. The new
ports do not establish a gameplay performance gain. Linux process mappings
confirmed substantial resident and swapped memory during loading; CPU
copying, descriptor sweeps and missing pipelines remain active work.

An RX 7600 trial with `VICTORY_CARD_WINDOWS_MB=768` allocated 768 MiB
of extra device-local buffer copies. Reported VRAM use peaked at 6119 MiB,
compared with 4708 MiB in the prior run. Shared system memory still reached
3974 MiB. These buffers keep CPU staging copies, and several world samples
sent more than 250 MB to the GPU per frame. The trial was stopped manually
after 124 seconds. World samples with at least 500 draws ranged from 1.2
to 2.8 fps. The two runs had different memory conditions and draw counts.
The setting stays experimental.

The descriptor sweep now caches ranges per heap. In a synthetic sweep
with 85,098 descriptors, 14,464 buffers and 15,463 changes between full
rebuilds, bookkeeping took 0.92 to 1.08 ms per pass, down from 1.84 to
1.94 ms across three trials. Rewriting every slot was slower with the cache.
The warmed RX 7600 build reached 1.7 to 2.7 fps in two world samples with
at least 500 draws. The monitor stopped it after 75 seconds, at 902 MiB
available RAM and 383 MiB free swap. The run still dropped draws for
missing pipelines and did not establish a gameplay speedup.

## What works today

| Area | State |
|---|---|
| Workspace build | `cargo build --workspace` and `--release` succeed on Linux |
| Tests | `cargo test --workspace` passes; Windows-only self-image tests are gated |
| Host probe | `bittersweet doctor` reads sysfs, procfs and `/etc/os-release` only |
| Host plan | `bittersweet plan` writes cache paths, VRAM budget, wave policy, vendor flags |
| Setup | `setup.sh` prepares Proton-GE, prefix and `vm.max_map_count`; idempotent |
| Windows runtime | Windows-target release cross-builds on Linux; account/player prompt rendered under GE-Proton11-12 |
| First mission | Not verified; simple scaling presents changing 3D and composed credits, but frame times remain far above the target |

Local execution on 2026-11-26 used a Ryzen 5 7600X, Radeon RX 7600,
Mesa RADV 26.2.7 and the Series S profile. All imports bound, and the
account/player prompt rendered after the presentation fixes. Synthetic GPU
tests under Proton create both compositor and both depth-helper pipelines.
These observations do not establish mission compatibility or world frame times.

Further tests with GE-Proton11-14 (vkd3d-proton revision
`v3.1-83-gcae8353ed`) passed the mesh-conversion abort observed with
GE-Proton11-12. Explicit root signatures repair the memory-comparison,
simple-scaling and output-packing helpers under Wine. With
`VICTORY_FFX_SIMPLE=1`, runs on 2026-11-26 presented changing 3D scenes
and overlaid credits. The packed-byte dot port raises host-pipeline creation from seven to 14
of 20 captured compute programs. The console upscaler remains incomplete;
simple scaling has no temporal reconstruction. Lighting faults remain.

On 2026-11-27 the rebuilt memory probe verified coherent CPU/GPU reads
and writes for private memory. File-mapping imports opened successfully
but failed actual data checks. Importing a mapped view by its CPU address
also failed GPU reads, copy writes and shader writes. Both mapped-memory
paths must retain copies on this setup.
The opt-in guarded in-process copy path passes 23 host tests under Proton
both enabled and disabled. In an isolated benchmark, 1,000 64 KiB writes
took 1.09 ms in process versus 24.67 ms through the kernel call. This is
not an end-to-end frame-rate measurement.

Two bounded title runs reached world draw submission. With in-process
copies enabled, sampled copy-back time fell to roughly 35 to 43 ms during
cold world loading, but large uploads, pipeline creation and memory
pressure still produced multi-second frames. A later sample with no
pipelines pending remained about 0.9 fps. No sustained title frame-rate
improvement, player-controlled mission or corrected intro timing is
established. The prior window capture was unavailable because the title window was
unmapped. Diagnostics now read back composed presentation as well as direct
copies; a synthetic two-layer hardware test verifies both layer colors.
The earlier visible-scene evidence remains separate.

A subsequent 176-second title test verified the composed capture path in
the real runtime: saved frames show the character, street and vehicles.
World samples remained about 0.2 to 0.7 fps with no pipelines pending.
Memory-mapping changes took roughly 979 ms per frame during streaming,
and process samples found wineserver using 70 to 90 percent of one core.
These measurements motivate testing the current cool_fox Wine mapping
optimization; they do not establish its benefit here. An isolated server
built from GE-Proton's base Wine revision failed the client protocol check
and was not used for a title test. Reproducing GE-Proton's release patches
and regenerating the protocol resolved that mismatch.

The matching private server with cool_fox PR 71's mapping view tree passes
a 16,384-view synthetic test, including data tags, shuffled unmaps and
overlap rejection. Mapping and unmapping took 500 ms versus 1,032 ms with
the stock server. A 226-second title comparison with the same executable
and graphics settings produced substantial 3D samples around 2 to 3 fps;
mapping time generally fell to single or tens of milliseconds per frame.
Later multi-second hitches, dropped draws and dark character materials
remain. These are advancing scenes, not a fixed-camera benchmark or proof
of mission playability. No device-removal fault occurred before the manual
stop, which does not establish stability. The original Proton installation
is unchanged; this server remains an external diagnostic build.

A bounded GPU material probe recorded 119 field observations in a later
158-second test. All sampled records used the regular deferred character
shader, and their source buffers still matched when reported. Fields zero
and four had resident views in all 17 records; several later fields were
null. This does not prove that missing color textures cause the black
characters, or that resident resources contain correct texels. Sampling now
reserves capacity for both character shader variants. The probe does not
change upload scheduling, and the character rendering fault remains open.

A prebuilt run reached world rendering with no pipelines pending and still
ran around 0.7 to 1 fps. It stopped after 164 seconds with a RADV GPUVM
read fault and device removal. A breadcrumb-enabled diagnostic run
continued for 422 seconds before being stopped manually without that
fault, but was slower. The failure remains unresolved; this does not
establish stability. The Proton build reports no D3D12 debug layer.

A synthetic regression reproduced globally coherent root-UAV loads being
rejected as typed root views. The translator now preserves coherence in
both handle properties and resource metadata, with mixed load/store DXIL
validation under Proton. The local structure-builder DLL now creates all
52 offline pipelines, up from 48. A quarantined title test then passed the
all-or-none guard with 53 runtime pipelines ready. The memory monitor
stopped that run at 55 seconds. A baseline with the builder disabled and
captures off also hit memory pressure, after 115 seconds. Its early world
samples were about 1.7 to 2.5 fps, with 280 to 400 ms preparing submissions,
520 to 890 MB of buffer copies per frame and 70 to 135 ms on the GPU.
These runs do not establish a lighting fix, a performance gain or sustained
builder correctness; the builder remains opt-in.

The per-list movie-plane upload port fixes a synthetic stale-frame GPU
readback regression. On 2026-11-27, the user tested the updated Proton run
with timed input disabled and confirmed smooth intro playback and arrival
at the main menu. This is user-confirmed visual behavior, not a measured
video cadence or evidence that world memory pressure is resolved.

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
   mission loading, sustained world rendering, memory pressure and player
   input still need end-to-end verification.

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
