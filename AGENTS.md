# AGENTS.md

## Mission

Build a small, well-structured, security-first hobby operating system in Rust.
The long-term direction is a Unix-like OS with:
- high performance
- strong isolation and memory safety
- clean architecture
- predictable builds
- incremental milestones instead of big rewrites

Do not try to build a full desktop OS in one step.
Always prefer the smallest correct next milestone that keeps the system bootable and testable.

## Product goals

Primary goals, in order:
1. Correctness
2. Reproducibility
3. Security
4. Simplicity
5. Performance
6. Ergonomics

Interpret "fast like macOS" as:
- low overhead design
- responsive boot and task scheduling
- efficient memory handling
- minimal unnecessary abstraction in hot paths

Interpret "secure like Linux" as:
- strict privilege boundaries
- careful unsafe usage
- process and memory isolation
- fail-closed defaults
- least-privilege design
- auditability

## Non-goals for now

Unless explicitly requested, do NOT:
- build a GUI
- clone macOS APIs or UX
- add networking stacks early
- add USB support early
- add package management
- add SMP/multicore support early
- add JITs, browsers, or app frameworks
- add features that require large dependency trees
- add speculative "future" abstractions without current need

## Architecture direction

Default architecture decisions:
- language: Rust stable unless a specific nightly feature is clearly required
- target architecture: x86_64 first
- boot path: UEFI preferred
- kernel style: monolithic modular kernel
- userspace model: Unix-like process model over time
- testing target: QEMU first, real hardware later
- toolchain: cargo + rustup + target-specific build tooling
- debugging: serial logs and GDB-friendly workflows
- formatting/linting: rustfmt + clippy

Avoid introducing a microkernel, capability system, custom VM model, or exotic scheduler unless there is a written design note justifying it.

## Delivery strategy

Always work in small vertical slices.
Each task must leave the repository in a state that is:
- buildable
- testable where possible
- documented
- easy to review

Preferred milestone order:
1. bootloader and entry
2. serial and screen output
3. interrupt setup
4. physical memory management
5. paging / virtual memory
6. kernel allocator
7. timer support
8. scheduler / threads
9. syscalls
10. ELF loading
11. user mode
12. basic filesystem
13. shell and simple userspace programs
14. security hardening
15. performance tuning

When unsure, continue the next unfinished milestone rather than starting a new subsystem.

## Planning rules

For any task estimated above ~200 lines changed, or any cross-cutting change:
1. Create or update `docs/plan/<short-topic>.md`
2. Write:
   - goal
   - current state
   - constraints
   - design choice
   - implementation steps
   - risks
   - verification steps
3. Then implement in small commits or logically separated patches

Do not perform broad refactors without a plan.

## Repository structure

Prefer this layout unless the repo already defines a different one:

- `kernel/` core kernel code
- `boot/` bootloader integration and boot-time code
- `arch/x86_64/` architecture-specific code
- `drivers/` device drivers
- `mm/` memory management
- `sched/` scheduling and tasking
- `fs/` filesystem code
- `user/` userspace programs and runtime
- `libs/` shared no_std crates
- `tools/` build, image, and debug tooling
- `tests/` integration and emulator-driven tests
- `docs/` design notes, plans, and architecture docs

If creating a new top-level directory, document why.

## Rust rules

Default Rust rules:
- prefer stable Rust
- prefer `#![forbid(unsafe_op_in_unsafe_fn)]`
- minimize `unsafe`
- every `unsafe` block must include a concise safety comment
- prefer `Result` over panics outside unrecoverable boot/kernel-fatal paths
- prefer explicit integer sizes in low-level code
- avoid hidden allocations
- avoid `unwrap()` in kernel code unless the failure is truly impossible and documented
- avoid macros when normal functions/types are clearer
- keep modules small and focused

Use `unsafe` only for:
- hardware interaction
- context switching
- page table manipulation
- FFI / boot protocol boundaries
- carefully justified low-level primitives

Whenever adding `unsafe`, also add:
- invariants in comments
- a test or verification note
- the smallest possible unsafe scope

## Security rules

Security is a first-class requirement.

Always prefer:
- narrow interfaces
- immutable data where practical
- checked arithmetic in non-hot paths
- bounds checks unless proven unnecessary
- explicit privilege separation
- W^X-minded memory policies where relevant
- validation at subsystem boundaries
- conservative parsing of binary formats
- zeroing or invalidating sensitive state where appropriate

