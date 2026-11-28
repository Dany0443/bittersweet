# bittersweet

An experimental Linux/Proton compatibility layer for Xbox Series X|S titles,
based on [klbf-lab](https://github.com/PastiiJay/klbf-lab). It runs the title's
CPU code and translates console graphics and shaders to D3D12, then Vulkan
through vkd3d-proton. No game files are included.

**The intro and main menu work. Stable world gameplay is still being developed.**

## Tested hardware and current results

The main test PC uses a **Radeon RX 7600 8 GB**, **Ryzen 5 7600X**, about
**14.5 GiB usable RAM** and **7.2 GiB swap**, running openSUSE Tumbleweed
with RADV and GE-Proton11-14. Results below combine user reports and local
profiling from 2026-11-27; they are not controlled benchmarks.

| Stage | RX 7600 result |
|---|---|
| Intro video | Smooth playback confirmed after the video-upload fix. |
| Main menu | Appears correctly; startup no longer receives automatic Enter input. |
| Lucia/Jason world-loading screen | About 30 fps reported. |
| Interactive 3D world | About 3 to 4 fps reported; profiling under memory pressure also recorded lower rates. Keyboard input works. |
| Textures and objects | Rendering works partially. Some character materials and objects remain black, missing or incorrect. |
| Stability | World loading can exhaust RAM and swap. Sustained playable mission progress is unverified. |

Earlier RX 7600 reports showed roughly 40% GPU use and 3 GB VRAM occupied.
Profiling found substantial CPU copying and shared-memory costs, so those
numbers do not mean the GPU can simply be turned up to reach 20 to 30 fps.
The next target is a sustained 10 fps in the interactive 3D world.
Some measurements used a private Wine mapping optimization, so stock Proton
may perform differently.

| GPU | Architecture | Observed Behavior | Known Issues |
|---|---|---|---|
| Radeon RX 5700 | RDNA 1 (gfx10.1) | Advances through the first 3D cutscene using the compute mesh fallback on RADV and GE-Proton11-12 with VKD3D-Proton `71cae09`. The latest 16 GB test reached its planned timeout. | Late cutscene frames often take two to three seconds, with longer stalls and dark or missing materials. Playable frame rates remain unverified. |

## Build and run on Linux

Install Git, Rust through rustup, Clang/LLVM with LLD, and a working Vulkan
driver. AMD testing uses RADV. Cross-compilation uses
[cargo-xwin](https://github.com/rust-cross/cargo-xwin).
Keep your game files outside this repository.

```bash
git clone https://github.com/Dany0443/bittersweet.git
cd bittersweet
cargo install --locked cargo-xwin
cargo xwin build --target x86_64-pc-windows-msvc \
  -p victory --release --features diagnostics --locked -j2
```

Prepare Proton and the host environment. Replace the Proton path with your
installed GE-Proton directory; this creates the prefix outside the checkout.

```bash
./setup.sh --proton /path/to/GE-Proton \
  --prefix "$HOME/.local/share/bittersweet/prefix" --init-prefix
source .runtime/bittersweet.env
source .runtime/prefix.env
```

Set your game directory using Wine's Windows path format.
`Z:\games\GTA6` below means `/games/GTA6` on Linux; replace it with your path.
These are conservative test settings, not a fix for world-loading memory use.
The texture limit caps background uploads in addition to the driver's VRAM
budget. The commit reserve leaves headroom in physical RAM and Windows commit.

```bash
export VICTORY_GAME_DIR='Z:\games\GTA6'
export SteamAppId=0 STEAM_COMPAT_APP_ID=0
export VICTORY_FFX_SIMPLE=1 VICTORY_DIRECT_COPY=1
export VICTORY_PIPELINE_WORKERS=1 VICTORY_SHADER_LOOP_LIMIT=1024
export VICTORY_WINDOW_KB=64 VICTORY_DSTORAGE_WORKERS=4
export VICTORY_TEXTURE_BUDGET_MB=2048 VICTORY_COMMIT_RESERVE_MB=2048
export VKD3D_CONFIG=no_upload_hvv
unset VICTORY_KEYS

"$PROTONPATH/proton" run \
  "$PWD/target/x86_64-pc-windows-msvc/release/victory.exe" \
  run "$VICTORY_GAME_DIR\\game_scarlett_master.exe" --console series-s --execute
```

Select your player manually and use the menu to start. Leave `VICTORY_KEYS`
unset: it injects timed input and can advance prompts or menu choices.
`VICTORY_FFX_SIMPLE=1` uses simple scaling while the console upscaler remains
incomplete. First runs can stutter during shader compilation. Caches and
saves default to the external Wine prefix; see
[victory.example.toml](victory.example.toml) to configure their locations.

After rebuilding, you can warm pipelines observed in earlier runs before
loading the world. Use the same Proton prefix and environment as above.

```bash
"$PROTONPATH/proton" run \
  "$PWD/target/x86_64-pc-windows-msvc/release/victory.exe" \
  translate --pipelines --cache --early-only --jobs 1 \
  "$VICTORY_GAME_DIR\\shaders.rpf"
```

Leave out `--early-only` to attempt every pipeline in the archive. The
first full compile can take longer, and the game can still create pipelines
that only appear at runtime.

`VICTORY_PRELOAD_TEXTURES=1` lets graphics frames stage small batches of
queued textures the game has already described, including while a loading
screen is visible. The runtime uses the existing memory limits. Geometry
and texture data must still arrive through the game's streaming requests.

To update, run `git pull --ff-only` and repeat the build command. For host
checks without starting the title, run `cargo run -p bittersweet -- doctor`.

Experimental: `VICTORY_WINDOW_REUSE=1` reuses overlapping buffer copies.
It stays off by default while memory use and rendering are compared.

For pipeline diagnostics, `VICTORY_NATIVE_COMPUTE=1` enables the experimental
native compute creation method. Leave it unset for gameplay. The RX 7600
test presented a black screen with it enabled.

To test without ray hits, set `VICTORY_RAY_MISS_ONLY=1` before starting
Proton. The structure builder stays off, and translated ray queries return
misses without removing the surrounding draws. Lighting, shadows, and
reflections can look different. Unset the variable to restore normal
shader translation.

With the ray-tracing structure builder off, `VICTORY_AMBIENT=1` tests a
constant indirect light after the screen-probe resolve. It is meant for
dark scenes and can flatten lighting.

`VICTORY_GUARD_FAST=1` tests a shorter write-watch path and skips advance
kernel queries for mapped buffers already protected by the runtime.

The renderer shares completed state-transition command lists. Set
`VICTORY_PRELUDE_POOL=0` to compare against per-list allocation.

`VICTORY_GPU_UPLOAD=1` tests VRAM-backed texture staging and command-list
scratch on GPUs that support upload heaps in VRAM. The runtime falls back
to system-memory uploads when the local GPU budget is tight.

On Proton, `VICTORY_PAGE_FILE_DIR='Z:\path\to\private-ssd-directory'` tests
temporary disk backing for console physical pages. Use an existing private
directory outside the repository with enough space for the console page
pools, which can occupy several GiB. The OS can reclaim inactive pages
through the filesystem, at the cost of extra disk I/O.

## Details

[Runtime status](docs/STATUS.md) · [Architecture](docs/architecture.md) ·
[Upstream Windows guide](docs/GUIDE.md) · [Credits](CREDITS.md) ·
[Asset policy](docs/asset-policy.md)

GPL-3.0-or-later; see [LICENSE](LICENSE). This independent project is not
affiliated with the platform holder or publisher. Supply your own legally
obtained title files. No retail binaries, game assets or console SDKs ship here.
