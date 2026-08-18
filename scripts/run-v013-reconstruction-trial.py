#!/usr/bin/env python3
"""Retain an honest-producer fresh-wakeup observation for v0.13."""

import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys


def run(argv, cwd, env=None, output=None):
    completed = subprocess.run(
        argv,
        cwd=cwd,
        env=env or os.environ.copy(),
        text=True,
        capture_output=True,
    )
    if output is not None:
        output.write_text(completed.stdout)
    if completed.returncode:
        raise RuntimeError(
            f"{' '.join(argv)} exited {completed.returncode}: {completed.stderr}"
        )
    return completed.stdout


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    if len(sys.argv) != 7:
        raise SystemExit(
            "usage: run-v013-reconstruction-trial.py SOURCE_MANIFEST "
            "CANDIDATE_CHECKOUT CANDIDATE MODEL GITHUB_RUN_ID OUTPUT_DIR"
        )
    source_manifest = Path(sys.argv[1]).resolve()
    candidate_repo = Path(sys.argv[2]).resolve()
    candidate, model = sys.argv[3], sys.argv[4]
    github_run_id = int(sys.argv[5])
    if github_run_id <= 0:
        raise RuntimeError("GitHub run ID must be positive")
    output_dir = Path(sys.argv[6]).resolve()
    output_dir.mkdir(parents=True, exist_ok=False)

    source = json.loads(source_manifest.read_text())
    if source.get("schema") != 1 or source.get("candidate_sha") != candidate:
        raise RuntimeError("source manifest does not bind the exact candidate")
    subject = source.get("stream_subject", "")
    prefix = "agents/handoff/"
    if not subject.startswith(prefix) or not subject[len(prefix) :]:
        raise RuntimeError("source manifest stream is not a handoff subject")
    if run(["git", "rev-parse", "HEAD"], candidate_repo).strip() != candidate:
        raise RuntimeError("candidate checkout HEAD differs from the workflow candidate")
    if run(["git", "status", "--porcelain"], candidate_repo).strip():
        raise RuntimeError("candidate checkout is not clean before the observation")

    candidate_day = (candidate_repo / "target" / "debug" / "day").resolve()
    if not candidate_day.is_file():
        raise RuntimeError("built candidate day binary is missing")
    codex = shutil.which("codex")
    kan = shutil.which("kan")
    if not codex or not kan:
        raise RuntimeError("codex and kan must be installed")

    source_repo = source_manifest.parent
    kan_path = output_dir / "kan.json"
    kan_argv = [kan, "show", "--all", "--json"]
    for author in source.get("kan_authors", []):
        kan_argv.extend(["--trust", author])
    run(kan_argv, source_repo, output=kan_path)
    json.loads(kan_path.read_text())

    raw_path = output_dir / "wakeup-events.jsonl"
    last_path = output_dir / "wakeup-last-message.txt"
    env = os.environ.copy()
    env["PATH"] = f"{candidate_day.parent}:{env['PATH']}"
    thread = subject[len(prefix) :]
    prompt = (
        f"Use the wakeup skill to wake into thread {thread!r}. This is a fresh session with "
        "no prior conversation transcript. Read the available kan claims, verify what you can, "
        "report the reconstructed working context and any uncertainty, then stop. This is an "
        "observational trial; follow the skill normally and do not manufacture evidence."
    )
    run(
        [
            codex,
            "exec",
            "--json",
            "--approve-for-me",
            "--sandbox",
            "workspace-write",
            "--model",
            model,
            "--output-last-message",
            str(last_path),
            prompt,
        ],
        source_repo,
        env=env,
        output=raw_path,
    )
    if not last_path.read_text().strip():
        raise RuntimeError("fresh wakeup produced no final assistant message")

    retained_source = output_dir / "source-manifest.json"
    retained_source.write_bytes(source_manifest.read_bytes())
    manifest = {
        "schema": 1,
        "evidence_layer": "observational-trial",
        "producer_assumption": "honest-producer",
        "authoritative": False,
        "lifecycle": "post-merge",
        "candidate_sha": candidate,
        "evidence_commit": run(["git", "rev-parse", "HEAD"], source_repo).strip(),
        "stream_subject": subject,
        "model": model,
        "github_run_id": github_run_id,
        "fresh_wakeup_had_transcript": False,
        "artifacts": {
            "source_manifest": {"path": retained_source.name, "sha256": sha256(retained_source)},
            "kan_read": {"path": kan_path.name, "sha256": sha256(kan_path)},
            "raw_events": {"path": raw_path.name, "sha256": sha256(raw_path)},
            "final_message": {"path": last_path.name, "sha256": sha256(last_path)},
        },
    }
    (output_dir / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")


if __name__ == "__main__":
    main()
