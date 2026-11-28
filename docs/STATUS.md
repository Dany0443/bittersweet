# Linux status and roadmap

This page records the current, locally verified state of the Linux/Proton
effort. External project status is not tracked in this repository; agents
should research current upstream sources when a task depends on them.

## Latest validation: 2026-11-28

With `VICTORY_AMBIENT=1` and the ray-tracing structure builder off, the
warmed RX 7600 build applied the fallback and reached the planned
155-second timeout without a logged GPU fault or memory stop. The median
was 2.9 fps across 17 world samples, with up to 5,883 draws per frame. A
late indoor frame showed Lucia's face and hair. Other frames were dark or
washed out. This trial had more free swap at launch than earlier runs and
did not isolate the fallback's visual or performance effect.

The same executable with a 1 MiB minimum for GPU-side buffer comparisons
placed 53 buffer copies on the card within the existing 512 MiB cap. Late
CPU scans covered roughly 171 to 178 MB per frame, compared with roughly
258 to 282 MB in late samples from the preceding run. Median world frame
rate was 3.2 fps across 19 samples, with 38% median card utilization and
5,987 MiB peak VRAM use. The run reached the 155-second limit without a
logged GPU fault. Scene mix and starting memory differed between runs, so
the trial shows less CPU scanning without establishing a sustained speedup.
The user observed 4 to 5 fps in the first seconds, then 3 to 4 fps.

The next trial used the same executable and 1 MiB threshold with
`VICTORY_RELAY=2` and `VICTORY_LATE_WAIT=3`. RADV logged a GPU virtual
memory fault and the runtime lost the device after 54 seconds. The only
world sample measured 0.5 fps. The relay remains off in the test profile.

The opt-in guarded-view shortcut passed nine Proton write-watch tests,
including tracked writes and the fallback for unarmed blocks. The warmed
title run reached the account picker, then exited after 86 seconds before
producing a world sample. Neither a memory stop nor a GPU fault was logged.
The runtime savings remain unmeasured.

Two further runs used the same warmed executable and scripted input. With
the shortcut enabled, 6,553 of 7,021 and 8,781 of 9,341 write-watch lookups
per frame used armed guards in the two world samples. Write-watch work took
18.6 and 23.2 ms in those samples. The median world rates were 2.6 fps with
the shortcut off and 3.2 fps with it on. The memory monitor ended both runs,
and the enabled run began with less free swap and produced only two world
samples. That confirms the kernel calls were avoided and leaves the FPS
effect uncertain.

A 960 by 540 internal resolution trial produced a median of 3.15 fps over
16 world samples. The user reported worse textures and objects. Restoring
1280 by 720 with the same executable and settings gave a median of 2.9 fps
over seven samples. Characters still appeared dark, and the indexed texture
queue reported zero uploads with about 30,000 slots waiting in late samples.
The monitor ended the restored-resolution run after 105 seconds under memory
pressure. The cutscene coverage and starting memory differed between the runs.
Keep the original scale while investigating texture residency and lighting.

At world load in the restored-resolution run, Proton reported 1,583 MiB
of available physical RAM and 3,825 MiB of available commit, which could
pause indexed uploads under the configured 2,048 MiB reserve even with
VRAM available. A later run with a 1,024 MiB reserve uploaded 1,429 indexed
slots (152 MiB) with 26,941 waiting in its only world sample. The monitor
stopped it after 53 seconds with 912 MiB RAM available and no free swap.
Starting memory differed, and the shorter trial left visual quality and
performance unverified. The default test reserve remains 2,048 MiB.

The SDWA half-result fix passed GPU readback on the RX 7600 under Proton.
The warmed game run started with 6,098 MiB of available RAM and 2,478 MiB
of free swap. The monitor stopped it after 47 seconds with 251 MiB of RAM
available and no free swap. It produced no world sample, so the fix's
effect on game visuals remains unverified.

Two RX 7600 runs used the same warmed build at 1280 by 720 internal
resolution. At the first upscale the pool had created five transition
command lists, compared with 17 with pooling disabled. The memory
monitor stopped the runs after 53 and 45 seconds. The disabled run also
logged a RADV GPU virtual-memory fault. It started with less free swap.
Neither run produced enough world frames to measure gameplay speed or
establish whether pooling affects device stability.

The indexed-buffer regression reproduced a refresh choosing a newly
registered overlapping buffer while the slot still pointed to the old
copy. The fix retained the bound copy and passed the Proton descriptor
tests. The warmed title run started with 1,121 MiB of free swap, reached
zero and exited during world loading after 42 seconds. It produced no
world sample, and the logs identify neither a GPU fault nor an exit cause.

A 256 MiB synthetic upload on the RX 7600 under Proton stayed in shared
memory with either setting of `no_upload_hvv`. An explicit GPU upload
heap placed the full 256 MiB in VRAM. Filling it from the CPU took about
59 to 61 ms, compared with 26 to 27 ms for ordinary upload memory.
The GPU copy readback passed. The VRAM path is opt-in for texture staging
and command-list scratch while its gameplay impact is assessed.

The warmed RX 7600 build used GPU upload heaps and presented the opening
3D scene at 1280 by 720 internal resolution. Its only world sample was
1.2 fps with 2,992 draws per frame, and characters still appeared dark.
The memory monitor stopped it after 45 seconds with no free swap. A run
of the same executable with GPU uploads disabled started with 556 MiB
less free swap and exited during world loading after 46 seconds with
no world sample. Neither logged a GPU fault. These runs establish that
the new path is used, but leave its gameplay impact unmeasured.

## Validation: 2026-11-27

