# Current milestone

- Active milestone: interrupt setup
- Subtask: wire a deterministic early IDT skeleton into firmware entry and lock transcript contracts around interrupt-init ordering.
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

- Completed slice: added a kernel-owned x86_64 IDT skeleton model for the first 32 exception vectors, a boot interrupt-init serial line contract, and UEFI boot-path wiring that initializes early interrupts between banner and completion output; smoke + host transcript tests now enforce this ordering.
- Next slice: replace the shared spin-loop exception stub with vector-specific deterministic handlers and add first exception logging coverage.

<!-- ci-status:start -->
## Latest CI automation

- Last CI conclusion: success
- Last CI run: `23010411791`
- Last tested commit: `55799d3c9db9b9f8b7340162ed07564cb2f7d0f6`
- Recommended next action for Codex: continue the next unfinished milestone slice; do not redo already-green validation work
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
