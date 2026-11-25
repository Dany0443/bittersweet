# Upstream sync — 2026-11-25T18:32:47Z

Sources: <https://vuyp.github.io/gta6-pc-race/>, <https://portwatch-vi.xyz/data/tracker.json>
Window: commits since `2026-11-25T00:00:00Z`  
Tracked repositories: **88**

## Newly discovered repositories

| Repository | Name | Family | Stage |
|---|---|---|---|
| [edsuuu/VI-BR-PORT](https://github.com/edsuuu/VI-BR-PORT) | VI-BR-PORT | unclassified | 6 |
| [LoverCatys/ruff-lab](https://github.com/LoverCatys/ruff-lab) | ruff-lab | unclassified | 5 |

## Incomplete API collection

- `mewiof/Holy-Interoperability`: prs; sync checkpoint preserved

## Renamed or moved

- `ZSlayerHQ/victory-world-runtime` now resolves to `ZSlayerHQ/zslayer-pc-port-runtime`

## Unreachable (flag kept, remote disabled)

- `batatinhafcbatatinha-pixel/xseriesrecomp`
- `Dark3581/AnyBox`
- `nullbrik/RageForge`
- `zohaiblazuli/GrandTheftXbox`

## Tracked through PortWatch VI

| Section | Repositories |
|---|---:|
| efforts | 35 |
| tools | 22 |
| candidates | 14 |

`candidates` are unreviewed and are not presented as trusted.

Nuked or deleted: `ItsNotRexed/Leonida`, `realdody/americas-recomp`, `Zettrox/ztrx`.

## Triage totals

| Bucket | Items |
|---|---|
| critical-linux | 1447 |
| frame-time | 2511 |
| stability | 2564 |
| rendering | 3306 |
| experimental | 583 |
| unclassified | 3544 |

## Critical Linux / Proton-GE fixes

- `Belobug/VIctory-Belobug-AMD` #165 — CI: the crates that run anywhere are tested on Linux too
- `Belobug/VIctory-Belobug-AMD` #166 — GPU: diagnostics builds log the title's asks for frame statistics
- `Belobug/VIctory-Belobug-AMD` #42 — Run on Proton: a Windows build of victory, launched inside a Proton prefix
- `Belobug/VIctory-Belobug-AMD` #41 — Compile and test on Linux, and an Ubuntu CI run
- `Belobug/VIctory-Belobug-AMD` #109 — Fix memory mapping and shader lowering
- `Belobug/VIctory-Belobug-AMD` #53 — Under Wine, title memory is copied, not used in place
- `Belobug/VIctory-Belobug-AMD` #69 — Under Wine, copies of title memory are not sparse buffers by default (GPU reset once the world loads)
- `Belobug/VIctory-Belobug-AMD` #124 — GPU: the empty host pipeline library is made from a null blob (vkd3d-proton refused it: no pipeline kept under Wine)
- `Belobug/VIctory-Belobug-AMD` #100 — GPU: fix Shadow::sparse_writing tile base for 4 KiB-aligned Map() pointers
- `boykopovar/AnyPS5` #26580 — fix(shader): round the comparison reference of 16_UNORM sampled reads as RDNA2 does
- `boykopovar/AnyPS5` #17144 — feat(libSceSystemService): implement AppMessaging queue and event flags
- `boykopovar/AnyPS5` #27272 — fix(libSceSaveData.native): raise filesystem errors from the module
- `boykopovar/AnyPS5` #25766 — feat(libc): add integer libc++ to_string
- `boykopovar/AnyPS5` #27260 — feat(libkernel): skip the end of file in poll with POLLINIGNEOF on sockets and pipes, as FreeBSD 11.4 does
- `boykopovar/AnyPS5` #21762 — fix(relinker): lower unaligned 256-bit stores for --to-rosetta
- `boykopovar/AnyPS5` #23443 — fix(libwebrtc): validate SequenceCheckerImpl constructor guest pointer before write
- `boykopovar/AnyPS5` #26280 — test(tools): split test_registration by test class
- `boykopovar/AnyPS5` #24319 — feat(libSceNKWebKit): implement six ContainerNode and Node element navigation exports
- `boykopovar/AnyPS5` #24332 — feat(libSceNKWebKit): implement the WebCore::FrameTree childCount, top, traverseNext and isDescendantOf exports
- `boykopovar/AnyPS5` #24339 — feat(libSceNKWebKit): implement the JSC::JSPromise status, result and isHandled exports
- `boykopovar/AnyPS5` #26662 — fix(agc): keep the old settle floor in agc_driver_linear_target_import_creation
- `boykopovar/AnyPS5` #24350 — feat(libSceNKWebKit): implement the WebCore::Position anchor constructors and containerNode exports
- `boykopovar/AnyPS5` #24915 — feat(relinker): record the host CPU features --to-avx output needs
- `boykopovar/AnyPS5` #24213 — feat(libSceJsc): implement JSC::JSCell::destroy
- `boykopovar/AnyPS5` #27259 — fix(libkernel): fill getrusage children and ru_maxrss from the host, as FreeBSD 11.4 does
- `boykopovar/AnyPS5` #27255 — fix(relinker): export absolute symbols within Windows images
- `boykopovar/AnyPS5` #22186 — feat(libSceIme): drive sceImeOpen with the controller keyboard
- `boykopovar/AnyPS5` #24399 — fix(libSceLibreSsl3): copy the sk_free stack guest record safely
- `boykopovar/AnyPS5` #18083 — feat(libSceFreeTypeOl): implement format support helpers
- `boykopovar/AnyPS5` #24124 — feat(libSceJsc): implement WTF::SHA1::hexDigest
- `boykopovar/AnyPS5` #22392 — fix(libkernel): follow firmware 13.60 for module unwind info flags and failures
- `boykopovar/AnyPS5` #26303 — feat(libc): implement the float complex trigonometric, exponential and power functions
- `boykopovar/AnyPS5` #22390 — fix(libSceIcu): copy tokenize and UTF-16 memory guest pointers safely
- `boykopovar/AnyPS5` #26238 — feat(libkernel): implement execl, execle and execlp
- `boykopovar/AnyPS5` #26736 — fix(tools): preserve progress history cache inputs
- `boykopovar/AnyPS5` #22622 — fix(libSceNpScoreRanking): validate npId and poll result pointers upfront
- `boykopovar/AnyPS5` #26232 — feat(relinker): resolve executable relocations to local, hidden and internal symbols
- `boykopovar/AnyPS5` #27184 — feat(libSceIcu): implement usearch_setText
- `boykopovar/AnyPS5` #27182 — feat(libSceIcu): implement usearch_setAttribute
- `boykopovar/AnyPS5` #27246 — feat(libkernel): print a host backtrace with module info
- `boykopovar/AnyPS5` #26479 — feat(libSceCamera): answer the pre-open constant getters measured on 13.60
- `boykopovar/AnyPS5` #25960 — fix(libkernel): validate guest destination pointers upfront in sceKernelBatchMap2 and sceKernelStopUnloadModule
- `boykopovar/AnyPS5` #23294 — fix(libSceLibreSsl3): lift ASN1_STRING_set 1 MiB cap and bound ASN.1 guest copies
- `boykopovar/AnyPS5` #25432 — fix(libSceIcu): validate guest UDataMemory pointer in udata_close_nid_postfix
- `boykopovar/AnyPS5` #27174 — feat(libSceResourceArbitrator): implement pre-init, shm, and feature checks
- `boykopovar/AnyPS5` #22639 — fix(libSceFontFt): preserve unsupported font metric fields
- `boykopovar/AnyPS5` #27217 — perf(tools): fast-scan conditional stubs in import_audit
- `boykopovar/AnyPS5` #27219 — perf(tools): speed up stub reachability and linkage scanning in progress
- `boykopovar/AnyPS5` #27262 — feat(agc): print the resource proof counts with APS5_PIPELINE_STATS
- `boykopovar/AnyPS5` #26643 — feat(libc): trap guest reads for the write watch on Windows
- `boykopovar/AnyPS5` #26067 — test(kernel): read library and module paths from the wide command line on Windows
- `boykopovar/AnyPS5` #13472 — fix(relinker): resolve LibreSSL import module aliases
- `boykopovar/AnyPS5` #27154 — fix(tools): validate native trace event consistency
- `boykopovar/AnyPS5` #27157 — fix(tools): validate sampler module and sample counts
- `boykopovar/AnyPS5` #27256 — fix(libkernel): cap getdtablesize at the console file limit
- `boykopovar/AnyPS5` #20502 — feat(libcairo): implement native font pipeline and device destruction
- `boykopovar/AnyPS5` #26103 — feat(libSceJpegParser): read firmware JPEG and PlayStation metadata
- `boykopovar/AnyPS5` #27183 — feat(libSceIcu): implement usearch_setPattern
- `boykopovar/AnyPS5` #26086 — fix(agc): read the capture trace and frame capture paths from the wide environment on Windows
- `boykopovar/AnyPS5` #27185 — feat(libSceIcu): implement usearch_next
- `boykopovar/AnyPS5` #10792 — feat(relinker): route ps5-payload-dev pathconf stub to libkernel export
- `boykopovar/AnyPS5` #23852 — feat(libScePngParser): read PNG metadata through the firmware 13.42 MBQfjtzRyh0 call
- `boykopovar/AnyPS5` #26354 — feat(libScePosixForWebKit): implement mkstemps
- `boykopovar/AnyPS5` #26376 — feat(libScePosixForWebKit): implement getpwuid_r and getpwnam_r
- `boykopovar/AnyPS5` #22643 — fix(libSceNetCtl): validate state pointers upfront with GuestWritable
- `boykopovar/AnyPS5` #26758 — fix(tools): reject zero-length intervals in complete trace exports
- `boykopovar/AnyPS5` #17241 — feat(libSceLibreSsl3): implement OBJ_obj2txt
- `boykopovar/AnyPS5` #13473 — fix(relinker): match JSON import module to declared provider
- `boykopovar/AnyPS5` #26732 — fix(tools): preserve checkout and Git path identity in sync reports
- `boykopovar/AnyPS5` #10497 — fix(agc): refuse writable buffer feedback with native depth
- `boykopovar/AnyPS5` #19542 — feat(libSceHubAppUtil): reproduce the firmware not-available returns
- `boykopovar/AnyPS5` #26076 — test: re-run self-spawning tests through a wide command line on Windows
- `boykopovar/AnyPS5` #26090 — fix(libSceNet): fail sceNetConnect to port 0 with EADDRNOTAVAIL, as libkernel's connect does
- `boykopovar/AnyPS5` #27186 — feat(libSceIcu): implement usearch_getCollator
- `boykopovar/AnyPS5` #26733 — fix(tools): read A/B logs from the checked stream
- `boykopovar/AnyPS5` #26734 — fix(tools): reject changed relinker corpus entries
- `boykopovar/AnyPS5` #13477 — fix(relinker): match AJM import module to native provider
- `boykopovar/AnyPS5` #27244 — fix(libc): preserve x87 double scaling semantics
- `boykopovar/AnyPS5` #25956 — feat(libkernel): keep POSIX permission bits on Windows
- `boykopovar/AnyPS5` #23350 — feat(libSceFreeTypeOt): implement FT_Request_Metrics
- …and 1367 more (see `tracking/sync-report.json`)

## Frame-time and CPU/GPU bottleneck wins

- `Archivist-Atlas/gta-vi-astral` `a3685ef921cf` — hle/dstorage: implement PAGE_DESTINATION physical-page streaming and Submit flush
- `Archivist-Atlas/gta-vi-astral` `bdbdcad3e00e` — hle: IAcpHal 6-slot ShapeContexts & message ack queue, WinHttp/Winsock offline errors, IGameInputDevice slot 10, ntuser API-set window hide
- `Archivist-Atlas/gta-vi-astral` `3dbeeb746266` — feat(gpu,hle): offscreen RenderTarget->SRV pipeline, D3D12 debug InfoQueue, and live GameInput gamepad polling
- `Archivist-Atlas/gta-vi-astral` `6ee6bd54a7e3` — gpu/hle/docs: DrawIndexedInstanced + Host IndexBuffer upload + Alpha Blending + zero-WARN xaccessibility, README 47%
- `Archivist-Atlas/gta-vi-astral` `9e258b1987aa` — shader/gpu/hle/docs: Compute cs_5_0 + Host Compute PSO/Dispatch + 5 live RDNA2 PSOs & 5185 draws, README 45%
- `Archivist-Atlas/gta-vi-astral` `06cd08ccd5bf` — hle/docs: shared O(1) COM vtables, XUser/XGameSave/XAccessibility, 780+ frames + GPU Dispatch, README 43%
- `Archivist-Atlas/gta-vi-astral` `8fc66cb838a9` — shader/gpu/hle: 93% RDNA2 coverage (v_cvt_pkrtz_f16_f32 + SMEM/MUBUF) + Host VB/CB upload + DeviceRemovedReason
- `Belobug/VIctory-Belobug-AMD` #123 — GPU: AVX2 fast-path and skip-unchanged writes in merge_gpu_writes*
- `Belobug/VIctory-Belobug-AMD` #164 — GPU: prioritize requested and early depth-only pipelines over early draw PSOs
- `Belobug/VIctory-Belobug-AMD` #162 — Remove the deferred-work lock from host command lists
- `Belobug/VIctory-Belobug-AMD` #155 — Defer one draw's buffer refreshes under one lock
- `Belobug/VIctory-Belobug-AMD` #156 — HLE: DualSense device paths keep their NUL (CreateFileW read past the copy)
- `Belobug/VIctory-Belobug-AMD` #88 — Stop scanning the whole shader on every descriptor-load lookup
- `Belobug/VIctory-Belobug-AMD` #75 — Shader IR passes without per-step allocations
- `Belobug/VIctory-Belobug-AMD` #153 — Fix physical page mapping and GPU test assumptions
- `Belobug/VIctory-Belobug-AMD` #152 — Skip the global texture lock when no host-written textures
- `Belobug/VIctory-Belobug-AMD` #139 — Submission CPU time, ambient light following the exposure, unattended benchmark
- `Belobug/VIctory-Belobug-AMD` #120 — GPU: batch hoisted mode-7 indirect argument clamps into 64-lane SIMD dispatches (22x faster)
- `Belobug/VIctory-Belobug-AMD` #145 — Remove per-command mutex from command list state
- `Belobug/VIctory-Belobug-AMD` #103 — GPU: add per-frame breakdown log, GPU list timestamp meter, and Windows driver watch
- `Belobug/VIctory-Belobug-AMD` #64 — Use MD5's constant table instead of 64 sin() per block
- `Belobug/VIctory-Belobug-AMD` #72 — Split vector ALU lowering to restore the asset-policy check
- `Belobug/VIctory-Belobug-AMD` #80 — Hash the DXIL type and constant tables with Fx instead of SipHash
- `Belobug/VIctory-Belobug-AMD` #94 — Shaders: separate bounded heap handle for buffer size queries (AMD compiler crash)
- `Belobug/VIctory-Belobug-AMD` #118 — FidelityFX: Series X host FSR and opt-in AMD frame generation
- `Belobug/VIctory-Belobug-AMD` #65 — Remove dead code in victory-gpu
- `Belobug/VIctory-Belobug-AMD` #58 — Build the indirect mismatch note only in diagnostics builds
- `Belobug/VIctory-Belobug-AMD` #55 — Rectangle expansion reuses the clamp's shader builder
- `Belobug/VIctory-Belobug-AMD` #56 — Skip the pipeline wait's lock and clock once the frame's budget is spent
- `Belobug/VIctory-Belobug-AMD` #54 — Count a list's draws with an atomic instead of a mutex
- `Belobug/VIctory-Belobug-AMD` #132 — timeline: wait for busy queues to finish before pump/submit returns (#116)
- `Belobug/VIctory-Belobug-AMD` #14 — xmem: enforce 16 MiB minimum and pre-validate pages in map_whole
- `Belobug/VIctory-Belobug-AMD` #18 — GPU: sparse copies snapshot and copy back their tiles on several threads
- `Belobug/VIctory-Belobug-AMD` #62 — Find the destination heap once per descriptor copy, not per descriptor
- `Belobug/VIctory-Belobug-AMD` #66 — Remove duplicated pad state, store status functions and helpers
- `Belobug/VIctory-Belobug-AMD` #127 — perf(gpu): cut per-draw and title-memory sync CPU overhead
- `Belobug/VIctory-Belobug-AMD` #126 — GPU: return the output pointer from GetDesc methods
- `Belobug/VIctory-Belobug-AMD` #89 — GPU: fix dynamic descriptor wraparound scan, per-heap walk cap, and capped headroom
- `Belobug/VIctory-Belobug-AMD` #76 — GPU: bound the per-frame pipeline wait with VICTORY_PIPELINE_WAIT_MS
- `Belobug/VIctory-Belobug-AMD` #21 — Reject overlapping depth slices in volume texture copies
- `Belobug/VIctory-Belobug-AMD` #10 — GPU: a submission's write ranges deduplicated through a set, not a scan
- `Belobug/VIctory-Belobug-AMD` #38 — docs: fix PowerShell launch instructions
- `Belobug/VIctory-Belobug-AMD` #37 — Copy equal-length tile pieces with a constant length when detiling
- `Belobug/VIctory-Belobug-AMD` `50a9fa4dbc89` — Show the frame rate and frame time in the picture
- `Belobug/VIctory-Belobug-AMD` `ff920f22e4e0` — The views table is a read-write lock: every list recorder reads it for every descriptor it binds
- `Belobug/VIctory-Belobug-AMD` `127830e5b6b4` — Merge pull request #34 from romainhedouin: A table made afresh is looked up by a quick hash and its record words are compared, instead of a SipHash over them (a collision now costs a copy, not wrong descriptors)
- `Belobug/VIctory-Belobug-AMD` `0145ca78a122` — Merge pull request #24 from romainhedouin: A root constant buffer not 256-byte aligned is copied into list scratch once for the draws of a list that bind it, until a title copy or run intervenes (up to 64 KiB less copied per repeat)
- `Belobug/VIctory-Belobug-AMD` `ed273efe6582` — Merge pull request #25 from romainhedouin: A pipeline a list binds again is not named, queried and kept again (one lookup by address instead of two QueryInterface calls, three locks and a name search per switch)
- `Belobug/VIctory-Belobug-AMD` `13d6b73a7ac0` — Merge pull request #27 from romainhedouin: The host heap behind a descriptor heap is returned without taking the pending lock once it is made, since nothing is queued after that
- `Belobug/VIctory-Belobug-AMD` `40b8b9b55c16` — A title buffer registered again at the same place retires its old copy instead of dropping it at once
- `Belobug/VIctory-Belobug-AMD` `64170dd54df3` — Merge pull request #18 from PikPakPik/perf-copies-paralleles: sparse copies snapshot and copy back their tiles on several threads
- `Belobug/VIctory-Belobug-AMD` `d6070f1a711a` — Merge pull request #45 from Arvi23/pr/integer-target-blend: no blending on integer render targets in host pipelines
- `Belobug/VIctory-Belobug-AMD` `86292110ff2c` — GPU: no blending on integer render targets in host pipelines
- `Belobug/VIctory-Belobug-AMD` `b41f3f3fde5c` — A table made afresh is looked up by a quick hash and its record words are compared, instead of a SipHash over them (a collision now costs a copy, not wrong descriptors)
- `Belobug/VIctory-Belobug-AMD` `b022d86024fe` — The host heap behind a descriptor heap is returned without taking the pending lock once it is made, since nothing is queued after that
- `Belobug/VIctory-Belobug-AMD` `92d353f8d1f4` — A pipeline a list binds again is not named, queried and kept again (one lookup by address instead of two QueryInterface calls, three locks and a name search per switch)
- `Belobug/VIctory-Belobug-AMD` `84ea9c12d3d5` — A root constant buffer not 256-byte aligned is copied into list scratch once for the draws of a list that bind it, until a title copy or run intervenes (up to 64 KiB less copied per repeat)
- `Belobug/VIctory-Belobug-AMD` `cf57ca333855` — A queue held on a wait first finishes the unwaited submission, whose bottom-of-pipe writes the wait may be for (it stood two seconds a frame)
- `Belobug/VIctory-Belobug-AMD` `a0a4591a357c` — Sparse copies: snapshots and copy-back of shader-written tiles on several threads, ranges already declared in the submission skipped, tiles found under one lock
- `Belobug/VIctory-Belobug-AMD` `918ab0380ba1` — Clamps hoisted before a run of indirect draws by default (VICTORY_HOIST_CLAMPS=0 turns it off)
- `Belobug/VIctory-Belobug-AMD` `ccab9730bcfe` — Clamps of a run of indirect draws go to a list before it (VICTORY_HOIST_CLAMPS=1); GPU time per kind of command; shader cache of another build
- `Belobug/VIctory-Belobug-AMD` `236eccefddc7` — Diagnostics: a draw's root buffers, index buffer and whole indirect record in the pass text
- `Belobug/VIctory-Belobug-AMD` `18e57db87d52` — Screen passes get pipeline workers and a wait budget of their own
- `Belobug/VIctory-Belobug-AMD` `b1105d18054c` — A host pipeline's state in its log line (diagnostics)
- `Belobug/VIctory-Belobug-AMD` `2aad398d32f5` — Twice the dynamic descriptor slots: tables of heavy frames no longer fit and their draws were dropped
- `Belobug/VIctory-Belobug-AMD` `1ebf29d5e1fb` — A descriptor's size words are asked of the host only when read as data
- `Belobug/VIctory-Belobug-AMD` `2823ba1556e1` — A first half of an interpolation other instructions read evaluates on its own
- `Belobug/VIctory-Belobug-AMD` `3afed0185896` — victory-gpu: place large buffers in VRAM with Wave64 GPU page hashing
- `Belobug/VIctory-Belobug-AMD` `df7154618038` — Large descriptor tables made once for every length shaders read them to
- `Belobug/VIctory-Belobug-AMD` `06031237748c` — A least amount of indirect light where the title's ray-traced probes have none
- `boykopovar/AnyPS5` #22252 — perf(tools): parallelize progress scanning and skip fetching present commits
- `boykopovar/AnyPS5` #12840 — fix(agc): write back BDA stores into writable heap mirrors
- `boykopovar/AnyPS5` #20805 — fix(libSceHttp): safely probe the outputs of the URI builders
- `boykopovar/AnyPS5` #26800 — perf(shader): keep specialized SPIR-V in the shader disk cache
- `boykopovar/AnyPS5` #20938 — feat(libSceNKWebKit): complete the HistoryItem layout and implement its constructors, destructor and copy
- `boykopovar/AnyPS5` #24965 — feat(libSceNKWebKit): implement WebCore::Path and the path DisplayList items
- `boykopovar/AnyPS5` #27268 — perf(agc): reuse PreparedDraw across layered mesh output windows
- `boykopovar/AnyPS5` #26340 — perf(agc): keep the worker's hot thread state in thread slots
- `boykopovar/AnyPS5` #21923 — perf(agc): release GPU mutex for early index waits
- `boykopovar/AnyPS5` #27269 — perf(agc): fast-path existing imports and open batches in WriteLabelOnGpu
- …and 2431 more (see `tracking/sync-report.json`)

## Stability and device-loss prevention

- `Archivist-Atlas/gta-vi-astral` `b5fbf7e9747f` — loader/shader/hle: continue non-fatal INT3 asserts, fix PS VINTRP scalar subscript & XApu 72B ABI
- `Archivist-Atlas/gta-vi-astral` `f2cf0238f958` — gpu/xmem: CopyPageMappingsBatchX + xmemMapAlias + CreatePlacedSRV(+40) + VirtualQuery guard (pixel-perfect BC3 spinner)
- `Belobug/VIctory-Belobug-AMD` #144 — GPU, Shaders: clamp view/sampler heap indices to dedicated null sentinels and bound StrideTable (#95)
- `Belobug/VIctory-Belobug-AMD` #44 — Windows 10: load the D3D12 Agility SDK runtime (shader model 6.6)
- `Belobug/VIctory-Belobug-AMD` #128 — Wave64 on 32-lane hosts: several console waves per group, wave reductions, permlanes and ds_(b)permute in halves (+ two fixes)
- `Belobug/VIctory-Belobug-AMD` #161 — Title memory is copied, not used in place, past half of the host GPU's system-memory budget (#147)
- `Belobug/VIctory-Belobug-AMD` #40 — Input: DualSense device paths end in a NUL
- `Belobug/VIctory-Belobug-AMD` #154 — Chores: clippy -D warnings clean, xtask policy green, stable tests
- `Belobug/VIctory-Belobug-AMD` #158 — GPU: VICTORY_COMPILER_GUARD=0 leaves the driver compiler's faults to the driver (NVIDIA)
- `Belobug/VIctory-Belobug-AMD` #157 — HLE: keep address space below 4 GiB for the title's 32-bit graphics memory (NVIDIA start-up crash)
- `Belobug/VIctory-Belobug-AMD` #110 — Shaders: a divergent loop's scalars read after it ask for the console wave size (#2's RDNA2 wave-size failures)
- `Belobug/VIctory-Belobug-AMD` #68 — Wave64 on 32-lane hosts: one console wave sharing LDS runs in the console flow (+2 NVIDIA fixes)
- `Belobug/VIctory-Belobug-AMD` #140 — Shader tests: WARP runs one draw or dispatch at a time (parallel runs crashed in d3d10warp.dll)
- `Belobug/VIctory-Belobug-AMD` #104 — victory system: this machine for test reports; the commit built from in --version and crash reports
- `Belobug/VIctory-Belobug-AMD` #52 — NVIDIA test failures, and victory-gpu clean with clippy -D warnings
- `Belobug/VIctory-Belobug-AMD` #146 — Shader/GPU: recover wave64 split scalar masks and report GetDeviceRemovedReason
- `Belobug/VIctory-Belobug-AMD` #108 — CI: require mutation coverage on changed lines
- `Belobug/VIctory-Belobug-AMD` #97 — Shaders: DXIL bitcode clean-up passes (VICTORY_SHADER_OPT), census, and loop latch fixes
- `Belobug/VIctory-Belobug-AMD` #78 — Check shader test expectations on 16-lane adapters
- `Belobug/VIctory-Belobug-AMD` #22 — Shaders: omit unnecessary fixed WaveSize for single-wave LDS compute shaders
- `Belobug/VIctory-Belobug-AMD` #90 — Shaders: follow EXEC set from a scalar the section made constant
- `Belobug/VIctory-Belobug-AMD` #71 — Fix stale and misplaced comments
- `Belobug/VIctory-Belobug-AMD` #115 — watch: the write-watch test counts only the writes since its first asking (#102)
- `Belobug/VIctory-Belobug-AMD` #93 — xmem: keep tiny reservations per page; map_whole only for large runs
- `Belobug/VIctory-Belobug-AMD` #134 — Shaders: recover wave64 masks restored from scalar copies
- `Belobug/VIctory-Belobug-AMD` #133 — Loader: reject garbage relocation sizes, serialise crash-log appends
- `Belobug/VIctory-Belobug-AMD` #92 — Shaders: keep 64-lane waves for wave64 programs on a 64-lane host
- `Belobug/VIctory-Belobug-AMD` #79 — GPU: VICTORY_ADAPTER picks the host adapter by name
- `Belobug/VIctory-Belobug-AMD` #67 — Look up views once per range in watch::writing, and drop ended pins
- `Belobug/VIctory-Belobug-AMD` #99 — xmem: map_alias of no pages changes nothing, next to a reservation shown whole too
- `Belobug/VIctory-Belobug-AMD` #60 — Keep pinned blocks writable on guard()'s fast path too
- `Belobug/VIctory-Belobug-AMD` #50 — dstorage: retry raw file read on ERROR_INVALID_USER_BUFFER
- `Belobug/VIctory-Belobug-AMD` #39 — xmem: keep the space below 4 GiB for the title's 32-bit memory (port of VIctory#1009)
- `Belobug/VIctory-Belobug-AMD` `684a9b5840b1` — Merge pull request #28 from romainhedouin: Clamps hoisted into one list bind their root signature, pipeline and budget once, not before every clamp (3 fewer D3D12 calls per indirect draw)
- `Belobug/VIctory-Belobug-AMD` `ab5ece8f20ff` — Clamps hoisted into one list bind their root signature, pipeline and budget once, not before every clamp (3 fewer D3D12 calls per indirect draw)
- `Belobug/VIctory-Belobug-AMD` `9b89ec2d384f` — Only the title's submissions may return before the host GPU is done; ours wait (the present's capture buffer was freed with its copy pending: device removed at the first frame with VICTORY_ASYNC_SUBMIT=1)
- `Belobug/VIctory-Belobug-AMD` `f2cc1009a73f` — The views are looked at once for a run of guarded blocks, and pages with one view make no list (less time under the guards' lock)
- `Belobug/VIctory-Belobug-AMD` `579a8d9dd96f` — Blocks the kernel is about to write stay writable for two seconds, at every view of their pages, and are reported written meanwhile (an asynchronous read into a block guarded again failed, and the title stopped)
- `Belobug/VIctory-Belobug-AMD` `e7e6c00b33d2` — Guarded blocks given back in a run that spans views are given back one by one when the run fails
- `Belobug/VIctory-Belobug-AMD` `1c7231d9e02c` — A view made read-only between its mapping and the runtime's notice is given back; a read-only view block no guard knows is opened on its fault
- `Belobug/VIctory-Belobug-AMD` `ae909379e6a4` — Every view of a guarded physical page is guarded with it: a write of the title's through another address is seen (parts of moving vehicles were left behind); VICTORY_ALWAYS_COPY modes (diagnostics)
- `Belobug/VIctory-Belobug-AMD` `76387c2dea07` — Hoisted clamps run one after another by default (mode 7): no flicker of mesh objects
- `Belobug/VIctory-Belobug-AMD` `afca18900a70` — Work in progress: switches to find the flicker of hoisted clamps (modes 2 to 7, VICTORY_HOIST_FILE)
- `Belobug/VIctory-Belobug-AMD` `1503444b8b08` — Buffer element counts by slot from the host's table, not a size query (AMD compiler crash)
- `Belobug/VIctory-Belobug-AMD` `beffaea6753a` — Views the title wrote last are made at each submission; memory by address clamped
- `boykopovar/AnyPS5` #26221 — fix(agc): refuse a host import over read-trapped pages instead of resolving the trap under its locks
- `boykopovar/AnyPS5` #26835 — feat(libSceVideoOutSecondary): implement firmware 13.60 slot table exports KJWkt9y7YC4 and s+isXKLpwS0
- `boykopovar/AnyPS5` #26759 — test(libSceNgs2): cover the FFT work-size geometry and argument checks
- `boykopovar/AnyPS5` #26679 — test(agc): wait for the draw-error record before reading the failure trace
- `boykopovar/AnyPS5` #26674 — feat(libSceIcu): implement Normalizer2Impl::addLcccChars
- `boykopovar/AnyPS5` #26884 — feat(libSceNKWebKit): implement WKBundleNodeHandle autofill button type operations
- `boykopovar/AnyPS5` #27134 — fix(libSceNet): probe guest msg_name before writing it in sceNetRecvmsg
- `boykopovar/AnyPS5` #26671 — feat(libSceIcu): implement the Normalizer and RuleBasedBreakIterator class IDs
- `boykopovar/AnyPS5` #26488 — test(tools): split the configure seed round trips into two CTest entries
- `boykopovar/AnyPS5` #27275 — fix(libkernel): read and write sceKernelWaitSema's timeout through guest copies
- `boykopovar/AnyPS5` #24347 — feat(libSceNKWebKit): implement the JSDocument, JSElement, JSEvent and JSNode toWrapped exports
- `boykopovar/AnyPS5` #24920 — feat(libSceNKWebKit): implement WKPage reload and stop loading controls
- `boykopovar/AnyPS5` #24905 — feat(libSceLibcInternal): implement wide num_put::_Iput
- `boykopovar/AnyPS5` #19864 — fix(libc): route exception deleting destructors through guest heap
- `boykopovar/AnyPS5` #24457 — feat(libSceNKWebKit): implement WKUserMediaPermissionCheckSetUserMediaAccessInfo
- `boykopovar/AnyPS5` #27225 — fix(libSceNpManager): probe guest pointers in value class helpers
- `boykopovar/AnyPS5` #21930 — feat(libc): wire the numpunct<char> facet vtable and its members
- `boykopovar/AnyPS5` #22774 — fix(shader): call load_bda_span outside read_bda_span's per-lane selection
- `boykopovar/AnyPS5` #22723 — fix(libSceFont): guard shared font reference increments
- `boykopovar/AnyPS5` #22132 — fix(libSceIcu): copy invariant conversion guest pointers safely
- `boykopovar/AnyPS5` #18671 — fix(libSceVoice): return InvalidArgument from sceVoiceInit for unreadable parameter pointer
- `boykopovar/AnyPS5` #26413 — feat(libSceJsc): implement WTFLogAlwaysV
- `boykopovar/AnyPS5` #26731 — fix(libc): guard strsep guest memory
- `boykopovar/AnyPS5` #22215 — fix(libSceAgc): validate and safely copy semaphore label pointer in sceAgcGetSemaphoreLabel
- `boykopovar/AnyPS5` #23081 — fix(libSceFreeType): copy FT_Get_Glyph_Name and FT_Get_Sfnt_Name outputs safely
- `boykopovar/AnyPS5` #27172 — feat(libSceCamera): validate inventory exports and implement sceCameraIsVrCaptureEnable
- `boykopovar/AnyPS5` #27173 — feat(libSceTtsCoreG3Bc): implement 6 verified engine helper exports
- `boykopovar/AnyPS5` #21550 — fix(libSceJxr): access the image encoder and its stream through checked guest copies
- `boykopovar/AnyPS5` #27175 — feat(libSceVideoNativeExtEssential): model Initialize, Finalize, and instance validation
- `boykopovar/AnyPS5` #27188 — fix(libSceUsbStorage): return 13.60 error codes on uninitialized and unregistered calls
- `boykopovar/AnyPS5` #27189 — feat(libSceUserService): implement 13.60 inventory settings validation and state
- `boykopovar/AnyPS5` #27190 — feat(libSceSysCore): add missing 13.60 exports and null-output guard
- `boykopovar/AnyPS5` #27191 — fix(libSceRemotePlayClientIpc): implement callback registration and idle terminate
- `boykopovar/AnyPS5` #27196 — fix(libSceLibreSsl3): enforce OID arc bounds and leading-whitespace rejection in OBJ_txt2obj
- `boykopovar/AnyPS5` #27143 — feat(libSceVnaWebsocket): implement config and callback-table initializers
- …and 2484 more (see `tracking/sync-report.json`)

## Rendering and gameplay completeness

- `Ahmet-Dedeler/awesome-gta6-reverse-engineering` `7e1a32c0bc3f` — Add ScarlettPort, openrage, xcl-mac, data dumps & save tools; relax overly strict inclusion rules
- `Albotterin/AWC_from_RPF9_Extractor` `3a869dad98b4` — Own station logo reader, no decoder download (v2.4.1)
- `Archivist-Atlas/gta-vi-astral` `7a3471084abb` — gpu/d3d12x: raise per-frame draw cap 64->512, CB 1->8 MiB & SRV/RTV heaps 256/64 (fixes Legal UI flicker & dropped text)
- `Archivist-Atlas/gta-vi-astral` `a35840d8fec5` — gpu/d3d12: refresh dynamic font atlas textures (FNV hash + 32 MiB staging) & fix R8/R8G8 SRV component mapping
- `Archivist-Atlas/gta-vi-astral` `50481d551c84` — hle: fenetre Win32 unique focus natif + GameInput clavier/UiNav/latching + ABI XUser/XGameSave
- `Archivist-Atlas/gta-vi-astral` `95900b3c4673` — fix(gameinput): reading ring-buffer (64 snapshots), GetNextReading/GetPreviousReading & InputEnabled status (0x3)
- `Archivist-Atlas/gta-vi-astral` `5096e6f110ba` — fix(gpu): bump UploadVB/IB to 32/16 MiB to fix Scaleform UI text corruption
- `Archivist-Atlas/gta-vi-astral` `a96f48549345` — feat(hle,shader): XAsyncRun/XStore/XGameUi, DirectStorage EnqueueSignal & hardware Compute PSOs (cs_5_0)
- `Archivist-Atlas/gta-vi-astral` `65b79edc1914` — gpu/shader: multi-slot CBV descriptor routing + generalized RECTLIST expansion (fixes post-profile textured Scaleform spinner)
- `Archivist-Atlas/gta-vi-astral` `4b38e44baa02` — feat(input): universal Windows gamepad support (DualSense PS5, DualShock 4, XInput 0..3, generic HID)
- `Archivist-Atlas/gta-vi-astral` `3e979713aafa` — feat(ui): add GTA VI Astral synthwave app icon (title bar + taskbar)
- `Archivist-Atlas/gta-vi-astral` `ec82f1ccf40c` — feat(gpu,shader): first real GTA VI intro pixels on D3D12 (100% Scaleform RDNA2->HLSL coverage)
- `Archivist-Atlas/gta-vi-astral` `c573d2997343` — shader/gpu/hle/docs: RDNA2 v[4+reg*4] VGPR inputs, COLOR0 RGBA8 stride=24, RECTLIST 6-vertex quad expansion, README 55%
- `Archivist-Atlas/gta-vi-astral` `a67b114ceb4e` — shader/gpu/hle/docs: structured SOPP CFG branches + RECTLIST topology + OMSetRenderTargets tracking, README 53%
- `Archivist-Atlas/gta-vi-astral` `4189da766324` — shader/hle/docs: VOPC/SOPC/VOP3 ops, disk-backed XGameSave, LEGAL.md, README 50% (GTA VI Astral)
- `Archivist-Atlas/gta-vi-astral` `19ef8a97b89c` — gpu/hle/docs: RDNA2 texture detiler (4KB/64KB + R8/BC1-BC7), hide duplicate engine window, README 40%
- `Archivist-Atlas/gta-vi-astral` `155c89b5679e` — shader/gpu/hle: ISG1 vertex attribute binding + 64-slot Host Texture2D SRV cache
- `Archivist-Atlas/gta-vi-astral` `80990df91752` — shader/gpu/hle: live RDNA2->HLSL->D3DCompile->ID3D12PipelineState (3/3 active PSOs)
- `Belobug/VIctory-Belobug-AMD` #150 — GPU: improve texture uploads and specialize single-record indirect draws
- `Belobug/VIctory-Belobug-AMD` #169 — HLE: harden media source resolver URL and UNC path validation
- `Belobug/VIctory-Belobug-AMD` #167 — Reuse initialized scratch space for texture reads
- `Belobug/VIctory-Belobug-AMD` #35 — Precompute the XMA inverse MDCT's sine/cosine tables
- `Belobug/VIctory-Belobug-AMD` #119 — Speed up bindless planning, BCPack decoding, and shader translation
- `Belobug/VIctory-Belobug-AMD` #159 — GPU: isolate unaligned and off-element buffer views in exact shadow copies
- `Belobug/VIctory-Belobug-AMD` #84 — Extract scalar and vector ALU lowering from lower.rs
- `Belobug/VIctory-Belobug-AMD` #83 — Shaders: s_cbranch_vccz/vccnz tests raw VCC, not VCC & EXEC
- `Belobug/VIctory-Belobug-AMD` #70 — Remove unreachable arms and dead code in the shader translator
- `Belobug/VIctory-Belobug-AMD` #138 — Shaders, GPU: port VIctory#2398, link over-budget mesh and pixel shaders under the 32 KiB output limit (#125)
- `Belobug/VIctory-Belobug-AMD` #81 — Shaders: 64-bit integer compares on their dwords (AMD orders a shared-dword i64 compare wrong)
- `Belobug/VIctory-Belobug-AMD` #43 — GPU: sanitize resource barrier bind flags and UAV targets
- `Belobug/VIctory-Belobug-AMD` #148 — Shaders: a buffer view picked from the resource heap by index is read as a raw buffer, not a constant buffer (missing world geometry and device removal on a Radeon 8060S)
- `Belobug/VIctory-Belobug-AMD` #3 — Clippy: victory-shader is clean again with -D warnings (10 lints, no behaviour change)
- `Belobug/VIctory-Belobug-AMD` #142 — Carry 20 upstream VIctory fixes (shaders, samplers, textures) and a mesh wave-ops flag fix
- `Belobug/VIctory-Belobug-AMD` #112 — watch: clear stale JOURNAL.written entries on title_memory_remapped (#102)
- `Belobug/VIctory-Belobug-AMD` #57 — Fix underflow for textures described with MipLevels 0
- `Belobug/VIctory-Belobug-AMD` #135 — Shaders: f16 results round to nearest even, and min/max/med3 read denormal operands as zeros
- `Belobug/VIctory-Belobug-AMD` #51 — gpu: skip self-CopyResource when src and dst share host texture
- `Belobug/VIctory-Belobug-AMD` #86 — translate --warm: translate every shader into the run's cache ahead of a run
- `Belobug/VIctory-Belobug-AMD` #129 — Shader tests: a cross-device VALU fuzzer (hardware adapters vs WARP), ignored by default
- `Belobug/VIctory-Belobug-AMD` #63 — Stop re-zeroing the texture scratch buffer, and reuse the placed-texture set
- `Belobug/VIctory-Belobug-AMD` #73 — Reuse one Opus decode buffer per connection
- `Belobug/VIctory-Belobug-AMD` #111 — ci: report all failing test targets
- `Belobug/VIctory-Belobug-AMD` #61 — Let bindless draws take the cheap check once the frame's rechecks are spent
- `Belobug/VIctory-Belobug-AMD` #47 — docs: clarify shader targeting and GPU diagnostics
- `Belobug/VIctory-Belobug-AMD` #7 — Add Xbox texture layouts for formats 189–191
- `Belobug/VIctory-Belobug-AMD` #32 — Check a texture for writes without building lists
- `Belobug/VIctory-Belobug-AMD` #31 — Read a host texture's flags once instead of on every use
- `Belobug/VIctory-Belobug-AMD` #23 — XMA decoder: lock only voices that are on, sleep while none is
- `Belobug/VIctory-Belobug-AMD` `c3bd5612c2a5` — A least light for reflections, set from the ambient file
- `Belobug/VIctory-Belobug-AMD` `58655f97c71c` — Merge pull request #29 from romainhedouin: A range shaders write is declared once until the title next writes memory, and the submission finds the unique ranges with a set, so a list of thousands of draws no longer scans and re-snapshots the same ranges per step
- `Belobug/VIctory-Belobug-AMD` `f53ae3db6a61` — Merge pull request #30 from romainhedouin: Buffer-to-texture copies take their staging bytes from the list's pooled scratch chunks instead of creating and mapping a committed upload buffer each, which is much cheaper
- `Belobug/VIctory-Belobug-AMD` `44dcd4371cc4` — Reflections held to the ambient light; copies and retired objects outlive the lists that name them
- `Belobug/VIctory-Belobug-AMD` `f870d806888f` — README: pull requests #45 and #18 merged, the ambient light's shading and tuning, what the hours of the day need, the ray passes' state
- `Belobug/VIctory-Belobug-AMD` `cd9f78ce010d` — Ambient light: shaded in creases and under objects from the scene's depth (VICTORY_AMBIENT_SHADE), tuned live from the ambient file
- `Belobug/VIctory-Belobug-AMD` `dd799aadf361` — A range shaders write is declared once until the title next writes memory, and the submission finds the unique ranges with a set, so a list of thousands of draws no longer scans and re-snapshots the same ranges per step
- `Belobug/VIctory-Belobug-AMD` `5e08eee09fe9` — Buffer-to-texture copies take their staging bytes from the list's pooled scratch chunks instead of creating and mapping a committed upload buffer each, which is much cheaper
- `Belobug/VIctory-Belobug-AMD` `4ffc4766014f` — The ambient light's colour can be set (VICTORY_AMBIENT_TINT), and level and colour tuned in a running game from a file (VICTORY_AMBIENT_FILE, diagnostics)
- `Belobug/VIctory-Belobug-AMD` `71652a2c15a0` — README: vehicles and distant trees work, the phone does not; memory and descriptor slots among the known problems
- `Belobug/VIctory-Belobug-AMD` `cf62e1aeaa69` — VICTORY_SHADER_SWAP replaces shader containers by files: the distant trees' variants without wind stand in for those with wind
- `Belobug/VIctory-Belobug-AMD` `03629f3f0d3f` — A pixel's quad is read through quad reads, helper lanes included: a derivative taken by hand at a triangle's edge was the whole value (lines on car paint, glass and skin)
- `Belobug/VIctory-Belobug-AMD` `4f45ab88f861` — The resolve's indirect diffuse light is replaced by a constant while rays do not run (magenta foliage, blocks of colours indoors); VICTORY_SHADER_PATCH changes a word of a shader's code (diagnostics)
- `Belobug/VIctory-Belobug-AMD` `7de1751166bd` — Diagnostics: a peeked view's bytes as a file; a test that detiles a peeked 256x16 texture
- `Belobug/VIctory-Belobug-AMD` `4d63a091e9da` — Ambient light off by default: the title's own light only
- `boykopovar/AnyPS5` #12792 — fix(agc): compare pending storage results over a surface whose page walk fails
- `boykopovar/AnyPS5` #27216 — feat(libSceAudioPropagation): render three-point material paths from console measurements
- `boykopovar/AnyPS5` #20860 — test(libSceSaveData): audit progress polling, cancel recovery, and mount result layout
- `boykopovar/AnyPS5` #22472 — fix(libSceNKWebKit): copy WTF::Persistence and WTF::URL guest pointers safely
- `boykopovar/AnyPS5` #26706 — fix(agc): cover the final stride in vertex bindings
- `boykopovar/AnyPS5` #26766 — build(shader): update SPIRV-Tools for float-control optimization
- `boykopovar/AnyPS5` #27218 — fix(agc): tell apart the storage images a cache release keeps for pending results and for use
- `boykopovar/AnyPS5` #16880 — fix(agc): read single-sample views of native multisample depth from guest memory
- `boykopovar/AnyPS5` #26655 — feat(libSceAudiodec.native): implement codec registration and unregistration
- `boykopovar/AnyPS5` #26700 — feat(libSceVideodec2): implement sceVideodec2CreateDecoderBid and sceVideodec2MapMemory
- `boykopovar/AnyPS5` #21330 — fix(libwebrtc): copy RTCP packet guest pointers safely
- `boykopovar/AnyPS5` #21611 — test(libSceFiber): audit switch register preservation, stack canaries, and roundtrips
- `boykopovar/AnyPS5` #9748 — feat(agc): blend into 10_11_11 unorm color targets in the pixel shader
- `boykopovar/AnyPS5` #7085 — feat(agc): back extra guest mip levels with storage images
- `boykopovar/AnyPS5` #27265 — refactor(ime): drop errno saves around guest copies
- `boykopovar/AnyPS5` #27266 — refactor(videoout): drop errno saves around guest copies
- `boykopovar/AnyPS5` #27267 — refactor(nkwebkitrequirements): drop errno saves around guest copies
- …and 3226 more (see `tracking/sync-report.json`)

## Experimental / unstable (hold)

- `Archivist-Atlas/gta-vi-astral` `3b2bbb74d259` — docs: add progress bar, WIP warning, and subsystem status table to README
- `Belobug/VIctory-Belobug-AMD` #16 — xmem tests take the per-page path again; a reservation shown whole has its own test
- `Belobug/VIctory-Belobug-AMD` `bdecda306a3e` — An unwaited submission is finished when it is waited for (its shaders' writes, read-backs and bottom-of-pipe writes), so every submission of the title's may go unwaited; the next one's buffer copies are refreshed meanwhile (VICTORY_ASYNC_SUBMIT=1, off by default)
- `Belobug/VIctory-Belobug-AMD` `0a32e89cf58a` — Tiles of title memory that is no longer there can be given back (VICTORY_SWEEP_TILES=2), so the copies stop growing; off by default
- `boykopovar/AnyPS5` #25017 — feat(libkernel): create SOCK_SEQPACKET socketpairs on macOS
- `boykopovar/AnyPS5` #19434 — feat(libSceLibcInternal): forward the firmware libc-internal export surface
- `boykopovar/AnyPS5` #1835 — feat(aot): discuss native ARM64 compiler architecture and integration
- `boykopovar/AnyPS5` #18082 — fix(agc): re-land pipeline cache persistence with pipeline binaries off on NVIDIA proprietary
- `boykopovar/AnyPS5` #17936 — fix(libkernel_sys): answer sceKernelGetPhysPageSize for the mapping holding an address
- `boykopovar/AnyPS5` #17719 — feat(libSceErrorDialog): show error dialogs as host message boxes with ANYPS5_ERROR_DIALOG_HOST
- `boykopovar/AnyPS5` #17626 — feat(libSceLibreSsl3): implement ECDSA_do_sign
- `boykopovar/AnyPS5` #17610 — feat(libc): fill the std::__sce_v2 classic locale with its facets
- `boykopovar/AnyPS5` #17555 — feat(debug): pause and resume a guest thread at a breakpoint
- `boykopovar/AnyPS5` #17261 — feat(libc): add a breakpoint table for guest NIDs, off by default
- `boykopovar/AnyPS5` #16791 — fix(videoout): end shutdown when a host thread cannot unwind
- `boykopovar/AnyPS5` #15297 — fix(agc): mirror the whole contiguous run of releasable allocations a joined heap belongs to
- `boykopovar/AnyPS5` #1395 — feat(agc): add experimental depth clear and copy recognition
- `boykopovar/AnyPS5` #13172 — feat(mscorlib.dll): declare 373 callable exports
- `boykopovar/AnyPS5` #12863 — feat(libSceVideoOut): opt-in host pause with a key or on focus loss
- `boykopovar/AnyPS5` #10656 — perf(agc): copy contiguous rows of a drawn linear target store in one region
- `boykopovar/AnyPS5` #25322 — fix(agc): use wave LDS memory barriers on NVIDIA drivers before 596
- `boykopovar/AnyPS5` #25824 — fix(agc): let draw recipes adopt replacement templates
- `boykopovar/AnyPS5` #25530 — feat(libSceNKWebKit): implement WKBundleRangeHandleCreate and WKBundleRangeHandleGetBoundingRectInWindowCoordinates
- `boykopovar/AnyPS5` #26253 — fix(libSceIcu): delete guest UObjects through their System V vtable
- `boykopovar/AnyPS5` #26150 — feat(libkernel_sys): implement randomized path query
- `boykopovar/AnyPS5` #20635 — feat(shader): allow directed rounding modes for packed f16 fma
- `boykopovar/AnyPS5` #20580 — fix(libc): order read-trap resolution against the GPU mutex
- `boykopovar/AnyPS5` #17303 — refactor(prx): name 191 more LibreSsl3 parent-export aliases the NID databases resolve
- `boykopovar/AnyPS5` #19904 — fix(agc): import uniform initial multisample contents under a compressed CMASK
- `boykopovar/AnyPS5` #11452 — feat(agc): count why a recorded draw builds no recipe
- `boykopovar/AnyPS5` #27147 — fix(tools): require a probe for opt-in-only oracle catalogs
- `boykopovar/AnyPS5` #21978 — fix(libkernel): prioritize queued flock upgrades
- `boykopovar/AnyPS5` #23545 — fix(agc): support four-fragment compressed FMASK
- `boykopovar/AnyPS5` #26338 — fix(agc): mark a resolve destination after releasing the source's import gate hold
- `boykopovar/AnyPS5` #26165 — fix(agc): wait a bounded time for a dispatch's compute pipeline built ahead before skipping it
- `boykopovar/AnyPS5` #26036 — fix(agc): take an indexed draw's base vertex as signed
- `boykopovar/AnyPS5` #26034 — fix(agc): settle host imports under a native depth fill before its watch check
- `boykopovar/AnyPS5` #25932 — fix(agc): avoid Intel UHD A78B pipeline library crashes
- `boykopovar/AnyPS5` #25609 — fix(shader): sum v_readfirstlane over the guest wave instead of shuffling it from the first lane
- `boykopovar/AnyPS5` #25573 — feat(libSceLibcInternal): implement numpunct, facet construction and stream runtime
- `boykopovar/AnyPS5` #25491 — fix(libkernel): lock macOS append-mode positioned writes against offset changes
- `boykopovar/AnyPS5` #25226 — fix(agc): wait briefly for a dispatch's background shader work instead of dropping it silently
- `boykopovar/AnyPS5` #25160 — feat(libSceNKWebKit): implement WTF::PrintStream::printfVariableFormat and WTF::MetaAllocatorHandle::dump
- `boykopovar/AnyPS5` #24819 — fix(libkernel): report module unwind names longer than MAX_PATH on Windows
- `boykopovar/AnyPS5` #24070 — fix(libkernel): stop losing _umtx_op umutex wakeups to a userland unlock
- `boykopovar/AnyPS5` #23632 — fix(shader): clamp image_gather4h_pck rows and read array and 3D slices
- `boykopovar/AnyPS5` #22684 — feat(libSceNKWebKit): implement WTF symbols and the symbol registry
- `boykopovar/AnyPS5` #22370 — feat(libSceLibcInternal): complete global locale lifecycle
- `boykopovar/AnyPS5` #22301 — fix(libkernel): safely probe the rwlock slot in scePthreadRwlockDestroy
- `boykopovar/AnyPS5` #20686 — feat(shader): translate v_fma_mixlo/mixhi_f16 with flushed f16 denormals
- `boykopovar/AnyPS5` #20344 — fix(agc): do not abort on a screenshot trigger that another process holds open
- `boykopovar/AnyPS5` #19697 — feat(agc): record depth-only draws on a recording pool behind APS5_PARALLEL_ATLAS
- `boykopovar/AnyPS5` #13547 — feat(libSceLibreSsl): implement X509_INFO_free
- `boykopovar/AnyPS5` #26796 — fix(libc): port casinhl, casinl, cacosl and cacoshl from FreeBSD's catrigl.c
- `boykopovar/AnyPS5` #20272 — fix(tools): ignore a carriage return at the end of debt lines in check_conventions
- `boykopovar/AnyPS5` #26802 — fix(agc): bind a PS4 V# of the range limit up to the end of the registered run behind it
- `boykopovar/AnyPS5` #9776 — feat(aot): add opt-in LLVM integer leaf compiler
- `boykopovar/AnyPS5` #6209 — perf(agc): a DCC key proof over untracked memory holds until the next ordering point
- `boykopovar/AnyPS5` #26797 — fix(shader): refuse a descriptor web once it holds 512 states
- `boykopovar/AnyPS5` #26686 — fix(agc): work around AMD mesh barrier compiler crash
- `boykopovar/AnyPS5` #22066 — fix(agc): publish the idle epoch under the driver mutex before waking waiters
- `boykopovar/AnyPS5` #1384 — feat(ngs2): add experimental I3DL2 reverb
- `boykopovar/AnyPS5` #11134 — fix(http): trust the system CA list for HTTPS requests
- `boykopovar/AnyPS5` #9633 — fix(shader): follow a conditional pointer chase through a V# behind a pointer-keyed record pointer
- `boykopovar/AnyPS5` #9457 — fix(shader): follow pointer hops and fixed-offset V# reads behind a record pointer
- `boykopovar/AnyPS5` #8939 — fix(shader): take lane-mask arms and a merged high word in a pointer-keyed record pointer
- `boykopovar/AnyPS5` #25354 — fix(libSceLibreSsl3): queue errors for types EVP_PKEY_new_mac_key rejects
- `boykopovar/AnyPS5` #23088 — feat(libSceLibreSsl): implement SSL_set_SSL_CTX
- `boykopovar/AnyPS5` #20349 — fix(libkernel): place a fixed mapping at any page-aligned address whatever its alignment
- `boykopovar/AnyPS5` #25077 — fix(libSceNpSessionSignaling): hold mutex across custom property exchange
- `boykopovar/AnyPS5` #26146 — fix(ngs2): ramp standard mastering gain across the next grain
- `boykopovar/AnyPS5` #26406 — fix(libSceIcu): keep trailing and doubled separators in the base name of a locale ID
- `boykopovar/AnyPS5` #21598 — fix(libSceLibreSsl): copy the CMAC message and tag through guest copies
- `boykopovar/AnyPS5` #26394 — fix(libSceIcu): drop root and und from a locale ID only where ICU does
- `boykopovar/AnyPS5` #25351 — fix(libSceLibreSsl3): cache name encodings of certificates loaded by X509_STORE_load_locations
- `boykopovar/AnyPS5` #25265 — fix(libc): support arguments, stop_token dispatch, and rvalue tasks in StartHostJThread
- `boykopovar/AnyPS5` #24466 — fix(libc): store _Mtx_init's and _Cnd_init's handles through checked guest copies
- `boykopovar/AnyPS5` #24409 — fix(libSceUsbd): check control-transfer output with a host write
- `boykopovar/AnyPS5` #24287 — feat(libSceJsc): implement JSC::CallFrame::isAnyWasmCallee
- `boykopovar/AnyPS5` #23913 — test(tools): copy Apple's Python framework parts with the title_run fake relinker
- …and 503 more (see `tracking/sync-report.json`)

## Per-repository activity

| Repository | Commits | Open PRs | Stage | Status |
|---|---|---|---|---|
| `Ahmet-Dedeler/awesome-gta6-reverse-engineering` | 5 | 0 | — | active |
| `Ahmet-Dedeler/bcpack-rs` | 3 | 0 | — | active |
| `Ahmet-Dedeler/rage9-re` | 0 | 0 | — | active |
| `Albotterin/AWC_from_RPF9_Extractor` | 10 | 0 | — | active |
| `anrq0/SeriesX-Emulator` | 1 | 0 | — | active |
| `Archivist-Atlas/gta-vi-astral` | 45 | 0 | — | candidate |
| `batatinhafcbatatinha-pixel/xseriesrecomp` | 0 | 0 | — | unreachable |
| `Belobug/VIctory-Belobug-AMD` | 76 | 110 | 6 | active |
| `boykopovar/AnyPS5` | 262 | 2667 | — | active |
| `brcly/TuxBox` | 185 | 5 | 5 | active |
| `breadbbbb/rpf9-explorer` | 0 | 1 | — | active |
| `californiaAlmonds/OpenScarlett` | 0 | 2 | 5 | active |
| `cocainebunnylaboratories/cblvictoryvi` | 51 | 0 | 6 | active |
| `daeken/XSXEmu` | 115 | 0 | — | candidate |
| `Dark3581/AnyBox` | 0 | 0 | — | unreachable |
| `DexNom/VIctory` | 153 | 15 | 6 | active |
| `edsuuu/VI-BR-PORT` | 173 | 0 | 6 | active |
| `edupode/anyxboxseries` | 5 | 0 | — | active |
| `ethan00013/keylime` | 62 | 0 | 5 | candidate |
| `fablinoo/kelvaroq` | 22 | 0 | 6 | candidate |
| `fishingpvalues/gta6-pc-port-hub` | 0 | 0 | — | archived |
| `Freminet/ScarlettX` | 0 | 26 | — | active |
| `fxceless/victory-fork` | 15 | 797 | 6 | active |
| `gookie-dev/g6shim` | 22 | 1 | 5 | active |
| `grzegorz2047/TrinityXSX` | 25 | 0 | — | candidate |
| `holdmysocks/SixNG` | 73 | 5 | — | active |
| `HolyRoses/gta6-explorer` | 4 | 0 | — | active |
| `Hylanderry/DedSec` | 0 | 0 | — | active |
| `Itsbatt/victory-unified` | 16 | 0 | 6 | candidate |
| `ItsNotRexed/Leonida` | 0 | 0 | — | nuked |
| `jadeowaowa/scarlett-layer` | 0 | 2 | — | active |
| `JamesCicada/GTAVI-Recomp` | 16 | 4 | — | active |
| `JMAN730/project-flamingo` | 431 | 4 | — | active |
| `JoshRob297/rpf9-research` | 0 | 3 | — | active |
| `JPTuba9/xlaunch-vi` | 0 | 56 | — | active |
| `ka1qux/BRVIctory-ka1qux` | 135 | 1 | 6 | candidate |
| `Kajcsi/G6Recomp` | 0 | 0 | — | active |
| `kellysix/kelly` | 7 | 0 | — | candidate |
| `kfedora/GTAVI-Xbox2PC` | 19 | 0 | — | candidate |
| `KomanderDG88/VIniVIdiVIci` | 59 | 0 | 6 | active |
| `kry-vex-gta/XBOX-Inspector` | 0 | 0 | — | active |
| `Librett0/vice-lens` | 0 | 0 | — | active |
| `ljm-233/Xbgtasix` | 0 | 0 | — | active |
| `LooperSalty/AnyXboxX` | 232 | 5 | 6 | active |
| `LoverCatys/ruff-lab` | 58 | 0 | 5 | active |
| `luizwidmer/palmetto` | 100 | 0 | 4 | active |
| `lynxnathan/rage-tools-vi` | 1 | 0 | — | active |
| `marvisak/VIctory` | 221 | 1235 | 6 | active |
| `meetox80/6stars` | 0 | 0 | — | active |
| `mewiof/Holy-Interoperability` | 0 | 0 | 4 | active |
| `mkr2tl/xenstride-public` | 1 | 0 | — | active |
| `myokex/victory-amd` | 3 | 11 | 6 | active |
| `nineforge0/forge9` | 21 | 0 | — | active |
| `nullbrik/RageForge` | 0 | 0 | — | unreachable |
| `omonkeyz/xbox-emulator` | 134 | 19 | 3 | active |
| `OnlyMoisties/GTA6-BootTest` | 0 | 0 | 5 | active |
| `OpenCommunityEdition/OpenScarlett` | 20 | 1 | 5 | active |
| `p-portela/SCARAB` | 0 | 0 | — | active |
| `pablogonz12/Scarlight` | 114 | 37 | 5 | active |
| `PastiiJay/klbf-lab` | 47 | 18 | 6 | active |
| `pragyaangaur/VIctory-macOS` | 441 | 9 | 5 | active |
| `ProhorovIlja/NativeX-Runtime` | 0 | 2 | — | active |
| `R0Y00/Xbox2Windows` | 461 | 155 | 5 | active |
| `realblastex/flamingo` | 17 | 1 | 6 | candidate |
| `realdody/americas-recomp` | 0 | 0 | — | nuked |
| `RickyDivjakovski/Revenge` | 3 | 0 | — | candidate |
| `rpatty-dev/PercyScarlett` | 3 | 0 | 5 | active |
| `rpatty-dev/PercyStar` | 9 | 1 | — | active |
| `sammwyy/xvalkyrie` | 25 | 3 | — | active |
| `SentinelInteractive/0x52504639` | 0 | 0 | — | active |
| `SSTGxf/ScarlettCompat` | 94 | 0 | — | active |
| `Steinkoloss/port-of-the-americas` | 1 | 0 | 5 | active |
| `sub-north/xbox-hle-lab` | 0 | 1 | — | active |
| `ThatRecompDev/five-star-recomp` | 452 | 6 | 5 | active |
| `TheoBIET/OpenVI` | 27 | 1 | — | active |
| `ubiquitarianism/leonida` | 253 | 1397 | 5 | active |
| `uiliannn/VItoriaBr` | 82 | 8 | 6 | active |
| `valouddsite/Xboxx` | 602 | 0 | — | candidate |
| `vitutu1/victory-perf` | 44 | 5 | 6 | active |
| `voidopcode/vilocity` | 25 | 0 | 6 | candidate |
| `walkthestars/GTA6-Asset-Info` | 0 | 0 | — | active |
| `Wertrex/Americas` | 203 | 1 | — | candidate |
| `woreksami/ttpf-studio` | 0 | 0 | — | active |
| `xcl-dev/xcl` | 318 | 1286 | 5 | active |
| `Zettrox/ztrx` | 0 | 0 | 6 | nuked |
| `ZigZagBoi1/ScarlettPort` | 19 | 2 | 6 | active |
| `zohaiblazuli/GrandTheftXbox` | 0 | 0 | — | unreachable |
| `ZSlayerHQ/victory-world-runtime` | 21 | 10 | — | renamed |

