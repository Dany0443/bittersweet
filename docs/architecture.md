# Architecture

In diagnostics builds, selected character draws can copy bounded material
records from their bound GPU buffer. Completion callbacks run after the
submission fence and cancel on list reset. The next texture-queue pass
reports descriptor residency without changing upload scheduling.

```
 user's files (outside repo)             VIctory
 ───────────────────────────             ─────────────────────────────────────────
 game assets ──────────────────────────► victory-pipeline + victory-game
                                         (victory-formats, victory-shader)
                                                    │
                                                    ▼
                                         cache_dir: converted data
                                                    │
 game executable ─► victory-loader ────► title code (native x86-64)
                    prepare, map, bind      │ imports
                                            ▼
                                         victory-hle ── host Windows (shared APIs)
                                            │ D3D12.X
                                            ▼
                                         victory-gpu ─► host D3D12
                                            │ shaders at pipeline creation
                                            ▼
                                         victory-shader ◄─► cache_dir
```

Shaders are translated when the title creates a pipeline, from the bytes it
passes, wherever it loaded them from (executable, archive, memory).
Converting them ahead of time from files is only a warm-up.

Title-memory window reuse is opt-in through `VICTORY_WINDOW_REUSE=1`.
The window cache owns each GPU copy once and keeps requested-range aliases
in a separate index. Retirement and in-place invalidation remove stale
aliases under the same cache lock; descriptors retain the buffer identity
selected for their range.

## Crates

| Crate | Role |
|-------|------|
| `victory-core` | config, asset-policy guard, logging |
| `victory-pe` | read-only PE32+ and import-library analysis |
| `victory-loader` | prepare, map, bind and start the title image |
| `victory-hle` | console API implementations, host pass-through policy |
| `victory-gpu` | D3D12.X objects, command lists and GPU memory → host D3D12 |
| `victory-shader` | Xbox shader containers → host shaders |
| `victory-formats` | the GPU's texture layouts (sizes, tiling, detiling), audio codecs; planned: BCPack |
| `victory-pipeline` | converter trait, content-addressed cache |
| `victory-game` | the title-specific formats, build identities, hooks |
| `bittersweet-host` | Linux host policy: probe, environment plan, GPU/wave/VRAM rules, cache and log paths |
| `apps/victory` | command-line front end |
| `apps/bittersweet` | Linux host CLI (`doctor`, `plan`, `check`, `pipeline`) |
| `xtask` | asset-policy check, git hooks |
| `tests/fixtures/hle-probe` | a stand-in "title" (our own code) for loader tests |

Everything except `victory-game` is platform-level and can be tested with
any Xbox Series title.

## Host layer and Linux builds

`bittersweet-host` is dependency-free (std only) and is the only crate that
reads the Linux host. It turns a probe into an ordered environment:

```
/sys, /proc, /etc/os-release
        │
        ▼
   probe.rs ──► plan.rs ──► bittersweet.env
        │         │
        │         ├─ gpu.rs     vendor family, wave width, mesh support
        │         ├─ vram.rs    texture budget and mip bias
        │         ├─ pipeline.rs early-key cap
        │         ├─ warm.rs    warm-up/pre-build conflict
        │         ├─ paths.rs   cache directories (empty env vars ignored)
        │         ├─ logging.rs log filter (empty env vars ignored)
        │         └─ kernel.rs  vm.max_map_count advice
        ▼
   apps/bittersweet: doctor, plan, check, pipeline
```

The probe prefers the DRM card with the most reported VRAM, with stable
card-name order for ties or unknown values. This chooses the host policy
profile; it does not bind a Vulkan device or override Proton GPU selection.
The synthetic probe tests include an integrated GPU beside a discrete GPU.

The runtime remains Windows-target code. Native Linux workspace builds
exercise its portable paths through `cfg(windows)` gates. For execution,
`cargo-xwin` builds the Windows runtime and Proton routes its D3D12 calls
through vkd3d-proton. This path has locally rendered the account/player
prompt; a native Vulkan backend is not implemented. `Cargo.toml` keeps
portable crates as `default-members`.

Host compositor and depth helpers serialize their root signatures directly.
They do not require Wine's HLSL compiler to embed root-signature attributes.
The card-memory comparison helper also uses structured UAV accesses so
Wine can compile its existing hash and change-detection algorithm.
Retail presentation records use a 72-byte stride, including all seven option
words, while retaining per-plane rectangles and layer composition.


## Import resolution

For every `dll!function` the title imports, in order:

