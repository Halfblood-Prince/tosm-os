## Codex execution protocol

For every task:
1. Read `README.md`, `AGENTS.md`, `docs/status/current.md`, and relevant subsystem docs.
2. Choose the next unfinished milestone only.
3. If the task is >200 LOC or cross-cutting, create/update `docs/plan/<topic>.md`.
4. Make the minimal coherent change.
5. Update tests/docs in the same change.
6. Run:
   - cargo fmt --all
   - cargo clippy --all-targets --all-features -- -D warnings
   - cargo test --all
   - make smoke
7. Do not mark a milestone complete unless CI passes.
8. If CI fails, treat workflow logs and PR feedback comment as the source of truth.
