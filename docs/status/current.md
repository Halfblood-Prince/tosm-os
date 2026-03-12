# Current milestone

- Active milestone: physical memory management
- Subtask: begin deterministic early physical-memory map model and reporting contracts.
- Status: in progress
- Note: Codex writes code/docs only and waits for GitHub Actions feedback after merge to `main`.

## Done criteria
- [x] make fmt
- [x] make lint
- [x] make test
- [x] make build
- [x] make smoke
- [x] docs updated

## Progress update

- Completed slice: replaced the single shared early exception handler pointer with 32 vector-specific early stubs and wired the early IDT population path so each exception vector gets its own deterministic handler address.
- Completed slice: added kernel-owned deterministic exception log line contracts for vectors 0-31 plus an unknown-vector fallback, with host tests and smoke contract checks covering representative vectors and fallback behavior.
- Completed slice: wired firmware-path exception dispatch logging through kernel dispatch contracts (including CRLF-safe byte accessors), emitting a deterministic vector 14 page-fault line between interrupt init and completion logs.
- Completed slice: extended host transcript/smoke contracts so boot output ordering now enforces banner -> idt init -> exception vector 14 -> completion.
- Completed slice: added a deterministic early physical-memory map model in `kernel` with stable region classification and totals reporting (usable/reserved/highest usable end), plus canonical CRLF memory-init serial contracts.
- Completed slice: integrated memory-init reporting into UEFI boot transcript output and smoke/QEMU contract checks so boot ordering is now banner -> idt init -> exception vector 14 -> memory init -> completion.
- Next slice: use the deterministic memory model to start paging bootstrap decisions (frame-window selection and initial mapping plan contracts).

<!-- ci-status:start -->
## Latest CI automation

- Last CI conclusion: success
- Last CI run: `23012312022`
- Last tested commit: `849910c3100d7fd87296a55ed5008b96ff3c257a`
- Recommended next action for Codex: continue the next unfinished milestone slice; do not redo already-green validation work
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
