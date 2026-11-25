# Upstream sync — 2026-11-25T17:46:23Z

Source: <https://vuyp.github.io/gta6-pc-race/>  
Window: commits since `2026-11-25T00:00:00Z`  
Tracked repositories: **86**

## Newly discovered repositories

_None: the manifest already covered every public link on the page._

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
| critical-linux | 84 |
| frame-time | 589 |
| stability | 269 |
| rendering | 859 |
| experimental | 69 |
| unclassified | 1819 |

## Critical Linux / Proton-GE fixes

- `Belobug/VIctory-Belobug-AMD` #42 — Run on Proton: a Windows build of victory, launched inside a Proton prefix
- `Belobug/VIctory-Belobug-AMD` #41 — Compile and test on Linux, and an Ubuntu CI run
- `Belobug/VIctory-Belobug-AMD` #109 — Fix memory mapping and shader lowering
- `Belobug/VIctory-Belobug-AMD` #53 — Under Wine, title memory is copied, not used in place
- `Belobug/VIctory-Belobug-AMD` #69 — Under Wine, copies of title memory are not sparse buffers by default (GPU reset once the world loads)
- `Belobug/VIctory-Belobug-AMD` #124 — GPU: the empty host pipeline library is made from a null blob (vkd3d-proton refused it: no pipeline kept under Wine)
- `boykopovar/AnyPS5` #27272 — fix(libSceSaveData.native): raise filesystem errors from the module
- `boykopovar/AnyPS5` `11c9e7764efa` — Merge pull request #24706 from olegon85/fix/libkernel-posix-mlock-locks-mapped-pages
- `brcly/TuxBox` `faf8b197d4e6` — d3d12, bvh: stop TuxBox deadlocking the title's queues on its fence labels
- `brcly/TuxBox` `4b4c08cfe7c8` — d3d12: look up cached shaders without scanning the cache directory
- `cocainebunnylaboratories/cblvictoryvi` `a00752cbd2b5` — Pipeline library: an empty host library is created from a null blob, so caching works on Wine/vkd3d-proton (gmankab, based on Marvisak/VIctory#347)
- `ethan00013/keylime` `361c7e94ab9d` — README: the driver deadlock behind the cutscene freeze
- `fxceless/victory-fork` #1011 — Indirect: the argument clamp in a form Wine's HLSL compiler builds too
- `fxceless/victory-fork` #1033 — GPU: defer live title-slot views of newly-allocated resources until earlier submissions finish (fixes RADV GPUVM faults)
- `fxceless/victory-fork` #175 — Scope title memory mapping locks to affected ranges
- `fxceless/victory-fork` #20 — GPU: compile our run-time HLSL with DXC when the system compiler refuses it (Wine/Proton)
- `fxceless/victory-fork` #259 — Pipeline cache: save the early list when there is no host pipeline library (Wine/Proton)
- `fxceless/victory-fork` #257 — Shaders: one latch per loop in emitted DXIL (dxil-spirv asserts on two back edges and stalls the pipeline workers under vkd3d-proton)
- `grzegorz2047/TrinityXSX` `dc17109d0405` — Tools: ps5-deploy-game.sh; docs: direct DXIL on vkd3d-proton
- `grzegorz2047/TrinityXSX` `d31896bb001d` — XCL: title window off the screens without DWM; vkd3d-proton: predicated buffer copies
- `grzegorz2047/TrinityXSX` `f5ee95b8f978` — Tools: ftp_rm_tree.py; docs: mesh shaders on the PS5 with vkd3d patch 0002
- `grzegorz2047/TrinityXSX` `7961be9df20a` — vkd3d-proton: mesh shaders without barycentrics for xcl.exe
- `grzegorz2047/TrinityXSX` `fd634e7e1be4` — CI: build vkd3d-proton in MSYS2 UCRT64; cache keys without the workflow file
- `grzegorz2047/TrinityXSX` `6a72a3a33772` — CI: auxiliary build of the patched XCL and vkd3d-proton, PS5 package
- `JamesCicada/GTAVI-Recomp` `7e541267da01` — ci: run clang-tidy in the Linux Clang build
- `LooperSalty/AnyXboxX` `ca8b76deafa6` — Merge pull request #199 from Exorion1er/feat/victory-linux-amd
- `LooperSalty/AnyXboxX` `873f359f5d4a` — Merge main into feat/victory-linux-amd (after #208)
- `LooperSalty/AnyXboxX` `df1f09c7ac92` — Merge main into feat/victory-linux-amd (after #207)
- `LooperSalty/AnyXboxX` `72470e593ca6` — Merge branch 'main' into feat/victory-linux-amd
- `LooperSalty/AnyXboxX` `ca61dc669c19` — Merge branch 'main' into feat/victory-linux-amd
- `LooperSalty/AnyXboxX` `5c35d38fdaf1` — docs: VIctory in the repository, Proton and AMD fixes
- `marvisak/VIctory` #1872 — GPU: the depth twin's converter builds its root signature when the compiler drops it (Wine)
- `pablogonz12/Scarlight` #96 — Zero-copy on the unified core: title memory in place by default for the class the probe proved, and a startup self-check that makes a silent vkd3d fallback one line (stacked on #88)
- `pablogonz12/Scarlight` #93 — docs: the Mesa shader cache, sized, per prefix, and set in the play script
- `pablogonz12/Scarlight` #77 — docs: architecture decision — Proton for the OS side, Mesa for the GPU side, step by step
- `pablogonz12/Scarlight` #15 — VIctory #994 (Linux/Proton + debug pixel shader) on top of the speed stack
- `pablogonz12/Scarlight` `8b3f4800b17c` — README: how it works with Steam in the picture, NVK badge, roadmap as it stands today
- `pablogonz12/Scarlight` `0be2f49f7656` — Merge pull request #83 from pablogonz12/steam-tool
- `pablogonz12/Scarlight` `446a310a5fa5` — Steam tool: keep the conf across upgrades, and one rule for the optional compiler
- `pablogonz12/Scarlight` `ab4f78bea942` — Announce the Steam compatibility tool and preserve the previous setup
- `pablogonz12/Scarlight` `1e7099cfe5c1` — Merge remote-tracking branch 'hub/main' into steam-tool
- `pablogonz12/Scarlight` `4d1346ad0eb2` — Steam tool: third review round (source archive, d3dcompiler precedence)
- `pablogonz12/Scarlight` `9859a45cc361` — Steam tool: the second review round, fixed
- `pablogonz12/Scarlight` `4ebf5e7ef88f` — Steam tool: the review's ten findings, fixed
- `pablogonz12/Scarlight` `cc5fd5c5b1f4` — Run inside Steam's runtime container, like Proton does
- `pablogonz12/Scarlight` `6457c823839e` — Steam compatibility tool: ship it, package it, document it
- `pablogonz12/Scarlight` `925bc25c3aa0` — Drop the local wine symlink that broke the asset-policy gate
- `pablogonz12/Scarlight` `ce65955fc942` — Merge pull request #75 from pablogonz12/ci/linux
- `pablogonz12/Scarlight` `86ff4a57daa7` — Launcher: read Windows profile paths, resolve wine through PATH
- `pablogonz12/Scarlight` `936fa64fd856` — ci: build the Windows exe on Linux, lint the launcher, show it in the README
- `pablogonz12/Scarlight` `7d6249b0631f` — Profiles: explicit lookup order, proven under Wine, documented for newcomers
- `pablogonz12/Scarlight` `0e6eead4b13f` — tools: document the DLL folder's WINEDLLPATH use
- `pablogonz12/Scarlight` `9cdc3109121b` — docs: present Scarlight as an Xbox Series compatibility layer for Linux on Mesa
- `pablogonz12/Scarlight` `51e9163976dd` — docs: README as the Mesa-first translation layer, one credits file, no internal logs
- `pablogonz12/Scarlight` `46279dc9577b` — docs: qualify watchdog switch by build; link Linux guide on main after tag checkout
- `pablogonz12/Scarlight` `f13bbad40f59` — docs: README as the Linux + Mesa port (pitch, Mesa badges, tested-on, test reports)
- `pablogonz12/Scarlight` `7fd912fe2bdd` — docs: README rewrite for the AMD + Linux port, current measured numbers
- `pablogonz12/Scarlight` `10161cf5450b` — docs: CodeRabbit follow-ups from #48 (script executable, Linux diagnostics build, contribution badge)
- `pablogonz12/Scarlight` `d3476c71e76a` — Merge pull request #48 from pablogonz12/docs/linux-first
- `pablogonz12/Scarlight` `3749f938577e` — docs: say Scarlight is a Linux + AMD RDNA4 first port
- `pablogonz12/Scarlight` `a3e89b48cfce` — INTEGRATION: row for VIctory #1831 (Wine write-watched copy-back)
- `PastiiJay/klbf-lab` #12 — Fix Wine pipeline worker overflows in shader translation
- `PastiiJay/klbf-lab` `7afe3f77244c` — Pipeline library: an empty host library is created from a null blob, so caching works on Wine/vkd3d-proton (gmankab, based on Marvisak/VIctory#347)
- `pragyaangaur/VIctory-macOS` #143 — diag(gpu): count the heap blocks Wine maps on their own, per frame
- `ProhorovIlja/NativeX-Runtime` #118 — Test native Linux Vulkan X11 swapchain resize and window pixels
- `ProhorovIlja/NativeX-Runtime` #117 — Test logical wave64 partitions on native Linux Vulkan
- `ubiquitarianism/leonida` #1980 — gpu-test-results: NVIDIA GB10 (DGX Spark, arm64 Linux) at 9e7720c (GPU-GB10)
- `ubiquitarianism/leonida` #1979 — gpu-test-results: RTX 5090 on Linux (NVIDIA 595.99.02, Ubuntu 26.04) at 9e7720c (GPU-RTX5090-LINUX)
- `ubiquitarianism/leonida` #1845 — WINE-RESOURCE-LOCK-SHAPE: Validate CTest resource locks before scheduling Wine tests
- `ubiquitarianism/leonida` #1876 — tests: import fcntl where it is used so test_run_game_wine loads on Windows (TEST-RUN-GAME-WINE-WINDOWS-IMPORT)
- `ubiquitarianism/leonida` #1964 — Keep Wine runner resources scoped to one prefix owner
- `ubiquitarianism/leonida` `98c2f7777276` — Merge pull request #1819 from aslan2142/tools-wine-game-run
- `ubiquitarianism/leonida` `597cf76623d6` — TOOLS-WINE-GAME-RUN: scripts/run_game_wine.sh, one bounded game run under Wine
- `ubiquitarianism/leonida` `237deb64ce2b` — Merge pull request #1539 from aslan2142/radv-import-pins
- `uiliannn/VItoriaBr` #23 — fix(gpu,shader): fix WARP depth bounds, indirect budget group race, and Linux CI
- `uiliannn/VItoriaBr` `8a85222ae269` — merge: PR #3 (@andysilva86) portable Linux CI and Portuguese overview docs
- `uiliannn/VItoriaBr` `d52d84c96977` — ci: run the portable crates' clippy and tests on Linux
- `vitutu1/victory-perf` `0436e0f2fd41` — Merge pull request #7 from Coinami23-cmd/fix/gpu-linux-host-guards
- `Wertrex/Americas` `98c2f7777276` — Merge pull request #1819 from aslan2142/tools-wine-game-run
- `Wertrex/Americas` `597cf76623d6` — TOOLS-WINE-GAME-RUN: scripts/run_game_wine.sh, one bounded game run under Wine
- …and 4 more (see `tracking/sync-report.json`)

## Frame-time and CPU/GPU bottleneck wins

- `Archivist-Atlas/gta-vi-astral` `bdbdcad3e00e` — hle: IAcpHal 6-slot ShapeContexts & message ack queue, WinHttp/Winsock offline errors, IGameInputDevice slot 10, ntuser API-set window hide
- `Archivist-Atlas/gta-vi-astral` `3dbeeb746266` — feat(gpu,hle): offscreen RenderTarget->SRV pipeline, D3D12 debug InfoQueue, and live GameInput gamepad polling
- `Archivist-Atlas/gta-vi-astral` `6ee6bd54a7e3` — gpu/hle/docs: DrawIndexedInstanced + Host IndexBuffer upload + Alpha Blending + zero-WARN xaccessibility, README 47%
- `Archivist-Atlas/gta-vi-astral` `9e258b1987aa` — shader/gpu/hle/docs: Compute cs_5_0 + Host Compute PSO/Dispatch + 5 live RDNA2 PSOs & 5185 draws, README 45%
- `Archivist-Atlas/gta-vi-astral` `06cd08ccd5bf` — hle/docs: shared O(1) COM vtables, XUser/XGameSave/XAccessibility, 780+ frames + GPU Dispatch, README 43%
- `Archivist-Atlas/gta-vi-astral` `8fc66cb838a9` — shader/gpu/hle: 93% RDNA2 coverage (v_cvt_pkrtz_f16_f32 + SMEM/MUBUF) + Host VB/CB upload + DeviceRemovedReason
- `Belobug/VIctory-Belobug-AMD` #139 — Submission CPU time, ambient light following the exposure, unattended benchmark
- `Belobug/VIctory-Belobug-AMD` #150 — GPU: improve texture uploads and specialize single-record indirect draws
- `Belobug/VIctory-Belobug-AMD` #120 — GPU: batch hoisted mode-7 indirect argument clamps into 64-lane SIMD dispatches (22x faster)
- `Belobug/VIctory-Belobug-AMD` #80 — Hash the DXIL type and constant tables with Fx instead of SipHash
- `Belobug/VIctory-Belobug-AMD` #88 — Stop scanning the whole shader on every descriptor-load lookup
- `Belobug/VIctory-Belobug-AMD` #58 — Build the indirect mismatch note only in diagnostics builds
- `Belobug/VIctory-Belobug-AMD` #56 — Skip the pipeline wait's lock and clock once the frame's budget is spent
- `Belobug/VIctory-Belobug-AMD` #132 — timeline: wait for busy queues to finish before pump/submit returns (#116)
- `Belobug/VIctory-Belobug-AMD` #86 — translate --warm: translate every shader into the run's cache ahead of a run
- `Belobug/VIctory-Belobug-AMD` #18 — GPU: sparse copies snapshot and copy back their tiles on several threads
- `Belobug/VIctory-Belobug-AMD` #62 — Find the destination heap once per descriptor copy, not per descriptor
- `Belobug/VIctory-Belobug-AMD` #127 — perf(gpu): cut per-draw and title-memory sync CPU overhead
- `Belobug/VIctory-Belobug-AMD` #89 — GPU: fix dynamic descriptor wraparound scan, per-heap walk cap, and capped headroom
- `Belobug/VIctory-Belobug-AMD` #76 — GPU: bound the per-frame pipeline wait with VICTORY_PIPELINE_WAIT_MS
- `Belobug/VIctory-Belobug-AMD` #38 — docs: fix PowerShell launch instructions
- `Belobug/VIctory-Belobug-AMD` #37 — Copy equal-length tile pieces with a constant length when detiling
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
- `boykopovar/AnyPS5` #24399 — fix(libSceLibreSsl3): copy the sk_free stack guest record safely
- `boykopovar/AnyPS5` #24915 — feat(relinker): record the host CPU features --to-avx output needs
- `boykopovar/AnyPS5` #21330 — fix(libwebrtc): copy RTCP packet guest pointers safely
- `boykopovar/AnyPS5` #24291 — feat(libSceIcu): implement the uhash UnicodeString hash and compare helpers
- `boykopovar/AnyPS5` #7085 — feat(agc): back extra guest mip levels with storage images
- `boykopovar/AnyPS5` #26801 — fix(libkernel_sys): copy kmq message and deadline in from guest memory
- `boykopovar/AnyPS5` #27010 — fix(libSceFaceTracker): copy callback outputs through guest memory
- `boykopovar/AnyPS5` #27271 — perf(agc): skip CPU index range scan for in-place draws without vertex attributes
- `boykopovar/AnyPS5` #27269 — perf(agc): fast-path existing imports and open batches in WriteLabelOnGpu
- `boykopovar/AnyPS5` #22132 — fix(libSceIcu): copy invariant conversion guest pointers safely
- `boykopovar/AnyPS5` #22390 — fix(libSceIcu): copy tokenize and UTF-16 memory guest pointers safely
- `boykopovar/AnyPS5` #22460 — fix(libSceIcu): copy lenient UTF-8 conversion and UTF-8 cursor guest pointers safely
- `boykopovar/AnyPS5` #22472 — fix(libSceNKWebKit): copy WTF::Persistence and WTF::URL guest pointers safely
- `boykopovar/AnyPS5` #22462 — fix(libSceLibcInternal): copy the time_get, time_put and getopt guest pointers safely
- `boykopovar/AnyPS5` #23433 — fix(libc): copy fputws and vfwprintf guest wide strings safely
- `boykopovar/AnyPS5` #22215 — fix(libSceAgc): validate and safely copy semaphore label pointer in sceAgcGetSemaphoreLabel
- `boykopovar/AnyPS5` #27238 — fix(libSceLibreSsl): queue LibreSSL's ASN.1 errors when RSA_verify rejects a DigestInfo
- `boykopovar/AnyPS5` #23150 — fix(libSceAjm.native): copy WVorbis header packet buffers safely
- `boykopovar/AnyPS5` #23136 — fix(libSceFreeType): copy FT_Load_Sfnt_Table outputs safely
- `boykopovar/AnyPS5` #23081 — fix(libSceFreeType): copy FT_Get_Glyph_Name and FT_Get_Sfnt_Name outputs safely
- `boykopovar/AnyPS5` #20633 — fix(libSceAmpr): copy remaining command buffer and cnd name guest pointers via GuestCopyIn/Out
- `boykopovar/AnyPS5` `3516d0483192` — Merge pull request #20018 from slopezjur/fix/agc-bound-pointerless-dispatch-writes
- `boykopovar/AnyPS5` `d3a34e400184` — Merge pull request #19899 from Sankalp7317/fix/agc-upload-cache-test-windows-env
- `boykopovar/AnyPS5` `60895223c21e` — Merge pull request #26419 from ToRaNek/fix/libressl3-crl-indirect-entries
- `brcly/TuxBox` #183 — d3d12: honor memory waits after compute dispatches
- `brcly/TuxBox` #178 — lanes: pin the descriptor limit the reclaim self-test counts on
- `brcly/TuxBox` `9399e4ffd12b` — refactor: split the two instruction emitters by instruction family
- `brcly/TuxBox` `a8f24b678ef6` — xmem: write DirectStorage data into legacy-mapped physical pages at their CPU copy
- `brcly/TuxBox` `9ca0060d6b50` — Merge pull request #172 from Vladimir-Human/d3d12/reclaim-descriptor-slots
- `brcly/TuxBox` `3bd0402aae31` — Merge pull request #60 from Valdi320/work/scarlett-cpu-oracle
- `brcly/TuxBox` `94412a53ed8a` — Merge pull request #167 from Valdi320/work/scarlett-pipeline-rank-accuracy-20261125
- `brcly/TuxBox` `96783e44f1de` — lanes, verify: add descriptor-reclaim GPU selftest and negative-control proof
- …and 509 more (see `tracking/sync-report.json`)

## Stability and device-loss prevention

- `Archivist-Atlas/gta-vi-astral` `f2cf0238f958` — gpu/xmem: CopyPageMappingsBatchX + xmemMapAlias + CreatePlacedSRV(+40) + VirtualQuery guard (pixel-perfect BC3 spinner)
- `Belobug/VIctory-Belobug-AMD` #68 — Wave64 on 32-lane hosts: one console wave sharing LDS runs in the console flow (+2 NVIDIA fixes)
- `Belobug/VIctory-Belobug-AMD` #140 — Shader tests: WARP runs one draw or dispatch at a time (parallel runs crashed in d3d10warp.dll)
- `Belobug/VIctory-Belobug-AMD` #104 — victory system: this machine for test reports; the commit built from in --version and crash reports
- `Belobug/VIctory-Belobug-AMD` #146 — Shader/GPU: recover wave64 split scalar masks and report GetDeviceRemovedReason
- `Belobug/VIctory-Belobug-AMD` #144 — GPU, Shaders: clamp view/sampler heap indices to dedicated null sentinels and bound StrideTable (#95)
- `Belobug/VIctory-Belobug-AMD` #94 — Shaders: separate bounded heap handle for buffer size queries (AMD compiler crash)
- `Belobug/VIctory-Belobug-AMD` #71 — Fix stale and misplaced comments
- `Belobug/VIctory-Belobug-AMD` #112 — watch: clear stale JOURNAL.written entries on title_memory_remapped (#102)
- `Belobug/VIctory-Belobug-AMD` #55 — Rectangle expansion reuses the clamp's shader builder
- `Belobug/VIctory-Belobug-AMD` #134 — Shaders: recover wave64 masks restored from scalar copies
- `Belobug/VIctory-Belobug-AMD` #133 — Loader: reject garbage relocation sizes, serialise crash-log appends
- `Belobug/VIctory-Belobug-AMD` #92 — Shaders: keep 64-lane waves for wave64 programs on a 64-lane host
- `Belobug/VIctory-Belobug-AMD` #128 — Wave64 on 32-lane hosts: several console waves per group, wave reductions in halves, permlanes (+ an SCC fix)
- `Belobug/VIctory-Belobug-AMD` #60 — Keep pinned blocks writable on guard()'s fast path too
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
- `boykopovar/AnyPS5` #18185 — fix(libSceAgcDriver): convert fallback D16S8 depth copies and sampling
- `boykopovar/AnyPS5` #22723 — fix(libSceFont): guard shared font reference increments
- `boykopovar/AnyPS5` #26731 — fix(libc): guard strsep guest memory
- `boykopovar/AnyPS5` `6c79267ac67f` — Merge pull request #25061 from gurtgg/fix-ime-filter-text-pointer-guards-overlap
- `boykopovar/AnyPS5` `d237f5d065b0` — Merge pull request #20021 from xXPBSXx/fix/kernel-sanitizer-replace-tables-null
- `boykopovar/AnyPS5` `d72479f314e3` — Merge pull request #19967 from blawesom/fix/agc-pixel-interlock-gfx11-race
- `boykopovar/AnyPS5` `d384d22326c6` — Merge pull request #26259 from MarrasFederico/feat/icu-locale-table-fallback
- `brcly/TuxBox` #181 — Lower wave32 masked DPP prefixes and totals in wave allocation
- `brcly/TuxBox` #179 — lanes: replay a GPU fence handoff in both directions
- `brcly/TuxBox` #176 — xshader: coalesce wave32 guards, pure intrinsics and local temporaries
- `brcly/TuxBox` `3aa924171f19` — Merge pull request #171 from Vladimir-Human/shader/wave32-wave64-diagnostics
- `brcly/TuxBox` `7a11b27723fd` — xshader: branch with the whole wave where targeted guards refuse
- `brcly/TuxBox` `98043df492dd` — d3d12: reclaim shader-visible descriptor slots with lease and fence pinning
- `brcly/TuxBox` `796b793db8e3` — tools: avoid stale and ambiguous pipeline failure attribution
- `brcly/TuxBox` `fdf5877b9d85` — tools: replay bounded native table selectors on a guarded GPU buffer
- `brcly/TuxBox` `2985d3348672` — xshader: allow verified single-wave floating sums on wave32 hosts
- `brcly/TuxBox` `5fb5cc224a2b` — xshader: run single-wave bitwise reductions on wave32 hosts
- `brcly/TuxBox` `e8898cebadab` — Merge main and preserve validated CLI fallback for persistent DXC
- `brcly/TuxBox` `4a02f494fd19` — xshader: implement signed integer clamp controls
- `breadbbbb/rpf9-explorer` #27 — Add clean-room BCPack BC6H/BC7 decoders, external history loading, and strict-mode mip fallback
- `cocainebunnylaboratories/cblvictoryvi` `1454e3fb7b78` — Record cashier progress, measured route and crash outcome
- `cocainebunnylaboratories/cblvictoryvi` `f8efb00d773b` — GPU: the rectangle fallback's f16 rounds to even and keeps NaN
- `cocainebunnylaboratories/cblvictoryvi` `31714deb2d70` — Take the fix/amd-view-heap-clamp branch of triippywhale's fork (as one commit)
- `daeken/XSXEmu` `f81161509bb0` — Validate fenced owned scalar descriptor loads on Metal
- `daeken/XSXEmu` `b13e5fb28ac9` — Validate fenced owned raw dword load and bitfield consumer on Metal
- `daeken/XSXEmu` `c0151f3a583d` — Check Mach fixture ledger and both parked guard neighbors
- `daeken/XSXEmu` `5d0fab06d6bb` — Read the sync facade timeout from Win64 R8D
- `DexNom/VIctory` #2 — gpu: isolate AMD driver compiler crashes (crashed.txt) and skip self-CopyResource
- `DexNom/VIctory` #7 — Fix RDNA 4 amdxc64.dll shader compiler crash and 30s GPU watchdog stalls
- `DexNom/VIctory` `76749ce58da2` — Recover voxel visibility shader when a wave64 mask is restored from scalar copies
- `DexNom/VIctory` `3c47fd4945dd` — Merge pull request #1562 from BastienRodz/gpu/fence1-iid
- `DexNom/VIctory` `e2a80cfd1a43` — Merge pull request #1159 from geiyiliyabumo/fix/crash-unwind-test-win10
- `DexNom/VIctory` `71ac8903c2f1` — Merge pull request #213 from Dufftwt/fix/crash-log-names-host
- `DexNom/VIctory` `a9d233fe5d49` — Merge pull request #1660 from HailLucifer666/fix/pipeline-library-atomic-write-race
- `DexNom/VIctory` `711e930a9ca7` — Merge pull request #1658 from HailLucifer666/fix/texture-staging-oob-and-committed-guard
- `DexNom/VIctory` `17de445ced6c` — Merge pull request #1539 from Texbirds/fix/loader-clwb-loop-overflow
- `DexNom/VIctory` `e011e886266d` — Merge pull request #1515 from Calvix2k/codex/ui-dialog-command-guard-033
- `ethan00013/keylime` `4e95a8cf82a6` — Flat GPU view of title memory through a sparse buffer; dispatch wait budget; park the thread-hang watchdog
- `ethan00013/keylime` `7420a080b3e7` — wave64 saveexec on both EXEC dwords, a trip cap per loop, GPU checkpoints on device loss, variants a draw waits for built urgently
- `ethan00013/keylime` `58d71338fdd0` — Shaders: run wave64 compute exactly (two console lanes per invocation); 25 missing instruction kinds
- `ethan00013/keylime` `62d92a85f7b1` — Compile what the title waits on first; structure loops entered in the middle; precompile command; park the GPU watchdog
- `ethan00013/keylime` `650550ffafb1` — Guard shader stores against memory no GPU resource or heap covers; fix DispatchMeshX arguments
- `ethan00013/keylime` `75791aa84a3c` — Media Foundation: console drive paths reach the host; log Windows' GPU timeout settings
- `ethan00013/keylime` `309a7df0f597` — GPU waits: D3D12 comparison codes, bounded fence waits, hang report
- `ethan00013/keylime` `eeccbc735bc0` — GPU: indirect arguments checked on the GPU; shorter memory-wait timeout; tighter shader loop cap
- `ethan00013/keylime` `d100db5ae8b4` — Survive world loading better: async pipeline builds, crash diagnostics, debug panel, shader guards
- `ethan00013/keylime` `7de9967d85f7` — README: reaches the main menu with all audio; current blocker is the crash at the first indirect call
- `fablinoo/kelvaroq` `94cdceb83e86` — Traduire les masques wave64 prouvés nuls dans une branche divergente
- `Freminet/ScarlettX` #41 — feat(xgameruntime): add the XDisplay HLE (HDR mode and timeout deferral)
- `fxceless/victory-fork` #1032 — GPU: in-place guarded writes for write_mapped when VICTORY_COPY_PLAIN=1 (-20 ms copy-back)
- `fxceless/victory-fork` #1029 — Intel: mwaitx waits sleep on the fence value (VICTORY_MWAITX_SLEEP=1)
- `fxceless/victory-fork` #1039 — gpu: isolate AMD driver compiler crashes (crashed.txt) and replace parked workers
- `fxceless/victory-fork` #1042 — Shaders: support saturating clamp on packed 16-bit integer ops (port of upstream VIctory #1142)
- …and 189 more (see `tracking/sync-report.json`)

## Rendering and gameplay completeness

- `Ahmet-Dedeler/awesome-gta6-reverse-engineering` `7e1a32c0bc3f` — Add ScarlettPort, openrage, xcl-mac, data dumps & save tools; relax overly strict inclusion rules
- `Albotterin/AWC_from_RPF9_Extractor` `3a869dad98b4` — Own station logo reader, no decoder download (v2.4.1)
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
- `Belobug/VIctory-Belobug-AMD` #110 — Shaders: a divergent loop's scalars read after it ask for the console wave size (#2's RDNA2 wave-size failures)
- `Belobug/VIctory-Belobug-AMD` #138 — Shaders, GPU: port VIctory#2398, link over-budget mesh and pixel shaders under the 32 KiB output limit (#125)
- `Belobug/VIctory-Belobug-AMD` #152 — Skip the global texture lock when no host-written textures
- `Belobug/VIctory-Belobug-AMD` #81 — Shaders: 64-bit integer compares on their dwords (AMD orders a shared-dword i64 compare wrong)
- `Belobug/VIctory-Belobug-AMD` #44 — Windows 10: load the D3D12 Agility SDK runtime (shader model 6.6)
- `Belobug/VIctory-Belobug-AMD` #148 — Shaders: a buffer view picked from the resource heap by index is read as a raw buffer, not a constant buffer (missing world geometry and device removal on a Radeon 8060S)
- `Belobug/VIctory-Belobug-AMD` #3 — Clippy: victory-shader is clean again with -D warnings (10 lints, no behaviour change)
- `Belobug/VIctory-Belobug-AMD` #70 — Remove unreachable arms and dead code in the shader translator
- `Belobug/VIctory-Belobug-AMD` #97 — Shaders: DXIL bitcode clean-up passes (VICTORY_SHADER_OPT), census, and loop latch fixes
- `Belobug/VIctory-Belobug-AMD` #83 — Shaders: s_cbranch_vccz/vccnz tests raw VCC, not VCC & EXEC
- `Belobug/VIctory-Belobug-AMD` #75 — Shader IR passes without per-step allocations
- `Belobug/VIctory-Belobug-AMD` #78 — Check shader test expectations on 16-lane adapters
- `Belobug/VIctory-Belobug-AMD` #22 — Shaders: omit unnecessary fixed WaveSize for single-wave LDS compute shaders
- `Belobug/VIctory-Belobug-AMD` #90 — Shaders: follow EXEC set from a scalar the section made constant
- `Belobug/VIctory-Belobug-AMD` #142 — Carry 20 upstream VIctory fixes (shaders, samplers, textures) and a mesh wave-ops flag fix
- `Belobug/VIctory-Belobug-AMD` #57 — Fix underflow for textures described with MipLevels 0
- `Belobug/VIctory-Belobug-AMD` #135 — Shaders: f16 results round to nearest even, and min/max/med3 read denormal operands as zeros
- `Belobug/VIctory-Belobug-AMD` #51 — gpu: skip self-CopyResource when src and dst share host texture
- `Belobug/VIctory-Belobug-AMD` #129 — Shader tests: a cross-device VALU fuzzer (hardware adapters vs WARP), ignored by default
- `Belobug/VIctory-Belobug-AMD` #63 — Stop re-zeroing the texture scratch buffer, and reuse the placed-texture set
- `Belobug/VIctory-Belobug-AMD` #73 — Reuse one Opus decode buffer per connection
- `Belobug/VIctory-Belobug-AMD` #119 — Speed up bindless planning, BCPack decoding, and shader translation
- `Belobug/VIctory-Belobug-AMD` #100 — GPU: fix Shadow::sparse_writing tile base for 4 KiB-aligned Map() pointers
- `Belobug/VIctory-Belobug-AMD` #40 — Input: DualSense device paths end in a NUL
- `Belobug/VIctory-Belobug-AMD` #21 — Reject overlapping depth slices in volume texture copies
- `Belobug/VIctory-Belobug-AMD` #47 — docs: clarify shader targeting and GPU diagnostics
- `Belobug/VIctory-Belobug-AMD` #7 — Add Xbox texture layouts for formats 189–191
- `Belobug/VIctory-Belobug-AMD` #32 — Check a texture for writes without building lists
- `Belobug/VIctory-Belobug-AMD` #31 — Read a host texture's flags once instead of on every use
- `Belobug/VIctory-Belobug-AMD` #23 — XMA decoder: lock only voices that are on, sleep while none is
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
- `boykopovar/AnyPS5` #20251 — perf(shader): convert runtime-format buffer components at their run-time width
- `boykopovar/AnyPS5` #9748 — feat(agc): blend into 10_11_11 unorm color targets in the pixel shader
- `boykopovar/AnyPS5` #22774 — fix(shader): call load_bda_span outside read_bda_span's per-lane selection
- `boykopovar/AnyPS5` #27268 — perf(agc): reuse PreparedDraw across layered mesh output windows
- `boykopovar/AnyPS5` #27253 — perf(agc): read thread-local live shadow ranges in AnyShadowedOverlaps
- `boykopovar/AnyPS5` #27265 — refactor(ime): drop errno saves around guest copies
- `boykopovar/AnyPS5` #27266 — refactor(videoout): drop errno saves around guest copies
- `boykopovar/AnyPS5` #27267 — refactor(nkwebkitrequirements): drop errno saves around guest copies
- `boykopovar/AnyPS5` #26766 — build(shader): update SPIRV-Tools for float-control optimization
- `boykopovar/AnyPS5` #25583 — fix(shader): leave a continued callee part before its callee on a target mismatch
- `boykopovar/AnyPS5` #26736 — fix(tools): preserve progress history cache inputs
- `boykopovar/AnyPS5` #26445 — feat(libwebrtc): implement the rtc::Thread message loop
- `boykopovar/AnyPS5` #14474 — test(agc): cover PM4 type-3 packet header bounds and invalid opcode decoding
- `boykopovar/AnyPS5` #23486 — fix(agc): account retained shader results in the draw cache
- `boykopovar/AnyPS5` #19852 — fix(shader): preserve runtime-proven formatted buffer read extents
- `boykopovar/AnyPS5` `d4553d2b6243` — Merge pull request #26601 from gccody/fix/shader-uniform-quad-perm-broadcast
- `boykopovar/AnyPS5` `e37d1227e33e` — Merge pull request #25852 from Remi-Bon/perf/shader-keep-specialized-spirv
- `boykopovar/AnyPS5` `f44efa2a47b8` — Merge pull request #20081 from AlexFigas/feat/shader-replay-stage-inputs
- `boykopovar/AnyPS5` `d28f9808ac05` — Merge pull request #20070 from c1b3r-p3d1a/feat/shader-interp-f16-float-modes
- `boykopovar/AnyPS5` `bd4a71415bd0` — fix(shader): translate v_sin_f32 and v_cos_f32 in f32 denormal modes 1 and 2 (#19608)
- …and 779 more (see `tracking/sync-report.json`)

## Experimental / unstable (hold)

- `Archivist-Atlas/gta-vi-astral` `3b2bbb74d259` — docs: add progress bar, WIP warning, and subsystem status table to README
- `Belobug/VIctory-Belobug-AMD` #118 — FidelityFX: Series X host FSR and opt-in AMD frame generation
- `Belobug/VIctory-Belobug-AMD` `bdecda306a3e` — An unwaited submission is finished when it is waited for (its shaders' writes, read-backs and bottom-of-pipe writes), so every submission of the title's may go unwaited; the next one's buffer copies are refreshed meanwhile (VICTORY_ASYNC_SUBMIT=1, off by default)
- `Belobug/VIctory-Belobug-AMD` `0a32e89cf58a` — Tiles of title memory that is no longer there can be given back (VICTORY_SWEEP_TILES=2), so the copies stop growing; off by default
- `cocainebunnylaboratories/cblvictoryvi` `805f29b24c2c` — Record timed doorway hold and first moving sample on the integrated renderer
- `daeken/XSXEmu` `262e5f47cc68` — Require a retained Metal backend for opt-in owned Xbox device creation
- `DexNom/VIctory` #8 — docs: RDNA4 (RX 9070 XT) results — setup holds 30 fps, world load crashes in amdxc64
- `DexNom/VIctory` `8cca4d228541` — Document experimental RT launch and measured City frame times
- `DexNom/VIctory` `d71f197957da` — Add an opt-in Windows shortcut for experimental ray tracing
- `ethan00013/keylime` `0afb8154c516` — A loop stopped by its trip cap ends the program; KEYLIME_ONE_QUEUE; per-command checkpoints opt-in
- `ethan00013/keylime` `b7dadd4dc2ca` — GPU: hold submissions until their waits are signalled; poll fences
- `Freminet/ScarlettX` #29 — feat(kernelx): implement GpuTilePool and clean-room kernelx.dll Game OS shim
- `fxceless/victory-fork` #994 — Ray tracing: the title's structure builder runs without crashing it (dispatches by thread counts, title memory by address made ready); node tests answer no hit while it is off (stacked on #854 and #993)
- `fxceless/victory-fork` #868 — Card copies: a census, and an opt-in switch that leaves write-combining memory on the card (−11…−20 % frame, 3 GPUs)
- `fxceless/victory-fork` #917 — GPU: a constant can stand in for indirect light while rays do not run (VICTORY_AMBIENT, off by default)
- `fxceless/victory-fork` #1013 — Ray tracing: the builder's quarantine holds (split out of #935)
- `JPTuba9/xlaunch-vi` #40 — Route console DLL dependencies and gate experimental GTA VI VM setup
- `JPTuba9/xlaunch-vi` #6 — Validate PE tables and verify cooperative mapped DLL unload
- `JPTuba9/xlaunch-vi` #3 — Integrate validated Vulkan renderer, C DLL graphics proposal and RDNA2 compute subset
- `ka1qux/BRVIctory-ka1qux` `463284a283bb` — README: ray tracing is on by default
- `ka1qux/BRVIctory-ka1qux` `d31de8d609a5` — GPU: ray tracing is on by default (VICTORY_RAYS=0 turns it off)
- `ka1qux/BRVIctory-ka1qux` `4749a0cec9d6` — GPU: the guards' clean blocks are a bit set, their regions 1 MiB, and holes can hold until a commit is told
- `ka1qux/BRVIctory-ka1qux` `6992367697fb` — Ray tracing: node tests answer no hit while the structures are not built, and view addresses read from the view heap by slot
- `ka1qux/BRVIctory-ka1qux` `c3adf856ad98` — Ray tracing: root views follow a replaced buffer, and remaps wait for the flights that may write them
- `ka1qux/BRVIctory-ka1qux` `7189565b7dd0` — GPU: a refresh can take only the 4 KiB steps that differ from the copy (VICTORY_REFRESH_COMPARE, off by default)
- `ka1qux/BRVIctory-ka1qux` `e907b8ed67b3` — GPU: card copies of title buffers can go below 1 MiB (VICTORY_CARD_BUFFERS_MIN_KB, off by default)
- `ka1qux/BRVIctory-ka1qux` `3d72256c0b1b` — GPU: draws that store to no buffer can keep an indirect draw window (VICTORY_INDIRECT_WINDOW_DRAWS, off by default)
- `ka1qux/BRVIctory-ka1qux` `574b317d44fd` — GPU: a copy-back's runs of the GPU's bytes are written under one hold of their guards
- `KomanderDG88/VIniVIdiVIci` `acf4626eb147` — Shaders: an image store of some channels keeps the others as the texel holds them
- `LooperSalty/AnyXboxX` `d2732341e048` — chore: cache the TomBox console DLLs in CI instead of rebuilding them
- `LooperSalty/AnyXboxX` `04738a72c280` — GPU: VICTORY_L9_ALIAS_UNCLEAN and VICTORY_HEAP_RECHECK are opt-in
- `LooperSalty/AnyXboxX` `0dbaf9fd4b91` — GPU: compute pipelines get their own waiting time while a world loads; ray-tracing resolves left out
- `luizwidmer/palmetto` `e8a23e3515fd` — platform: deliver the measured user-change event and offline async completions, both opt-in
- `marvisak/VIctory` #2617 — HLE: hold DirectStorage reads behind held status entries and preserve sequence order
- `marvisak/VIctory` `9cc37c5f5b20` — GPU: lists wait on memory values; queues hold work until its waits hold
- `mkr2tl/xenstride-public` `3f9a603c74a6` — Prepare experimental runtime v0.2.0
- `omonkeyz/xbox-emulator` `a618736df799` — Merge #347: tombox-xshader: size structured buffer elements from the FSSM stride (stacked on #289)
- `omonkeyz/xbox-emulator` `0e6bde2f7f3e` — Merge #348: d3d12_xs: opt-in GPU-based validation with TOMBOX_D3D12_GBV=1
- `omonkeyz/xbox-emulator` `8c481f43f1c1` — d3d12_xs: opt-in GPU-based validation with TOMBOX_D3D12_GBV=1
- `pablogonz12/Scarlight` #86 — victory-gpu: title buffer copies on the host GPU through the buffers' host copies (stacked on the unification PR)
- `pablogonz12/Scarlight` #87 — Experimental: direct RDNA2 → SPIR-V shader emitter (not wired in)
- `pablogonz12/Scarlight` `29ac46272b82` — shader tests: hold the one-at-a-time lock while the lane-read tests dispatch
- `pablogonz12/Scarlight` `7fbdb2398805` — GPU: write-back holds find the handler, wait before opening, retry released reads
- `pragyaangaur/VIctory-macOS` `9cc37c5f5b20` — GPU: lists wait on memory values; queues hold work until its waits hold
- `R0Y00/Xbox2Windows` #2468 — startup: opt-in release of retired slot-133 title-memory textures (slice 2, stacked on #2443)
- `R0Y00/Xbox2Windows` #2357 — tests: add opt-in DXC compile cache for the Python shader checks
- `R0Y00/Xbox2Windows` `061c0958d33f` — startup: optional Tracy zones for the stand-ins (X2W_TRACY, off by default) (#2371)
- `realblastex/flamingo` `7ca2674bce20` — DispatchX by thread counts (USE_THREAD_DIMENSIONS) dispatches the groups that hold them and masks the lanes past them (after victory-fork #814): the ray-tracing structure builder no longer stores past its arrays; merge of victory-fork main 465b94f (frame meters, opt-in VICTORY_JOIN_PARTS, VICTORY_BACK_OWN_VIEW, VICTORY_INDIRECT_GATHER)
- `realblastex/flamingo` `5a7bbf4e4345` — Progress log: runs 85-88 (the cutscene's dropped work is the ray-tracing builder; the builder still stops at a pointer it never wrote)
- `realblastex/flamingo` `dc0c736c2a74` — victory-fork main merged; the mesh edge-flag fix (#1); Belobug's opt-ins; Clippy clean
- `sammwyy/xvalkyrie` `e14d33c8ca6a` — fix(d3d12): tables past the host's part of a title's view heap are copied into a ring at its end when the list runs, so that what they hold is seen
- `sammwyy/xvalkyrie` `2efd90369b10` — fix(d3d12): only shaders whose file says what their user registers hold read the heaps themselves, as compute shaders already did
- `ThatRecompDev/five-star-recomp` `6efb71888a6b` — Add --d3d12-nan-scan: find the draw inputs that hold NaN or Inf
- `ThatRecompDev/five-star-recomp` `e932bde74936` — Diagnostics: stack captures never hold a thread while managed code runs
- `ThatRecompDev/five-star-recomp` `6752f701d6c0` — Give the ray-tracing program tests names of their own
- `ThatRecompDev/five-star-recomp` `d53d5fc6ff34` — Merge lane-dxilswitch: stores past the user space reach nothing (span guard); name holds for ray-tracing-only passes
- `ThatRecompDev/five-star-recomp` `0a962c4d7afe` — Hold programs that only serve ray tracing
- `ThatRecompDev/five-star-recomp` `6f98e5810041` — Diagnostics: ray-tracing stand-in fills are logged and counted
- `ThatRecompDev/five-star-recomp` `44afe4155e54` — Merge lane-dxilswitch: no program reads on game threads; failed builds always give a stand-in; ray-tracing markers survive translator updates
- `ThatRecompDev/five-star-recomp` `9b9eb0d26123` — Merge lane-world: --depth-twins (opt-in) gives depth UAVs a colour twin copied back before depth tests
- `ThatRecompDev/five-star-recomp` `3c9a3f52963a` — Merge lane-lowvram: --typed-image-views (opt-in) reads and writes integer image views as integers
- `ThatRecompDev/five-star-recomp` `2539efdc2bda` — Merge lane-dxilswitch: held ray-tracing pipelines never reach the driver; disposed runtimes stop being active; fence-only barriers in the switch form
- `ThatRecompDev/five-star-recomp` `7b96f203ecc2` — Merge lane-lowvram: backstop test registers its own handler; ray-tracing test owns its runtime
- `uiliannn/VItoriaBr` `32d784861a55` — GPU: hold SHADER_HEAPS across descriptor mirror and check query offsets
- `uiliannn/VItoriaBr` `708b45015726` — HLE: load a host DLL outside the module cache's lock
- `xcl-dev/xcl` #1979 — Recompiler: direct DXIL applies NEG_HI as ABS on v_dot2_f32_f16's accumulator (stacked on #1286)
- `xcl-dev/xcl` #2268 — Scripts: pr_check holds the 125 % peak commit to medians of 3 runs
- `xcl-dev/xcl` `f8ce21de8f59` — Config: an alias pattern with several '*' matches its DLLs
- `xcl-dev/xcl` `1a0bbe3a2ba9` — Merge pull request #1782 from imnicho/sleep-hold-pc-calls

## Per-repository activity

| Repository | Commits | Open PRs | Stage | Status |
|---|---|---|---|---|
| `Ahmet-Dedeler/awesome-gta6-reverse-engineering` | 5 | 0 | — | active |
| `Ahmet-Dedeler/bcpack-rs` | 3 | 0 | — | active |
| `Ahmet-Dedeler/rage9-re` | 0 | 0 | — | active |
| `Albotterin/AWC_from_RPF9_Extractor` | 9 | 0 | — | active |
| `anrq0/SeriesX-Emulator` | 1 | 0 | — | active |
| `Archivist-Atlas/gta-vi-astral` | 38 | 0 | — | active |
| `batatinhafcbatatinha-pixel/xseriesrecomp` | 0 | 0 | — | unreachable |
| `Belobug/VIctory-Belobug-AMD` | 73 | 99 | 6 | active |
| `boykopovar/AnyPS5` | 100 | 100+ | — | active |
| `brcly/TuxBox` | 100 | 8 | 5 | active |
| `breadbbbb/rpf9-explorer` | 0 | 1 | — | active |
| `californiaAlmonds/OpenScarlett` | 0 | 2 | 5 | active |
| `cocainebunnylaboratories/cblvictoryvi` | 51 | 0 | 6 | active |
| `daeken/XSXEmu` | 100 | 0 | — | active |
| `Dark3581/AnyBox` | 0 | 0 | — | unreachable |
| `DexNom/VIctory` | 100 | 10 | 6 | active |
| `edupode/anyxboxseries` | 5 | 0 | — | active |
| `ethan00013/keylime` | 54 | 0 | 5 | active |
| `fablinoo/kelvaroq` | 22 | 0 | 6 | active |
| `fishingpvalues/gta6-pc-port-hub` | 0 | 0 | — | archived |
| `Freminet/ScarlettX` | 0 | 26 | — | active |
| `fxceless/victory-fork` | 15 | 100+ | 6 | active |
| `gookie-dev/g6shim` | 21 | 2 | 5 | active |
| `grzegorz2047/TrinityXSX` | 24 | 0 | — | candidate |
| `holdmysocks/SixNG` | 71 | 3 | — | active |
| `HolyRoses/gta6-explorer` | 4 | 0 | — | active |
| `Hylanderry/DedSec` | 0 | 0 | — | active |
| `Itsbatt/victory-unified` | 16 | 0 | 6 | active |
| `ItsNotRexed/Leonida` | 0 | 0 | — | nuked |
| `jadeowaowa/scarlett-layer` | 0 | 2 | — | active |
| `JamesCicada/GTAVI-Recomp` | 16 | 4 | — | active |
| `JMAN730/project-flamingo` | 100 | 12 | — | active |
| `JoshRob297/rpf9-research` | 0 | 3 | — | active |
| `JPTuba9/xlaunch-vi` | 0 | 56 | — | active |
| `ka1qux/BRVIctory-ka1qux` | 100 | 3 | 6 | active |
| `Kajcsi/G6Recomp` | 0 | 0 | — | active |
| `kellysix/kelly` | 7 | 0 | — | candidate |
| `kfedora/GTAVI-Xbox2PC` | 19 | 0 | — | candidate |
| `KomanderDG88/VIniVIdiVIci` | 56 | 0 | 5 | active |
| `kry-vex-gta/XBOX-Inspector` | 0 | 0 | — | active |
| `Librett0/vice-lens` | 0 | 0 | — | active |
| `ljm-233/Xbgtasix` | 0 | 0 | — | active |
| `LooperSalty/AnyXboxX` | 100 | 12 | 6 | active |
| `luizwidmer/palmetto` | 95 | 0 | 4 | active |
| `lynxnathan/rage-tools-vi` | 1 | 0 | — | active |
| `marvisak/VIctory` | 100 | 100+ | 6 | active |
| `meetox80/6stars` | 0 | 0 | — | active |
| `mewiof/Holy-Interoperability` | 0 | 0 | 4 | active |
| `mkr2tl/xenstride-public` | 1 | 0 | — | active |
| `myokex/victory-amd` | 3 | 11 | 6 | active |
| `nineforge0/forge9` | 21 | 0 | — | active |
| `nullbrik/RageForge` | 0 | 0 | — | unreachable |
| `omonkeyz/xbox-emulator` | 100 | 18 | 3 | active |
| `OnlyMoisties/GTA6-BootTest` | 0 | 0 | 5 | active |
| `OpenCommunityEdition/OpenScarlett` | 20 | 1 | 5 | active |
| `p-portela/SCARAB` | 0 | 0 | — | active |
| `pablogonz12/Scarlight` | 87 | 34 | 5 | active |
| `PastiiJay/klbf-lab` | 43 | 17 | 6 | active |
| `pragyaangaur/VIctory-macOS` | 100 | 4 | 5 | active |
| `ProhorovIlja/NativeX-Runtime` | 0 | 2 | — | active |
| `R0Y00/Xbox2Windows` | 100 | 100+ | 5 | active |
| `realblastex/flamingo` | 16 | 0 | 6 | active |
| `realdody/americas-recomp` | 0 | 0 | — | nuked |
| `RickyDivjakovski/Revenge` | 3 | 0 | — | candidate |
| `rpatty-dev/PercyScarlett` | 3 | 0 | 5 | active |
| `rpatty-dev/PercyStar` | 9 | 1 | — | active |
| `sammwyy/xvalkyrie` | 25 | 3 | — | active |
| `SentinelInteractive/0x52504639` | 0 | 0 | — | active |
| `SSTGxf/ScarlettCompat` | 88 | 0 | — | active |
| `Steinkoloss/port-of-the-americas` | 1 | 0 | 5 | active |
| `sub-north/xbox-hle-lab` | 0 | 1 | — | active |
| `ThatRecompDev/five-star-recomp` | 100 | 4 | 5 | active |
| `TheoBIET/OpenVI` | 27 | 1 | — | active |
| `ubiquitarianism/leonida` | 100 | 100+ | 5 | active |
| `uiliannn/VItoriaBr` | 82 | 8 | 6 | active |
| `valouddsite/Xboxx` | 100 | 0 | — | candidate |
| `vitutu1/victory-perf` | 28 | 4 | 6 | active |
| `voidopcode/vilocity` | 22 | 0 | 6 | active |
| `walkthestars/GTA6-Asset-Info` | 0 | 0 | — | active |
| `Wertrex/Americas` | 100 | 1 | — | candidate |
| `woreksami/ttpf-studio` | 0 | 0 | — | active |
| `xcl-dev/xcl` | 100 | 100+ | 5 | active |
| `Zettrox/ztrx` | 0 | 0 | 6 | nuked |
| `ZigZagBoi1/ScarlettPort` | 19 | 1 | 6 | active |
| `zohaiblazuli/GrandTheftXbox` | 0 | 0 | — | unreachable |
| `ZSlayerHQ/victory-world-runtime` | 21 | 10 | — | renamed |

