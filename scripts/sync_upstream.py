#!/usr/bin/env python3
"""Sync the bittersweet tracking manifest against the public standings page.

What it does, in order:

1. scrape https://vuyp.github.io/gta6-pc-race/#standings (see
   ``scrape_standings.py``);
2. diff the scraped repository links against ``UPSTREAM_REPOS.json`` and
   register any repository that is new to the manifest;
3. flag repositories that disappeared (offline / renamed / takedown);
4. ask GitHub for every tracked repository's default branch, licence, new
   commits in the sync window, and open pull requests;
5. sort every commit and PR into the five triage buckets used by the
   project, and write a machine report plus a Markdown changelog.

It is read-only against upstream: no pushes, no writes outside this
checkout.  It never looks for, opens or executes a game binary.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import os
import re
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
sys.path.insert(0, str(HERE))
import scrape_standings  # noqa: E402
import fetch_portwatch  # noqa: E402

BUCKETS = [
    "critical-linux",
    "frame-time",
    "stability",
    "rendering",
    "experimental",
]

# Weighted phrases.  A phrase scores its weight once per occurrence.
RULES: dict[str, list[tuple[str, int]]] = {
    "critical-linux": [
        ("linux", 6), ("proton", 6), ("wine", 5), ("mesa", 4), ("radv", 4),
        ("vkd3d", 5), ("dxvk", 4), ("posix", 4), ("mmap", 5),
        ("memory mapping", 5), ("memory-map", 5), ("xdg", 4), ("home dir", 3),
        ("cache dir", 3), ("deadlock", 5), ("futex", 4), ("pthread", 3),
        ("steam", 3), ("flatpak", 2), ("wayland", 3), ("pipewire", 2),
        ("case-sensitive", 3), ("filesystem", 2), ("/proc", 2), ("sysfs", 2),
    ],
    "frame-time": [
        ("fps", 5), ("frame rate", 5), ("frame-rate", 5), ("frame time", 5),
        ("frame-time", 5), ("copy", 2), ("upload", 3), ("batch", 4),
        ("coalesc", 4), ("hash", 3), ("indirect", 3), ("descriptor", 2),
        ("pipeline cache", 4), ("pipeline", 2), ("cpu", 3), ("instruction", 3),
        ("mip", 3), ("vram", 4), ("texture budget", 5), ("redundant", 4),
        ("dispatch", 2), ("queue", 2), ("submit", 2), ("warm", 2),
        ("prebuild", 3), ("pre-build", 3), ("faster", 4), ("speed", 4),
        ("overhead", 4), ("stall", 4), ("latency", 3),
    ],
    "stability": [
        ("guard", 4), ("out-of-bounds", 6), ("out of bounds", 6), ("oob", 5),
        ("overflow", 5), ("underflow", 4), ("fallback", 4), ("wave32", 4),
        ("wave64", 4), ("64-lane", 5), ("32-lane", 5), ("watchdog", 5),
        ("assert", 4), ("timeout", 5), ("device loss", 6), ("device-lost", 6),
        ("device removed", 6), ("clamp", 3), ("quarantine", 4), ("fence", 3),
        ("leak", 5), ("stale", 3), ("race", 4), ("use-after-free", 6),
        ("null", 2), ("crash", 4), ("xid", 5), ("hang", 5),
    ],
    "rendering": [
        ("shadow", 5), ("water", 5), ("lighting", 5), ("light", 2),
        ("ray tracing", 3), ("ray-tracing", 3), ("foliage", 5), ("hud", 5),
        ("ui", 3), ("text", 3), ("audio", 4), ("input", 3), ("save", 4),
        ("texture", 3), ("decod", 4), ("mesh", 3), ("bloom", 4), ("sky", 4),
        ("terrain", 4), ("skin", 4), ("vehicle", 4), ("translucen", 4),
        ("tone", 2), ("geometry", 3), ("shader", 2), ("rt", 2),
    ],
    "experimental": [
        ("experimental", 8), ("opt-in", 4), ("off by default", 4),
        ("proprietary", 8), ("windows sdk", 8), ("dll", 4),
        ("ray tracing", 4), ("ray-tracing", 4), ("unstable", 6),
        ("not merged", 4), ("stacked on", 3), ("wip", 4), ("hold", 6),
    ],
}

HOLD_MARKERS = ("proprietary", "windows sdk", "microsoft", "dll", "experimental")


def now_utc() -> dt.datetime:
    return dt.datetime.now(dt.timezone.utc)


def iso(t: dt.datetime) -> str:
    return t.astimezone(dt.timezone.utc).isoformat(timespec="seconds").replace("+00:00", "Z")


def gh_json(args: list[str]):
    """Call the GitHub CLI and parse JSON; None on failure."""
    try:
        out = subprocess.run(
            ["gh", "api", *args],
            capture_output=True, text=True, timeout=60, check=True,
        ).stdout
        return json.loads(out) if out.strip() else None
    except (OSError, subprocess.CalledProcessError, subprocess.TimeoutExpired, json.JSONDecodeError) as exc:
        msg = getattr(exc, "stderr", "") or str(exc)
        print(f"  ! gh api {' '.join(args[:1])}: {msg.strip().splitlines()[0][:160]}", file=sys.stderr)
        return None


def gh_items(endpoint: str, page_size: int = 100):
    """Read every page; a failed page must never look like an empty feed."""
    items = []
    page = 1
    while True:
        batch = gh_json([f"{endpoint}&per_page={page_size}&page={page}"])
        if not isinstance(batch, list):
            return None
        items.extend(batch)
        if len(batch) < page_size:
            return items
        page += 1


def triage(text: str) -> tuple[str, dict[str, int]]:
    low = text.lower()
    scores = {b: 0 for b in BUCKETS}
    for bucket, rules in RULES.items():
        for phrase, weight in rules:
            # Anchor at a word start so "rt" does not match "start" and
            # "ui" does not match "build"; suffixes are still allowed so
            # stems such as "coalesc" and "decod" work.
            n = len(re.findall(r"\b" + re.escape(phrase), low))
            if n:
                scores[bucket] += weight * min(n, 3)
    # Experimental is a deliberate override: a change that needs a
    # proprietary Windows component, or says it is off by default, is held
    # even if it also promises speed.
    if any(m in low for m in ("proprietary", "windows sdk", "not merged", "experimental")):
        scores["experimental"] += 10
    best = max(scores, key=lambda b: scores[b])
    return (best if scores[best] > 0 else "unclassified"), scores


def classify_family(repo_id: str, name: str, note: str) -> str:
    low = f"{repo_id} {name} {note}".lower()
    if "xcl" in low or "xenstride" in low:
        return "xcl-family"
    if any(k in low for k in ("recomp", "leonida", "boottest", "boot-test", "keylime", "astral")):
        return "recompilation-toolchain"
    if "emu" in low or "ps5" in low:
        return "emulator"
    if any(k in low for k in ("scarlight", "tuxbox", "americas", "linux", "proton")):
        return "linux-runtime"
    if any(k in low for k in ("scarlett", "shim", "xbox2windows", "vilocity", "percy")):
        return "compatibility-layer"
    if any(k in low for k in ("victory", "victoria", "flamingo", "klbf", "anyxbox", "ztrx",
                              "dexnom", "palmetto", "brvictory")):
        return "victory-ecosystem"
    return "unclassified"


def load_manifest(path: Path) -> dict:
    if path.exists():
        return json.loads(path.read_text())
    return {"schema": 1, "source": scrape_standings.SOURCE, "repos": []}


def discover(manifest: dict, snapshot: dict, stamp: str) -> list[dict]:
    known = {r["id"].lower() for r in manifest["repos"]}
    # GitHub repository names are case-insensitive; compare on the lower-case
    # slug so a case-only rename is not reported as a new project.
    found = []
    entries = list(snapshot["projects"]) + list(snapshot["watched"]) + list(snapshot["no_published_work"])
    for p in entries:
        rid = p.get("repo")
        if not rid or rid.lower() in known:
            continue
        known.add(rid.lower())
        rec = {
            "id": rid,
            "name": p.get("name") or rid,
            "url": p.get("url") or f"https://github.com/{rid}",
            "family": classify_family(rid, p.get("name", ""), p.get("note", "")),
            "role": "discovered",
            "status": "new",
            "base": False,
            "first_seen": stamp,
            "discovered_from": "https://vuyp.github.io/gta6-pc-race/#standings",
            "last_sync_at": None,
            "last_sync_sha": None,
            "stage": p.get("stage"),
            "position": p.get("position"),
            "notes": "newly listed on the standings page",
        }
        manifest["repos"].append(rec)
        found.append(rec)
    return found


def collect(repo: dict, since: dt.datetime, pr_limit: int) -> dict:
    rid = repo["id"]
    meta = gh_json([f"repos/{rid}"])
    result = {"id": rid, "commits": [], "prs": [], "buckets": {b: 0 for b in BUCKETS},
              "unclassified": 0, "hold": [], "notes": [], "collection_complete": False}
    if not meta:
        # A previously nuked repository stays nuked when GitHub still answers
        # 404; anything else is an outage to investigate.
        if repo.get("status") == "nuked":
            result["status"] = "nuked"
        else:
            result["status"] = "unreachable"
            repo["status"] = "unreachable"
        return result

    actual = meta.get("full_name", rid)
    if actual.lower() != rid.lower():
        result["renamed_to"] = actual
    if result.get("renamed_to"):
        repo["renamed_to"] = actual
        repo["status"] = "renamed"
    elif meta.get("archived"):
        repo["status"] = "archived"
    elif repo.get("status") in ("nuked", "unreachable"):
        # It came back: record the recovery instead of keeping a stale nuke.
        repo["status"] = "active"
        repo["restored_at"] = dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat()
    elif repo.get("status") == "candidate":
        repo["status"] = "candidate"
    else:
        repo["status"] = "active"
    if (repo.get("portwatch") or {}).get("section") == "candidates":
        # Reachability or a standings listing cannot vet a candidate.
        repo["status"] = "candidate"
    repo["default_branch"] = meta.get("default_branch")
    lic = (meta.get("license") or {}).get("spdx_id")
    if lic and lic != "NOASSERTION":
        repo["license"] = lic
    repo["archived"] = bool(meta.get("archived"))
    repo["pushed_at"] = meta.get("pushed_at")
    repo["description"] = (meta.get("description") or "")[:300]
    repo["stars"] = meta.get("stargazers_count")
    if repo.get("archived"):
        repo["status"] = "archived"

    if meta.get("private"):
        result["notes"].append("private repository; public-source collection skipped")
        result["collection_complete"] = False
        return result

    branch = meta.get("default_branch") or "main"
    commits = gh_items(f"repos/{actual}/commits?sha={branch}&since={iso(since)}")
    result["collection_errors"] = []
    if commits is None:
        result["collection_errors"].append("commits")
        commits = []
    for c in commits:
        msg = (c.get("commit", {}).get("message") or "").split("\n", 1)[0]
        sha = (c.get("sha") or "")[:12]
        bucket, _ = triage(msg)
        result["commits"].append({"sha": sha, "message": msg, "bucket": bucket,
                                  "date": c.get("commit", {}).get("author", {}).get("date")})
        if bucket == "unclassified":
            result["unclassified"] += 1
        else:
            result["buckets"][bucket] += 1

    prs = gh_items(f"repos/{actual}/pulls?state=open&sort=updated&direction=desc", pr_limit)
    if prs is None:
        result["collection_errors"].append("prs")
        prs = []
    for pr in prs:
        title = pr.get("title") or ""
        body = pr.get("body") or ""
        bucket, _ = triage(title + "\n" + body)
        rec = {"number": pr.get("number"), "title": title, "bucket": bucket,
               "updated_at": pr.get("updated_at"), "draft": bool(pr.get("draft")),
               "url": pr.get("html_url"), "head_sha": (pr.get("head") or {}).get("sha")}
        result["prs"].append(rec)
        if bucket == "unclassified":
            result["unclassified"] += 1
        else:
            result["buckets"][bucket] += 1
        low = (title + "\n" + body).lower()
        if any(m in low for m in HOLD_MARKERS + ("timeout", "device loss", "device-lost", "device removed")):
            result["hold"].append({"kind": "pr", "id": pr.get("number"), "title": title})

    result["pr_count_capped"] = False
    result["collection_complete"] = not result["collection_errors"]
    repo["last_checked_at"] = iso(now_utc())
    if not result["collection_complete"]:
        return result
    if commits:
        repo["last_sync_sha"] = commits[0]["sha"]
    else:
        # No commit in the window: still record the branch head, so the next
        # run has a real base to diff from.
        head = gh_json([f"repos/{actual}/git/ref/heads/{branch}"])
        if head and head.get("object", {}).get("sha"):
            repo["last_sync_sha"] = head["object"]["sha"]
    repo["last_sync_at"] = iso(now_utc())
    repo["last_checked_at"] = iso(now_utc())
    return result


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--manifest", default=str(ROOT / "UPSTREAM_REPOS.json"))
    ap.add_argument("--snapshot", default=str(ROOT / "tracking" / "standings.json"))
    ap.add_argument("--report", default=str(ROOT / "tracking" / "sync-report.json"))
    ap.add_argument("--changelog", default=str(ROOT / "tracking" / "CHANGELOG.md"))
    ap.add_argument("--remotes", default=str(ROOT / "scripts" / "remotes.txt"))
    ap.add_argument("--since", help="ISO timestamp; defaults to the current UTC day")
    ap.add_argument("--offline-page", help="use a saved HTML page instead of fetching")
    ap.add_argument("--no-api", action="store_true", help="discovery only, skip GitHub calls")
    ap.add_argument("--pr-limit", type=int, default=100,
                    help="PR page size (1-100); all pages are collected")
    ap.add_argument("--offline-portwatch", type=Path, help="use a saved PortWatch feed")
    args = ap.parse_args(argv)
    if not 1 <= args.pr_limit <= 100:
        ap.error("--pr-limit must be between 1 and 100")
    if args.offline_page and not args.offline_portwatch:
        ap.error("--offline-page requires --offline-portwatch to keep both sources offline")

    stamp = iso(now_utc())
    manifest_path = Path(args.manifest)
    manifest = load_manifest(manifest_path)

    page = (Path(args.offline_page).read_text(encoding="utf-8")
            if args.offline_page else scrape_standings.fetch(scrape_standings.SOURCE))
    snapshot = {
        "source": scrape_standings.SOURCE,
        "scraped_at": stamp,
        "projects": scrape_standings.parse_projects(page),
        "watched": scrape_standings.parse_watched(page),
        "no_published_work": scrape_standings.parse_no_work(page),
    }
    Path(args.snapshot).parent.mkdir(parents=True, exist_ok=True)
    Path(args.snapshot).write_text(json.dumps(snapshot, indent=2, sort_keys=True) + "\n")

    discovered = discover(manifest, snapshot, stamp)
    tracker = (json.loads(args.offline_portwatch.read_text(encoding="utf-8"))
               if args.offline_portwatch else fetch_portwatch.fetch(fetch_portwatch.SOURCE))
    found = fetch_portwatch.normalise(tracker)
    _, updated, new_ids = fetch_portwatch.merge(manifest, found)
    discovered.extend(r for r in found if r["id"] in new_ids)
    raw = {k: v for k, v in tracker.items() if k not in ("headlines", "ignored")}
    raw["ignored_count"] = len(tracker.get("ignored", []))
    # Keep custom/test manifests isolated from the checkout's live tracking files.
    portwatch_dir = Path(args.snapshot).parent
    (portwatch_dir / "portwatch.json").write_text(json.dumps(raw, ensure_ascii=False, indent=1) + "\n")
    (portwatch_dir / "PORTWATCH.md").write_text(fetch_portwatch.digest(tracker, found, new_ids, updated))

    # Merge the page's current position/stage into every tracked entry and
    # mark entries that the page no longer lists.
    listed = {}
    for p in snapshot["projects"]:
        if p.get("repo"):
            listed[p["repo"].lower()] = p
    for w in snapshot["watched"]:
        if w.get("repo"):
            listed.setdefault(w["repo"].lower(), w)
    for repo in manifest["repos"]:
        p = listed.get(repo["id"].lower())
        if p:
            repo["stage"] = p.get("stage")
            repo["position"] = p.get("position")
            repo["on_page"] = True
            if repo.get("status") in ("offline", "renamed", "takedown"):
                repo["status"] = "active"
        else:
            repo["on_page"] = False
            # A repository discovered through PortWatch is not expected on
            # the standings page; its absence there is not an outage.
            if repo.get("status") in ("nuked", "candidate", "renamed", "takedown"):
                pass
            elif repo.get("portwatch"):
                if repo.get("status") == "offline":
                    repo["status"] = "active"
            else:
                repo["status"] = "offline"

    since = dt.datetime.fromisoformat(args.since.replace("Z", "+00:00")) if args.since else None
    if since is None:
        # Bootstrap window: the current UTC day, so a first run reports the
        # day's activity without pretending to know an older sync point.
        since = now_utc().replace(hour=0, minute=0, second=0, microsecond=0)

    results = []
    if not args.no_api:
        for i, repo in enumerate(manifest["repos"], 1):
            print(f"[{i}/{len(manifest['repos'])}] {repo['id']}", file=sys.stderr)
            results.append(collect(repo, since, args.pr_limit))
    else:
        results = [{"id": r["id"], "commits": [], "prs": [],
                    "buckets": {b: 0 for b in BUCKETS}, "unclassified": 0, "hold": [],
                    "collection_complete": False, "notes": ["GitHub collection skipped (--no-api)"]}
                   for r in manifest["repos"]]

    totals = {b: sum(r["buckets"].get(b, 0) for r in results) for b in BUCKETS}
    totals["unclassified"] = sum(r.get("unclassified", 0) for r in results)
    report = {
        "schema": 1,
        "generated_at": stamp,
        "source": scrape_standings.SOURCE,
        "sources": [scrape_standings.SOURCE, fetch_portwatch.SOURCE],
        "window_since": iso(since),
        "repo_count": len(manifest["repos"]),
        "api_skipped": args.no_api,
        "new_repositories": discovered,
        "renamed": [{"id": r["id"], "to": r.get("renamed_to")} for r in results if r.get("renamed_to")],
        "unreachable": [r["id"] for r in results if r.get("status") == "unreachable"],
        "totals": totals,
        "per_repo": results,
    }
    # Full pagination can produce thousands of PR records. Compact JSON keeps
    # the complete dataset within the existing tracking-file policy ceiling.
    Path(args.report).write_text(json.dumps(report, separators=(",", ":"), sort_keys=True) + "\n")

    manifest["generated_at"] = stamp
    manifest["repo_count"] = len(manifest["repos"])
    manifest_path.write_text(json.dumps(manifest, indent=2) + "\n")

    lines = []
    for r in manifest["repos"]:
        if r.get("base") or r.get("status") in ("nuked", "takedown", "offline", "unreachable"):
            continue
        lines.append(f"{r['id']}\thttps://github.com/{r['id']}.git")
    Path(args.remotes).write_text("\n".join(lines) + "\n")

    write_changelog(Path(args.changelog), report, manifest, since)
    print(f"tracked={len(manifest['repos'])} new={len(discovered)} "
          f"totals={totals}", file=sys.stderr)
    return 1 if report["unreachable"] or any(r.get("collection_errors") for r in results) else 0


def write_changelog(path: Path, report: dict, manifest: dict, since: dt.datetime) -> None:
    by_id = {r["id"]: r for r in manifest["repos"]}
    out = [f"# Upstream sync - {report['generated_at']}", "",
           f"Sources: " + ", ".join(f"<{source}>" for source in report.get("sources", [report["source"]])),
           f"Window: commits since `{report['window_since']}`  ",
           f"Tracked repositories: **{report['repo_count']}**", ""]

    out += ["## Newly discovered repositories", ""]
    if report["new_repositories"]:
        out += ["| Repository | Name | Family | Stage |", "|---|---|---|---|"]
        for r in report["new_repositories"]:
            out.append(f"| [{r['id']}](https://github.com/{r['id']}) | {r['name']} | {r['family']} | {r.get('stage') or '-'} |")
    else:
        out.append("_None: the manifest already covered every public link on the page._")
    out.append("")

    incomplete = [r for r in report["per_repo"] if r.get("collection_errors")]
    if incomplete:
        out += ["## Incomplete API collection", ""]
        out += [f"- `{r['id']}`: {', '.join(r['collection_errors'])}; sync checkpoint preserved"
                for r in incomplete]
        out.append("")

    if report["renamed"]:
        out += ["## Renamed or moved", ""]
        for r in report["renamed"]:
            out.append(f"- `{r['id']}` now resolves to `{r['to']}`")
        out.append("")
    if report["unreachable"]:
        out += ["## Unreachable (flag kept, remote disabled)", ""]
        out += [f"- `{i}`" for i in report["unreachable"]]
        out.append("")

    portwatch = [r for r in manifest["repos"] if r.get("portwatch")]
    if portwatch:
        counts: dict[str, int] = {}
        for r in portwatch:
            section = (r.get("portwatch") or {}).get("section") or "unknown"
            counts[section] = counts.get(section, 0) + 1
        out += ["## Tracked through PortWatch VI", "",
                "| Section | Repositories |", "|---|---:|"]
        for section in ("efforts", "tools", "candidates"):
            out.append(f"| {section} | {counts.get(section, 0)} |")
        out.append("")
        out.append("`candidates` are unreviewed and are not presented as trusted.")
        nuked = [r for r in portwatch if r.get("status") == "nuked"]
        if nuked:
            out.append("")
            out.append("Nuked or deleted: " + ", ".join(f"`{r['id']}`" for r in nuked) + ".")
        out.append("")

    out += ["## Triage totals", "", "| Bucket | Items |", "|---|---|"]
    for b in BUCKETS + ["unclassified"]:
        out.append(f"| {b} | {report['totals'].get(b, 0)} |")
    out.append("")

    names = {
        "critical-linux": "Critical Linux / Proton-GE fixes",
        "frame-time": "Frame-time and CPU/GPU bottleneck wins",
        "stability": "Stability and device-loss prevention",
        "rendering": "Rendering and gameplay completeness",
        "experimental": "Experimental / unstable (hold)",
    }
    for bucket in BUCKETS:
        out += [f"## {names[bucket]}", ""]
        rows = []
        for r in report["per_repo"]:
            for pr in r["prs"]:
                if pr["bucket"] == bucket:
                    rows.append(f"- `{r['id']}` #{pr['number']} - {pr['title']}")
            for c in r["commits"]:
                if c["bucket"] == bucket:
                    rows.append(f"- `{r['id']}` `{c['sha']}` - {c['message']}")
        out += (rows[:80] if rows else ["_No items this window._"])
        if len(rows) > 80:
            out.append(f"- …and {len(rows) - 80} more (see `tracking/sync-report.json`)")
        out.append("")

    out += ["## Per-repository activity", "",
            "| Repository | Commits | Open PRs | Stage | Status |", "|---|---|---|---|---|"]
    for r in report["per_repo"]:
        repo = by_id.get(r["id"], {})
        prs = len(r["prs"])
        cap = "+" if r.get("pr_count_capped") else ""
        out.append(f"| `{r['id']}` | {len(r['commits'])} | {prs}{cap} | "
                   f"{repo.get('stage') or '-'} | {repo.get('status', '-')} |")
    out.append("")
    path.write_text("\n".join(line.rstrip() for line in out) + "\n")


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