1. our implementation in `victory-hle::modules` (always wins);
2. the host's own function, if `HostPolicy` allows that DLL;
3. an export of a DLL the title ships beside its executable, mapped like
   the executable (only DLLs neither of the above serves);
4. a generated stub that names the import when called (`--unimplemented
   trap` breaks into the debugger and aborts; `zero` logs once and returns 0).

`LoadLibrary*` of a console-only DLL we implement (e.g. `xgameruntime.dll`)
returns a pseudo handle; `GetProcAddress` on it finds our functions.
`LoadLibrary*` of a title DLL returns its mapped image, mapping it first if
nothing imports it. By name, as by import, a title gets only host DLLs
`HostPolicy` passes through, and `GetProcAddress` on one returns our
override where import binding would.
`QueryApiImpl` hands out our runtime objects by provider and interface id
(`victory-hle::modules::xgameruntime` lists the areas); online areas answer
as an offline console. Game saves are files under `VICTORY_DATA_DIR`.

Title storage is a drive on the console, and titles build paths on that.
`victory-hle::modules::files` binds every path-taking title import
(kernel32 and the C/C++ runtime) and rewrites paths on a virtual drive to
the host folder behind it; persistent local storage is one such drive over
a folder under `VICTORY_DATA_DIR`. Other paths reach the host unchanged.

An exe's code reaches its static TLS as slot 0; mapped as a DLL it gets
another slot, so the loader redirects those loads (`victory-loader::tls`).
`victory run --execute` installs a crash reporter that names faults as
`module+rva` and unwinds the faulting thread's stack
(`victory-loader::crash`).

`victory census` shows this classification without running anything (it
does not map title DLLs; `victory run` without `--execute` does).

## Shaders

`victory_shader::ir::build` turns a native program into SSA, one
instruction per machine instruction. `victory_shader::structure` then
removes EXEC: lane masks become per-lane booleans, wave-wide mask tests
become per-lane branches, and vector instructions keep a guard only where
a lane can reach them with its EXEC bit clear. `victory_shader::resource`
traces each descriptor and scalar memory address to the root parameter it
comes from (table, heap, root descriptor or constant). A descriptor a
waterfall loop reads from the first lane (`v_readfirstlane_b32`) is traced
to the per-lane load or choice that gives each lane its own (a mask's
`v_cndmask_b32`, or an if/else whose arms each set it: traced on the paths
the lanes take, from structurisation), and accessed with a non-uniform heap
index in one pass. `victory translate`
runs every shader it finds (containers, SGD5 files) through these and
lists what they cannot handle yet.

The portable lowering also handles DPP fetch-inactive semantics and inactive
swizzle sources using the structurised EXEC guard. Pixel helper lanes take
their own first-lane value so descriptor waterfalls can finish. Zero-stride
indexed buffer operations address raw bytes at the instruction's offset.
These paths have synthetic Linux tests; host GPU execution is not verified.

Texture staging uses `victory-gpu::content` for content checks. Inputs below
1 MiB stay serial; larger inputs use BLAKE3's Rayon tree hashing with the same
checksum. The helper's tests are portable; staging remains Windows-target
code. This does not change the shader or pipeline cache hash algorithms.

A uniform branch may select either of two static samplers. Lowering samples
both bindings with the original derivatives and selects the result on the
matching predecessor edge. Static sampler indices remain in the existing
shader binding layout.

Cube samples with explicit face gradients use the base-2 footprint LOD and
apply the instruction's minimum LOD before `SampleLevel`. Synthetic cube
mip fixtures compile on Linux; numerical D3D12 readback tests are included
for Windows, but have not been run here.

DirectStorage partial batches remain queued until the title submits them,
after preparing destination memory. A full queue forces submission only
when no previously submitted reads can make room. The diagnostic override
`VICTORY_DSTORAGE_HALF=1` restores early submission for comparison.

Pipeline compilation retains its existing CPU-based worker default and
256 MiB worker stacks. `VICTORY_PIPELINE_WORKERS` accepts an explicit count
bounded to 1 through 64, allowing a single compiler during memory-constrained
validation without changing shader output or compilation requirements.

## Prebuild resumption identity

The GPU shader cache exposes its translation fingerprint to the offline
prebuild command. Completed pipeline lists record that fingerprint beside
the library size. Resumption requires the current fingerprint to match
and the library to retain at least its recorded size. Legacy lists without
a fingerprint are rebuilt once. This keeps a different loop limit or
translation setting from reusing the previous run's completion records.
