# Codex Execution Prompt

Read AGENTS.md and the files under docs/status/ first.

You are working inside an automated CI-driven development workflow.
Your job is to advance the repository toward the next milestone by implementing a complete milestone slice.

Follow this contract strictly.

Development Contract
Allowed actions

Write code and documentation only.

You may modify any files in the repository.

You may add new files when needed.

Prefer implementing complete functionality rather than partial scaffolding.

# Forbidden actions

- Do not run builds, tests, or commands locally.

- Do not claim tests passed unless docs/status/ explicitly says so.

- Do not fabricate CI results.

- Do not claim verification that has not been recorded in the status docs.

Source of Truth

When determining project state, use these sources in priority order:

docs/status/ci-feedback.json

docs/status/latest-ci.md

docs/status/reports/

docs/status/logs/

docs/status/current.md

If current.md conflicts with CI artifacts, trust CI artifacts.

CI Handling Rules
If latest CI failed

Identify the smallest concrete failure from:

docs/status/logs/

docs/status/reports/

Then fix only that failure with the smallest change that resolves it.

Do not implement new features until CI failures are resolved.

If latest CI succeeded

Advance the project by implementing one complete milestone slice.

A milestone slice must:

create a real new capability

advance the milestone toward completion

leave the repository in a strictly more complete state

Examples of acceptable slices include:

implementing a loader stage

wiring kernel entry

building an EFI application

adding image creation

connecting boot stages

adding minimal runtime support

implementing a build step required for boot

Avoid cosmetic edits or documentation-only changes unless necessary.

Commit Scope Rules

To prevent micro-changes:

Prefer coherent multi-file edits when required.

Do not intentionally minimize the diff.

Avoid commits smaller than ~30 lines unless the change is truly trivial.

Implement the entire slice end-to-end, not partial scaffolding.

The goal is steady milestone progress, not minimal diffs.

Slice Selection

Determine the next slice by reading:

docs/plan/

docs/status/current.md

Choose the next unfinished milestone slice.

Do not repeat slices that are already completed.

If the milestone appears complete, mark it complete and advance to the next milestone defined in docs/plan.

Required Workflow

Perform the following steps:

1. Summarize project state

Summarize the current state using:

docs/status/current.md

docs/status/ci-feedback.json

docs/status/latest-ci.md

docs/status/reports/

docs/status/logs/

docs/plan/

Your summary must identify:

current milestone

CI status

last completed slice

next unfinished slice

2. Select the next slice

Identify exactly one next milestone slice.

Explain briefly why it is the correct next step.

3. Implement the slice

Write the code necessary to implement the slice fully.

You may:

modify multiple files

add modules

add build steps

update documentation

Ensure the change moves the milestone forward.

4. Update project status

Update docs/status/current.md to reflect:

current milestone

completed slice

status

next slice

brief explanation of progress

If the milestone is complete, mark it complete and identify the next milestone.

Quality Rules

Prefer simple, deterministic implementations.

Avoid unnecessary abstractions.

Keep code readable and minimal.

Ensure the repository remains internally consistent.

Completion Condition

Stop after:

implementing one milestone slice

updating status documentation

Do not implement multiple slices in a single run.

Final Output

Your response should include:

Project state summary

Selected milestone slice

Implemented changes

Updated docs/status/current.md
