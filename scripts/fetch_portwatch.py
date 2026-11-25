#!/usr/bin/env python3
"""Merge the PortWatch VI tracker into the bittersweet manifest.

PortWatch publishes one machine-readable file, ``data/tracker.json``, with
effort records, toolkits, unvetted candidates, news, DMCA entries and nuke
status.  This script is read-only against the site: it fetches that file,
normalises the repositories, merges them into ``UPSTREAM_REPOS.json`` and
writes a human-readable digest.

Sections and trust:

* ``efforts`` and ``tools`` are vetted by PortWatch and tracked normally.
* ``candidates`` are unreviewed; they are recorded with status
  ``candidate`` and never presented as trusted.
* ``flagged`` and ``ignored`` are never imported.  They exist to keep
  malware and re-uploads out of the manifest.

Nothing here pushes, comments or downloads a repository archive.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import sys
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SOURCE = "https://portwatch-vi.xyz/data/tracker.json"
PAGE = "https://portwatch-vi.xyz/"
RAW = ROOT / "tracking" / "portwatch.json"
DIGEST = ROOT / "tracking" / "PORTWATCH.md"
MANIFEST = ROOT / "UPSTREAM_REPOS.json"
REMOTES = ROOT / "scripts" / "remotes.txt"


def fetch(url: str) -> dict:
    req = urllib.request.Request(url, headers={"User-Agent": "bittersweet-tracking/1"})
    with urllib.request.urlopen(req, timeout=60) as response:
        return json.load(response)


def now() -> str:
    return dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat()


def repo_id(value: str) -> str:
    return value.strip().lower()


def normalise(tracker: dict) -> list[dict]:
    """One record per repository, from the vetted and candidate sections."""
    seen: dict[str, dict] = {}
    sections = (
        ("efforts", "effort", "portwatch-effort"),
        ("tools", "toolkit", "toolkit"),
        ("candidates", "candidate", "unclassified"),
    )
    for section, role, family in sections:
        for entry in tracker.get(section, []):
            repo = entry.get("repo")
            if not repo:
                continue
            key = repo_id(repo)
            if key in seen:
                # An effort that is also a candidate keeps the vetted entry.
                continue
            nuked = entry.get("nuked") or {}
            status = "nuked" if nuked else ("candidate" if role == "candidate" else "active")
            stats = entry.get("stats") or {}
            record = {
                "id": repo,
                "name": entry.get("name") or repo.rsplit("/", 1)[-1],
                "url": f"https://github.com/{repo}",
                "family": family,
                "role": role,
                "status": status,
                "base": False,
                "first_seen": now(),
                "last_sync_at": None,
                "last_sync_sha": None,
                "stage": None,
                "position": None,
                "notes": f"discovered on PortWatch VI ({section})",
                "on_page": False,
                "default_branch": entry.get("branch") or entry.get("defaultBranch"),
                "license": None,
                "archived": False,
                "pushed_at": stats.get("pushedAt") or entry.get("pushedAt"),
                "description": entry.get("summary") or entry.get("desc") or entry.get("description") or "",
                "stars": stats.get("stars") if stats.get("stars") is not None else entry.get("stars"),
                "last_checked_at": None,
                "portwatch": {
                    "section": section,
                    "tier": entry.get("tier"),
                    "gpu": entry.get("gpu") or [],
                    "approach": entry.get("approach"),
                    "target": entry.get("target"),
                    "proof": entry.get("proof"),
                    "nuked_at": nuked.get("at"),
                    "nuked_note": nuked.get("note"),
                },
            }
            seen[key] = record
    return list(seen.values())


def merge(manifest: dict, found: list[dict]) -> tuple[int, int, list[str]]:
    """Add new records and refresh cross-references. Returns (new, updated, ids)."""
    existing = {repo_id(r["id"]): r for r in manifest["repos"]}
    new = 0
    updated = 0
    new_ids: list[str] = []
    for record in found:
        key = repo_id(record["id"])
        current = existing.get(key)
        if current is None:
            manifest["repos"].append(record)
            existing[key] = record
            new += 1
            new_ids.append(record["id"])
            continue
        changed = False
        if "portwatch" not in current:
            current["portwatch"] = record["portwatch"]
            changed = True
        if record["status"] == "nuked" and current.get("status") not in ("renamed", "nuked"):
            current["status"] = "nuked"
            current["nuked_at"] = record["portwatch"].get("nuked_at")
            current["nuked_note"] = record["portwatch"].get("nuked_note")
            changed = True
        if changed:
            updated += 1
    manifest["repos"].sort(key=lambda r: r["id"].lower())
    manifest["repo_count"] = len(manifest["repos"])
    manifest["generated_at"] = now()
    return new, updated, new_ids


def write_remotes(manifest: dict) -> None:
    lines = []
    for repo in sorted(manifest["repos"], key=lambda r: r["id"].lower()):
        if repo.get("status") in ("nuked", "takedown", "offline"):
            continue
        lines.append(f"{repo['id']}\thttps://github.com/{repo['id']}.git")
    REMOTES.write_text("\n".join(lines) + "\n", encoding="utf-8")


def digest(tracker: dict, found: list[dict], new_ids: list[str], updated: int) -> str:
    updated_at = tracker.get("updated", "?")
    nuked = [r for r in found if r["status"] == "nuked"]
    efforts = [r for r in found if r["role"] == "effort"]
    tools = [r for r in found if r["role"] == "toolkit"]
    candidates = [r for r in found if r["role"] == "candidate"]
    new_set = {repo_id(i) for i in new_ids}

    out: list[str] = []
    out.append("# PortWatch VI digest")
    out.append("")
    out.append(f"Source: <{PAGE}>  ")
    out.append(f"Tracker data updated: `{updated_at}`  ")
    out.append(f"Fetched: `{now()}`")
    out.append("")
    out.append(
        "PortWatch is an independent fan tracker. Its news headlines are quoted "
        "here for the record; product and company names belong to their owners. "
        "`candidates` are unreviewed and are never presented as trusted."
    )
    out.append("")
    out.append("## Snapshot")
    out.append("")
    out.append("| Section | Repositories |")
    out.append("|---|---:|")
    out.append(f"| Efforts (vetted) | {len(efforts)} |")
    out.append(f"| Tools (vetted) | {len(tools)} |")
    out.append(f"| Candidates (unreviewed) | {len(candidates)} |")
    out.append(f"| Nuked or removed | {len(nuked)} |")
    out.append(f"| New to this manifest | {len(new_ids)} |")
    out.append(f"| Records refreshed | {updated} |")
    out.append("")
    out.append("## New to the manifest")
    out.append("")
    if new_ids:
        out.append("| Repository | Section | Status |")
        out.append("|---|---|---|")
        by_id = {repo_id(r["id"]): r for r in found}
        for ident in sorted(new_ids, key=str.lower):
            r = by_id[repo_id(ident)]
            out.append(f"| `{r['id']}` | {r['role']} | {r['status']} |")
    else:
        out.append("None: every vetted repository was already tracked.")
    out.append("")
    out.append("## Nuked, deleted or taken down")
    out.append("")
    if nuked:
        out.append("| Repository | Date | Note |")
        out.append("|---|---|---|")
        for r in sorted(nuked, key=lambda r: r["id"].lower()):
            p = r["portwatch"]
            note = (p.get("nuked_note") or "").replace("|", "\\|")
            out.append(f"| `{r['id']}` | {p.get('nuked_at') or '?'} | {note} |")
    else:
        out.append("None reported.")
    out.append("")
    out.append("## Recent changes")
    out.append("")
    for change in tracker.get("changes", [])[:20]:
        text = change.get("text", "").replace("\n", " ")
        out.append(f"- **{change.get('date', '?')}** ({change.get('kind', '?')}): {text}")
    out.append("")
    out.append("## News")
    out.append("")
    for item in tracker.get("news", [])[:20]:
        out.append(f"### {item.get('date', '?')} - {item.get('category', '?')}: {item.get('title', '')}")
        out.append("")
        dek = item.get("dek")
        if dek:
            out.append(dek)
            out.append("")
        for src in item.get("sources", [])[:4]:
            out.append(f"- [{src.get('outlet', 'source')}]({src.get('url', '')}): {src.get('title', '')}")
        out.append("")
    out.append("## DMCA")
    out.append("")
    for entry in tracker.get("dmca", []):
        tracked = ", ".join(entry.get("tracked") or []) or "none tracked"
        out.append(
            f"- {entry.get('date', '?')}: `{entry.get('path', '?')}` "
            f"({entry.get('repoCount', '?')} repositories; {tracked}) "
            f"[notice]({entry.get('url', '')})"
        )
    out.append("")
    out.append("## Toolkit")
    out.append("")
    for tool in tools:
        out.append(f"- `{tool['id']}` - {tool['description']}")
    out.append("")
    return "\n".join(out)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--offline", type=Path, help="use a saved tracker.json")
    ap.add_argument("--dry-run", action="store_true", help="do not write the manifest")
    args = ap.parse_args()

    tracker = json.loads(args.offline.read_text(encoding="utf-8")) if args.offline else fetch(SOURCE)
    found = normalise(tracker)
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    new, updated, new_ids = merge(manifest, found)

    RAW.parent.mkdir(parents=True, exist_ok=True)
    # Keep the parts that are auditable and useful. The full feed also carries
    # Google News redirect links and a 500-entry ignore list; neither is
    # needed, and opaque long links have no place in this repository.
    snapshot = {k: v for k, v in tracker.items() if k not in ("headlines", "ignored")}
    snapshot["ignored_count"] = len(tracker.get("ignored", []))
    RAW.write_text(json.dumps(snapshot, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    DIGEST.write_text(digest(tracker, found, new_ids, updated), encoding="utf-8")
    if not args.dry_run:
        MANIFEST.write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
        write_remotes(manifest)

    print(
        f"portwatch: {len(found)} repositories, {new} new, {updated} refreshed, "
        f"{sum(1 for r in found if r['status'] == 'nuked')} nuked"
    )
    if new_ids:
        print("new: " + ", ".join(sorted(new_ids, key=str.lower)))
    return 0


if __name__ == "__main__":
    sys.exit(main())
