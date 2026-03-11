#!/usr/bin/env python3

from __future__ import annotations

import json
import os
import pathlib
import re
from datetime import datetime, timezone

ROOT = pathlib.Path(__file__).resolve().parents[2]
ARTIFACTS_DIR = ROOT / ".ci-artifacts"
STATUS_DIR = ROOT / "docs" / "status"
REPORTS_OUT_DIR = STATUS_DIR / "reports"
LOGS_OUT_DIR = STATUS_DIR / "logs"
CURRENT_MD = STATUS_DIR / "current.md"
LATEST_CI_MD = STATUS_DIR / "latest-ci.md"
CI_FEEDBACK_JSON = STATUS_DIR / "ci-feedback.json"

REPORTS_OUT_DIR.mkdir(parents=True, exist_ok=True)
LOGS_OUT_DIR.mkdir(parents=True, exist_ok=True)


def env(name: str, default: str = "") -> str:
    return os.environ.get(name, default)


def read_text(path: pathlib.Path) -> str:
    return path.read_text(encoding="utf-8") if path.exists() else ""


def write_text(path: pathlib.Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


def find_first(pattern: str) -> pathlib.Path | None:
    matches = sorted(ARTIFACTS_DIR.glob(pattern))
    return matches[0] if matches else None


def parse_report_items(report_text: str) -> dict[str, str]:
    items: dict[str, str] = {}
    for line in report_text.splitlines():
        match = re.match(r"-\s+([^:]+):\s+(.+)$", line.strip())
        if match:
            items[match.group(1).strip()] = match.group(2).strip()
    return items


def copy_report(src: pathlib.Path | None, dest_name: str) -> str:
    if src is None or not src.exists():
        content = f"# {dest_name}\n\nReport artifact was not found for the latest CI run.\n"
    else:
        content = read_text(src)
    write_text(REPORTS_OUT_DIR / dest_name, content)
    return content


def summarize_log(src: pathlib.Path | None, dest_name: str, tail_lines: int = 200) -> dict[str, object]:
    if src is None or not src.exists():
        content = "log artifact was not found for the latest CI run\n"
        exists = False
    else:
        lines = read_text(src).splitlines()
        content = "\n".join(lines[-tail_lines:]).strip() + "\n"
        exists = True
    write_text(LOGS_OUT_DIR / dest_name, content)

    error_lines = []
    for line in content.splitlines():
        lowered = line.lower()
        if any(token in lowered for token in ["error", "failed", "panic", "assert", "undefined reference"]):
            error_lines.append(line.strip())
    return {
        "exists": exists,
        "excerpt_path": f"docs/status/logs/{dest_name}",
        "error_excerpt": error_lines[:20],
    }


def update_current_md(generated_block: str) -> None:
    current = read_text(CURRENT_MD)
    start = "<!-- ci-status:start -->"
    end = "<!-- ci-status:end -->"
    replacement = f"{start}\n{generated_block.rstrip()}\n{end}"
    if start in current and end in current:
        current = re.sub(
            rf"{re.escape(start)}.*?{re.escape(end)}",
            replacement,
            current,
            flags=re.DOTALL,
        )
    else:
        current = current.rstrip() + "\n\n" + replacement + "\n"
    write_text(CURRENT_MD, current)


workflow = env("WORKFLOW_NAME", "CI")
run_id = env("RUN_ID", "")
conclusion = env("CONCLUSION", "unknown")
head_branch = env("HEAD_BRANCH", "")
head_sha = env("HEAD_SHA", "")
run_attempt = env("RUN_ATTEMPT", "")
server_url = env("SERVER_URL", "https://github.com")
repo = env("REPOSITORY", "")
event_name = env("EVENT_NAME", "")
updated_at = datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")
run_url = f"{server_url}/{repo}/actions/runs/{run_id}" if repo and run_id else ""

rust_report_path = find_first("**/rust-checks.md")
smoke_report_path = find_first("**/boot-smoke.md")

rust_report = copy_report(rust_report_path, "rust-checks.md")
smoke_report = copy_report(smoke_report_path, "boot-smoke.md")

fmt_log = summarize_log(find_first("**/fmt.log"), "fmt.log")
clippy_log = summarize_log(find_first("**/clippy.log"), "clippy.log")
test_log = summarize_log(find_first("**/test.log"), "test.log")
build_log = summarize_log(find_first("**/build.log"), "build.log")
smoke_log = summarize_log(find_first("**/smoke.log"), "smoke.log")

rust_items = parse_report_items(rust_report)
smoke_items = parse_report_items(smoke_report)

job_results = {
    "format": rust_items.get("Format", "unknown"),
    "clippy": rust_items.get("Clippy", "unknown"),
    "tests": rust_items.get("Tests", "unknown"),
    "build": rust_items.get("Build", "unknown"),
    "smoke": smoke_items.get("Smoke test", "unknown"),
}

if conclusion == "success":
    codex_next = "continue the next unfinished milestone slice; do not redo already-green validation work"
elif conclusion == "unknown":
    codex_next = "read the generated CI files after the first post-merge automation run"
else:
    codex_next = "fix the smallest concrete failure from the latest CI excerpts before adding new scope"

feedback = {
    "workflow": workflow,
    "run_id": run_id,
    "run_attempt": run_attempt,
    "conclusion": conclusion,
    "head_branch": head_branch,
    "head_sha": head_sha,
    "event": event_name,
    "run_url": run_url,
    "updated_at": updated_at,
    "job_results": job_results,
    "codex_next_action": codex_next,
    "reports": {
        "rust_checks": "docs/status/reports/rust-checks.md",
        "boot_smoke": "docs/status/reports/boot-smoke.md",
    },
    "log_excerpts": {
        "fmt": fmt_log["excerpt_path"],
        "clippy": clippy_log["excerpt_path"],
        "tests": test_log["excerpt_path"],
        "build": build_log["excerpt_path"],
        "smoke": smoke_log["excerpt_path"],
    },
}
write_text(CI_FEEDBACK_JSON, json.dumps(feedback, indent=2) + "\n")

failure_lines = []
for label, info in [
    ("fmt", fmt_log),
    ("clippy", clippy_log),
    ("tests", test_log),
    ("build", build_log),
    ("smoke", smoke_log),
]:
    for line in info["error_excerpt"]:
        failure_lines.append(f"- `{label}`: {line}")
if not failure_lines:
    failure_lines = ["- No obvious error lines were detected in the stored log excerpts."]

latest_ci = f"""# Latest CI feedback

- Workflow: {workflow}
- Conclusion: {conclusion}
- Branch: {head_branch or 'unknown'}
- Commit: `{head_sha or 'unknown'}`
- Run ID: {run_id or 'unknown'}
- Run attempt: {run_attempt or 'unknown'}
- Event: {event_name or 'unknown'}
- Updated at: {updated_at}
- Run URL: {run_url or 'unavailable'}

## Job results

- Format: {job_results['format']}
- Clippy: {job_results['clippy']}
- Tests: {job_results['tests']}
- Build: {job_results['build']}
- Smoke: {job_results['smoke']}

## Codex handoff

Codex should read `docs/status/ci-feedback.json`, the reports under `docs/status/reports/`, and the excerpts under `docs/status/logs/` before writing more code.

Recommended next action: **{codex_next}**.

## Failure excerpt summary

{"\n".join(failure_lines)}

## Stored reports

- `docs/status/reports/rust-checks.md`
- `docs/status/reports/boot-smoke.md`
- `docs/status/logs/fmt.log`
- `docs/status/logs/clippy.log`
- `docs/status/logs/test.log`
- `docs/status/logs/build.log`
- `docs/status/logs/smoke.log`
"""
write_text(LATEST_CI_MD, latest_ci.rstrip() + "\n")

current_generated = f"""## Latest CI automation

- Last CI conclusion: {conclusion}
- Last CI run: `{run_id or 'unknown'}`
- Last tested commit: `{head_sha or 'unknown'}`
- Recommended next action for Codex: {codex_next}
- Detailed summary: `docs/status/latest-ci.md`
"""
update_current_md(current_generated)
