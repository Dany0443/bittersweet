#!/usr/bin/env python3
"""Seed UPSTREAM_REPOS.json with the set named in the project brief.

This is the baseline *before* an update check. Running
``sync_upstream.py`` afterwards is what discovers repositories that the
standings page lists but this baseline does not. Keeping the seed in a
script makes the discovery step reproducible and auditable.

It refuses to overwrite an existing manifest unless ``--force`` is given.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# (repository, display name, family, role) — exactly the projects named in
# the brief, before the page is consulted.
BRIEF = [
    ("marvisak/VIctory", "VIctory", "victory-ecosystem", "upstream-base"),
    ("fxceless/victory-fork", "victory-fork", "victory-ecosystem", "primary-active-fork"),
    ("ZSlayerHQ/victory-world-runtime", "victory-world-runtime", "victory-ecosystem", "fix-aggregator"),
    ("PastiiJay/klbf-lab", "klbf-lab", "victory-ecosystem", "base"),
    ("realblastex/flamingo", "Flamingo", "victory-ecosystem", "fork"),
    ("JMAN730/project-flamingo", "project-flamingo", "victory-ecosystem", "fork"),
    ("vitutu1/victory-perf", "victory-perf", "victory-ecosystem", "fork"),
    ("LooperSalty/AnyXboxX", "AnyXboxX", "victory-ecosystem", "integration-bundle"),
    ("Zettrox/ztrx", "ztrx", "victory-ecosystem", "fork"),
    ("Belobug/VIctory-Belobug-AMD", "VIctory-Belobug-AMD", "victory-ecosystem", "amd-fork"),
    ("ka1qux/BRVIctory-ka1qux", "BRVIctory-ka1qux", "victory-ecosystem", "amd-fork"),
    ("DexNom/VIctory", "DexNom Runtime", "victory-ecosystem", "rt-experiment"),
    ("uiliannn/VItoriaBr", "VItoriaBr", "victory-ecosystem", "fork"),
    ("ZigZagBoi1/ScarlettPort", "ScarlettPort", "victory-ecosystem", "fork"),
    ("myokex/victory-amd", "victory-amd", "victory-ecosystem", "amd-fork"),
    ("pragyaangaur/VIctory-macOS", "VIctory-macOS", "victory-ecosystem", "mac-port"),
    ("KomanderDG88/VIniVIdiVIci", "VIniVIdiVIci", "victory-ecosystem", "nvidia-fork"),
    ("luizwidmer/palmetto", "palmetto", "victory-ecosystem", "clean-room-runtime"),
    ("pablogonz12/Scarlight", "Scarlight", "linux-runtime", "linux-port"),
    ("brcly/TuxBox", "TuxBox", "linux-runtime", "linux-port"),
    ("Steinkoloss/port-of-the-americas", "Port of the Americas", "linux-runtime", "linux-port"),
    ("xcl-dev/xcl", "XCL", "xcl-family", "independent-layer"),
    ("mkr2tl/xenstride-public", "Xenstride", "xcl-family", "xcl-derivative"),
    ("voidopcode/vilocity", "vilocity", "compatibility-layer", "independent-layer"),
    ("rpatty-dev/PercyScarlett", "PercyScarlett", "compatibility-layer", "independent-layer"),
    ("californiaAlmonds/OpenScarlett", "OpenScarlett", "compatibility-layer", "independent-layer"),
    ("OpenCommunityEdition/OpenScarlett", "OpenScarlett (Community Edition)", "compatibility-layer", "independent-layer"),
    ("R0Y00/Xbox2Windows", "Xbox2Windows", "compatibility-layer", "translation-toolchain"),
    ("gookie-dev/g6shim", "g6shim", "compatibility-layer", "system-shim"),
    ("ThatRecompDev/five-star-recomp", "five-star-recomp", "recompilation-toolchain", "recompiler"),
    ("ubiquitarianism/leonida", "Leonida Continued", "recompilation-toolchain", "recompiler"),
    ("Kajcsi/G6Recomp", "G6Recomp", "recompilation-toolchain", "recompiler"),
    ("OnlyMoisties/GTA6-BootTest", "GTA6-BootTest", "recompilation-toolchain", "boot-harness"),
    ("ethan00013/keylime", "keylime", "recompilation-toolchain", "native-runtime"),
    ("Archivist-Atlas/gta-vi-astral", "gta-vi-astral", "recompilation-toolchain", "native-runtime"),
    ("sammwyy/xvalkyrie", "XValkyrie", "compatibility-layer", "independent-layer"),
    ("daeken/XSXEmu", "XSXEmu", "emulator", "emulator"),
]


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default=str(ROOT / "UPSTREAM_REPOS.json"))
    ap.add_argument("--force", action="store_true")
    args = ap.parse_args()

    out = Path(args.out)
    if out.exists() and not args.force:
        raise SystemExit(f"{out} exists; pass --force to overwrite")
    now = dt.datetime.now(dt.timezone.utc).isoformat(timespec="seconds")
    repos = [
        {
            "id": rid,
            "name": name,
            "url": f"https://github.com/{rid}",
            "family": family,
            "role": role,
            "status": "tracked",
            "base": rid == "PastiiJay/klbf-lab",
            "first_seen": now,
            "last_sync_at": None,
            "last_sync_sha": None,
            "stage": None,
            "position": None,
            "notes": "named in the project brief",
        }
        for rid, name, family, role in BRIEF
    ]
    doc = {
        "schema": 1,
        "source": "https://vuyp.github.io/gta6-pc-race/#standings",
        "generated_at": now,
        "repo_count": len(repos),
        "repos": repos,
    }
    out.write_text(json.dumps(doc, indent=2) + "\n")
    print(f"seeded {len(repos)} repositories into {out}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
