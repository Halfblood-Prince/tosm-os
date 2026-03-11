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

## Codex operating contract

Codex writes code and docs only.
Codex must **not** treat local execution as verification and must **not** claim validation was completed unless the generated GitHub status files say so.

Before making changes, always read these files in order:
1. `docs/status/current.md`
2. `docs/status/ci-feedback.json`
3. `docs/status/latest-ci.md`
4. `docs/status/reports/*.md`
5. `docs/status/logs/*.log`

Decision rules:
- If the latest CI conclusion is not `success`, fix the smallest concrete failure shown in the stored reports/log excerpts before starting new feature work.
- If the latest CI conclusion is `success`, continue the next unfinished milestone slice.
- Never mark checks as passed based on local assumptions.
- Never delete generated status evidence just to make the repo look clean.
- After writing code, update the relevant plan/status docs, then stop. Verification is delegated to GitHub Actions after merge to `main`.

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

Verification steps in plan docs are the commands GitHub Actions is expected to run after merge. Codex may reference them, but should not claim they were run locally unless a human explicitly provides the results.

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
