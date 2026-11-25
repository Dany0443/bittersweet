#!/usr/bin/env bash
# bittersweet setup — Linux/Proton-GE host preparation.
#
# Idempotent. Re-running it repairs what is missing and leaves everything
# else alone. It never looks for, copies or launches a game executable; the
# only optional launch is `wineboot` to create the prefix, and only when the
# user asks for it.
#
#   ./setup.sh                         interactive
#   ./setup.sh --non-interactive       use defaults, no prompts
#   ./setup.sh --proton /path/to/GE    use an existing Proton/GE build
#   ./setup.sh --proton auto           download the latest GE release
#   ./setup.sh --prefix ./prefix       choose the Wine prefix
#   ./setup.sh --init-prefix           create the prefix with wineboot
#   ./setup.sh --no-sysctl             never offer to change vm.max_map_count
#
# Exit codes: 0 ok, 1 runtime failure, 2 missing required tool.

set -euo pipefail

SELF_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
RUNTIME="${BITTERSWEET_RUNTIME:-$SELF_DIR/.runtime}"
PREFIX="${BITTERSWEET_PREFIX:-$RUNTIME/prefix}"
PROTON_MODE="ask"           # ask | auto | none | <path>
PROTON_PATH_IN=""
NONINTERACTIVE=0
INIT_PREFIX=0
ALLOW_SYSCTL=1
SKIP_CHECKSUM=0
FORCE=0

GE_REPO="GloriousEggroll/proton-ge-custom"
GE_API="https://api.github.com/repos/$GE_REPO/releases/latest"
MAX_MAP_MIN=1048576
MAX_MAP_REC=2147483642

say()  { printf '%s\n' "$*"; }
info() { printf '  %s\n' "$*"; }
warn() { printf 'warning: %s\n' "$*" >&2; }
die()  { printf 'error: %s\n' "$*" >&2; exit 1; }
have() { command -v "$1" >/dev/null 2>&1; }

usage() {
  sed -n '2,24p' "$0" | sed 's/^# \{0,1\}//'
  exit 0
}

while [ $# -gt 0 ]; do
  case "$1" in
    --non-interactive|-y) NONINTERACTIVE=1 ;;
    --proton) PROTON_MODE="${2:?--proton needs a value}"; shift ;;
    --proton=*) PROTON_MODE="${1#*=}" ;;
    --prefix) PREFIX="${2:?--prefix needs a value}"; shift ;;
    --prefix=*) PREFIX="${1#*=}" ;;
    --runtime) RUNTIME="${2:?--runtime needs a value}"; shift ;;
    --runtime=*) RUNTIME="${1#*=}" ;;
    --init-prefix) INIT_PREFIX=1 ;;
    --no-sysctl) ALLOW_SYSCTL=0 ;;
    --skip-checksum) SKIP_CHECKSUM=1 ;;
    --force) FORCE=1 ;;
    -h|--help) usage ;;
    *) die "unknown argument: $1 (try --help)" ;;
  esac
  shift
done

ask() { # ask <prompt> <default: y|n> -> 0 yes, 1 no
  local prompt="$1" default="${2:-n}" answer
  if [ "$NONINTERACTIVE" -eq 1 ] || [ ! -t 0 ]; then
    [ "$default" = "y" ]
    return
  fi
  read -r -p "$prompt " answer || true
  answer="${answer:-$default}"
  case "$answer" in y|Y|yes|YES) return 0 ;; *) return 1 ;; esac
}

# ---------------------------------------------------------------- tools ---
say "== 1/5  Host tools"
MISSING_REQUIRED=()
MISSING_OPTIONAL=()
check() {
  local name="$1" required="$2" hint="$3"
  if have "$name"; then
    info "ok       $name"
  elif [ "$required" = "required" ]; then
    info "MISSING  $name  ($hint)"
    MISSING_REQUIRED+=("$name")
  else
    info "optional $name  ($hint)"
    MISSING_OPTIONAL+=("$name")
  fi
}
check git        required "clone and update the repository"
check curl       required "download Proton-GE"
check jq         required "parse the GitHub release JSON"
check tar        required "extract Proton-GE"
check sha512sum  required "verify the Proton-GE checksum"
check gh         optional "GitHub release API access (falls back to curl)"
check cargo      optional "build/run the host planner (a prebuilt binary is used otherwise)"
check gcc        optional "shader tooling; clang is equally fine"
check clang      optional "shader tooling; gcc is equally fine"
check glslangValidator optional "SPIR-V shader translation tests"
check spirv-val  optional "SPIR-V validation tests"
check cmake      optional "some upstream tools build with CMake"
check ninja      optional "CMake generator"

if [ "${#MISSING_REQUIRED[@]}" -gt 0 ]; then
  warn "install these first: ${MISSING_REQUIRED[*]}"
  say  "  Arch:   sudo pacman -S --needed git curl jq tar coreutils"
  say  "  Debian: sudo apt install git curl jq tar coreutils"
  die "required tools are missing"
