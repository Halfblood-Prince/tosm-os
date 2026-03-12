# Current milestone

- Active milestone: interrupt setup
- Subtask: ship vector-specific early exception stubs plus deterministic exception-log message contracts for vectors 0-31.
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
- Next slice: wire real exception dispatch logging in firmware path (or a host model of it) so an observed exception vector can emit the matching deterministic serial line.

<!-- ci-status:start -->
## Latest CI automation

- Last CI conclusion: success
- Last CI run: `23011136363`
- Last tested commit: `e9f024329eb2704f7f62ddae2f363aa2bc13c5e7`
- Recommended next action for Codex: continue the next unfinished milestone slice; do not redo already-green validation work
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