Never:
- expose kernel internals directly to userspace
- trust userspace pointers or lengths
- mix privileged and unprivileged responsibilities without a written reason
- add debug backdoors that remain enabled by default
- silently ignore memory corruption indicators

When implementing syscalls or loaders:
- validate all inputs
- return clear error codes
- document invariants and attack surface

## Performance rules

Performance matters, but only after correctness.

Always:
- measure before claiming a speedup
- keep hot paths allocation-light
- avoid unnecessary copies
- prefer static dispatch in hot paths
- document algorithmic complexity when relevant

Do not:
- micro-optimize bootstrapping code prematurely
- replace readable code with clever code without evidence
- weaken safety or correctness to chase small speedups

If optimizing:
1. describe the bottleneck
2. measure before
3. change code
4. measure after
5. record results in `docs/perf/` if material

## Dependencies

Keep dependencies minimal.

Default rules:
- prefer `core`/`alloc` where possible
- prefer in-repo code for tiny low-level utilities
- avoid large transitive trees
- avoid crates that require unstable features unless justified
- avoid adding dependencies for one small helper function

When adding a dependency:
- explain why the stdlib or existing code is insufficient
- note no_std compatibility
- note maintenance and security implications
- pin or constrain versions appropriately

## Documentation rules

Every substantive subsystem must have a short design note in `docs/`.

Required when introducing a subsystem:
- purpose
- key data structures
- invariants
- failure modes
- testing approach

Update docs when code behavior changes.
Do not leave docs knowingly stale.

## Testing and verification

Every change should include the best available verification.

Use as many of these as apply:
- `cargo fmt`
- `cargo clippy -- -D warnings`
- unit tests
- integration tests
- boot test in QEMU
- screenshot/log assertion tests where practical
- image build verification
- basic smoke tests for boot and shell flow

If a task cannot be fully tested automatically:
- state what was verified
- state what remains unverified
- add a focused TODO in docs or tests, not a vague note

Do not claim something works unless it was verified.

## Build and run expectations

If the repo already defines commands, use them.
Otherwise prefer creating consistent commands such as:

- `cargo fmt --all`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all`
- `make run`
- `make qemu`
- `make image`
- `make test-integration`

When adding a new command, document it in `README.md`.

## Change discipline

Prefer narrow, reviewable changes.

For each task:
- inspect existing code first
- preserve style and naming consistency
- avoid unrelated cleanup
- avoid renaming files/symbols unless needed
- keep public interfaces small
- update docs/tests in the same change

If you discover a larger issue while implementing:
- note it
- finish the scoped task first unless blocked
- then propose a follow-up

## Commit / PR expectations

When preparing a change, include:
- what changed
- why
- risks
- how it was verified
- follow-up work

Good change summaries mention:
- subsystem touched
- design choice made
- safety impact
- test evidence

## Behavior rules for Codex

Before editing:
1. read `README.md`
2. read this file
3. inspect relevant code paths
4. inspect any docs for the subsystem
5. summarize the intended change briefly in your reasoning

When implementing:
- make the minimal coherent change
- create missing docs/tests as needed
- keep TODOs specific and actionable
- prefer completing one subsystem slice over scattering partial work

After implementing:
1. run formatting
2. run linting
3. run tests relevant to the change
4. report exact results
5. mention anything not verified

Never fabricate:
- benchmark results
- test results
- boot success
- hardware compatibility
- security guarantees

## First milestones if repo is empty

If the repository is mostly empty, build in this order:

Phase 1:
- workspace layout
- target configuration
- bootable kernel entry
- serial logging
- QEMU boot instructions
- basic CI for fmt/clippy/build

Phase 2:
- interrupt descriptor setup
- panic handling
- physical frame allocator
- paging primitives
- heap allocator

Phase 3:
- timer interrupts
- cooperative or simple preemptive scheduler
- kernel threads
- syscall boundary
- minimal user mode entry

Phase 4:
- ELF loader
- simple userspace init
- ramfs or simple filesystem
- shell with a tiny command set

Do not skip to GUI or advanced drivers before these phases exist.

## Definition of done

A task is done only when:
- code is committed cleanly
- it builds
- relevant tests pass, or missing verification is explicitly stated
- docs are updated
- no known broken partial scaffolding is left behind without explanation

## Style

Write code for maintainers.
Be explicit, boring, and correct.
Favor small interfaces, clear invariants, and incremental progress.