fi
if [ "${#MISSING_OPTIONAL[@]}" -gt 0 ]; then
  info "optional tools not found: ${MISSING_OPTIONAL[*]}"
fi

mkdir -p "$RUNTIME"

# ------------------------------------------------------------- GPU / env ---
say "== 2/5  GPU and kernel"
PLANNER="$SELF_DIR/target/release/bittersweet"
if [ ! -x "$PLANNER" ] && have cargo && [ -f "$SELF_DIR/Cargo.toml" ]; then
  info "building the host planner (release)"
  ( cd "$SELF_DIR" && cargo build -q --release -p bittersweet ) || warn "planner build failed; falling back to shell detection"
fi

if [ -x "$PLANNER" ]; then
  "$PLANNER" doctor || true
  "$PLANNER" plan --write "$RUNTIME" >/dev/null
  info "wrote $RUNTIME/bittersweet.env"
else
  warn "no planner binary; writing a minimal environment file"
  : > "$RUNTIME/bittersweet.env"
  cat >> "$RUNTIME/bittersweet.env" <<'EOF'
# Minimal fallback plan: run `cargo build --release -p bittersweet` and
# re-run setup.sh for vendor-specific tuning.
export WINEDEBUG="-all"
export DXVK_STATE_CACHE="1"
export MESA_SHADER_CACHE_MAX_SIZE="12G"
export BITTERSWEET_ENABLE_RT="0"
EOF
fi

MAX_MAP="$(cat /proc/sys/vm/max_map_count 2>/dev/null || echo 0)"
if [ "${MAX_MAP:-0}" -lt "$MAX_MAP_MIN" ] 2>/dev/null; then
  warn "vm.max_map_count=$MAX_MAP is too low for console memory-page aliasing."
  say  "  Suggested: vm.max_map_count=$MAX_MAP_REC"
  if [ "$ALLOW_SYSCTL" -eq 1 ] && ask "Write /etc/sysctl.d/99-bittersweet.conf and apply it now? [y/N]" n; then
    if have sudo; then
      printf 'vm.max_map_count=%s\n' "$MAX_MAP_REC" | sudo tee /etc/sysctl.d/99-bittersweet.conf >/dev/null
      sudo sysctl --system >/dev/null 2>&1 || warn "sysctl --system failed; reboot to apply"
      info "applied vm.max_map_count=$MAX_MAP_REC"
    else
      warn "sudo not found; apply the line above with your usual method"
    fi
  else
    info "skipped; a long session may fail with ENOMEM"
  fi
else
  info "vm.max_map_count=$MAX_MAP ok (recommended $MAX_MAP_REC for long sessions)"
fi

# --------------------------------------------------------------- Proton ---
say "== 3/5  Proton-GE"

latest_ge_json() {
  if have gh; then
    gh api "repos/$GE_REPO/releases/latest" 2>/dev/null && return 0
  fi
  curl -fsSL -H 'Accept: application/vnd.github+json' "$GE_API"
}

install_ge() {
  local json tag url sha_url tmp archive sum_file expected actual
  json="$(latest_ge_json)" || die "cannot query $GE_REPO releases"
  tag="$(printf '%s' "$json" | jq -r '.tag_name // empty')"
  url="$(printf '%s' "$json" | jq -r '.assets[] | select(.name|endswith(".tar.gz")) | .browser_download_url' | head -n1)"
  sha_url="$(printf '%s' "$json" | jq -r '.assets[] | select(.name|test("sha512")) | .browser_download_url' | head -n1)"
  [ -n "$tag" ] || die "could not read the latest GE tag"
  [ -n "$url" ] || die "no .tar.gz asset in $tag"

  local dest="$RUNTIME/proton-ge"
  local marker="$dest/.installed-tag"
  if [ -f "$marker" ] && [ "$(cat "$marker")" = "$tag" ] && [ "$FORCE" -eq 0 ]; then
    info "already installed: $tag"
    PROTONPATH="$dest/$tag"
    [ -x "$PROTONPATH/proton" ] || PROTONPATH="$(find "$dest" -maxdepth 2 -name proton -type f -printf '%h\n' | head -n1)"
    return 0
  fi

  mkdir -p "$dest"
  tmp="$(mktemp -d)"
  trap 'rm -rf "$tmp"' RETURN
  archive="$tmp/ge.tar.gz"
  info "downloading $tag"
  curl -fL --retry 3 --progress-bar -o "$archive" "$url" || die "download failed"

  if [ -n "$sha_url" ]; then
    sum_file="$tmp/ge.sha512"
    curl -fsSL -o "$sum_file" "$sha_url" || die "checksum download failed"
    expected="$(grep -Eo '[0-9a-fA-F]{128}' "$sum_file" | head -n1 | tr 'A-F' 'a-f')"
    actual="$(sha512sum "$archive" | awk '{print $1}')"
    [ -n "$expected" ] || die "checksum file did not contain a SHA-512"
    if [ "$expected" != "$actual" ]; then
      die "SHA-512 mismatch for $tag (expected $expected, got $actual)"
    fi
    info "SHA-512 verified"
  elif [ "$SKIP_CHECKSUM" -eq 1 ]; then
    warn "no checksum asset; continuing because --skip-checksum was given"
  else
    die "no SHA-512 asset published for $tag; refusing to install (use --skip-checksum to override)"
  fi

  tar -xzf "$archive" -C "$dest" || die "extraction failed"
  rm -f "$marker"
  printf '%s\n' "$tag" > "$marker"
  PROTONPATH="$dest/$tag"
  if [ ! -x "$PROTONPATH/proton" ]; then
    PROTONPATH="$(find "$dest" -maxdepth 2 -name proton -type f -printf '%h\n' | head -n1)"
  fi
  [ -n "${PROTONPATH:-}" ] && [ -x "$PROTONPATH/proton" ] || die "extracted tree has no proton script"
  info "installed $tag"
}

