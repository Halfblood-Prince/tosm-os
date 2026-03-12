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
- Maintenance update: fixed the CI Clippy blocker by routing early exception handler address capture through an explicit function-pointer-to-raw-pointer conversion helper so `-D warnings` no longer fails on `function-casts-as-integer`.
- Next slice: replace the shared spin-loop exception stub with vector-specific deterministic handlers and add first exception logging coverage.

<!-- ci-status:start -->
## Latest CI automation

- Last CI conclusion: failure
- Last CI run: `23010865488`
- Last tested commit: `5714a67039ff5dcbfe2216c476df36c15875a1ad`
- Recommended next action for Codex: fix the smallest concrete failure from the latest CI excerpts before adding new scope
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
