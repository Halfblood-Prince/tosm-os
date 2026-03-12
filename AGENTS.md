# AGENTS.md

Agent operating guide for this repository.

This file explains how an autonomous coding agent should work in this repo.
For development strategy and milestone progression, also read `CODEX.md`.
If `CODEX.md` exists, treat it as the primary strategic instruction set.
`AGENTS.md` defines repository mechanics, validation flow, and working rules.

---

## Instruction priority

Follow instructions in this order:

1. direct user request
2. `CODEX.md`
3. `AGENTS.md`
4. inline code comments and local module conventions
5. general best practices

If `CODEX.md` says to continue from the current state and push the next milestone, do that.

---

## Repository purpose

This repository contains a hobby operating system project.

The goal is steady forward progress toward a usable hobby OS, not endless refactoring or tiny cosmetic edits.

Agents should prefer shipping working subsystems over repeatedly polishing already-working code.

---

## Working mode

Before making changes:

1. inspect the repo structure
2. identify the current implemented milestone
3. identify the next incomplete milestone
4. make a coherent implementation plan
5. execute meaningful progress in one run

Do not spend a run making trivial edits unless the user asked for a trivial edit.

Prefer substantial, coherent progress over micro-patches.

---

## Required startup behavior

At the start of work, the agent should inspect at least:

- `CODEX.md`
- `README.md`
- build files (`Makefile`, `Cargo.toml`, workspace manifests)
- boot configuration
- kernel entrypoints
- architecture-specific folders
- test and smoke-test scripts
- `docs/status/` if present

Use the current repository state as the source of truth.

Do not restart the project from scratch.
Do not replace working subsystems without a strong reason.

---

## Development strategy

The agent should work milestone-by-milestone.

Typical milestone sequence for this repo class:

- boot path works
- kernel entry works
- screen / serial output works
- panic and logging works
- memory initialization works
- interrupts and exceptions work
- timer works
- keyboard input works
- shell / command loop works
- storage / filesystem basics work
- tasking / process model basics work

If the repo already passed one stage, move to the next one.

---

## Milestone completion rule

When a milestone is functional enough for a hobby OS, mark it complete and move on.

A milestone should be considered complete when it is:

- implemented
- integrated
- buildable
- not obviously broken
- sufficient to unblock the next subsystem

Do not keep reworking the same completed milestone for marginal improvements.

Examples of bad behavior:

- repeatedly rewriting VGA or framebuffer output after it already works
- endlessly renaming modules without enabling new functionality
- spending many runs on stylistic cleanup while major subsystems are missing

Examples of good behavior:

- output works -> move to interrupts
- interrupts work -> move to timer / keyboard
- keyboard works -> move to shell input
- shell works -> move to storage or tasking

---

## Expected change size

Default expectation: make meaningful progress, not tiny edits.

Preferred patch size:
- usually 100-800 lines when implementing a subsystem
- may be larger if the change is coherent and safe
- may be smaller only when the task truly requires it

Do not artificially constrain changes to very small patches.

A good run may include:
- multiple related files
- new module(s)
- integration wiring
- tests or smoke-test updates
- status document updates when appropriate

---

## What to optimize for

Optimize for:

- forward progress
- working code
- coherent subsystem implementation
- clear boundaries between modules
- bootability
- debuggability
- maintainability sufficient for a hobby OS

Do not optimize for:
- overengineering
- speculative abstractions
- premature performance tuning
- large-scale rewrites without need

---

## Build and validation

Before finishing, run the relevant validation commands if available.

Rust validation flow:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --verbose
cargo build --workspace --verbose

If this repo uses a different command set, use the repo’s actual commands.

If the repository includes a smoke test, run it:

./tools/smoke-test.sh

If QEMU smoke testing exists, use it when practical.

Do not claim success without checking whether the code still builds.

Failure handling

If validation fails:

fix the failure if reasonably possible in the same run

avoid introducing unrelated changes

leave the repository in a more working state than before

If a full fix is not possible, prioritize:

restoring buildability

isolating unfinished code

documenting the blocker clearly

Do not leave the repo in a knowingly broken state unless the user explicitly asked for exploratory changes.

Status tracking

If docs/status/ exists, treat it as the progress ledger.

When completing a milestone or materially advancing one:

update the relevant status file

mark completed milestones clearly

note the next target milestone

keep entries concise and factual

Do not spam status files with trivial updates.

Status updates should reflect real progress, not intent alone.

Coding rules

Prefer:

small, clear modules

explicit names

simple control flow

minimal unsafe surface area

comments where hardware or boot assumptions matter

integration with existing repo style

Avoid:

broad rewrites with no milestone gain

introducing unused abstractions

duplicate subsystems

dead code

placeholder-only implementations unless clearly labeled and necessary

OS-specific guidance

For hobby OS work:

choose the simplest implementation that advances the next milestone

prefer reliable debug output early

prioritize boot/debug/interrupt/memory/input before advanced features

keep architecture assumptions explicit

document unsafe and low-level invariants near the code

When choosing between elegance and bootable progress, prefer bootable progress.

Repo navigation hints

Agents should quickly identify and understand:

bootloader or boot entry code

kernel entry and init path

memory initialization

interrupt descriptor setup

device input path

build pipeline

emulator / smoke-test tooling

If the repo uses architecture folders, inspect those before changing cross-cutting code.

Autonomy policy

The agent is expected to continue from the repo’s current state with minimal hand-holding.

When invoked with:

Read CODEX.md and follow the instructions there.

the agent should:

read CODEX.md

inspect the current repo state

determine the next incomplete milestone

implement a meaningful slice toward that milestone

validate the result

update status tracking if appropriate

Do not wait for extra prompting if the next milestone is reasonably clear.

Commit-quality standard

A good autonomous run should usually produce one of:

a completed milestone

a clearly advanced milestone

a working subsystem skeleton integrated into the build

a fix that unblocks the next milestone

A bad run is one that only:

tweaks wording

renames files pointlessly

makes cosmetic cleanup

adds tiny fragments without integration

repeats optimization of already-good-enough code

Human-facing output expectations

When reporting work, summarize:

what milestone was targeted

what changed

what now works

what remains next

Keep it brief and concrete.

Avoid overstating completeness.
