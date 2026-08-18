#!/usr/bin/env python3
"""Run the preregistered /askme scenarios through a real Codex CLI session."""

import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile


def run(argv, cwd, env, output=None):
    completed = subprocess.run(argv, cwd=cwd, env=env, text=True, capture_output=True)
    if output:
        output.write_text(completed.stdout)
    if completed.returncode:
        raise RuntimeError(f"{' '.join(argv)} exited {completed.returncode}: {completed.stderr}")
    return completed.stdout


def values(value):
    if isinstance(value, dict):
        for key, child in value.items():
            if key in {"thread_id", "session_id"} and isinstance(child, str):
                yield child
            yield from values(child)
    elif isinstance(value, list):
        for child in value:
            yield from values(child)


def session_id(jsonl):
    for line in jsonl.splitlines():
        try:
            found = list(values(json.loads(line)))
        except json.JSONDecodeError:
            continue
        if found:
            return found[0]
    raise RuntimeError("Codex JSONL carried no thread/session identifier")


def kan_snapshot(cwd, env):
    completed = subprocess.run(
        ["kan", "show", "--all", "--json"], cwd=cwd, env=env, text=True, capture_output=True
    )
    if completed.returncode:
        raise RuntimeError(
            f"kan show --all --json exited {completed.returncode}: {completed.stderr}"
        )
    try:
        parsed = json.loads(completed.stdout)
    except json.JSONDecodeError as error:
        raise RuntimeError(f"kan show --all --json returned invalid JSON: {error}") from error
    claims = []

    def walk(value):
        if isinstance(value, dict):
            if "cid" in value:
                claims.append(value)
            for child in value.values():
                walk(child)
        elif isinstance(value, list):
            for child in value:
                walk(child)

    walk(parsed)
    return {
        "raw": completed.stdout,
        "claims": len({claim.get("cid") for claim in claims if claim.get("cid")}),
        "texts": [claim["text"] for claim in claims if isinstance(claim.get("text"), str)],
    }


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def record_receipts(raw_files):
    receipts = []
    for path in raw_files:
        for line in path.read_text().splitlines():
            try:
                event = json.loads(line)
            except json.JSONDecodeError:
                continue
            item = event.get("item", {})
            command = item.get("command")
            if (
                event.get("type") != "item.completed"
                or item.get("type") != "command_execution"
                or not isinstance(command, str)
                or "acquired-input" not in command
                or "record" not in command
            ):
                continue
            output = item.get("aggregated_output", "")
            exit_code = item.get("exit_code")
            match = re.search(r"\((bafy[^)]+)\)", output) if exit_code == 0 else None
            receipts.append(
                {
                    "command": command,
                    "status": item.get("status"),
                    "exit_code": exit_code,
                    "output": output,
                    "cid": match.group(1) if match else None,
                }
            )
    return receipts


