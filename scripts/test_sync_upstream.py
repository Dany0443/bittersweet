"""Synthetic, offline regression tests for ecosystem collection."""
import datetime as dt
import unittest
import json
import tempfile
from pathlib import Path
import fetch_portwatch
from unittest.mock import patch

import sync_upstream as sync


class CollectionTests(unittest.TestCase):
    def test_collects_later_pages_and_holds_timeout_prs(self):
        def api(args):
            endpoint = args[0]
            if endpoint == "repos/owner/repo":
                return {"full_name": "owner/repo", "default_branch": "main"}
            if "/commits?" in endpoint:
                page = int(endpoint.rsplit("page=", 1)[1]) if "&page=" in endpoint else 1
                return [{"sha": str(i), "commit": {"message": "fix"}}
                        for i in range(100)] if page == 1 else [{"sha": "later", "commit": {"message": "POSIX mapping"}}]
            if "/pulls?" in endpoint:
                page = int(endpoint.rsplit("page=", 1)[1]) if "&page=" in endpoint else 1
                return ([{"number": 1, "title": "rendering"}, {"number": 2, "title": "fix"}]
                        if page == 1 else [{"number": 3, "title": "Faster shaders",
                                           "body": "Introduces GPU device timeouts", "head": {"sha": "full-head"}}])
            return None
        repo = {"id": "owner/repo", "status": "candidate", "on_page": True}
        with patch.object(sync, "gh_json", side_effect=api):
            result = sync.collect(repo, dt.datetime.now(dt.timezone.utc), 2)
        self.assertEqual(len(result["commits"]), 101)
        self.assertEqual(len(result["prs"]), 3)
        self.assertFalse(result["pr_count_capped"])
        self.assertEqual(result["prs"][-1]["head_sha"], "full-head")
        self.assertEqual(result["hold"][-1]["id"], 3)
        self.assertEqual(repo["status"], "candidate")

    def test_failed_collection_does_not_advance_sync(self):
        repo = {"id": "owner/repo", "last_sync_at": "previous", "last_sync_sha": "old"}
        with patch.object(sync, "gh_json", side_effect=[{"default_branch": "main"}, None, []]):
            result = sync.collect(repo, dt.datetime.now(dt.timezone.utc), 100)
        self.assertEqual(repo["last_sync_at"], "previous")
        self.assertEqual(repo["last_sync_sha"], "old")
        self.assertIn("commits", result["collection_errors"])

    def test_reachable_candidate_is_not_vetted_by_github(self):
        repo = {"id": "owner/repo", "status": "active", "on_page": True,
                "portwatch": {"section": "candidates"}}
        with patch.object(sync, "gh_json", side_effect=[{"default_branch": "main"}, [], [], {"object": {"sha": "head"}}]):
            result = sync.collect(repo, dt.datetime.now(dt.timezone.utc), 100)
        self.assertTrue(result["collection_complete"])
        self.assertEqual(repo["status"], "candidate")

    def test_unified_offline_discovery_reads_both_sources(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = root / "manifest.json"
            manifest.write_text(json.dumps({"repos": []}))
            page = root / "page.html"
            page.write_text("synthetic standings")
            feed = root / "feed.json"
            feed.write_text(json.dumps({
                "efforts": [{"repo": "portwatch/vetted"}],
                "candidates": [{"repo": "portwatch/candidate"}],
                "flagged": [{"repo": "blocked/flagged"}],
                "ignored": [{"repo": "blocked/ignored"}],
            }))
            with patch.object(sync.scrape_standings, "parse_projects", return_value=[{"repo": "standings/public"}]), \
                 patch.object(sync.scrape_standings, "parse_watched", return_value=[]), \
                 patch.object(sync.scrape_standings, "parse_no_work", return_value=[]), \
                 patch.object(sync, "gh_json", side_effect=AssertionError("offline test contacted GitHub")), \
                 patch.object(fetch_portwatch, "fetch", side_effect=AssertionError("offline test contacted PortWatch")):
                code = sync.main(["--manifest", str(manifest), "--snapshot", str(root / "standings.json"),
                                  "--report", str(root / "report.json"), "--changelog", str(root / "CHANGELOG.md"),
                                  "--remotes", str(root / "remotes.txt"), "--offline-page", str(page),
                                  "--offline-portwatch", str(feed), "--no-api"])
            self.assertEqual(code, 0)
            records = {r["id"]: r for r in json.loads(manifest.read_text())["repos"]}
            self.assertEqual(set(records), {"standings/public", "portwatch/vetted", "portwatch/candidate"})
            self.assertEqual(records["portwatch/candidate"]["status"], "candidate")
            self.assertTrue((root / "PORTWATCH.md").exists())
            self.assertEqual(len(json.loads((root / "report.json").read_text())["sources"]), 2)


if __name__ == "__main__":
    unittest.main()
