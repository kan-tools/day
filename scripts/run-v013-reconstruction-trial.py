#!/usr/bin/env python3
"""Run a fresh v0.13 wakeup against an immutable signed-claim checkout."""

import hashlib
import json
import os
from pathlib import Path
import shlex
import shutil
import subprocess
import sys
import tempfile


def run(argv, cwd, output=None):
    completed = subprocess.run(
        argv, cwd=cwd, env=os.environ.copy(), text=True, capture_output=True
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
    if len(sys.argv) != 6:
        raise SystemExit(
            "usage: run-v013-reconstruction-trial.py SOURCE_MANIFEST "
            "CANDIDATE_CHECKOUT CANDIDATE MODEL OUTPUT_DIR"
        )
    source_manifest = Path(sys.argv[1]).resolve()
    candidate_repo = Path(sys.argv[2]).resolve()
    candidate, model = sys.argv[3], sys.argv[4]
    output_dir = Path(sys.argv[5]).resolve()
    output_dir.mkdir(parents=True, exist_ok=False)
    source = json.loads(source_manifest.read_text())
    if source.get("schema") != 1 or source.get("candidate_sha") != candidate:
        raise RuntimeError("source manifest does not bind the exact candidate")
    authors = source.get("kan_authors")
    if not isinstance(authors, list) or not authors:
        raise RuntimeError("source manifest has no kan signing principals")

    source_repo = source_manifest.parent
    if run(["git", "rev-parse", "HEAD"], candidate_repo).strip() != candidate:
        raise RuntimeError("candidate checkout HEAD differs from the workflow candidate")
    if run(["git", "status", "--porcelain"], candidate_repo).strip():
        raise RuntimeError("candidate checkout is not clean before the fresh wakeup")
    real_kan = shutil.which("kan")
    if real_kan is None:
        raise RuntimeError("pinned kan binary is not on PATH")
    kan_path = output_dir / "kan.json"
    kan_argv = [real_kan, "show", "--all", "--json"]
    for author in authors:
        kan_argv.extend(["--trust", author])
    run(kan_argv, source_repo, kan_path)
    json.loads(kan_path.read_text())

    wrapper_dir = Path(tempfile.mkdtemp(prefix="day-v013-kan-wrapper-"))
    wrapper = wrapper_dir / "kan"
    trust = " ".join(
        f"--trust {shlex.quote(author)}" for author in authors
    )
    wrapper.write_text(
        "#!/bin/sh\n"
        f"cd {shlex.quote(str(source_repo))}\n"
        'if [ "$#" -eq 3 ] && [ "$1" = show ] && [ "$2" = --all ] '
        '&& [ "$3" = --json ]; then\n'
        f"  exec {shlex.quote(real_kan)} \"$@\" {trust}\n"
        "fi\n"
        f"exec {shlex.quote(real_kan)} \"$@\"\n"
    )
    wrapper.chmod(0o755)
    os.environ["PATH"] = f"{wrapper_dir}:{os.environ['PATH']}"

    raw_path = output_dir / "wakeup-events.jsonl"
    last_path = output_dir / "wakeup-last-message.txt"
    subject = source["stream_subject"]
    prefix = "agents/handoff/"
    if not subject.startswith(prefix) or not subject[len(prefix) :]:
        raise RuntimeError("source manifest stream is not a handoff subject")
    thread = subject[len(prefix) :]
    prompt = (
        f"Use the wakeup skill to wake into thread {thread!r}. This is a fresh session with "
        "no prior conversation transcript. Perform every required read and verification, "
        "report the result in the skill's required order, and stop. In that report, name the "
        "exact acquired-input, intervention, and handoff CIDs and the exact suite, census, and "
        "CI coordinates you recovered from the signed claims. Execute `kan show --all --json` "
        "exactly. Before the suite, execute `git rev-parse HEAD` and `git status --porcelain` "
        "exactly to establish this clean candidate checkout. Execute the recovered suite argv "
        "exactly, execute the recovered census as "
        "`just census-demonstrations BASE..HEAD` exactly, and execute the recovered CI check as "
        "`gh run view RUN_ID --json headSha,conclusion` exactly; do not wrap or combine them."
    )
    run(
        [
            "codex",
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
        candidate_repo,
        raw_path,
    )
    rendered = last_path.read_text()
    if not rendered.strip():
        raise RuntimeError("fresh wakeup produced no final assistant message")

    wakeup_path = output_dir / "wakeup.json"
    wakeup_path.write_text(
        json.dumps(
            {
                "schema": 1,
                "session_kind": "fresh",
                "raw_transcript_supplied": False,
                "kan_command": ["kan", "show", "--all", "--json"],
                "stream_subject": subject,
                "claims_read": [
                    source["acquired_input_cid"],
                    source["intervention_cid"],
                    source["handoff_cid"],
                ],
                "candidate_sha": candidate,
                "rendered_context": rendered,
            },
            indent=2,
        )
        + "\n"
    )
    source.update(
        {
            "wakeup_evidence_path": wakeup_path.name,
            "wakeup_evidence_sha256": sha256(wakeup_path),
            "wakeup_raw_events": {
                "path": raw_path.name,
                "sha256": sha256(raw_path),
            },
            "kan_read_path": kan_path.name,
            "kan_read_sha256": sha256(kan_path),
            "fresh_wakeup_had_transcript": False,
        }
    )
    (output_dir / "manifest.json").write_text(json.dumps(source, indent=2) + "\n")


if __name__ == "__main__":
    main()