def main():
    if len(sys.argv) != 6:
        raise SystemExit(
            "usage: run-v013-askme-trial.py PROTOCOL CANDIDATE MODEL GITHUB_RUN_ID OUTPUT_DIR"
        )
    protocol_path = Path(sys.argv[1]).resolve()
    candidate, model = sys.argv[2], sys.argv[3]
    github_run_id = int(sys.argv[4])
    if github_run_id <= 0:
        raise RuntimeError("GitHub run ID must be positive")
    output_dir = Path(sys.argv[5]).resolve()
    output_dir.mkdir(parents=True, exist_ok=False)
    protocol = json.loads(protocol_path.read_text())
    root = Path.cwd()
    real_day = (root / "target" / "debug" / "day").resolve()
    if not real_day.is_file():
        raise RuntimeError(f"candidate day binary is missing at {real_day}")

    wrapper_stage = Path(tempfile.mkdtemp(prefix="day-v013-askme-wrapper-"))
    wrapper_dir = Path(f"/opt/day-v013-askme-{github_run_id}")
    shutil.copy2(real_day, wrapper_stage / "day.real")
    wrapper = wrapper_stage / "day"
    wrapper.write_text(f"#!/bin/sh\nexec '{wrapper_dir / 'day.real'}' \"$@\"\n")
    wrapper.chmod(0o755)
    run(
        ["sudo", "install", "-d", "-o", "root", "-g", "root", "-m", "0555", str(wrapper_dir)],
        root,
        os.environ.copy(),
    )
    for staged in wrapper_stage.iterdir():
        run(
            ["sudo", "install", "-o", "root", "-g", "root", "-m", "0555", str(staged), str(wrapper_dir / staged.name)],
            root,
            os.environ.copy(),
        )

    entries = []
    for scenario in protocol["scenarios"]:
        scenario_dir = output_dir / scenario["id"]
        repo = scenario_dir / "repo"
        repo.mkdir(parents=True)
        run(["git", "init", "-q", "-b", "trial"], repo, os.environ.copy())
        run(["git", "config", "user.email", "trial@example.invalid"], repo, os.environ.copy())
        run(["git", "config", "user.name", "v0.13 trial"], repo, os.environ.copy())
        run(["git", "commit", "-qm", "scratch", "--allow-empty"], repo, os.environ.copy())

        command_log = scenario_dir / "commands.log"
        env = os.environ.copy()
        env["PATH"] = f"{wrapper_dir}:{env['PATH']}"
        if scenario["id"] != "context-free-repo":
            run([str(real_day), "init"], repo, env)

        before = kan_snapshot(repo, env)
        assistant_turns = []
        raw_files = []
        last = scenario_dir / "assistant-0.txt"
        raw = scenario_dir / "events-0.jsonl"
        prompt = (
            f"Use the askme skill for this topic: {scenario['topic']!r}. "
            f"Use work subject trial/askme/{scenario['id']}. Ask exactly one question, then stop "
            "and wait for the next user turn. Do not simulate or anticipate the user's answers. "
            "If explicit consent eventually requires recording, invoke "
            f"{str(wrapper_dir / 'day')!r} acquired-input record as one standalone one-line "
            "shell command; do not use another day path or combine it with another command."
        )
        jsonl = run(
            [
                "codex", "exec", "--json", "--approve-for-me", "--sandbox", "workspace-write",
                "--model", model, "--output-last-message", str(last), prompt,
            ],
            repo,
            env,
            raw,
        )
        thread = session_id(jsonl)
        assistant_turns.append(last.read_text())
        raw_files.append(raw)

        for index, user_turn in enumerate(scenario["turns"], 1):
            last = scenario_dir / f"assistant-{index}.txt"
            raw = scenario_dir / f"events-{index}.jsonl"
            run(
                [
                    "codex", "exec", "resume", "--json", "--model", model,
                    "--output-last-message", str(last), thread, user_turn,
                ],
                repo,
                env,
                raw,
            )
            assistant_turns.append(last.read_text())
            raw_files.append(raw)

        after = kan_snapshot(repo, env)
        evidence_path = scenario_dir / "evidence.json"
        receipts = record_receipts(raw_files)
        command_log.write_text(
            "".join(json.dumps(receipt, separators=(",", ":")) + "\n" for receipt in receipts)
        )
        commands = [receipt["command"] for receipt in receipts]
        evidence_path.write_text(
            json.dumps(
                {
                    "schema": 1,
                    "id": scenario["id"],
                    "candidate_sha": candidate,
                    "user_turns": scenario["turns"],
                    "assistant_turns": assistant_turns,
                    "commands": commands,
                    "claims_before": before["claims"],
                    "claims_after": after["claims"],
                    "durable_claim_texts": after["texts"],
                },
                indent=2,
            )
            + "\n"
        )
        kan_before = scenario_dir / "kan-before.json"
        kan_after = scenario_dir / "kan-after.json"
        kan_before.write_text(before["raw"])
        kan_after.write_text(after["raw"])
        entries.append(
            {
                "id": scenario["id"],
                "path": str(evidence_path.relative_to(output_dir)),
                "sha256": sha256(evidence_path),
                "raw_events": [
                    {
                        "path": str(path.relative_to(output_dir)),
                        "sha256": sha256(path),
                    }
                    for path in raw_files
                ],
                "command_log": {
                    "path": str(command_log.relative_to(output_dir)),
                    "sha256": sha256(command_log),
                },
                "kan_before": {
                    "path": str(kan_before.relative_to(output_dir)),
                    "sha256": sha256(kan_before),
                },
                "kan_after": {
                    "path": str(kan_after.relative_to(output_dir)),
                    "sha256": sha256(kan_after),
                },
            }
        )

    version = run(["codex", "--version"], root, os.environ.copy()).strip()
    (output_dir / "manifest.json").write_text(
        json.dumps(
            {
                "schema": 1,
                "candidate_sha": candidate,
                "github_run_id": github_run_id,
                "protocol_sha256": sha256(protocol_path),
                "harness": "codex-cli",
                "harness_version": version,
                "model": model,
                "scenarios": entries,
            },
            indent=2,
        )
        + "\n"
    )
    shutil.copy2(protocol_path, output_dir / "protocol.json")


if __name__ == "__main__":
    main()
