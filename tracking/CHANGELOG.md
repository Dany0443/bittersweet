# Upstream sync - 2026-11-25T19:50:51Z

Sources: <https://vuyp.github.io/gta6-pc-race/>, <https://portwatch-vi.xyz/data/tracker.json>
Window: commits since `2026-11-25T18:32:47Z`
Tracked repositories: **91**

## Newly discovered repositories

| Repository | Name | Family | Stage |
|---|---|---|---|
| [TaynanGT/victory-windows](https://github.com/TaynanGT/victory-windows) | victory-windows | victory-ecosystem | 6 |
| [AntoninMichelq/VIctory-forkAMD](https://github.com/AntoninMichelq/VIctory-forkAMD) | VIctory-forkAMD | victory-ecosystem | - |
| [LowBacon/HybridGt](https://github.com/LowBacon/HybridGt) | HybridGt | victory-ecosystem | - |

## Incomplete API collection

- `mewiof/Holy-Interoperability`: prs; sync checkpoint preserved

## Renamed or moved

- `ZSlayerHQ/victory-world-runtime` now resolves to `ZSlayerHQ/zslayer-pc-port-runtime`

## Unreachable (flag kept, remote disabled)

- `batatinhafcbatatinha-pixel/xseriesrecomp`
- `Dark3581/AnyBox`
- `JPTuba9/xlaunch-vi`
- `nullbrik/RageForge`
- `TaynanGT/victory-windows`
- `vitutu1/victory-perf`
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
| critical-linux | 1376 |
| frame-time | 1685 |
| stability | 2184 |
| rendering | 2048 |
| experimental | 471 |
| unclassified | 488 |

## Critical Linux / Proton-GE fixes

- `Belobug/VIctory-Belobug-AMD` #165 - CI: the crates that run anywhere are tested on Linux too
- `Belobug/VIctory-Belobug-AMD` #166 - GPU: diagnostics builds log the title's asks for frame statistics
- `Belobug/VIctory-Belobug-AMD` #42 - Run on Proton: a Windows build of victory, launched inside a Proton prefix
- `Belobug/VIctory-Belobug-AMD` #41 - Compile and test on Linux, and an Ubuntu CI run
- `Belobug/VIctory-Belobug-AMD` #109 - Fix memory mapping and shader lowering
- `Belobug/VIctory-Belobug-AMD` #53 - Under Wine, title memory is copied, not used in place
- `Belobug/VIctory-Belobug-AMD` #69 - Under Wine, copies of title memory are not sparse buffers by default (GPU reset once the world loads)
- `Belobug/VIctory-Belobug-AMD` #124 - GPU: the empty host pipeline library is made from a null blob (vkd3d-proton refused it: no pipeline kept under Wine)
- `Belobug/VIctory-Belobug-AMD` #100 - GPU: fix Shadow::sparse_writing tile base for 4 KiB-aligned Map() pointers
- `boykopovar/AnyPS5` #25483 - fix(relinker): accept short unwind ranges and name .esbak modules after their SELF
- `boykopovar/AnyPS5` #27301 - fix(libScePosixForWebKit): fill times child counters and round with CONVTCK, as FreeBSD 11.4 does
- `boykopovar/AnyPS5` #26157 - fix(libc): match Dinkumware decimal float conversion
- `boykopovar/AnyPS5` #26391 - fix(relinker): preserve leading spaces in desktop entry names
- `boykopovar/AnyPS5` #27300 - feat(libkernel): take O_SHLOCK and O_EXLOCK locks in open, as FreeBSD 11.4 does
- `boykopovar/AnyPS5` #24152 - fix(relinker): reject imports of data exports at address 0 on Linux
- `boykopovar/AnyPS5` #27303 - fix(libkernel): accept shared anonymous, MAP_ALIGNED and hint flags in mmap, as FreeBSD 11.4 does
- `boykopovar/AnyPS5` #23718 - test(libSceAgcDriver): build the upload cache flush test on Windows
- `boykopovar/AnyPS5` #22366 - fix(agc): set the lease mirror switch with _putenv_s on Windows in the upload cache test
- `boykopovar/AnyPS5` #24341 - ci(build): run macOS full build and CPU tests nightly
- `boykopovar/AnyPS5` #26447 - fix(relinker): copy a moved guest load at its alignment
- `boykopovar/AnyPS5` #27292 - docs(build): explain compilation database inventory variants
- `boykopovar/AnyPS5` #27297 - feat(libkernel): honour RTLD_NODELETE and RTLD_NOLOAD in dlopen, as FreeBSD 11 rtld does
- `boykopovar/AnyPS5` #27299 - fix(libkernel): return EAGAIN from a recursive mutex relocked at its largest count, as FreeBSD 11.4 does
- `boykopovar/AnyPS5` #23420 - fix(relinker): preserve PT_LOAD alignment congruence
- `boykopovar/AnyPS5` #26131 - feat(icu): implement custom break rules and iterator cloning
- `boykopovar/AnyPS5` #26605 - fix(nkwebkit): make nkwebkit_wtf scratch names unique per process
- `boykopovar/AnyPS5` #24062 - fix(relinker): keep the bytes before a Linux PT_LOAD without file byt…
- `boykopovar/AnyPS5` #26149 - feat(font): initialize AGC font render target surfaces
- `boykopovar/AnyPS5` #26393 - fix(libSceIcu): read the text after '@' as a POSIX variant when it has no '='
- `boykopovar/AnyPS5` #27260 - feat(libkernel): skip the end of file in poll with POLLINIGNEOF on sockets and pipes, as FreeBSD 11.4 does
- `boykopovar/AnyPS5` #26235 - feat(libc): implement strptime
- `boykopovar/AnyPS5` #27259 - fix(libkernel): fill getrusage children and ru_maxrss from the host, as FreeBSD 11.4 does
- `boykopovar/AnyPS5` #25524 - feat(libSceIcu): implement the udata swapper exports
- `boykopovar/AnyPS5` #16052 - feat(libSceNpCppWebApi): implement the shared transaction bookkeeping
- `boykopovar/AnyPS5` #16018 - feat(libSceNpCppWebApi): allocate String, Vector and IntrusivePtr objects from the LibContext
- `boykopovar/AnyPS5` #25956 - feat(libkernel): keep POSIX permission bits on Windows
- `boykopovar/AnyPS5` #27157 - fix(tools): validate sampler module and sample counts
- `boykopovar/AnyPS5` #26376 - feat(libScePosixForWebKit): implement getpwuid_r and getpwnam_r
- `boykopovar/AnyPS5` #26354 - feat(libScePosixForWebKit): implement mkstemps
- `boykopovar/AnyPS5` #26303 - feat(libc): implement the float complex trigonometric, exponential and power functions
- `boykopovar/AnyPS5` #25959 - feat(shader): support MODE status bitfields in s_getreg_b32
- `boykopovar/AnyPS5` #24987 - fix(agc): account the draw cache's retained DrawDecode storage
- `boykopovar/AnyPS5` #24350 - feat(libSceNKWebKit): implement the WebCore::Position anchor constructors and containerNode exports
- `boykopovar/AnyPS5` #24339 - feat(libSceNKWebKit): implement the JSC::JSPromise status, result and isHandled exports
- `boykopovar/AnyPS5` #24332 - feat(libSceNKWebKit): implement the WebCore::FrameTree childCount, top, traverseNext and isDescendantOf exports
- `boykopovar/AnyPS5` #24319 - feat(libSceNKWebKit): implement six ContainerNode and Node element navigation exports
- `boykopovar/AnyPS5` #24213 - feat(libSceJsc): implement JSC::JSCell::destroy
- `boykopovar/AnyPS5` #24124 - feat(libSceJsc): implement WTF::SHA1::hexDigest
- `boykopovar/AnyPS5` #18112 - feat(libs): mirror libkernel exports through libkernel_web
- `boykopovar/AnyPS5` #22390 - fix(libSceIcu): copy tokenize and UTF-16 memory guest pointers safely
- `boykopovar/AnyPS5` #21516 - fix(shader): answer HW_ID1 and HW_ID2 reads with zero
- `boykopovar/AnyPS5` #21229 - docs(build): document the release, seed and manifest tools
- `boykopovar/AnyPS5` #20316 - feat(libSceNKWebKit): implement WebCore::FileHandle
- `boykopovar/AnyPS5` #18083 - feat(libSceFreeTypeOl): implement format support helpers
- `boykopovar/AnyPS5` #16654 - feat(libSceLibreSsl3): release BIO references through BIO_vfree
- `boykopovar/AnyPS5` #24197 - feat(libSceJsc): implement JSC::JSCell::getCallData and getConstructData
- `boykopovar/AnyPS5` #20763 - feat(libSceNpCppWebApi): export the Leaderboards instantiations the titles import
- `boykopovar/AnyPS5` #26580 - fix(shader): round the comparison reference of 16_UNORM sampled reads as RDNA2 does
- `boykopovar/AnyPS5` #17144 - feat(libSceSystemService): implement AppMessaging queue and event flags
- `boykopovar/AnyPS5` #27272 - fix(libSceSaveData.native): raise filesystem errors from the module
- `boykopovar/AnyPS5` #25766 - feat(libc): add integer libc++ to_string
- `boykopovar/AnyPS5` #21762 - fix(relinker): lower unaligned 256-bit stores for --to-rosetta
- `boykopovar/AnyPS5` #23443 - fix(libwebrtc): validate SequenceCheckerImpl constructor guest pointer before write
- `boykopovar/AnyPS5` #26280 - test(tools): split test_registration by test class
- `boykopovar/AnyPS5` #26662 - fix(agc): keep the old settle floor in agc_driver_linear_target_import_creation
- `boykopovar/AnyPS5` #24915 - feat(relinker): record the host CPU features --to-avx output needs
- `boykopovar/AnyPS5` #27255 - fix(relinker): export absolute symbols within Windows images
- `boykopovar/AnyPS5` #22186 - feat(libSceIme): drive sceImeOpen with the controller keyboard
- `boykopovar/AnyPS5` #24399 - fix(libSceLibreSsl3): copy the sk_free stack guest record safely
- `boykopovar/AnyPS5` #22392 - fix(libkernel): follow firmware 13.60 for module unwind info flags and failures
- `boykopovar/AnyPS5` #26238 - feat(libkernel): implement execl, execle and execlp
- `boykopovar/AnyPS5` #26736 - fix(tools): preserve progress history cache inputs
- `boykopovar/AnyPS5` #22622 - fix(libSceNpScoreRanking): validate npId and poll result pointers upfront
- `boykopovar/AnyPS5` #26232 - feat(relinker): resolve executable relocations to local, hidden and internal symbols
- `boykopovar/AnyPS5` #27184 - feat(libSceIcu): implement usearch_setText
- `boykopovar/AnyPS5` #27182 - feat(libSceIcu): implement usearch_setAttribute
- `boykopovar/AnyPS5` #27246 - feat(libkernel): print a host backtrace with module info
- `boykopovar/AnyPS5` #26479 - feat(libSceCamera): answer the pre-open constant getters measured on 13.60
- `boykopovar/AnyPS5` #25960 - fix(libkernel): validate guest destination pointers upfront in sceKernelBatchMap2 and sceKernelStopUnloadModule
- `boykopovar/AnyPS5` #23294 - fix(libSceLibreSsl3): lift ASN1_STRING_set 1 MiB cap and bound ASN.1 guest copies
- …and 1296 more (see `tracking/sync-report.json`)

## Frame-time and CPU/GPU bottleneck wins

- `AntoninMichelq/VIctory-forkAMD` #11 - Variante 3/4 : tests d'instructions d'upstream + 3 corrections du traducteur
- `AntoninMichelq/VIctory-forkAMD` #10 - Variante 2/4 : niveau de mip correct pour les cubes avec gradients
- `AntoninMichelq/VIctory-forkAMD` #9 - Variante 1/4 : préparer les shaders à l'avance (translate --cache)
- `AntoninMichelq/VIctory-forkAMD` #4 - GPU: world frame rate (submission preparation, indexed SRVs, pipeline reuse)
- `AntoninMichelq/VIctory-forkAMD` #3 - Check queue fence completion before retiring resources
- `Belobug/VIctory-Belobug-AMD` #178 - in_parallel shares its work with kept threads instead of making threads at every call
- `Belobug/VIctory-Belobug-AMD` #179 - Region epochs are per 64 KiB block, not per 64 MiB (12,300 to 2,550 block walks a world frame)
- `Belobug/VIctory-Belobug-AMD` #162 - Remove the deferred-work lock from host command lists
- `Belobug/VIctory-Belobug-AMD` #139 - Submission CPU time, ambient light following the exposure, unattended benchmark
- `Belobug/VIctory-Belobug-AMD` #155 - Defer one draw's buffer refreshes under one lock
- `Belobug/VIctory-Belobug-AMD` #132 - timeline: wait for busy queues to finish before pump/submit returns (#116)
- `Belobug/VIctory-Belobug-AMD` #156 - HLE: DualSense device paths keep their NUL (CreateFileW read past the copy)
- `Belobug/VIctory-Belobug-AMD` #127 - perf(gpu): cut per-draw and title-memory sync CPU overhead
- `Belobug/VIctory-Belobug-AMD` #44 - Windows 10: load the D3D12 Agility SDK runtime (shader model 6.6)
- `Belobug/VIctory-Belobug-AMD` #123 - GPU: AVX2 fast-path and skip-unchanged writes in merge_gpu_writes*
- `Belobug/VIctory-Belobug-AMD` #88 - Stop scanning the whole shader on every descriptor-load lookup
- `Belobug/VIctory-Belobug-AMD` #75 - Shader IR passes without per-step allocations
- `Belobug/VIctory-Belobug-AMD` #153 - Fix physical page mapping and GPU test assumptions
- `Belobug/VIctory-Belobug-AMD` #152 - Skip the global texture lock when no host-written textures
- `Belobug/VIctory-Belobug-AMD` #120 - GPU: batch hoisted mode-7 indirect argument clamps into 64-lane SIMD dispatches (22x faster)
- `Belobug/VIctory-Belobug-AMD` #145 - Remove per-command mutex from command list state
- `Belobug/VIctory-Belobug-AMD` #64 - Use MD5's constant table instead of 64 sin() per block
- `Belobug/VIctory-Belobug-AMD` #72 - Split vector ALU lowering to restore the asset-policy check
- `Belobug/VIctory-Belobug-AMD` #80 - Hash the DXIL type and constant tables with Fx instead of SipHash
- `Belobug/VIctory-Belobug-AMD` #118 - FidelityFX: Series X host FSR and opt-in AMD frame generation
- `Belobug/VIctory-Belobug-AMD` #65 - Remove dead code in victory-gpu
- `Belobug/VIctory-Belobug-AMD` #58 - Build the indirect mismatch note only in diagnostics builds
- `Belobug/VIctory-Belobug-AMD` #55 - Rectangle expansion reuses the clamp's shader builder
- `Belobug/VIctory-Belobug-AMD` #56 - Skip the pipeline wait's lock and clock once the frame's budget is spent
- `Belobug/VIctory-Belobug-AMD` #54 - Count a list's draws with an atomic instead of a mutex
- `Belobug/VIctory-Belobug-AMD` #14 - xmem: enforce 16 MiB minimum and pre-validate pages in map_whole
- `Belobug/VIctory-Belobug-AMD` #18 - GPU: sparse copies snapshot and copy back their tiles on several threads
- `Belobug/VIctory-Belobug-AMD` #62 - Find the destination heap once per descriptor copy, not per descriptor
- `Belobug/VIctory-Belobug-AMD` #66 - Remove duplicated pad state, store status functions and helpers
- `Belobug/VIctory-Belobug-AMD` #126 - GPU: return the output pointer from GetDesc methods
- `Belobug/VIctory-Belobug-AMD` #21 - Reject overlapping depth slices in volume texture copies
- `Belobug/VIctory-Belobug-AMD` #10 - GPU: a submission's write ranges deduplicated through a set, not a scan
- `Belobug/VIctory-Belobug-AMD` #38 - docs: fix PowerShell launch instructions
- `Belobug/VIctory-Belobug-AMD` #37 - Copy equal-length tile pieces with a constant length when detiling
- `boykopovar/AnyPS5` #27198 - perf(shader): order v_med3_f16 operands once in the integer key domain
- `boykopovar/AnyPS5` #25443 - perf(agc): keep a reused image table's cache entries while none was erased
- `boykopovar/AnyPS5` #26345 - fix(tools): queue pad pulses through symbolic links
- `boykopovar/AnyPS5` #26349 - fix(libSceCesCs): access Unicode conversion buffers through guest copies
- `boykopovar/AnyPS5` #24523 - perf(shader): prepare table-image packed 32-bit unsigned modes at use
- `boykopovar/AnyPS5` #26913 - perf(agc): find the last lookup record of each texture with a hash table
- `boykopovar/AnyPS5` #26800 - perf(shader): keep specialized SPIR-V in the shader disk cache
- `boykopovar/AnyPS5` #26502 - refactor(libSceImeDialog): drop redundant errno saves around guest copies
- `boykopovar/AnyPS5` #26593 - fix(shader): read an image the same shader stores to through its storage binding
- `boykopovar/AnyPS5` #26510 - refactor(libScePsml_debug): drop redundant errno saves around guest copies
- `boykopovar/AnyPS5` #16321 - feat(shader): implement MIMG BY2/BY4 stores for the measured 2D subset
- `boykopovar/AnyPS5` #26509 - refactor(libScePosixForWebKit): drop redundant errno saves around guest copies
- `boykopovar/AnyPS5` #27271 - perf(agc): skip CPU index range scan for in-place draws without vertex attributes
- `boykopovar/AnyPS5` #26512 - refactor(libSceShare): drop redundant errno saves around guest copies
- `boykopovar/AnyPS5` #26513 - refactor(libharfbuzz): drop redundant errno saves around guest copies
- `boykopovar/AnyPS5` #26514 - refactor(libSceNpEntitlementAccess): drop redundant errno saves around guest copies
- `boykopovar/AnyPS5` #26515 - refactor(libSceNpGameIntent): drop redundant errno saves around guest copies
- `boykopovar/AnyPS5` #27268 - perf(agc): reuse PreparedDraw across layered mesh output windows
- `boykopovar/AnyPS5` #20519 - feat(agc): opt-in bindless image heap for sampled image tables
- `boykopovar/AnyPS5` #27288 - fix(agc): give label flushes with no waiter a longer deadline
- `boykopovar/AnyPS5` #21138 - test(libSceVideoOut): audit VRR active status event registration and handle bounds
- `boykopovar/AnyPS5` #27144 - feat(libSceMetadataReaderWriter): implement ParserInterface, Value, Metadata count, and Thumbnail helpers
- `boykopovar/AnyPS5` #12840 - fix(agc): write back BDA stores into writable heap mirrors
- `boykopovar/AnyPS5` #20937 - test(libSceFios2): audit IO filter callbacks, psarc dearchiver descriptor, and cache bounds
- `boykopovar/AnyPS5` #27281 - feat(libSceVdecSavc2): implement sceSdecCreateSw2 and sceSdecDestroySw2 handle lifecycle
- `boykopovar/AnyPS5` #27283 - feat(libSceOpusCeltEnc): implement multistream encoder create and ctl
- `boykopovar/AnyPS5` #21948 - fix(agc): size HTILE that is not pipe-aligned by its own meta blocks
- `boykopovar/AnyPS5` #27093 - refactor(libSceRazorCpu): drop redundant errno saves around guest copies
- `boykopovar/AnyPS5` #26582 - perf(agc): answer draw finisher overlap queries from per-block write counts
- `boykopovar/AnyPS5` #26340 - perf(agc): keep the worker's hot thread state in thread slots
- `boykopovar/AnyPS5` #26041 - fix(agc): read zero from a stride-0 OOB_SELECT 1 V# when every index is past NUM_RECORDS
- `boykopovar/AnyPS5` #25583 - fix(shader): leave a continued callee part before its callee on a target mismatch
- `boykopovar/AnyPS5` #25110 - feat(libSceLibcInternal): implement _Atqexit and _Atthreadexit
- `boykopovar/AnyPS5` #24965 - feat(libSceNKWebKit): implement WebCore::Path and the path DisplayList items
- `boykopovar/AnyPS5` #24445 - perf(agc): clear a storage image on the GPU when its guest memory holds one repeated pattern
- `boykopovar/AnyPS5` #24291 - feat(libSceIcu): implement the uhash UnicodeString hash and compare helpers
- `boykopovar/AnyPS5` #23244 - perf(recompiler): run each wave of a multi-wave compute workgroup as its own host workgroup behind a runtime switch
- `boykopovar/AnyPS5` #23243 - perf(recompiler): add a window-select form for register windows behind a runtime switch
- `boykopovar/AnyPS5` #20130 - fix(agc): reuse a data-only draw stage through a moved table and its data
- `boykopovar/AnyPS5` #19620 - fix(agc): resolve unbounded indirect vertex inputs before copying
- `boykopovar/AnyPS5` #18757 - perf(agc): learn a draw's relocation when its entry is inserted
- …and 1605 more (see `tracking/sync-report.json`)

## Stability and device-loss prevention

- `Belobug/VIctory-Belobug-AMD` #128 - Wave64 on 32-lane hosts: several console waves per group, wave reductions, permlanes and ds_(b)permute in halves (+ two fixes)
- `Belobug/VIctory-Belobug-AMD` #175 - GPU: drop overlapping write-watch pins on title_memory_remapped (#174)
- `Belobug/VIctory-Belobug-AMD` #158 - GPU: VICTORY_COMPILER_GUARD=0 leaves the driver compiler's faults to the driver (NVIDIA)
- `Belobug/VIctory-Belobug-AMD` #157 - HLE: keep address space below 4 GiB for the title's 32-bit graphics memory (NVIDIA start-up crash)
- `Belobug/VIctory-Belobug-AMD` #173 - Shaders: v_cmp_f and v_cmp_tru are constants (fcmp false/true removed WARP's device)
- `Belobug/VIctory-Belobug-AMD` #110 - Shaders: a divergent loop's scalars read after it ask for the console wave size (#2's RDNA2 wave-size failures)
- `Belobug/VIctory-Belobug-AMD` #144 - GPU, Shaders: clamp view/sampler heap indices to dedicated null sentinels and bound StrideTable (#95)
- `Belobug/VIctory-Belobug-AMD` #161 - Title memory is copied, not used in place, past half of the host GPU's system-memory budget (#147)
- `Belobug/VIctory-Belobug-AMD` #40 - Input: DualSense device paths end in a NUL
- `Belobug/VIctory-Belobug-AMD` #68 - Wave64 on 32-lane hosts: one console wave sharing LDS runs in the console flow (+2 NVIDIA fixes)
- `Belobug/VIctory-Belobug-AMD` #140 - Shader tests: WARP runs one draw or dispatch at a time (parallel runs crashed in d3d10warp.dll)
- `Belobug/VIctory-Belobug-AMD` #104 - victory system: this machine for test reports; the commit built from in --version and crash reports
- `Belobug/VIctory-Belobug-AMD` #52 - NVIDIA test failures, and victory-gpu clean with clippy -D warnings
- `Belobug/VIctory-Belobug-AMD` #78 - Check shader test expectations on 16-lane adapters
- `Belobug/VIctory-Belobug-AMD` #22 - Shaders: omit unnecessary fixed WaveSize for single-wave LDS compute shaders
- `Belobug/VIctory-Belobug-AMD` #71 - Fix stale and misplaced comments
- `Belobug/VIctory-Belobug-AMD` #115 - watch: the write-watch test counts only the writes since its first asking (#102)
- `Belobug/VIctory-Belobug-AMD` #67 - Look up views once per range in watch::writing, and drop ended pins
- `Belobug/VIctory-Belobug-AMD` #99 - xmem: map_alias of no pages changes nothing, next to a reservation shown whole too
- `Belobug/VIctory-Belobug-AMD` #60 - Keep pinned blocks writable on guard()'s fast path too
- `Belobug/VIctory-Belobug-AMD` #50 - dstorage: retry raw file read on ERROR_INVALID_USER_BUFFER
- `Belobug/VIctory-Belobug-AMD` #39 - xmem: keep the space below 4 GiB for the title's 32-bit memory (port of VIctory#1009)
- `boykopovar/AnyPS5` #26242 - test(libSceHttp): cover abort request stub
- `boykopovar/AnyPS5` #26221 - fix(agc): refuse a host import over read-trapped pages instead of resolving the trap under its locks
- `boykopovar/AnyPS5` #23405 - fix(libSceJxr): validate guest BitIo* io before attachISRead dereference
- `boykopovar/AnyPS5` #23801 - feat(libSceJsc): implement JSC::HandleSet::writeBarrier
- `boykopovar/AnyPS5` #27302 - fix(libkernel): handle robust PI and PP umutexes in _umtx_op as FreeBSD 11 does
- `boykopovar/AnyPS5` #27236 - fix(libSceJxr): probe guest pointers in factory, stream, and bit-io exports
- `boykopovar/AnyPS5` #27134 - fix(libSceNet): probe guest msg_name before writing it in sceNetRecvmsg
- `boykopovar/AnyPS5` #27293 - fix(tools): reject unknown technical debt report labels
- `boykopovar/AnyPS5` #19628 - feat(icu): implement localized date-time pattern generation
- `boykopovar/AnyPS5` #16427 - feat(libSceAgcDriver): sample guest X24_8_32_FLOAT images through D32_SFLOAT_S8_UINT
- `boykopovar/AnyPS5` #27296 - test(libSceUsbStorage): pin the shell-core mount point guards
- `boykopovar/AnyPS5` #27298 - fix(libc): pass a null key to the comparator in bsearch and lfind and accept a zero bsearch element size, as FreeBSD 11.4 does
- `boykopovar/AnyPS5` #21222 - feat(libSceIcu): implement currency service APIs
- `boykopovar/AnyPS5` #26756 - fix(shader): keep waves that end early at their workgroup's later barriers
- `boykopovar/AnyPS5` #27163 - feat(libSceAsyncStorageInternal): implement global handler slots and stat counters
- `boykopovar/AnyPS5` #27162 - feat(libSceVideoStreamingEngineAudioOnly): implement not-initialized state guards
- `boykopovar/AnyPS5` #27278 - fix(libSceNgs2): fix natural-order packing and header layout in FFT exports
- `boykopovar/AnyPS5` #27160 - feat(libSceWebTransport): implement config, logging, stats, and backend mode
- `boykopovar/AnyPS5` #26720 - feat(libSceNpSignaling): report the heap statistics from the module's mspace
- `boykopovar/AnyPS5` #27141 - feat(ulobjmgr): implement 0x3ffc object freelist and global slot exports
- `boykopovar/AnyPS5` #25647 - feat(libSceNKWebKit): implement WKBundleCreateWKDataFromUInt8Array
- `boykopovar/AnyPS5` #27142 - feat(libSceVnaInternal): implement sceVnaGetErrorSeverity and VNA util state exports
- `boykopovar/AnyPS5` #11831 - feat(libc): implement fcloseall with tracked stream lifetimes
- `boykopovar/AnyPS5` #26671 - feat(libSceIcu): implement the Normalizer and RuleBasedBreakIterator class IDs
- `boykopovar/AnyPS5` #26099 - fix(shader): round v_div_fmas_f32 in the directed FLOAT_MODE rounding modes
- `boykopovar/AnyPS5` #27284 - feat(libSceImageUtil): implement callback registration and agent stop exports
- `boykopovar/AnyPS5` #24920 - feat(libSceNKWebKit): implement WKPage reload and stop loading controls
- `boykopovar/AnyPS5` #24905 - feat(libSceLibcInternal): implement wide num_put::_Iput
- `boykopovar/AnyPS5` #24347 - feat(libSceNKWebKit): implement the JSDocument, JSElement, JSEvent and JSNode toWrapped exports
- `boykopovar/AnyPS5` #22774 - fix(shader): call load_bda_span outside read_bda_span's per-lane selection
- `boykopovar/AnyPS5` #22132 - fix(libSceIcu): copy invariant conversion guest pointers safely
- `boykopovar/AnyPS5` #21930 - feat(libc): wire the numpunct<char> facet vtable and its members
- `boykopovar/AnyPS5` #17711 - feat(libSceGnmDriver): share mode-independent command writers
- `boykopovar/AnyPS5` #20947 - test(libSceFiber): audit green threading, context parameters, stack guard pages, and run error bounds
- `boykopovar/AnyPS5` #27285 - feat(libSceAudioPropagation): validate system handles in inventory query exports
- `boykopovar/AnyPS5` #27286 - feat(libSceGvMp4Parser): implement 13.60 validation and return gates
- `boykopovar/AnyPS5` #27287 - feat(libSceNpTrophy2): implement 13.60 system handle, context, callback, and icon URI exports
- `boykopovar/AnyPS5` #24439 - fix(libSceIcu): copy uloc_getKeywordValue and uloc_getParent guest pointers safely
- `boykopovar/AnyPS5` #26227 - fix(shader): write both bytes of a 16-bit buffer store that crosses a dword outside compute
- `boykopovar/AnyPS5` #26835 - feat(libSceVideoOutSecondary): implement firmware 13.60 slot table exports KJWkt9y7YC4 and s+isXKLpwS0
- `boykopovar/AnyPS5` #26759 - test(libSceNgs2): cover the FFT work-size geometry and argument checks
- `boykopovar/AnyPS5` #26679 - test(agc): wait for the draw-error record before reading the failure trace
- `boykopovar/AnyPS5` #26674 - feat(libSceIcu): implement Normalizer2Impl::addLcccChars
- `boykopovar/AnyPS5` #26884 - feat(libSceNKWebKit): implement WKBundleNodeHandle autofill button type operations
- `boykopovar/AnyPS5` #26488 - test(tools): split the configure seed round trips into two CTest entries
- `boykopovar/AnyPS5` #27275 - fix(libkernel): read and write sceKernelWaitSema's timeout through guest copies
- `boykopovar/AnyPS5` #19864 - fix(libc): route exception deleting destructors through guest heap
- `boykopovar/AnyPS5` #24457 - feat(libSceNKWebKit): implement WKUserMediaPermissionCheckSetUserMediaAccessInfo
- `boykopovar/AnyPS5` #27225 - fix(libSceNpManager): probe guest pointers in value class helpers
- `boykopovar/AnyPS5` #22723 - fix(libSceFont): guard shared font reference increments
- `boykopovar/AnyPS5` #18671 - fix(libSceVoice): return InvalidArgument from sceVoiceInit for unreadable parameter pointer
- `boykopovar/AnyPS5` #26413 - feat(libSceJsc): implement WTFLogAlwaysV
- `boykopovar/AnyPS5` #26731 - fix(libc): guard strsep guest memory
- `boykopovar/AnyPS5` #22215 - fix(libSceAgc): validate and safely copy semaphore label pointer in sceAgcGetSemaphoreLabel
- `boykopovar/AnyPS5` #23081 - fix(libSceFreeType): copy FT_Get_Glyph_Name and FT_Get_Sfnt_Name outputs safely
- `boykopovar/AnyPS5` #27172 - feat(libSceCamera): validate inventory exports and implement sceCameraIsVrCaptureEnable
- `boykopovar/AnyPS5` #27173 - feat(libSceTtsCoreG3Bc): implement 6 verified engine helper exports
- `boykopovar/AnyPS5` #21550 - fix(libSceJxr): access the image encoder and its stream through checked guest copies
- …and 2104 more (see `tracking/sync-report.json`)

## Rendering and gameplay completeness

- `Ahmet-Dedeler/bcpack-rs` #8 - rANS: speed up multi-lane slice decoder by 1.8-2.8x
- `AntoninMichelq/VIctory-forkAMD` #8 - Stable "Jouer" line of 2026-11-25 as main (FSR, texture fixes, warm shader cache)
- `AntoninMichelq/VIctory-forkAMD` #1 - Integration: upstream main + useful open upstream PRs + our fixes
- `AntoninMichelq/VIctory-forkAMD` #13 - Variante 5/5 : préchargement des shaders (version Jouer du 25/11 au matin)
- `AntoninMichelq/VIctory-forkAMD` #12 - Variante 4/4 : la version rapide testée cette nuit (mémoire stable, environ 6 fps)
- `AntoninMichelq/VIctory-forkAMD` #2 - Preserve first-lane selection for dynamic code-table offsets
- `Archivist-Atlas/gta-vi-astral` `d5b7912e0ca7` - fix(audio/pacing): 60Hz WaitFrameEventX timer pacing and README audio update
- `Archivist-Atlas/gta-vi-astral` `3f37783b6513` - feat(audio): real pure-Go Opus decoding in xapu.dll and AcpHal ABI fixes
- `Archivist-Atlas/gta-vi-astral` `4fe50e86e0ed` - shader/hlsl: implement per-lane EXEC mask (fixes 12.92x sRGB blowup) & MIMG dmask channel write
- `Archivist-Atlas/gta-vi-astral` `c4b7fc86bff3` - shader/gpu: force PS StageOut to float4 SV_Target0 (fixes E_INVALIDARG 0x80070057 on Legal screen PSOs)
- `Belobug/VIctory-Belobug-AMD` #138 - Shaders, GPU: port VIctory#2398, link over-budget mesh and pixel shaders under the 32 KiB output limit (#125)
- `Belobug/VIctory-Belobug-AMD` #182 - runtime: validate config, Device8 IID and query heaps
- `Belobug/VIctory-Belobug-AMD` #181 - formats: read the first BC7 dictionary entry's 32 indices
- `Belobug/VIctory-Belobug-AMD` #180 - shader: fix source-less VOP1 length and VOP3 CMPX destination
- `Belobug/VIctory-Belobug-AMD` #150 - GPU: improve texture uploads and specialize single-record indirect draws
- `Belobug/VIctory-Belobug-AMD` #119 - Speed up bindless planning, BCPack decoding, and shader translation
- `Belobug/VIctory-Belobug-AMD` #167 - Reuse initialized scratch space for texture reads
- `Belobug/VIctory-Belobug-AMD` #169 - HLE: harden media source resolver URL and UNC path validation
- `Belobug/VIctory-Belobug-AMD` #7 - Add Xbox texture layouts for formats 189–190
- `Belobug/VIctory-Belobug-AMD` #35 - Precompute the XMA inverse MDCT's sine/cosine tables
- `Belobug/VIctory-Belobug-AMD` #84 - Extract scalar and vector ALU lowering from lower.rs
- `Belobug/VIctory-Belobug-AMD` #70 - Remove unreachable arms and dead code in the shader translator
- `Belobug/VIctory-Belobug-AMD` #81 - Shaders: 64-bit integer compares on their dwords (AMD orders a shared-dword i64 compare wrong)
- `Belobug/VIctory-Belobug-AMD` #43 - GPU: sanitize resource barrier bind flags and UAV targets
- `Belobug/VIctory-Belobug-AMD` #148 - Shaders: a buffer view picked from the resource heap by index is read as a raw buffer, not a constant buffer (missing world geometry and device removal on a Radeon 8060S)
- `Belobug/VIctory-Belobug-AMD` #3 - Clippy: victory-shader is clean again with -D warnings (10 lints, no behaviour change)
- `Belobug/VIctory-Belobug-AMD` #142 - Carry 20 upstream VIctory fixes (shaders, samplers, textures) and a mesh wave-ops flag fix
- `Belobug/VIctory-Belobug-AMD` #112 - watch: clear stale JOURNAL.written entries on title_memory_remapped (#102)
- `Belobug/VIctory-Belobug-AMD` #57 - Fix underflow for textures described with MipLevels 0
- `Belobug/VIctory-Belobug-AMD` #135 - Shaders: f16 results round to nearest even, and min/max/med3 read denormal operands as zeros
- `Belobug/VIctory-Belobug-AMD` #51 - gpu: skip self-CopyResource when src and dst share host texture
- `Belobug/VIctory-Belobug-AMD` #86 - translate --warm: translate every shader into the run's cache ahead of a run
- `Belobug/VIctory-Belobug-AMD` #129 - Shader tests: a cross-device VALU fuzzer (hardware adapters vs WARP), ignored by default
- `Belobug/VIctory-Belobug-AMD` #63 - Stop re-zeroing the texture scratch buffer, and reuse the placed-texture set
- `Belobug/VIctory-Belobug-AMD` #73 - Reuse one Opus decode buffer per connection
- `Belobug/VIctory-Belobug-AMD` #111 - ci: report all failing test targets
- `Belobug/VIctory-Belobug-AMD` #61 - Let bindless draws take the cheap check once the frame's rechecks are spent
- `Belobug/VIctory-Belobug-AMD` #47 - docs: clarify shader targeting and GPU diagnostics
- `Belobug/VIctory-Belobug-AMD` #32 - Check a texture for writes without building lists
- `Belobug/VIctory-Belobug-AMD` #31 - Read a host texture's flags once instead of on every use
- `Belobug/VIctory-Belobug-AMD` #23 - XMA decoder: lock only voices that are on, sleep while none is
- `boykopovar/AnyPS5` #27295 - fix(agc): refuse image-table entries whose base address is not mapped
- `boykopovar/AnyPS5` #27306 - fix(agc): run PS4 indirect draws that patch the VS, ES or LS user data
- `boykopovar/AnyPS5` #26167 - fix(shader): count a MUBUF LDS load as an m0 read when splitting a table call
- `boykopovar/AnyPS5` #26766 - build(shader): update SPIRV-Tools for float-control optimization
- `boykopovar/AnyPS5` #26405 - fix(libSceJxr): translate the end of the WMP record for native code on hosts with a 32-bit long
- `boykopovar/AnyPS5` #27304 - feat(tools): model compiler options and save-temps safely
- `boykopovar/AnyPS5` #27305 - fix(shader): avoid the Defender divide-by-65 signature
- `boykopovar/AnyPS5` #22692 - feat(shader): store cached SPIR-V compressed with zstd
- `boykopovar/AnyPS5` #6977 - feat: run PS4 games on the AGC driver — Limbo (CUSA01664) in game (integration branch)
- `boykopovar/AnyPS5` #26430 - feat(libSceNKWebKit): add the legacy static properties of RegExp
- `boykopovar/AnyPS5` #27289 - fix(tools): preserve trace diff table cells for multiline names
- `boykopovar/AnyPS5` #27290 - docs(testing): explain how to align trace comparison windows
- `boykopovar/AnyPS5` #27291 - docs(input): explain repeat timing in controller previews
- `boykopovar/AnyPS5` #27294 - docs(tracing): show how to isolate a Windows sampling window
- `boykopovar/AnyPS5` #27013 - feat(tools): add pr_overlap --from-file to take names from a file
- `boykopovar/AnyPS5` #26390 - fix(libSceIcu): normalize the keywords of uloc_canonicalize as ICU does
- `boykopovar/AnyPS5` #27277 - feat(libSceAudiodecCpu): implement DVD LPCM decode and HE-VAG lifecycle
- `boykopovar/AnyPS5` #26040 - feat(agc): render PS4 color views of several slices through the vertex render target index
- `boykopovar/AnyPS5` #27279 - feat(libSceVideodec2): implement 13.60 validation gates for slice decode exports
- `boykopovar/AnyPS5` #27280 - fix(libSceVdecShevc): check inner decoder pointer in no-decoder gate
- `boykopovar/AnyPS5` #27216 - feat(libSceAudioPropagation): render three-point material paths from console measurements
- `boykopovar/AnyPS5` #27282 - feat(libSceAudioOut): implement 13.60 validation gates for AudioOut2 inventory exports
- `boykopovar/AnyPS5` #24907 - feat(libSceLibcInternal): implement char num_put::_Fput
- `boykopovar/AnyPS5` #26630 - fix(agc): size tessellation ESGS items from the domain program
- `boykopovar/AnyPS5` #7085 - feat(agc): back extra guest mip levels with storage images
- `boykopovar/AnyPS5` #26706 - fix(agc): cover the final stride in vertex bindings
- `boykopovar/AnyPS5` #26197 - feat(agc): support sampler border colour tables
- `boykopovar/AnyPS5` #22462 - fix(libSceLibcInternal): copy the time_get, time_put and getopt guest pointers safely
- `boykopovar/AnyPS5` #21611 - test(libSceFiber): audit switch register preservation, stack canaries, and roundtrips
- `boykopovar/AnyPS5` #21520 - fix(libSceAgcDriver): correct resource registration lifecycle results
- `boykopovar/AnyPS5` #21330 - fix(libwebrtc): copy RTCP packet guest pointers safely
- `boykopovar/AnyPS5` #27139 - feat(libSceVdecCore.native): implement BID/SVP decoder creation, map memory, and SVP input
- `boykopovar/AnyPS5` #20860 - test(libSceSaveData): audit progress polling, cancel recovery, and mount result layout
- `boykopovar/AnyPS5` #12792 - fix(agc): compare pending storage results over a surface whose page walk fails
- `boykopovar/AnyPS5` #26575 - fix(agc): keep the minification filter of an S# without a mip filter
- `boykopovar/AnyPS5` #22472 - fix(libSceNKWebKit): copy WTF::Persistence and WTF::URL guest pointers safely
- `boykopovar/AnyPS5` #27218 - fix(agc): tell apart the storage images a cache release keeps for pending results and for use
- `boykopovar/AnyPS5` #16880 - fix(agc): read single-sample views of native multisample depth from guest memory
- `boykopovar/AnyPS5` #26655 - feat(libSceAudiodec.native): implement codec registration and unregistration
- …and 1968 more (see `tracking/sync-report.json`)

## Experimental / unstable (hold)

- `Belobug/VIctory-Belobug-AMD` #172 - Shaders: leave out branches no lane takes before lowering (works around an out-of-memory pipeline creation)
- `Belobug/VIctory-Belobug-AMD` #16 - xmem tests take the per-page path again; a reservation shown whole has its own test
- `boykopovar/AnyPS5` #26394 - fix(libSceIcu): drop root and und from a locale ID only where ICU does
- `boykopovar/AnyPS5` #19697 - feat(agc): record depth-only draws on a recording pool behind APS5_PARALLEL_ATLAS
- `boykopovar/AnyPS5` #26406 - fix(libSceIcu): keep trailing and doubled separators in the base name of a locale ID
- `boykopovar/AnyPS5` #26036 - fix(agc): take an indexed draw's base vertex as signed
- `boykopovar/AnyPS5` #19434 - feat(libSceLibcInternal): forward the firmware libc-internal export surface
- `boykopovar/AnyPS5` #26802 - fix(agc): bind a PS4 V# of the range limit up to the end of the registered run behind it
- `boykopovar/AnyPS5` #25077 - fix(libSceNpSessionSignaling): hold mutex across custom property exchange
- `boykopovar/AnyPS5` #25017 - feat(libkernel): create SOCK_SEQPACKET socketpairs on macOS
- `boykopovar/AnyPS5` #1835 - feat(aot): discuss native ARM64 compiler architecture and integration
- `boykopovar/AnyPS5` #18082 - fix(agc): re-land pipeline cache persistence with pipeline binaries off on NVIDIA proprietary
- `boykopovar/AnyPS5` #17936 - fix(libkernel_sys): answer sceKernelGetPhysPageSize for the mapping holding an address
- `boykopovar/AnyPS5` #17719 - feat(libSceErrorDialog): show error dialogs as host message boxes with ANYPS5_ERROR_DIALOG_HOST
- `boykopovar/AnyPS5` #17626 - feat(libSceLibreSsl3): implement ECDSA_do_sign
- `boykopovar/AnyPS5` #17610 - feat(libc): fill the std::__sce_v2 classic locale with its facets
- `boykopovar/AnyPS5` #17555 - feat(debug): pause and resume a guest thread at a breakpoint
- `boykopovar/AnyPS5` #17261 - feat(libc): add a breakpoint table for guest NIDs, off by default
- `boykopovar/AnyPS5` #16791 - fix(videoout): end shutdown when a host thread cannot unwind
- `boykopovar/AnyPS5` #15297 - fix(agc): mirror the whole contiguous run of releasable allocations a joined heap belongs to
- `boykopovar/AnyPS5` #1395 - feat(agc): add experimental depth clear and copy recognition
- `boykopovar/AnyPS5` #13172 - feat(mscorlib.dll): declare 373 callable exports
- `boykopovar/AnyPS5` #12863 - feat(libSceVideoOut): opt-in host pause with a key or on focus loss
- `boykopovar/AnyPS5` #10656 - perf(agc): copy contiguous rows of a drawn linear target store in one region
- `boykopovar/AnyPS5` #25322 - fix(agc): use wave LDS memory barriers on NVIDIA drivers before 596
- `boykopovar/AnyPS5` #25824 - fix(agc): let draw recipes adopt replacement templates
- `boykopovar/AnyPS5` #25530 - feat(libSceNKWebKit): implement WKBundleRangeHandleCreate and WKBundleRangeHandleGetBoundingRectInWindowCoordinates
- `boykopovar/AnyPS5` #26253 - fix(libSceIcu): delete guest UObjects through their System V vtable
- `boykopovar/AnyPS5` #26150 - feat(libkernel_sys): implement randomized path query
- `boykopovar/AnyPS5` #20635 - feat(shader): allow directed rounding modes for packed f16 fma
- `boykopovar/AnyPS5` #20580 - fix(libc): order read-trap resolution against the GPU mutex
- `boykopovar/AnyPS5` #17303 - refactor(prx): name 191 more LibreSsl3 parent-export aliases the NID databases resolve
- `boykopovar/AnyPS5` #19904 - fix(agc): import uniform initial multisample contents under a compressed CMASK
- `boykopovar/AnyPS5` #11452 - feat(agc): count why a recorded draw builds no recipe
- `boykopovar/AnyPS5` #27147 - fix(tools): require a probe for opt-in-only oracle catalogs
- `boykopovar/AnyPS5` #21978 - fix(libkernel): prioritize queued flock upgrades
- `boykopovar/AnyPS5` #23545 - fix(agc): support four-fragment compressed FMASK
- `boykopovar/AnyPS5` #26338 - fix(agc): mark a resolve destination after releasing the source's import gate hold
- `boykopovar/AnyPS5` #26165 - fix(agc): wait a bounded time for a dispatch's compute pipeline built ahead before skipping it
- `boykopovar/AnyPS5` #26034 - fix(agc): settle host imports under a native depth fill before its watch check
- `boykopovar/AnyPS5` #25932 - fix(agc): avoid Intel UHD A78B pipeline library crashes
- `boykopovar/AnyPS5` #25609 - fix(shader): sum v_readfirstlane over the guest wave instead of shuffling it from the first lane
- `boykopovar/AnyPS5` #25573 - feat(libSceLibcInternal): implement numpunct, facet construction and stream runtime
- `boykopovar/AnyPS5` #25491 - fix(libkernel): lock macOS append-mode positioned writes against offset changes
- `boykopovar/AnyPS5` #25226 - fix(agc): wait briefly for a dispatch's background shader work instead of dropping it silently
- `boykopovar/AnyPS5` #25160 - feat(libSceNKWebKit): implement WTF::PrintStream::printfVariableFormat and WTF::MetaAllocatorHandle::dump
- `boykopovar/AnyPS5` #24819 - fix(libkernel): report module unwind names longer than MAX_PATH on Windows
- `boykopovar/AnyPS5` #24070 - fix(libkernel): stop losing _umtx_op umutex wakeups to a userland unlock
- `boykopovar/AnyPS5` #23632 - fix(shader): clamp image_gather4h_pck rows and read array and 3D slices
- `boykopovar/AnyPS5` #22684 - feat(libSceNKWebKit): implement WTF symbols and the symbol registry
- `boykopovar/AnyPS5` #22370 - feat(libSceLibcInternal): complete global locale lifecycle
- `boykopovar/AnyPS5` #22301 - fix(libkernel): safely probe the rwlock slot in scePthreadRwlockDestroy
- `boykopovar/AnyPS5` #20686 - feat(shader): translate v_fma_mixlo/mixhi_f16 with flushed f16 denormals
- `boykopovar/AnyPS5` #20344 - fix(agc): do not abort on a screenshot trigger that another process holds open
- `boykopovar/AnyPS5` #13547 - feat(libSceLibreSsl): implement X509_INFO_free
- `boykopovar/AnyPS5` #26796 - fix(libc): port casinhl, casinl, cacosl and cacoshl from FreeBSD's catrigl.c
- `boykopovar/AnyPS5` #20272 - fix(tools): ignore a carriage return at the end of debt lines in check_conventions
- `boykopovar/AnyPS5` #9776 - feat(aot): add opt-in LLVM integer leaf compiler
- `boykopovar/AnyPS5` #6209 - perf(agc): a DCC key proof over untracked memory holds until the next ordering point
- `boykopovar/AnyPS5` #26797 - fix(shader): refuse a descriptor web once it holds 512 states
- `boykopovar/AnyPS5` #26686 - fix(agc): work around AMD mesh barrier compiler crash
- `boykopovar/AnyPS5` #22066 - fix(agc): publish the idle epoch under the driver mutex before waking waiters
- `boykopovar/AnyPS5` #1384 - feat(ngs2): add experimental I3DL2 reverb
- `boykopovar/AnyPS5` #11134 - fix(http): trust the system CA list for HTTPS requests
- `boykopovar/AnyPS5` #9633 - fix(shader): follow a conditional pointer chase through a V# behind a pointer-keyed record pointer
- `boykopovar/AnyPS5` #9457 - fix(shader): follow pointer hops and fixed-offset V# reads behind a record pointer
- `boykopovar/AnyPS5` #8939 - fix(shader): take lane-mask arms and a merged high word in a pointer-keyed record pointer
- `boykopovar/AnyPS5` #25354 - fix(libSceLibreSsl3): queue errors for types EVP_PKEY_new_mac_key rejects
- `boykopovar/AnyPS5` #23088 - feat(libSceLibreSsl): implement SSL_set_SSL_CTX
- `boykopovar/AnyPS5` #20349 - fix(libkernel): place a fixed mapping at any page-aligned address whatever its alignment
- `boykopovar/AnyPS5` #26146 - fix(ngs2): ramp standard mastering gain across the next grain
- `boykopovar/AnyPS5` #21598 - fix(libSceLibreSsl): copy the CMAC message and tag through guest copies
- `boykopovar/AnyPS5` #25351 - fix(libSceLibreSsl3): cache name encodings of certificates loaded by X509_STORE_load_locations
- `boykopovar/AnyPS5` #25265 - fix(libc): support arguments, stop_token dispatch, and rvalue tasks in StartHostJThread
- `boykopovar/AnyPS5` #24466 - fix(libc): store _Mtx_init's and _Cnd_init's handles through checked guest copies
- `boykopovar/AnyPS5` #24409 - fix(libSceUsbd): check control-transfer output with a host write
- `boykopovar/AnyPS5` #24287 - feat(libSceJsc): implement JSC::CallFrame::isAnyWasmCallee
- `boykopovar/AnyPS5` #23913 - test(tools): copy Apple's Python framework parts with the title_run fake relinker
- `boykopovar/AnyPS5` #23396 - fix(agc): keep image pooling when only another heap is over budget
- `boykopovar/AnyPS5` #22849 - feat(libSceHmd): answer 17 screen and capture calls as firmware 13.42 does before their open
- …and 391 more (see `tracking/sync-report.json`)

## Per-repository activity

| Repository | Commits | Open PRs | Stage | Status |
|---|---|---|---|---|
| `Ahmet-Dedeler/awesome-gta6-reverse-engineering` | 0 | 0 | - | active |
| `Ahmet-Dedeler/bcpack-rs` | 0 | 1 | - | active |
| `Ahmet-Dedeler/rage9-re` | 0 | 0 | - | active |
| `Albotterin/AWC_from_RPF9_Extractor` | 0 | 0 | - | active |
| `anrq0/SeriesX-Emulator` | 0 | 0 | - | active |
| `AntoninMichelq/VIctory-forkAMD` | 0 | 10 | - | active |
| `Archivist-Atlas/gta-vi-astral` | 5 | 0 | - | candidate |
| `batatinhafcbatatinha-pixel/xseriesrecomp` | 0 | 0 | - | unreachable |
| `Belobug/VIctory-Belobug-AMD` | 0 | 101 | 6 | active |
| `boykopovar/AnyPS5` | 9 | 2685 | - | active |
| `brcly/TuxBox` | 9 | 4 | 5 | active |
| `breadbbbb/rpf9-explorer` | 0 | 1 | - | active |
| `californiaAlmonds/OpenScarlett` | 0 | 2 | 5 | active |
| `cocainebunnylaboratories/cblvictoryvi` | 0 | 0 | 6 | active |
| `daeken/XSXEmu` | 2 | 0 | - | candidate |
| `Dark3581/AnyBox` | 0 | 0 | - | unreachable |
| `DexNom/VIctory` | 0 | 31 | 6 | active |
| `edsuuu/VI-BR-PORT` | 9 | 0 | 6 | active |
| `edupode/anyxboxseries` | 0 | 0 | - | active |
| `ethan00013/keylime` | 0 | 0 | 5 | candidate |
| `fablinoo/kelvaroq` | 0 | 0 | 6 | candidate |
| `fishingpvalues/gta6-pc-port-hub` | 0 | 0 | - | archived |
| `Freminet/ScarlettX` | 0 | 26 | - | active |
| `fxceless/victory-fork` | 3 | 819 | 6 | active |
| `gookie-dev/g6shim` | 0 | 1 | 5 | active |
| `grzegorz2047/TrinityXSX` | 1 | 0 | - | candidate |
| `holdmysocks/SixNG` | 9 | 3 | - | active |
| `HolyRoses/gta6-explorer` | 0 | 0 | - | active |
| `Hylanderry/DedSec` | 0 | 0 | - | active |
| `Itsbatt/victory-unified` | 2 | 0 | 6 | candidate |
| `ItsNotRexed/Leonida` | 0 | 0 | - | nuked |
| `jadeowaowa/scarlett-layer` | 0 | 2 | - | active |
| `JamesCicada/GTAVI-Recomp` | 0 | 5 | - | active |
| `JMAN730/project-flamingo` | 2 | 4 | - | active |
| `JoshRob297/rpf9-research` | 0 | 3 | - | active |
| `JPTuba9/xlaunch-vi` | 0 | 0 | - | unreachable |
| `ka1qux/BRVIctory-ka1qux` | 9 | 0 | 6 | candidate |
| `Kajcsi/G6Recomp` | 0 | 0 | - | active |
| `kellysix/kelly` | 0 | 0 | - | candidate |
| `kfedora/GTAVI-Xbox2PC` | 0 | 0 | - | candidate |
| `KomanderDG88/VIniVIdiVIci` | 16 | 0 | 6 | active |
| `kry-vex-gta/XBOX-Inspector` | 0 | 0 | - | active |
| `Librett0/vice-lens` | 0 | 0 | - | active |
| `ljm-233/Xbgtasix` | 0 | 0 | - | active |
| `LooperSalty/AnyXboxX` | 6 | 4 | 6 | active |
| `LoverCatys/ruff-lab` | 3 | 0 | 5 | active |
| `LowBacon/HybridGt` | 2 | 0 | - | active |
| `luizwidmer/palmetto` | 9 | 0 | 4 | active |
| `lynxnathan/rage-tools-vi` | 0 | 0 | - | active |
| `marvisak/VIctory` | 0 | 1265 | 6 | active |
| `meetox80/6stars` | 0 | 0 | - | active |
| `mewiof/Holy-Interoperability` | 0 | 0 | 4 | active |
| `mkr2tl/xenstride-public` | 0 | 0 | - | active |
| `myokex/victory-amd` | 6 | 11 | 6 | active |
| `nineforge0/forge9` | 0 | 0 | - | active |
| `nullbrik/RageForge` | 0 | 0 | - | unreachable |
| `omonkeyz/xbox-emulator` | 0 | 21 | 3 | active |
| `OnlyMoisties/GTA6-BootTest` | 0 | 0 | 5 | active |
| `OpenCommunityEdition/OpenScarlett` | 0 | 1 | 5 | active |
| `p-portela/SCARAB` | 0 | 0 | - | active |
| `pablogonz12/Scarlight` | 53 | 25 | 5 | active |
| `PastiiJay/klbf-lab` | 4 | 20 | 6 | active |
| `pragyaangaur/VIctory-macOS` | 0 | 13 | 5 | active |
| `ProhorovIlja/NativeX-Runtime` | 0 | 2 | - | active |
| `R0Y00/Xbox2Windows` | 51 | 137 | 5 | active |
| `realblastex/flamingo` | 3 | 1 | 6 | candidate |
| `realdody/americas-recomp` | 0 | 0 | - | nuked |
| `RickyDivjakovski/Revenge` | 0 | 0 | - | candidate |
| `rpatty-dev/PercyScarlett` | 0 | 3 | 5 | active |
| `rpatty-dev/PercyStar` | 0 | 1 | - | active |
| `sammwyy/xvalkyrie` | 0 | 3 | - | active |
| `SentinelInteractive/0x52504639` | 0 | 0 | - | active |
| `SSTGxf/ScarlettCompat` | 0 | 0 | - | active |
| `Steinkoloss/port-of-the-americas` | 0 | 0 | 5 | active |
| `sub-north/xbox-hle-lab` | 0 | 1 | - | active |
| `TaynanGT/victory-windows` | 0 | 0 | 6 | unreachable |
| `ThatRecompDev/five-star-recomp` | 24 | 4 | 5 | active |
| `TheoBIET/OpenVI` | 0 | 1 | - | active |
| `ubiquitarianism/leonida` | 19 | 1426 | 5 | active |
| `uiliannn/VItoriaBr` | 0 | 12 | 6 | active |
| `valouddsite/Xboxx` | 5 | 0 | - | candidate |
| `vitutu1/victory-perf` | 0 | 0 | 6 | unreachable |
| `voidopcode/vilocity` | 1 | 0 | 6 | candidate |
| `walkthestars/GTA6-Asset-Info` | 0 | 0 | - | active |
| `Wertrex/Americas` | 0 | 1 | - | candidate |
| `woreksami/ttpf-studio` | 0 | 0 | - | active |
| `xcl-dev/xcl` | 43 | 1281 | 5 | active |
| `Zettrox/ztrx` | 0 | 0 | 6 | nuked |
| `ZigZagBoi1/ScarlettPort` | 3 | 3 | 6 | active |
| `zohaiblazuli/GrandTheftXbox` | 0 | 0 | - | unreachable |
| `ZSlayerHQ/victory-world-runtime` | 0 | 10 | - | renamed |

