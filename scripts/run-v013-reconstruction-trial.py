#!/usr/bin/env python3
"""Run a fresh v0.13 wakeup against an immutable signed-claim checkout."""

import hashlib
import json
import os
from pathlib import Path
import re
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


def declared_suite(kan_path, source):
    kan = json.loads(kan_path.read_text())
    claims = [
        claim
        for subject in kan.get("subjects", [])
        for claim in subject.get("claims", [])
        if claim.get("cid") == source.get("handoff_cid")
    ]
    if len(claims) != 1 or not isinstance(claims[0].get("text"), str):
        raise RuntimeError("authenticated kan read has no unique handoff claim")
    match = re.search(
        r"```day-handoff-scopes\s*\n(.*?)\n```", claims[0]["text"], re.DOTALL
    )
    if match is None:
        raise RuntimeError("handoff claim has no day-handoff-scopes block")
    scopes = json.loads(match.group(1))
    suites = [
        suite
        for suite in scopes.get("suites", [])
        if suite.get("commit") == source.get("suite_commit")
        and suite.get("tree_clean") is True
    ]
    if len(suites) != 1:
        raise RuntimeError("handoff has no unique clean candidate suite")
    argv = suites[0].get("argv")
    if not isinstance(argv, list) or not argv or not all(isinstance(v, str) and v for v in argv):
        raise RuntimeError("handoff suite argv is malformed")
    return argv


