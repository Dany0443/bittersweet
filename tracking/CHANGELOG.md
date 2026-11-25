# Upstream sync — 2026-11-25T16:34:42Z

Source: <https://vuyp.github.io/gta6-pc-race/>  
Window: commits since `2026-11-25T00:00:00Z`  
Tracked repositories: **43**

## Newly discovered repositories

| Repository | Name | Family | Stage |
|---|---|---|---|
| [cocainebunnylaboratories/cblvictoryvi](https://github.com/cocainebunnylaboratories/cblvictoryvi) | CBL VIctory | victory-ecosystem | 6 |
| [mewiof/Holy-Interoperability](https://github.com/mewiof/Holy-Interoperability) | Holy-Interoperability | unclassified | 4 |
| [omonkeyz/xbox-emulator](https://github.com/omonkeyz/xbox-emulator) | xbox-emulator | emulator | 3 |
| [boykopovar/AnyPS5](https://github.com/boykopovar/AnyPS5) | AnyPS5 | emulator | — |
| [Freminet/ScarlettX](https://github.com/Freminet/ScarlettX) | ScarlettX | compatibility-layer | — |
| [jadeowaowa/scarlett-layer](https://github.com/jadeowaowa/scarlett-layer) | scarlett-layer | compatibility-layer | — |

## Renamed or moved

- `ZSlayerHQ/victory-world-runtime` now resolves to `ZSlayerHQ/zslayer-pc-port-runtime`

## Triage totals

| Bucket | Items |
|---|---|
| critical-linux | 61 |
| frame-time | 514 |
| stability | 249 |
| rendering | 746 |
| experimental | 65 |
| unclassified | 1391 |

## Critical Linux / Proton-GE fixes

- `marvisak/VIctory` #2334 — Fix the Linux build and clippy gates (carries #1017)
- `fxceless/victory-fork` #1011 — Indirect: the argument clamp in a form Wine's HLSL compiler builds too
- `fxceless/victory-fork` #20 — GPU: compile our run-time HLSL with DXC when the system compiler refuses it (Wine/Proton)
- `fxceless/victory-fork` #259 — Pipeline cache: save the early list when there is no host pipeline library (Wine/Proton)
- `fxceless/victory-fork` #257 — Shaders: one latch per loop in emitted DXIL (dxil-spirv asserts on two back edges and stalls the pipeline workers under vkd3d-proton)
- `PastiiJay/klbf-lab` #12 — Fix Wine pipeline worker overflows in shader translation
- `PastiiJay/klbf-lab` `7afe3f77244c` — Pipeline library: an empty host library is created from a null blob, so caching works on Wine/vkd3d-proton (gmankab, based on Marvisak/VIctory#347)
- `vitutu1/victory-perf` `0436e0f2fd41` — Merge pull request #7 from Coinami23-cmd/fix/gpu-linux-host-guards
- `LooperSalty/AnyXboxX` `ca8b76deafa6` — Merge pull request #199 from Exorion1er/feat/victory-linux-amd
- `LooperSalty/AnyXboxX` `873f359f5d4a` — Merge main into feat/victory-linux-amd (after #208)
- `LooperSalty/AnyXboxX` `df1f09c7ac92` — Merge main into feat/victory-linux-amd (after #207)
- `LooperSalty/AnyXboxX` `72470e593ca6` — Merge branch 'main' into feat/victory-linux-amd
- `LooperSalty/AnyXboxX` `ca61dc669c19` — Merge branch 'main' into feat/victory-linux-amd
- `LooperSalty/AnyXboxX` `5c35d38fdaf1` — docs: VIctory in the repository, Proton and AMD fixes
- `Zettrox/ztrx` #2 — Run on Proton: a Windows build of victory, launched inside a Proton prefix
- `Zettrox/ztrx` #119 — Shaders: every loop has one back edge (vkd3d-proton's converter recursed without end on four compute pipelines)
- `Zettrox/ztrx` #1 — Linux: compile, test and run the analysis commands + format all files
- `Belobug/VIctory-Belobug-AMD` #53 — Under Wine, title memory is copied, not used in place
- `Belobug/VIctory-Belobug-AMD` #42 — Run on Proton: a Windows build of victory, launched inside a Proton prefix
- `Belobug/VIctory-Belobug-AMD` #69 — Under Wine, copies of title memory are not sparse buffers by default (GPU reset once the world loads)
- `Belobug/VIctory-Belobug-AMD` #124 — GPU: the empty host pipeline library is made from a null blob (vkd3d-proton refused it: no pipeline kept under Wine)
- `Belobug/VIctory-Belobug-AMD` #109 — Fix memory mapping and shader lowering
- `Belobug/VIctory-Belobug-AMD` #41 — Compile and test on Linux, and an Ubuntu CI run
- `uiliannn/VItoriaBr` #23 — fix(gpu,shader): fix WARP depth bounds, indirect budget group race, and Linux CI
- `uiliannn/VItoriaBr` `8a85222ae269` — merge: PR #3 (@andysilva86) portable Linux CI and Portuguese overview docs
- `uiliannn/VItoriaBr` `d52d84c96977` — ci: run the portable crates' clippy and tests on Linux
- `pablogonz12/Scarlight` #83 — Scarlight as a Steam compatibility tool: the tool, a release tarball, and the docs
- `pablogonz12/Scarlight` #82 — Announcement: Scarlight as a Steam compatibility tool, previous setup preserved on legacy/amd-hub
- `pablogonz12/Scarlight` #81 — Drop the committed wine symlink that fails the policy gate
- `pablogonz12/Scarlight` #77 — docs: architecture decision — Proton for the OS side, Mesa for the GPU side, step by step
- `pablogonz12/Scarlight` #15 — VIctory #994 (Linux/Proton + debug pixel shader) on top of the speed stack
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
- `brcly/TuxBox` `faf8b197d4e6` — d3d12, bvh: stop TuxBox deadlocking the title's queues on its fence labels
- `brcly/TuxBox` `4b4c08cfe7c8` — d3d12: look up cached shaders without scanning the cache directory
- `xcl-dev/xcl` #2001 — D3D12.X: offload PSO builds, defer lazy compile-on-draw, and wire mmap ID3D12PipelineLibrary1
- `xcl-dev/xcl` #877 — Memory: lazily map sparse VRAM tiles for Wine graphics heaps (#175)
- `xcl-dev/xcl` #871 — Memory: avoid amdgpu userptr stalls under Wine (#175)
- `xcl-dev/xcl` #867 — D3D12.X: preserve native vtables and wrap fences for vkd3d-proton under Wine (#175)
- `xcl-dev/xcl` #1698 — XAudio2: under Wine GetState names the oldest queued buffer of a voice XCL does not decode
- `ubiquitarianism/leonida` #1946 — Honor CTest fixture cleanup ordering in the Wine runner
- `ubiquitarianism/leonida` #1845 — WINE-RESOURCE-LOCK-SHAPE: Validate CTest resource locks before scheduling Wine tests
- `ubiquitarianism/leonida` `98c2f7777276` — Merge pull request #1819 from aslan2142/tools-wine-game-run
- `ubiquitarianism/leonida` `597cf76623d6` — TOOLS-WINE-GAME-RUN: scripts/run_game_wine.sh, one bounded game run under Wine
- `ubiquitarianism/leonida` `237deb64ce2b` — Merge pull request #1539 from aslan2142/radv-import-pins
- `ethan00013/keylime` `361c7e94ab9d` — README: the driver deadlock behind the cutscene freeze
- `cocainebunnylaboratories/cblvictoryvi` `a00752cbd2b5` — Pipeline library: an empty host library is created from a null blob, so caching works on Wine/vkd3d-proton (gmankab, based on Marvisak/VIctory#347)
- `boykopovar/AnyPS5` #25956 — feat(libkernel): keep POSIX permission bits on Windows
- `boykopovar/AnyPS5` `11c9e7764efa` — Merge pull request #24706 from olegon85/fix/libkernel-posix-mlock-locks-mapped-pages

## Frame-time and CPU/GPU bottleneck wins

- `marvisak/VIctory` #2599 — GPU: read UAV MipSlice from offset 8 in image_words
- `marvisak/VIctory` #2342 — Shaders: descriptors chosen by uniform trees of joins and selects
- `marvisak/VIctory` #2447 — GPU: probe 4 KiB OS pages in descriptor sync_heaps
- `marvisak/VIctory` #2363 — GPU: descriptor dwords read as data — the record count and the format
- `marvisak/VIctory` #2262 — Tests: release placed descriptor heaps before reusing their memory
- `marvisak/VIctory` #1033 — victory-shader: build the per-instruction read/write lists once (ir::build -13%)
- `marvisak/VIctory` #2009 — Fix #721, #722: copy-back writes only changed bytes; mark pages current conditionally
- `marvisak/VIctory` #2592 — CLI: reject VS+GS graphics pipelines in prebuild
- `marvisak/VIctory` #2388 — Shaders: accept loaded buffer descriptors rewritten with base intact (#815)
- `marvisak/VIctory` #2201 — gpu: compact non-empty indirect draw and dispatch arguments
- `marvisak/VIctory` #2290 — HLE: stub DirectStorage queue GetErrorEvent and RetrieveErrorRecord
- `marvisak/VIctory` #2128 — GPU: populate unbound table slots with typed null descriptors
- `marvisak/VIctory` #2110 — GPU diagnostics: keep pipeline drop reasons until they are reported
- `marvisak/VIctory` #1449 — GPU: report host resources held by descriptor heap slots in diagnostics
- `marvisak/VIctory` #559 — GPU: track mip states for 2D color targets
- `marvisak/VIctory` #562 — GPU: reuse immutable descriptor tables across command lists
- `marvisak/VIctory` #2365 — Shaders: name the instruction that ended a whole-wave section
- `marvisak/VIctory` #2312 — GPU, Shaders: spread compute dispatches past 65,535 groups in x across z (fixes #2256)
- `marvisak/VIctory` #2471 — GPU: a refused pipeline says what the debug layer answered
- `marvisak/VIctory` #2545 — GPU: avoid illegal COPY_SOURCE self-transition and keep HostTexture alive in Host::present
- `marvisak/VIctory` #1904 — GPU: VICTORY_LIBRARY_TIMING measures the host FidelityFX upscaler's GPU and CPU cost
- `marvisak/VIctory` #2472 — GPU: hoist and batch indirect argument clamps into the list prelude
- `marvisak/VIctory` `54ab40676850` — Shaders: descriptors joined by nested control flow, again
- `marvisak/VIctory` `850b66ffd7e0` — Shaders: one handle per descriptor at a fixed address, and per join
- `marvisak/VIctory` `ea812894eeb0` — GPU: a submission's texture uploads become readable in one barrier batch
- `marvisak/VIctory` `e790217a0e75` — Shaders: an image descriptor read only for its size takes it from the host
- `marvisak/VIctory` `b2f6c9559601` — GPU: upload tests count their own texture's uploads
- `marvisak/VIctory` `d75491e99155` — GPU: a texture written in pages of its own is uploaded without hashing it
- `marvisak/VIctory` `fbac5bb64c8e` — Shaders: descriptors joined by nested control flow stay refused for now
- `marvisak/VIctory` `ff4746389a03` — Shaders: a descriptor no lane brings along an edge needs no source there
- `marvisak/VIctory` `59c78bb5f7c3` — Shaders: descriptors joined by nested control flow
- `marvisak/VIctory` `a2c50db87298` — GPU: a drop counted by its reason names the pipeline's shaders
- `marvisak/VIctory` `ac7ffc0ae14e` — GPU: large heap allocations name their caller; list upload memory in the memory line (diagnostics)
- `marvisak/VIctory` `ed887f24f0bf` — GPU: diagnostics: copies redone piece by piece are timed; GPU timing keys keep the pipeline
- `marvisak/VIctory` `eddde73437a7` — HLE: DirectStorage reads into physical pages wait for the title's Submit
- `marvisak/VIctory` `0086ccbc3621` — GPU: traced shaders' indirect records are logged in every frame
- `marvisak/VIctory` `1365fe11e5b2` — GPU: diagnostics for the frame's cost: heap sync time, video memory, GPU time per pipeline
- `marvisak/VIctory` `940d32f69b22` — GPU: a marked page's copy-back does not save the page for its own copy
- `marvisak/VIctory` `cd94d3538876` — GPU: shaders mark the pages they write through views; copy-backs compare only those
- `marvisak/VIctory` `24effa8a739e` — Merge pull request #1446 from geiyiliyabumo/fix/list-create-initial-pipeline
- `fxceless/victory-fork` #614 — Where the frame time goes on an AMD RX 6600: indirect group counts in the timing run, GPU timing needs DRED, a guide to slow shaders
- `fxceless/victory-fork` #45 — gpu: scale texture VRAM floor and per-frame staging budget from VRAM
- `fxceless/victory-fork` #980 — Hot title memory in VRAM (#772) with live ray tracing (#814): a third less copying
- `fxceless/victory-fork` #1018 — GPU: skip CPU BLAKE3 texture hashing via xmem dirty-generation digests (VICTORY_TEXTURE_DIRTY_GEN=1)
- `fxceless/victory-fork` #51 — GPU: no blending on integer render targets in host pipelines (port of upstream #806)
- `fxceless/victory-fork` #1016 — GPU: specialize the indirect argument clamp for single records
- `fxceless/victory-fork` #1014 — The card compare hashes a piece with a group of 64 threads: our passes on the card from about 35 to about 10 ms a frame
- `fxceless/victory-fork` #834 — GPU: indirect dispatches past 65,535 groups in x run the rest in a second execution
- `fxceless/victory-fork` #792 — Mirror copied descriptors from the fingerprinted record
- `fxceless/victory-fork` #995 — Frame rate: indirect clamps in runs, landing fetches from card slots, re-check fix (2.5 -> 7.5 fps on a 5070 Ti)
- `fxceless/victory-fork` #1001 — GPU: batch requested indirect prefixes in one prelude (opt-in)
- `fxceless/victory-fork` #1010 — Descriptors: a view heap with room past the title's slots makes them all dynamic, up to 2^20
- `fxceless/victory-fork` #998 — gpu: compact empty indirect records in device-local clamp
- `fxceless/victory-fork` #188 — Series S: the second present plane, and a plain stretch for the upscale dispatch
- `fxceless/victory-fork` #1000 — GPU: indirect records clamped into a ring of the list's behind one enhanced barrier, and in the list's prelude where the list did not write them
- `fxceless/victory-fork` #997 — gpu: coalesce card-memory piece hashing across 64 threads per piece
- `fxceless/victory-fork` #992 — GPU: the compare of card copies hashes each 64 KiB piece with a group of threads (a pass 0.2-0.8 ms -> 0.002-0.11 ms on an RTX 5090)
- `fxceless/victory-fork` #989 — GPU: bound detiling slices to the upload allocation
- `fxceless/victory-fork` #976 — Dispatches past the host's group limit run in pieces, each told where its groups start
- `fxceless/victory-fork` #860 — Upload fills: stop false trips of the fill-ahead latch; quiet depth U…
- `fxceless/victory-fork` #977 — GPU: keep card meter timestamp pairs through queue completion
- `fxceless/victory-fork` `465b94f54e2f` — Scattered indirect records clamped once per list (VICTORY_INDIRECT_GATHER=1): 10.3 fps in the first rooms
- `fxceless/victory-fork` `39731795d2d5` — What a frame is made of, measured; parts joined and copy-back through our own view (9.6 fps in the first rooms)
- `fxceless/victory-fork` `a25370661ed1` — Frame rate, third session: a frame log and a card meter, the driver watch, a forbidden self-copy removed; draw windows measured at 6.0 -> 8.9 frames a second
- `fxceless/victory-fork` `f85101b6095e` — Frame rate, second night: a submit thread of the layer's own, a copy queue for fetches, indirect draw windows made safer, a profiler; all behind switches
- `fxceless/victory-fork` `9376fd60c0ba` — Frame rate, processor side: a cheap fingerprint in the descriptor sync, draw-table slots left to their draws, no failing write-watch calls; card fetches inside their flights (off by default)
- `fxceless/victory-fork` `a26859fc5f51` — The first room renders right: byte-copied descriptor records are mirrored, views off their copy's elements get their own bytes, card room for the first world frame, levels on demand run on the card
- `fxceless/victory-fork` `8dd30ab47269` — A write through one view of a page is seen at its other views; page writes counted; levels on demand (off); warm-up; card memory numbers
- `ZSlayerHQ/victory-world-runtime` #7 — GPU: leave out passes that only feed or trace ray-tracing structures while none are built (~20% faster world frames)
- `ZSlayerHQ/victory-world-runtime` #8 — Copy-back of GPU-written textures, depth planes, target formats, submission stall removal, profiling tools
- `ZSlayerHQ/victory-world-runtime` `f25e7352498e` — README: warming the shader cache is recommended, and why
- `PastiiJay/klbf-lab` #16 — perf(gpu): shard HostHeap::mirror across 256 descriptor stripes
- `PastiiJay/klbf-lab` #15 — perf(cpu): pin GPU and host threads to P-cores and optional disk I/O to E-cores
- `PastiiJay/klbf-lab` `334915456130` — Beginner's guide: copy-paste commands from installing the tools to playing, page file, troubleshooting
- `PastiiJay/klbf-lab` `a5a177f50a33` — Run: step-by-step guide, with what the pipeline pre-build is and why
- `PastiiJay/klbf-lab` `a1402ea4e1cc` — README: frame rate per scene (prologue in the cell ~7.3, prologue while playing 5.7, Gone Fishing 6.3)
- `PastiiJay/klbf-lab` `746e3fcac161` — README: headline frame rate from a real play session (5.6 average, peak 9.8); still-camera number listed apart
- `PastiiJay/klbf-lab` `a67ab78ea969` — README: in-world frame rate with average, median, 1% and 0.1% lows and peak; how we measure
- `PastiiJay/klbf-lab` `2eb2103fe502` — README: 7.3 fps is the average of the 74 min run, with min/peak
- `PastiiJay/klbf-lab` `55f15a380160` — README: 7.3 fps, 74 minutes without a crash, up front
- …and 434 more (see `tracking/sync-report.json`)

## Stability and device-loss prevention

- `marvisak/VIctory` #1777 — GPU: size the view heap clamp from what the host takes (NVIDIA 1,000,000 cap)
- `marvisak/VIctory` #573 — Shaders: scope wave64 ds_bpermute_b32 to 32-lane halves
- `marvisak/VIctory` #2448 — Shader: ignore guarded or modified v_mov_b32 in Lower::konst
- `marvisak/VIctory` #2415 — GPU: remapping one mapping of aliased pages leaves the others guarded
- `marvisak/VIctory` #491 — GPU: remember pipelines whose creation crashed the process (VICTORY_PIPELINE_JOURNAL)
- `marvisak/VIctory` #2068 — Shaders: lower unlisted table constant buffer loads as guarded raw SRVs
- `marvisak/VIctory` #450 — GPU: preserve typed null render target slots
- `marvisak/VIctory` #2561 — GPU: a page whose other mapping cannot be guarded stays changed
- `marvisak/VIctory` #2558 — Shaders: heap indices from data clamped before the root views they index
- `marvisak/VIctory` #2441 — GPU: live 1440p present fallback when 4K primaries stay unwritten (fixes #841)
- `marvisak/VIctory` `315cea185d25` — GPU: barriers start from the states resources are in: prelude order, the clamp's own buffer
- `marvisak/VIctory` `889b87182b9f` — Shaders: a scalar VALU result takes no EXEC guard
- `marvisak/VIctory` `36ca299602a2` — GPU: a retake marks the copies without opening their guards
- `marvisak/VIctory` `a3b0f36e3cfc` — GPU: dropping a tracker keeps guards another mapping's tracker relies on
- `marvisak/VIctory` `280b1505891b` — GPU: change tracking guards only the runtime's memory, region by region
- `marvisak/VIctory` `3c47fd4945dd` — Merge pull request #1562 from BastienRodz/gpu/fence1-iid
- `fxceless/victory-fork` #1020 — Park emulated MONITORX/MWAITX waits on GPU fence signals
- `fxceless/victory-fork` #854 — Shaders: wave64 programs in a paired form on 32-lane GPUs, mesh programs over 32 KiB linked with their pixel program, and more (230 more programs pass)
- `fxceless/victory-fork` #1006 — Shaders: the code table starts after the instructions (fixes RTX 5090 Xid 13; port of Marvisak/VIctory#2143)
- `fxceless/victory-fork` #1009 — Watch: a kernel write into title memory refused by a guard set again is opened and made
- `fxceless/victory-fork` #1008 — Tests: the view-guard tests' views are forgotten by the journal too, and holes in title memory are tested
- `fxceless/victory-fork` #823 — NVIDIA (32-lane waves): four refused compute programs run, the whole structure builder has pipelines, xmem maps page runs as one view
- `fxceless/victory-fork` #979 — GPU: walk only dirty gaps when refreshing guarded views
- `fxceless/victory-fork` #986 — GPU: stop when a direct submission fence wait fails
- `fxceless/victory-fork` #984 — Shaders: one index clamp per heap handle (AMD's DXIL compiler crashes on shared ones); tools for driver compiler faults
- `fxceless/victory-fork` #987 — GPU: cheaper guards and copy refreshes in the world (seven switches, all off by default)
- `fxceless/victory-fork` #959 — Fix the 7–11 minute pool crash (upstream audio fixes), plus GPU stability fixes
- `fxceless/victory-fork` #799 — NVIDIA (wave32) host fixes: CBV clamp, loose-LDS fallback, scalar ops
- `fxceless/victory-fork` #500 — Shaders: VICTORY_LDS_EXACT=1 orders LDS in a lane-branch region by the dwords it touches (369cbe906ea72764 on 32-lane GPUs)
- `fxceless/victory-fork` `31714deb2d70` — Take the fix/amd-view-heap-clamp branch of triippywhale's fork (as one commit)
- `ZSlayerHQ/victory-world-runtime` `31714deb2d70` — Take the fix/amd-view-heap-clamp branch of triippywhale's fork (as one commit)
- `PastiiJay/klbf-lab` #13 — Clarify beta retests and startup crash reports in the Windows guide
- `PastiiJay/klbf-lab` #2 — Shaders: compile the four compute shaders the RDNA 4 compiler crashes on
- `realblastex/flamingo` `baae9df07c60` — Wave64 programs in a paired form on 32-lane GPUs (victory-fork #854): no pipeline refused, no program untranslated; progress log through run 108
- `JMAN730/project-flamingo` #593 — fx-4090 stack + VICTORY_WAVE64_DOUBLE (wave64 compute on 32-lane hosts)
- `JMAN730/project-flamingo` `0d34a0e9ce63` — Merge pull request #700 from Alan-Arriero/docs/agent-contribution-guardrails
- `JMAN730/project-flamingo` `ca734c57cc89` — Merge branch 'main' into docs/agent-contribution-guardrails
- `JMAN730/project-flamingo` `e2b5fb11ac12` — Merge branch 'main' into docs/agent-contribution-guardrails
- `JMAN730/project-flamingo` `1d1a57a65d6e` — Merge pull request #658 from JMAN730/cursor/gpu-warp-null-srv-cbv-485-111a
- `JMAN730/project-flamingo` `03a57414fcce` — Merge main into fix/mesh-psv-nvidia-crash, set StageIo::Ps.rsrc2, and extract select_tests
- `JMAN730/project-flamingo` `38ca9d75247c` — Docs: add concise agent contribution guardrails
- `JMAN730/project-flamingo` `48ca4f80e5d4` — Shaders: rustfmt the clamped-add guard
- `JMAN730/project-flamingo` `2fcdb1b8e4c9` — Shaders: keep the clamped-add refusal after the directory merge
- `JMAN730/project-flamingo` `a644208d05de` — Shaders: avoid if-let match guards for MSRV 1.88
- `JMAN730/project-flamingo` `3a4a5d2bb63c` — GPU: drop stale refresh_special_buffers comment
- `JMAN730/project-flamingo` `5c7e5d27de28` — Merge pull request #675 from JMAN730/cursor/shaders-int-clamp-440-111a
- `JMAN730/project-flamingo` `d965b149b250` — Shaders: skip untyped heap-direct CBV at null slot only (#485)
- `JMAN730/project-flamingo` `e83865181933` — GPU: skip heap-direct CBV loads at the title's null SRV slot
- `vitutu1/victory-perf` #12 — shader: recover disjoint and lane-private wave64 LDS on wave32 hosts
- `vitutu1/victory-perf` `dd4402b1afdb` — Eliminate watch::guard and 5-thread command-recording lock contention
- `vitutu1/victory-perf` `406c41024680` — fix(ci): clear test clippy warnings and serialize watch guard tests
- `vitutu1/victory-perf` `31714deb2d70` — Take the fix/amd-view-heap-clamp branch of triippywhale's fork (as one commit)
- `LooperSalty/AnyXboxX` `5c95069030a3` — docs: note wave64 complétée par les mesures RTX et WARP (#70)
- `LooperSalty/AnyXboxX` `d1f3e94fbabc` — feat: programmes wave64 recompilés en HLSL pour des GPU à waves de 32 lignes (#70)
- `LooperSalty/AnyXboxX` `8461610993b4` — feat: mode wave64 dans l'interpréteur de référence RDNA 2 (#70)
- `LooperSalty/AnyXboxX` `f666e2517f84` — docs: note de conception wave64 sur des GPU à waves de 32 lignes (#70)
- `LooperSalty/AnyXboxX` `618b0caa8847` — GPU: constant buffer views held to the 64 KiB a shader can index; null buffer views named
- `Zettrox/ztrx` #134 — bcpack: the destination guard, the dictionary's reach, and the degenerate streams
- `Zettrox/ztrx` #109 — Shaders: give buffer size queries their own bounded heap handle (AMD compiler crash)
- `Zettrox/ztrx` #61 — Shaders: leave the reuse pass out on AMD, whose compiler crashes on it
- `Zettrox/ztrx` #50 — GPU: clamp every host view heap to a null slot, and key the shader cache by it
- `Zettrox/ztrx` #32 — Fix stale frames for guarded textures
- `Zettrox/ztrx` #20 — NVIDIA: reserve the null clamp slot in every capped view heap and key shader cache by cap
- `Zettrox/ztrx` #30 — Shaders: automatically guard divergent single-console-wave barriers on narrower host waves
- `Belobug/VIctory-Belobug-AMD` #140 — Shader tests: WARP runs one draw or dispatch at a time (parallel runs crashed in d3d10warp.dll)
- `Belobug/VIctory-Belobug-AMD` #139 — Submission CPU time: helper threads kept, bindless plan lists kept and counted, guard bit sets, unattended benchmark
- `Belobug/VIctory-Belobug-AMD` #68 — Wave64 on 32-lane hosts: one console wave sharing LDS runs in the console flow (+2 NVIDIA fixes)
- `Belobug/VIctory-Belobug-AMD` #71 — Fix stale and misplaced comments
- `Belobug/VIctory-Belobug-AMD` #112 — watch: clear stale JOURNAL.written entries on title_memory_remapped (#102)
- `Belobug/VIctory-Belobug-AMD` #55 — Rectangle expansion reuses the clamp's shader builder
- `Belobug/VIctory-Belobug-AMD` #134 — Shaders: recover wave64 masks restored from scalar copies
- `Belobug/VIctory-Belobug-AMD` #133 — Loader: reject garbage relocation sizes, serialise crash-log appends
- `Belobug/VIctory-Belobug-AMD` #92 — Shaders: keep 64-lane waves for wave64 programs on a 64-lane host
- `Belobug/VIctory-Belobug-AMD` #128 — Wave64 on 32-lane hosts: several console waves per group, wave reductions in halves, permlanes (+ an SCC fix)
- `Belobug/VIctory-Belobug-AMD` #104 — victory system: this machine for test reports; the commit built from in --version and crash reports
- `Belobug/VIctory-Belobug-AMD` #94 — Shaders: separate bounded heap handle for buffer size queries (AMD compiler crash)
- `Belobug/VIctory-Belobug-AMD` #60 — Keep pinned blocks writable on guard()'s fast path too
- `Belobug/VIctory-Belobug-AMD` #28 — Bind the clamp pipeline once per clamp list
- `Belobug/VIctory-Belobug-AMD` `9b89ec2d384f` — Only the title's submissions may return before the host GPU is done; ours wait (the present's capture buffer was freed with its copy pending: device removed at the first frame with VICTORY_ASYNC_SUBMIT=1)
- `Belobug/VIctory-Belobug-AMD` `f2cc1009a73f` — The views are looked at once for a run of guarded blocks, and pages with one view make no list (less time under the guards' lock)
- …and 169 more (see `tracking/sync-report.json`)

## Rendering and gameplay completeness

- `marvisak/VIctory` #2600 — Shaders: lower binary64 truncation
- `marvisak/VIctory` #2548 — Shaders: lower v_mov_b32 abs/neg modifiers bitwise
- `marvisak/VIctory` #1799 — fix(shader,gpu): eliminate 22s s_barrier world TDRs, zero invalid indirect mesh args, and reach Vice City opening credits & world frames
- `marvisak/VIctory` #1555 — GPU: coalesce nearby shadow-buffer refreshes without spanning huge heap gaps
- `marvisak/VIctory` #1473 — Shaders: track per-lane export state across mixed pixel-shader joins
- `marvisak/VIctory` #2598 — GPU: giving up a texture leaves the dynamic slots of recorded lists alone
- `marvisak/VIctory` #2314 — GPU: copy compatible multisampled texture aliases directly
- `marvisak/VIctory` #1962 — GPU: make placed-texture heap slots that shaders actually read
- `marvisak/VIctory` #2357 — Shader tests: heap buffer views are raw, as the host makes them
- `marvisak/VIctory` #1765 — GPU tests: the castable pair is two host textures where the host refuses castable formats
- `marvisak/VIctory` #1623 — gfx10: keep an instruction's words inline instead of in a heap vector (decode -18..-45%, translation -1..-5%)
- `marvisak/VIctory` #2596 — Shader tests: an empty root-constant list is set as the one constant it declares (#2571)
- `marvisak/VIctory` #2594 — Shader: lower f32 fused multiply-adds through f64
- `marvisak/VIctory` #2593 — Shader tests: BCPack's zigzag decode as console code matches the formats function
- `marvisak/VIctory` #1739 — GPU: clear final mesh pipeline-stream padding after typed moves
- `marvisak/VIctory` #1738 — Shaders: preserve live interpolation values (on #453)
- `marvisak/VIctory` #1737 — Shaders: honor permlane EXEC and FI/BC flags (on #1014)
- `marvisak/VIctory` #1871 — Shaders: lower indexed tbuffer_load_format_* and indexed buffer_store_{byte,short}
- `marvisak/VIctory` #2591 — Formats: add a runnable BC5 record reconstruction shader example
- `marvisak/VIctory` #1406 — GPU: fix NULL/UNKNOWN SRV/UAVs on promoted-typeless textures and MSAA/array/3D NULL RTV/DSVs
- `marvisak/VIctory` #2382 — Shaders: follow EXEC set from a scalar the section made constant
- `marvisak/VIctory` #2358 — GPU: prepare and upload only dirty texture bands (opt-in)
- `marvisak/VIctory` #2478 — GPU: an option to refresh a texture once every N frames
- `marvisak/VIctory` #2588 — Decode RGB9E5 in diagnostic PNG captures
- `marvisak/VIctory` #2047 — GPU: guard subresource and region bounds in texture copies and MSAA resolves
- `marvisak/VIctory` #2587 — Shaders: skip Set*Root32BitConstants for empty root-constant slices
- `marvisak/VIctory` #1374 — GPU: read same-list GPU writes in placed buffer-to-texture copies
- `marvisak/VIctory` #602 — GPU: report retained snapshot, texture scratch and upload memory
- `marvisak/VIctory` #2398 — Shaders, GPU: link over-budget mesh and pixel shaders under D3D12's 32 KiB output limit
- `marvisak/VIctory` #601 — GPU: bound texture upload and capture slices to the allocated buffer
- `marvisak/VIctory` #1273 — GPU: mirror placed unordered texture views
- `marvisak/VIctory` #2509 — GPU: upload the stencil plane with guest textures
- `marvisak/VIctory` #1422 — GPU: ignore clear values on non-target textures and avoid caching failed host textures in PLACED_TEXTURES
- `marvisak/VIctory` #2341 — GPU: refresh a texture alias after its source is written again
- `marvisak/VIctory` #2319 — GPU: create a texture without a host UAV format without the flag
- `marvisak/VIctory` #2528 — GPU diagnostics: count texture data without row padding
- `marvisak/VIctory` #2484 — Shaders: look up the second page of straddling title-memory writes in the page table
- `marvisak/VIctory` #1776 — GPU: exclude 4:2:2 packed formats 68/69 from castable texture formats
- `marvisak/VIctory` #1915 — Shaders: lower 32-bit and 64-bit integer global atomics (cmpswap, csub, inc, dec, _x2)
- `marvisak/VIctory` `67e8ae1401df` — Shader tests: the compute harness zeroes its UAVs
- `marvisak/VIctory` `da82e444ec46` — Shader: normalized buffer codes convert to the float nearest their quotient
- `marvisak/VIctory` `4015a182e399` — Shaders: quad reads in pixel programs reach the helpers at triangle edges
- `marvisak/VIctory` `a1a2077a3d9a` — Shaders: vertex, geometry and mesh programs with UAVs say so in their flags
- `marvisak/VIctory` `8681044f025d` — Shader tests: the differential harness binds the view-stride table
- `marvisak/VIctory` `286a00e9b698` — Shaders: the DXIL writer leaves out what no edge or reader reaches
- `marvisak/VIctory` `8f2a6f145475` — GPU: a texture refresh reads title memory into its thread's own buffer
- `marvisak/VIctory` `11e9a54448a8` — GPU: textures fed from title memory are placed in heaps made ahead
- `marvisak/VIctory` `f68442ffe04c` — GPU: a texture a host copy writes is the host's from the copy's submission
- `marvisak/VIctory` `ab2e2b24a0a6` — GPU: the textures of slots shaders read are made several at a time
- `marvisak/VIctory` `8a3d975940b3` — GPU: textures' staging space is carved from large upload buffers
- `marvisak/VIctory` `8ff791351662` — Shader: rectangles complete opposite their right angle, wherever it is
- `marvisak/VIctory` `56f75f858306` — GPU: textures are brought up outside the heaps' locks, several at a time
- `marvisak/VIctory` `c6e95001b05c` — GPU: a texture's refresh asks its tracker's clean bit before taking its pages
- `marvisak/VIctory` `ed25c0e24ed5` — GPU: a slot the title rewrites keeps no per-texture list of slots
- `marvisak/VIctory` `cea278c8973c` — GPU: textures for heap slots shaders read are made outside the heaps' lock
- `marvisak/VIctory` `4959d3498d6f` — GPU: textures the heaps view are brought up to date only when shaders read them lately
- `marvisak/VIctory` `69d1e8e79a70` — GPU: host textures fed from title memory are given up when unused, and past the card's budget
- `marvisak/VIctory` `672e7169e52f` — Shader: the read-mark test's doc comment is no list
- `marvisak/VIctory` `2d6c0dfcb637` — Shader: read marks only for texture views taken from the heap itself
- `marvisak/VIctory` `52d14b1098b8` — GPU, Shader: placed textures made when shaders read their heap slots
- `marvisak/VIctory` `47cde30fe290` — Shaders: a test of register arrays indexed in waterfall loops
- `marvisak/VIctory` `4a3166e68039` — Shaders: lanes skipping a compare's region read 0 of its lane mask
- `marvisak/VIctory` `58b5255838d8` — Shaders: an address half set on every taken edge into its join is known
- `marvisak/VIctory` `cc5f96125900` — GPU: a texture re-detiles only the subresources whose pages changed
- `marvisak/VIctory` `fdf34ab7bac2` — GPU: texture staging buffers come from a small shared pool, not one per texture
- `marvisak/VIctory` `658a0d199335` — GPU: a retake marks host buffer copies alone, not textures' staging
- `marvisak/VIctory` `4243a96a787a` — Shader: 16-bit typed buffer accesses pass bits through integer views
- `marvisak/VIctory` `f342dcfcfba9` — Shaders: inputs a counted v_movreld loop overwrites first need no value
- `marvisak/VIctory` `e8e9f0c2aef9` — Shaders: a lane mask joined with data at a uniform join is its number
- `marvisak/VIctory` `23a4b62bbb7d` — GPU: a texture is uploaded once per submission, and only when its bytes changed
- `marvisak/VIctory` `97721d1d2ca0` — GPU: the title memory read to detile a tracked texture is not kept
- `marvisak/VIctory` `5cb63d8b28b9` — GPU: texture to buffer copies read back into the title's buffer
- `marvisak/VIctory` `5622159f29e8` — GPU: debug lines for sampled texture uploads
- `marvisak/VIctory` `a7381ed606cb` — GPU: host textures made on first use
- `marvisak/VIctory` `67e330a68672` — GPU: process memory and a host texture census in the statistics
- `marvisak/VIctory` `f1c55a8281bd` — GPU: a placed texture's slot left null is not made again at every heap sync
- `marvisak/VIctory` `ccfb02996eac` — Tests: a machine with no GPU has no hardware adapter to check shaders on
- `marvisak/VIctory` `42363ab6cd52` — Tests: hardware-only shader checks skip on a machine without a GPU
- `marvisak/VIctory` `4436fca34dd4` — Merge pull request #1696 from Calvix2k/codex/shader-interp-p2-f16-029
- `marvisak/VIctory` `5fa5de35eec2` — Merge pull request #1733 from moe-miau/fix/shader-helper-lane-readfirstlane
- …and 666 more (see `tracking/sync-report.json`)

## Experimental / unstable (hold)

- `marvisak/VIctory` #2389 — GPU: streamed textures take video memory only for the levels the title holds (opt-in, ~5 GB less)
- `marvisak/VIctory` #2292 — GPU diagnostics: a pipeline whose creation faults is refused and the title goes on (opt-in, on #491)
- `marvisak/VIctory` #2146 — GPU: add opt-in VICTORY_TEXTURE_MIP_DROP quality switch (fixes #737)
- `marvisak/VIctory` #2112 — GPU: title memory in place instead of copied, opt-in (VICTORY_IN_PLACE=1; stacked on #2088, towards #880)
- `marvisak/VIctory` #2120 — GPU: copies of read-only title buffers in GPU upload heaps (opt-in)
- `marvisak/VIctory` #1917 — FidelityFX: add opt-in AMD frame generation to host upscaling
- `marvisak/VIctory` `9cc37c5f5b20` — GPU: lists wait on memory values; queues hold work until its waits hold
- `fxceless/victory-fork` #868 — Card copies: a census, and an opt-in switch that leaves write-combining memory on the card (−11…−20 % frame, 3 GPUs)
- `fxceless/victory-fork` #994 — Ray tracing: the title's structure builder runs without crashing it (dispatches by thread counts, title memory by address made ready); node tests answer no hit while it is off (stacked on #854 and #993)
- `fxceless/victory-fork` #1013 — Ray tracing: the builder's quarantine holds (split out of #935)
- `fxceless/victory-fork` #917 — GPU: a constant can stand in for indirect light while rays do not run (VICTORY_AMBIENT, off by default)
- `fxceless/victory-fork` #945 — Diagnostics: opt-in HLE/read tracing and configurable view heap cap
- `fxceless/victory-fork` #648 — Card copies' pieces come down inside their own flights: relay fetch wait 139 ? 0 ms/frame (VICTORY_CARD_FETCH_INFLIGHT=1, off by default)
- `realblastex/flamingo` `5a7bbf4e4345` — Progress log: runs 85-88 (the cutscene's dropped work is the ray-tracing builder; the builder still stops at a pointer it never wrote)
- `realblastex/flamingo` `dc0c736c2a74` — victory-fork main merged; the mesh edge-flag fix (#1); Belobug's opt-ins; Clippy clean
- `LooperSalty/AnyXboxX` `d2732341e048` — chore: cache the TomBox console DLLs in CI instead of rebuilding them
- `LooperSalty/AnyXboxX` `04738a72c280` — GPU: VICTORY_L9_ALIAS_UNCLEAN and VICTORY_HEAP_RECHECK are opt-in
- `LooperSalty/AnyXboxX` `0dbaf9fd4b91` — GPU: compute pipelines get their own waiting time while a world loads; ray-tracing resolves left out
- `Zettrox/ztrx` #139 — GPU: argument clamp with enhanced barriers (VICTORY_NARROW_WAITS, opt-in)
- `Zettrox/ztrx` #111 — device: hold the claimed capabilities against what the host reports
- `Zettrox/ztrx` #92 — Pass user32.dll through to the host, as the policy already intends
- `Belobug/VIctory-Belobug-AMD` #118 — FidelityFX: Series X host FSR and opt-in AMD frame generation
- `Belobug/VIctory-Belobug-AMD` `bdecda306a3e` — An unwaited submission is finished when it is waited for (its shaders' writes, read-backs and bottom-of-pipe writes), so every submission of the title's may go unwaited; the next one's buffer copies are refreshed meanwhile (VICTORY_ASYNC_SUBMIT=1, off by default)
- `Belobug/VIctory-Belobug-AMD` `0a32e89cf58a` — Tiles of title memory that is no longer there can be given back (VICTORY_SWEEP_TILES=2), so the copies stop growing; off by default
- `ka1qux/BRVIctory-ka1qux` `574b317d44fd` — GPU: a copy-back's runs of the GPU's bytes are written under one hold of their guards
- `DexNom/VIctory` `8cca4d228541` — Document experimental RT launch and measured City frame times
- `DexNom/VIctory` `d71f197957da` — Add an opt-in Windows shortcut for experimental ray tracing
- `uiliannn/VItoriaBr` `32d784861a55` — GPU: hold SHADER_HEAPS across descriptor mirror and check query offsets
- `uiliannn/VItoriaBr` `708b45015726` — HLE: load a host DLL outside the module cache's lock
- `pragyaangaur/VIctory-macOS` `9cc37c5f5b20` — GPU: lists wait on memory values; queues hold work until its waits hold
- `luizwidmer/palmetto` `e8a23e3515fd` — platform: deliver the measured user-change event and offline async completions, both opt-in
- `pablogonz12/Scarlight` #86 — victory-gpu: title buffer copies on the host GPU through the buffers' host copies (stacked on the unification PR)
- `pablogonz12/Scarlight` `29ac46272b82` — shader tests: hold the one-at-a-time lock while the lane-read tests dispatch
- `pablogonz12/Scarlight` `7fbdb2398805` — GPU: write-back holds find the handler, wait before opening, retry released reads
- `xcl-dev/xcl` #2268 — Scripts: pr_check holds the 125 % peak commit to medians of 3 runs
- `xcl-dev/xcl` #1331 — D3D12.X: hold BASES lock across DEPTH_BIAS_PSOS in depth_bias_pso and forget_base
- `xcl-dev/xcl` `f8ce21de8f59` — Config: an alias pattern with several '*' matches its DLLs
- `xcl-dev/xcl` `1a0bbe3a2ba9` — Merge pull request #1782 from imnicho/sleep-hold-pc-calls
- `R0Y00/Xbox2Windows` #2357 — tests: add opt-in DXC compile cache for the Python shader checks
- `R0Y00/Xbox2Windows` #1638 — shader: lower experimental gfx1030 2D sample clamps
- `R0Y00/Xbox2Windows` `061c0958d33f` — startup: optional Tracy zones for the stand-ins (X2W_TRACY, off by default) (#2371)
- `ThatRecompDev/five-star-recomp` `6752f701d6c0` — Give the ray-tracing program tests names of their own
- `ThatRecompDev/five-star-recomp` `d53d5fc6ff34` — Merge lane-dxilswitch: stores past the user space reach nothing (span guard); name holds for ray-tracing-only passes
- `ThatRecompDev/five-star-recomp` `0a962c4d7afe` — Hold programs that only serve ray tracing
- `ThatRecompDev/five-star-recomp` `44afe4155e54` — Merge lane-dxilswitch: no program reads on game threads; failed builds always give a stand-in; ray-tracing markers survive translator updates
- `ThatRecompDev/five-star-recomp` `9b9eb0d26123` — Merge lane-world: --depth-twins (opt-in) gives depth UAVs a colour twin copied back before depth tests
- `ThatRecompDev/five-star-recomp` `3c9a3f52963a` — Merge lane-lowvram: --typed-image-views (opt-in) reads and writes integer image views as integers
- `ThatRecompDev/five-star-recomp` `2539efdc2bda` — Merge lane-dxilswitch: held ray-tracing pipelines never reach the driver; disposed runtimes stop being active; fence-only barriers in the switch form
- `ThatRecompDev/five-star-recomp` `7b96f203ecc2` — Merge lane-lowvram: backstop test registers its own handler; ray-tracing test owns its runtime
- `ThatRecompDev/five-star-recomp` `1910d2089aea` — Status: ray-tracing passes held without --raytracing; first 18-minute world boot with no removal
- `ThatRecompDev/five-star-recomp` `c30e31efdf6f` — Merge lane-dxilswitch: ray-tracing pipelines stay stand-ins without --raytracing; LDS barriers; flaky tests fixed
- `ThatRecompDev/five-star-recomp` `3558b38db19f` — Merge lane-present: static samplers from their S# words, --fast-waves (off by default)
- `ubiquitarianism/leonida` #1924 — Stream output capture wip
- `ethan00013/keylime` `0afb8154c516` — A loop stopped by its trip cap ends the program; KEYLIME_ONE_QUEUE; per-command checkpoints opt-in
- `ethan00013/keylime` `b7dadd4dc2ca` — GPU: hold submissions until their waits are signalled; poll fences
- `Archivist-Atlas/gta-vi-astral` `3b2bbb74d259` — docs: add progress bar, WIP warning, and subsystem status table to README
- `sammwyy/xvalkyrie` `e14d33c8ca6a` — fix(d3d12): tables past the host's part of a title's view heap are copied into a ring at its end when the list runs, so that what they hold is seen
- `sammwyy/xvalkyrie` `2efd90369b10` — fix(d3d12): only shaders whose file says what their user registers hold read the heaps themselves, as compute shaders already did
- `daeken/XSXEmu` `262e5f47cc68` — Require a retained Metal backend for opt-in owned Xbox device creation
- `cocainebunnylaboratories/cblvictoryvi` `805f29b24c2c` — Record timed doorway hold and first moving sample on the integrated renderer
- `omonkeyz/xbox-emulator` `a618736df799` — Merge #347: tombox-xshader: size structured buffer elements from the FSSM stride (stacked on #289)
- `omonkeyz/xbox-emulator` `0e6bde2f7f3e` — Merge #348: d3d12_xs: opt-in GPU-based validation with TOMBOX_D3D12_GBV=1
- `omonkeyz/xbox-emulator` `8c481f43f1c1` — d3d12_xs: opt-in GPU-based validation with TOMBOX_D3D12_GBV=1
- `boykopovar/AnyPS5` #20519 — feat(agc): opt-in bindless image heap for sampled image tables
- `Freminet/ScarlettX` #29 — feat(kernelx): implement GpuTilePool and clean-room kernelx.dll Game OS shim

## Per-repository activity

| Repository | Commits | Open PRs | Stage | Status |
|---|---|---|---|---|
| `marvisak/VIctory` | 100 | 100+ | 6 | active |
| `fxceless/victory-fork` | 13 | 100+ | 6 | active |
| `ZSlayerHQ/victory-world-runtime` | 21 | 10 | — | renamed |
| `PastiiJay/klbf-lab` | 40 | 6 | 6 | active |
| `realblastex/flamingo` | 15 | 0 | 6 | active |
| `JMAN730/project-flamingo` | 100 | 9 | — | active |
| `vitutu1/victory-perf` | 28 | 3 | 6 | active |
| `LooperSalty/AnyXboxX` | 100 | 8 | 6 | active |
| `Zettrox/ztrx` | 1 | 100+ | 6 | active |
| `Belobug/VIctory-Belobug-AMD` | 48 | 100+ | 6 | active |
| `ka1qux/BRVIctory-ka1qux` | 99 | 3 | 6 | active |
| `DexNom/VIctory` | 100 | 5 | 6 | active |
| `uiliannn/VItoriaBr` | 77 | 6 | 6 | active |
| `ZigZagBoi1/ScarlettPort` | 19 | 0 | 6 | active |
| `myokex/victory-amd` | 3 | 11 | 6 | active |
| `pragyaangaur/VIctory-macOS` | 100 | 1 | 5 | active |
| `KomanderDG88/VIniVIdiVIci` | 37 | 0 | 5 | active |
| `luizwidmer/palmetto` | 91 | 0 | 4 | active |
| `pablogonz12/Scarlight` | 69 | 24 | 5 | active |
| `brcly/TuxBox` | 100 | 3 | 5 | active |
| `Steinkoloss/port-of-the-americas` | 1 | 0 | 5 | active |
| `xcl-dev/xcl` | 100 | 100+ | 5 | active |
| `mkr2tl/xenstride-public` | 0 | 0 | — | active |
| `voidopcode/vilocity` | 19 | 0 | 6 | active |
| `rpatty-dev/PercyScarlett` | 3 | 0 | 5 | active |
| `californiaAlmonds/OpenScarlett` | 0 | 2 | 5 | active |
| `OpenCommunityEdition/OpenScarlett` | 20 | 1 | 5 | active |
| `R0Y00/Xbox2Windows` | 100 | 100+ | 5 | active |
| `gookie-dev/g6shim` | 20 | 1 | 5 | active |
| `ThatRecompDev/five-star-recomp` | 100 | 0 | 5 | active |
| `ubiquitarianism/leonida` | 100 | 100+ | 5 | active |
| `Kajcsi/G6Recomp` | 0 | 0 | — | active |
| `OnlyMoisties/GTA6-BootTest` | 0 | 0 | 5 | active |
| `ethan00013/keylime` | 52 | 0 | 5 | active |
| `Archivist-Atlas/gta-vi-astral` | 33 | 0 | — | active |
| `sammwyy/xvalkyrie` | 25 | 3 | — | active |
| `daeken/XSXEmu` | 100 | 0 | — | active |
| `cocainebunnylaboratories/cblvictoryvi` | 51 | 0 | 6 | active |
| `mewiof/Holy-Interoperability` | 0 | 0 | 4 | active |
| `omonkeyz/xbox-emulator` | 100 | 17 | 3 | active |
| `boykopovar/AnyPS5` | 100 | 100+ | — | active |
| `Freminet/ScarlettX` | 0 | 26 | — | active |
| `jadeowaowa/scarlett-layer` | 0 | 2 | — | active |