case "$PROTON_MODE" in
  auto) install_ge ;;
  none) info "no Proton selected (existing Wine/Proton path may be used later)" ;;
  ask)
    say "  A. download the latest Proton-GE into $RUNTIME/proton-ge (checksum verified)"
    say "  B. use an existing Proton-GE / custom Wine build"
    say "  C. skip"
    if [ "$NONINTERACTIVE" -eq 1 ] || [ ! -t 0 ]; then
      install_ge
    else
      read -r -p "  choose [A/b/c]: " choice || choice=A
      case "${choice:-A}" in
        A|a|"") install_ge ;;
        B|b|"")
          read -r -p "  path to the Proton/GE directory: " PROTON_PATH_IN || true
          [ -n "$PROTON_PATH_IN" ] || die "no path given"
          [ -x "$PROTON_PATH_IN/proton" ] || die "$PROTON_PATH_IN does not contain an executable proton"
          PROTONPATH="$PROTON_PATH_IN"
          ;;
        *) info "skipped" ;;
      esac
    fi
    ;;
  *)
    [ -x "$PROTON_MODE/proton" ] || die "$PROTON_MODE does not contain an executable proton"
    PROTONPATH="$PROTON_MODE"
    info "using $PROTONPATH"
    ;;
esac

# --------------------------------------------------------------- prefix ---
say "== 4/5  Prefix"
mkdir -p "$PREFIX" "$RUNTIME/steam-client" "$RUNTIME/shader-cache"
if [ ! -d "$PREFIX/pfx" ]; then
  info "creating prefix directory (not booting anything)"
  mkdir -p "$PREFIX/pfx"
fi

cat > "$RUNTIME/prefix.env" <<EOF
# Prefix and compatibility-tool settings. Source after bittersweet.env.
export STEAM_COMPAT_DATA_PATH="$PREFIX"
export STEAM_COMPAT_CLIENT_INSTALL_PATH="$RUNTIME/steam-client"
export WINEPREFIX="$PREFIX/pfx"
export WINEDLLOVERRIDES="d3d12,d3d11,dxgi=n,b"
export WINE_LARGE_ADDRESS_AWARE="1"
export PROTON_LOG_DIR="$RUNTIME/proton-logs"
EOF
mkdir -p "$RUNTIME/proton-logs"
info "wrote $RUNTIME/prefix.env"

if [ -n "${PROTONPATH:-}" ]; then
  printf 'export PROTONPATH="%s"\n' "$PROTONPATH" >> "$RUNTIME/prefix.env"
fi

if [ "$INIT_PREFIX" -eq 1 ] && [ -n "${PROTONPATH:-}" ]; then
  say "  creating the Wine prefix with wineboot (the game is not involved)"
  # shellcheck disable=SC1090
  . "$RUNTIME/prefix.env"
  . "$RUNTIME/bittersweet.env"
  ( cd "$PROTONPATH" && ./proton run wineboot -u ) || warn "wineboot returned non-zero"
fi

# A launcher the user can inspect, edit, and run themselves. It does not
# choose or find a game binary; the caller passes the command to run.
cat > "$RUNTIME/launch.sh" <<'LAUNCH'
#!/usr/bin/env bash
# Usage: .runtime/launch.sh <program> [args...]
# Sources the generated environment, then execs exactly what you pass.
set -euo pipefail
HERE="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck disable=SC1091
. "$HERE/bittersweet.env"
# shellcheck disable=SC1091
. "$HERE/prefix.env"
[ $# -gt 0 ] || { echo "usage: $0 <program> [args...]" >&2; exit 2; }
exec "$@"
LAUNCH
chmod +x "$RUNTIME/launch.sh"
info "wrote $RUNTIME/launch.sh"

# ---------------------------------------------------------------- report ---
say "== 5/5  Done"
say "  environment : $RUNTIME/bittersweet.env"
say "  prefix      : $RUNTIME/prefix.env  (prefix at $PREFIX)"
say "  launcher    : $RUNTIME/launch.sh"
[ -n "${PROTONPATH:-}" ] && say "  proton      : $PROTONPATH"
say ""
say "Next: review the environment, then run your own command through"
say "  $RUNTIME/launch.sh <command>"
say "setup.sh never starts the target title."