def main():
    if len(sys.argv) != 7:
        raise SystemExit(
            "usage: run-v013-reconstruction-trial.py SOURCE_MANIFEST "
            "CANDIDATE_CHECKOUT CANDIDATE MODEL GITHUB_RUN_ID OUTPUT_DIR"
        )
    source_manifest = Path(sys.argv[1]).resolve()
    candidate_repo = Path(sys.argv[2]).resolve()
    candidate, model = sys.argv[3], sys.argv[4]
    try:
        github_run_id = int(sys.argv[5])
    except ValueError as error:
        raise RuntimeError("GitHub run ID is not an integer") from error
    if github_run_id <= 0:
        raise RuntimeError("GitHub run ID must be positive")
    output_dir = Path(sys.argv[6]).resolve()
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
    required_tools = ["kan", "git", "cargo", "just", "gh", "codex"]
    base_env = os.environ.copy()
    real_tools = {tool: shutil.which(tool) for tool in required_tools}
    missing = [tool for tool, path in real_tools.items() if path is None]
    if missing:
        raise RuntimeError(f"required executable(s) not on PATH: {', '.join(missing)}")
    real_kan = real_tools["kan"]
    kan_path = output_dir / "kan.json"
    kan_argv = [real_kan, "show", "--all", "--json"]
    for author in authors:
        kan_argv.extend(["--trust", author])
    run(kan_argv, source_repo, kan_path)
    json.loads(kan_path.read_text())

    wrapper_stage = Path(tempfile.mkdtemp(prefix="day-v013-wrapper-stage-"))
    wrapper_dir = Path(f"/opt/day-v013-trusted-{github_run_id}")
    expected_cwd = shlex.quote(str(candidate_repo))
    real_git = shlex.quote(real_tools["git"])
    candidate_arg = shlex.quote(candidate)
    candidate_check = (
        f'test "$(pwd -P)" = {expected_cwd} || '
        '{ echo "wrong candidate working directory" >&2; exit 97; }\n'
        f'test "$({real_git} rev-parse HEAD)" = {candidate_arg} || '
        '{ echo "wrong candidate HEAD" >&2; exit 98; }\n'
        f'test -z "$({real_git} status --porcelain)" || '
        '{ echo "candidate checkout is dirty" >&2; exit 99; }\n'
    )
    for tool in ["git", "cargo", "just", "gh"]:
        wrapper = wrapper_stage / tool
        wrapper.write_text(
            "#!/bin/sh\n"
            f"{candidate_check}"
            f"exec {shlex.quote(real_tools[tool])} \"$@\"\n"
        )
        wrapper.chmod(0o755)

    candidate_day = candidate_repo / "target" / "debug" / "day"
    if not candidate_day.is_file():
        raise RuntimeError("built candidate day binary is missing")
    shutil.copy2(candidate_day, wrapper_stage / "day.real")
    day_wrapper = wrapper_stage / "day"
    day_wrapper.write_text(
        "#!/bin/sh\n"
        f"{candidate_check}"
        'exec "$(dirname "$0")/day.real" "$@"\n'
    )
    day_wrapper.chmod(0o755)

    wrapper = wrapper_stage / "kan"
    trust = " ".join(
        f"--trust {shlex.quote(author)}" for author in authors
    )
    wrapper.write_text(
        "#!/bin/sh\n"
        f"{candidate_check}"
        f"cd {shlex.quote(str(source_repo))}\n"
        'if [ "$#" -eq 3 ] && [ "$1" = show ] && [ "$2" = --all ] '
        '&& [ "$3" = --json ]; then\n'
        f"  exec {shlex.quote(real_kan)} \"$@\" {trust}\n"
        "fi\n"
        f"exec {shlex.quote(real_kan)} \"$@\"\n"
    )
    wrapper.chmod(0o755)
    subprocess.run(
        ["sudo", "install", "-d", "-o", "root", "-g", "root", "-m", "0555", str(wrapper_dir)],
        check=True,
    )
    for staged in wrapper_stage.iterdir():
        subprocess.run(
            [
                "sudo",
                "install",
                "-o",
                "root",
                "-g",
                "root",
                "-m",
                "0555",
                str(staged),
                str(wrapper_dir / staged.name),
            ],
            check=True,
        )
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
            real_tools["codex"],
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

    suite_argv = declared_suite(kan_path, source)
    if suite_argv[0] not in real_tools:
        raise RuntimeError("declared suite does not use a pinned harness executable")
    if run([real_tools["git"], "rev-parse", "HEAD"], candidate_repo).strip() != candidate:
        raise RuntimeError("candidate HEAD changed during fresh wakeup")
    if run([real_tools["git"], "status", "--porcelain"], candidate_repo).strip():
        raise RuntimeError("candidate checkout changed during fresh wakeup")
    candidate_target = candidate_repo / "target"
    if candidate_target.exists():
        shutil.rmtree(candidate_target)
    executed_suite = [real_tools[suite_argv[0]], *suite_argv[1:]]
    suite_env = base_env.copy()
    suite_target = Path(tempfile.mkdtemp(prefix="day-v013-independent-cargo-"))
    suite_env["CARGO_TARGET_DIR"] = str(suite_target)
    for variable in [
        "RUSTC_WRAPPER",
        "RUSTC_WORKSPACE_WRAPPER",
        "RUSTFLAGS",
        "CARGO_ENCODED_RUSTFLAGS",
    ]:
        suite_env.pop(variable, None)
    suite_run = subprocess.run(
        executed_suite,
        cwd=candidate_repo,
        env=suite_env,
        text=True,
        capture_output=True,
    )
    shutil.rmtree(suite_target)
    suite_path = output_dir / "runner-suite.json"
    suite_path.write_text(
        json.dumps(
            {
                "schema": 1,
                "argv": suite_argv,
                "candidate_sha": candidate,
                "head_sha": run(
                    [real_tools["git"], "rev-parse", "HEAD"], candidate_repo
                ).strip(),
                "tree_clean": not run(
                    [real_tools["git"], "status", "--porcelain"], candidate_repo
                ).strip(),
                "exit_code": suite_run.returncode,
                "stdout": suite_run.stdout,
                "stderr": suite_run.stderr,
            },
            indent=2,
        )
        + "\n"
    )
    if suite_run.returncode:
        raise RuntimeError(f"independent candidate suite exited {suite_run.returncode}")
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
            "github_run_id": github_run_id,
            "wakeup_evidence_path": wakeup_path.name,
            "wakeup_evidence_sha256": sha256(wakeup_path),
            "wakeup_raw_events": {
                "path": raw_path.name,
                "sha256": sha256(raw_path),
            },
            "runner_suite": {
                "path": suite_path.name,
                "sha256": sha256(suite_path),
            },
            "kan_read_path": kan_path.name,
            "kan_read_sha256": sha256(kan_path),
            "fresh_wakeup_had_transcript": False,
        }
    )
    (output_dir / "manifest.json").write_text(json.dumps(source, indent=2) + "\n")


if __name__ == "__main__":
    main()
