#!/usr/bin/env python3
"""Scrape the public standings table into a machine-readable snapshot.

Source: https://vuyp.github.io/gta6-pc-race/#standings

The page is a static, hand-generated document.  We deliberately parse only
the structural parts (board rows, links, lane attributes) and keep the prose
as-is, so a wording change on the page cannot silently drop a project.

Output (stdout or --out):
{
  "source": ..., "checked_at": ..., "projects": [...],
  "watched": [...], "private_builds": [...], "offline": [...]
}

This script never touches a game binary or game data; it reads one web page.
"""

from __future__ import annotations

import argparse
import datetime as dt
import html
import json
import re
import sys
import urllib.request

SOURCE = "https://vuyp.github.io/gta6-pc-race/"
USER_AGENT = "bittersweet-upstream-tracker/1 (+local; no game files)"

ROW_RE = re.compile(r'<li class="board__row"(?P<attrs>[^>]*)>(?P<body>.*?)</li>', re.S)
ATTR_RE = re.compile(r'data-(os|gpu|picture|speed|stability)="([^"]*)"')
LINK_RE = re.compile(r'<a class="name" href="(https://github\.com/[^"]+)"[^>]*>(.*?)</a>', re.S)
STAGE_RE = re.compile(r"Stage\s*(\d)\s*of\s*7\s*:\s*([^<]+)")
POS_RE = re.compile(r'aria-label="Position\s*([0-9=]+)"')
CHANGED_RE = re.compile(r'Changed\s*<time datetime="([^"]+)"')
TAG_RE = re.compile(r"<[^>]+>")


def fetch(url: str) -> str:
    req = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    with urllib.request.urlopen(req, timeout=30) as resp:
        return resp.read().decode("utf-8", "replace")


def text(fragment: str) -> str:
    return html.unescape(TAG_RE.sub("", fragment)).strip()


def parse_projects(page: str) -> list[dict]:
    out: list[dict] = []
    for m in ROW_RE.finditer(page):
        body = m.group("body")
        attrs = dict(ATTR_RE.findall(m.group("attrs")))
        link = LINK_RE.search(body)
        stage = STAGE_RE.search(body)
        pos = POS_RE.search(m.group("attrs")) or POS_RE.search(body)
        changed = CHANGED_RE.search(body)
        if not link and not stage:
            continue
        name = text(link.group(2)) if link else text(body.split("</h3>")[0])
        url = link.group(1) if link else None
        slug = "/".join(url.rstrip("/").split("/")[-2:]) if url else None
        out.append(
            {
                "position": pos.group(1) if pos else None,
                "name": name,
                "repo": slug,
                "url": url,
                "stage": int(stage.group(1)) if stage else None,
                "stage_label": text(stage.group(2)) if stage else None,
                "os": attrs.get("os", ""),
                "gpu": attrs.get("gpu", ""),
                "picture": int(attrs["picture"]) if attrs.get("picture") else None,
                "speed": int(attrs["speed"]) if attrs.get("speed") else None,
                "stability": int(attrs["stability"]) if attrs.get("stability") else None,
                "changed_at": changed.group(1) if changed else None,
                "private": url is None,
            }
        )
    return out


def parse_watched(page: str) -> list[dict]:
    """The 'Also watched' list: project name, optional repo link, prose."""
    start = page.find("Also watched")
    if start < 0:
        return []
    end = page.find("Also checked", start)
    if end < 0:
        end = page.find('id="forecast"', start)
    if end < 0:
        end = start + 20000
    seg = page[start:end]
    out = []
    for m in re.finditer(r"<li>(.*?)</li>", seg, re.S):
        item = m.group(1)
        named = re.search(r'<span class="rows__name">(.*?)</span>', item, re.S)
        link = LINK_RE.search(item) or re.search(
            r'<a[^>]+href="(https://github\.com/[^"]+)"[^>]*>(.*?)</a>', item, re.S
        )
        prose = text(re.sub(r"<a[^>]*>.*?</a>", "", item, flags=re.S))
        name = text(named.group(1)) if named else prose.split(" ")[0]
        url = link.group(1) if link else None
        out.append(
            {
                "name": name,
                "repo": "/".join(url.rstrip("/").split("/")[-2:]) if url else None,
                "url": url,
                "note": prose,
                "online": bool(url),
            }
        )
    return out


def parse_no_work(page: str) -> list[dict]:
    """The 'Also checked, with no <title> work published' inline list."""
    start = page.find("Also checked")
    if start < 0:
        return []
    end = page.find("</p>", start)
    seg = page[start:end if end > 0 else start + 2000]
    out = []
    for m in re.finditer(
        r'<a[^>]+href="(https://github\.com/[^"]+)"[^>]*>(.*?)</a>', seg, re.S
    ):
        url, name = m.group(1), text(m.group(2))
        out.append(
            {
                "name": name,
                "repo": "/".join(url.rstrip("/").split("/")[-2:]),
                "url": url,
                "online": True,
                "note": "listed as checked, with no work published",
            }
        )
    return out


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--url", default=SOURCE)
    ap.add_argument("--out")
    ap.add_argument("--page", help="read a saved page instead of fetching")
    args = ap.parse_args(argv)

    page = open(args.page, encoding="utf-8").read() if args.page else fetch(args.url)
    projects = parse_projects(page)
    watched = parse_watched(page)
    no_work = parse_no_work(page)

    checked = re.search(r'id="checked"[^>]*>([^<]+)', page)
    snapshot = {
        "source": args.url,
        "scraped_at": dt.datetime.now(dt.timezone.utc).isoformat(timespec="seconds"),
        "page_checked_at": text(checked.group(1)) if checked else None,
        "projects": projects,
        "watched": watched,
        "no_published_work": no_work,
        "private_builds": [p for p in projects if p["private"]],
        "offline": [w for w in watched if not w["online"]],
    }
    blob = json.dumps(snapshot, indent=2, sort_keys=True) + "\n"
    if args.out:
        with open(args.out, "w", encoding="utf-8") as fh:
            fh.write(blob)
    else:
        sys.stdout.write(blob)
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
