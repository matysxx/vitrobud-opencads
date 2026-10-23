"""Synthetic container-packaging checks; no user CAD or runtime data."""
import pathlib
import subprocess
import tempfile
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[1]


class PackagingTests(unittest.TestCase):
    def test_cache_stamp_is_repeatable_and_tracks_bundle_changes(self):
        with tempfile.TemporaryDirectory() as directory:
            bundle = pathlib.Path(directory)
            worker = bundle / "sw.js"
            worker.write_text('const CACHE_VERSION = "v1"; // placeholder\n')
            asset = bundle / "index.html"
            asset.write_text("first")
            command = ["sh", str(ROOT / "scripts/stamp-container-cache.sh"), directory]
            subprocess.run(command, check=True)
            first = worker.read_text()
            subprocess.run(command, check=True)
            self.assertEqual(first, worker.read_text())
            asset.write_text("second")
            subprocess.run(command, check=True)
            self.assertNotEqual(first, worker.read_text())

    def test_missing_worker_fails(self):
        with tempfile.TemporaryDirectory() as directory:
            result = subprocess.run(
                ["sh", str(ROOT / "scripts/stamp-container-cache.sh"), directory],
                capture_output=True,
            )
            self.assertNotEqual(result.returncode, 0)

    def test_git_excludes_private_files_but_retains_templates(self):
        private = [".env.pre-test", "src/.env.backup", ".ai/wiki/tasks/test/summary.md",
                   ".ai/prd/task-status.local.md", "dev-ops/storage/test/data"]
        public = [".env.dist", "src/.env.dist", ".ai/wiki/context-policy.md"]
        result = subprocess.run(
            ["git", "check-ignore", "--no-index", "--stdin"], cwd=ROOT,
            input="\n".join(private + public) + "\n", text=True, capture_output=True,
        )
        self.assertEqual(set(result.stdout.splitlines()), set(private))

    def test_container_context_rules_match(self):
        podman = (ROOT / ".containerignore").read_text()
        self.assertEqual(podman, (ROOT / ".dockerignore").read_text())
        self.assertTrue({".ai/wiki/tasks", ".ai/prd/task-status.local.md",
                         "**/.env.*", "!**/.env.dist", "dev-ops/.runtime"}
                        .issubset(set(podman.splitlines())))


if __name__ == "__main__":
    unittest.main()
