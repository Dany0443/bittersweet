# Linux status and roadmap

This page is the honest state of the Linux/Proton effort. It is updated
whenever a check changes what we know; the machine-readable record is
`tracking/sync-report.json`, the source digest is `tracking/PORTWATCH.md`.

## What works today

| Area | State |
|---|---|
| Workspace build | `cargo build --workspace` and `--release` succeed on Linux |
| Tests | `cargo test --workspace` passes; Windows-only self-image tests are gated |
| Host probe | `bittersweet doctor` reads sysfs, procfs and `/etc/os-release` only |
| Host plan | `bittersweet plan` writes cache paths, VRAM budget, wave policy, vendor flags |
| Setup | `setup.sh` prepares Proton-GE, prefix and `vm.max_map_count`; idempotent |
| Windows runtime | Compiles on Linux after `cfg` gates; **not functional under Proton** |

The last row is the important one. A green build means the code is
portable, not that the title runs.

## What is missing

1. **A Linux graphics backend.** The runtime still creates host D3D12
   objects through Windows APIs. Under Proton those calls must reach
   vkd3d-proton; on native Linux they must reach Vulkan. The gates only
   keep the Windows path from breaking the Linux build.
2. **Wine memory semantics.** Title memory is not always mapped in place
   under Wine. Upstream builds copy it, and one class of copies must not be
   sparse buffers or the GPU resets when the world loads.
3. **Pipeline-library behaviour without host support.** vkd3d-proton can
   refuse the empty host library blob, which is why the early-pipeline list
   must still be written when no library exists.
4. **Shader-conversion recursion limits.** vkd3d-proton converts DXIL on
   the calling thread; large traversal programs need one latch per loop and
   a larger worker stack.
5. **A real run.** No one has run this tree against a user-supplied copy
   under Proton. That is deliberate until the backend exists.

## Evidence from the ecosystem

The 2026-11-25 check found several Linux-specific results worth recording.
These are upstream claims, not our measurements:

- **TuxBox** reports reaching the target title's story intro on Linux with
  stock Proton Experimental, and calls it not playable yet. It is the most
  advanced Linux-first record in the tracked set.
- **Scarlight** now presents itself as a Linux + Mesa port, with notes on
  RADV's shader cache outgrowing its default, zero-copy title memory, and a
  Proton-first architecture decision.
- **VIctory** and its forks still lead on Windows; the Linux work is
  concentrated in Scarlight, TuxBox and the Wine-specific pull requests.
- **vkd3d-proton** needs one back edge per loop header. Builds that emit
  several latches stall or hang in pipeline workers.

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
| Root signature made beside Wine-compiled HLSL | victory-fork #1011 | Not ported; Windows-only |
| Compile runtime HLSL with DXC when the system compiler refuses | victory-fork #20 | Not ported; Windows-only |
| Depth twin's root signature when the compiler drops it | VIctory #1872 | Not ported; Windows-only |
| Title memory copied, not in place, under Wine | Belobug #53 | Not ported; needs the Linux backend first |
| Non-sparse copies under Wine | Belobug #69 | Not ported; same |
| Empty host library from a null blob | Belobug #124 | Not ported; same |

The Windows-only items are deliberately queued behind a Windows build.
Porting them blind would add code we cannot compile or test on this host.

## Risk register

| Risk | Current handling |
|---|---|
| The leaked executable and DMCA pressure | No game files, no executable, no download links; only public repositories are tracked |
| Malware and lookalike repositories | PortWatch's `flagged` and `ignored` lists are never imported; candidates are untrusted |
| Lost upstream work | Nuked repositories stay in the manifest with a date and note, so the loss is visible |
| Bit-rot in the base | The base snapshot is pinned; every port carries its upstream PR number |
| False confidence from a green build | This page and the README state that the runtime is not functional on Linux |

## Next milestones

1. Define the Linux device boundary: one trait for device, queue, list,
   resource and descriptor operations, with a Windows implementation and a
   Vulkan implementation behind it.
2. Move title-memory mapping behind that boundary, including the copied
   and non-sparse cases.
3. Make the pipeline cache work with vkd3d-proton's refusal path.
4. Run the portable shader translator over a user-supplied shader pack and
   compare counts with the upstream reports.
5. Only then attempt a Proton run, with a user-supplied copy, and publish
   the log with the same care as the upstream test reports.