The merged RX 5700 build reached the first 3D cutscene and presented
changing frames from 219 through 269 seconds. Several late world samples
took about two to three seconds per frame, with longer stalls and dark
or missing materials. The run reached its planned 270-second timeout
without a GPU reset or an OOM kill. Sampled peaks were 6,399 MiB VRAM,
7,365 MiB RAM and 2,847 MiB swap. The preceding run peaked at 8,104 MiB
VRAM and was stopped by systemd-oomd. The builds and starting memory
conditions differed, so the comparison does not isolate a single fix.

The RX 7600 trials of merge `b8e5a19` used the same warmed executable
and Series S profile. The default submission path produced one substantial
world sample at 1.2 fps with 2,448 draws. With `VICTORY_LATE_WAIT=1`, three
world samples ranged from 2.4 to 3.4 fps with 576 to 3,357 draws. The
memory monitor stopped both runs after 53 and 61 seconds respectively.
Starting RAM and swap varied, and the samples covered different stages of
loading. The results do not establish a gameplay speedup.

The overlap run still copied 451 to 628 MB per frame in its two denser
samples. A queue report attributed 54.4 ms of its 67.1 ms GPU wait per
frame to title memory changes overlapping unfinished shader writes.
Reported card load was typically low and the denser samples still dropped
423 to 1,378 draws for missing pipelines. Neither run logged device loss.

The per-heap texture remap regression passed on Proton using two D3D12
view heaps. Both heaps scheduled the texture update independently. On the
RX 7600, the warmed build reached early world rendering. The first
world sweep found that its remap history had expired and scheduled a
rescan of 21,039 placed slots. The memory monitor stopped the run after
45 seconds with 986 MiB RAM available and 49 MiB free swap. No dense
world sample followed, so the run leaves material appearance and
sustained frame rate unverified.

With `VICTORY_RAY_MISS_ONLY=1`, the RX 7600 reached the 3D world at
1.2 and 2.3 fps in two samples. Those samples issued 994 and 1,276 draws
per frame. Almost all of the 1,365 and 982 dropped operations per frame
were stages of the disabled ray-tracing structure builder. The run
ended at its 155-second limit with no logged GPU fault. Available RAM
fell as low as 952 MiB, and free swap reached zero. The trial did not
establish a speedup or verify world material appearance. The mode stays
opt-in.

The write-watch journal now reports the latest observed write generation
to each refresh. The Proton regression test verifies that a cached query
stops reporting an already copied page as dirty and still detects a later
write. The cold RX 7600 trial entered the world with more than 1,600
pipelines pending and took 13 to 25 seconds per frame. A separate run with
ray-miss mode off and 2,422 prewarmed pipelines drew the account picker
and loading screens at about 30 fps. The title also produced audio in its
PipeWire stream at startup. The memory monitor stopped the run during
world loading with 657 MiB of available RAM and no free swap, before any
world performance sample could be measured.

With window reuse enabled on the same warmed build, a separate trial
reached 3.5 fps at 501 draws and 1.7 fps at 1,710 draws. The cache reported
2,048 reused range lookups representing 176 MiB of avoided duplicate
windows. The memory monitor stopped the trial after 71 seconds with
942 MiB of available RAM and no free swap. Starting memory and input timing
differed from the preceding trial, so the results cannot isolate a change
in frame time or prove that the memory savings persist in gameplay.

The Proton xmem suite passed 11 tests with the DirectStorage page-view
notifications, including a write that spans two pages and reaches two
views of the same page. The warmed RX 7600 build reached the first 3D
cutscene, with world samples from 1.5 to 3.2 fps and up to 4,254 draws per
frame. A captured frame showed character silhouettes and highlights, with
materials still very dark. The monitor stopped the run after 83 seconds
with 754 MiB of available RAM and no free swap. Starting free swap was
higher than in the preceding trial. The run leaves any texture or
performance improvement unverified.

The RX 7600 reproduces the scalar min/max SCC error addressed by DexNom
PR #139. The corrected lowering passes GPU readback checks for signed and
unsigned min/max, zero, equal operands and selected values under Proton.

Shared texture-address tables from DexNom PR #118 retain byte-identical
addressing and pass concurrent sharing and texture round-trip tests. In a
synthetic 10,000-layout construction/clone benchmark, requested allocations
fell from 15.72 MB to 4.96 MB and construction time from 10.27 to 10.71 ms
to 2.25 to 2.49 ms across three runs per version. These are allocation and
construction measurements, not gameplay frame rates.

Wine's compiler rejected the original mesh converter's raw UAV loads.
The RX 7600 trial used structured dword accesses and an explicit root
signature to test indexed and expanded output, count limits and 64-bit
address carry. The merged converter reads inputs through SRVs and uses
a structured UAV for its output.

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

The experimental native compute path translated at least 256 creation
requests under Proton. At least 1,024 requests still lacked a usable
translation. The enabled trial presented one draw per frame and a black
window at the observed startup checkpoint. It was stopped manually after
64 seconds. The same executable with the switch off reached the world.
This path remains opt-in. The test validates that real descriptions reach
the translator, while the rendered output and execution of the translated
programs still need investigation.

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

The RX 5700 rendered the character and outdoor geometry in a bounded
Series S profile run on 2026-11-27. The test used GE-Proton11-12 with local
VKD3D-Proton build `71cae09` and the compute mesh fallback. Visible 3D
appeared after more than three minutes, with dark and missing materials.
Sampled world frames took 14 to 27 seconds. Peak use was 7.4 GiB RAM,
7.9 GiB VRAM and 4.0 GiB swap. Systemd-oomd stopped the test scope after
231 seconds under system-wide memory pressure. The kernel logged no new
GPU reset. Sustained play remains unverified.

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
