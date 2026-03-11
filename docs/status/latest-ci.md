# Latest CI feedback

- Workflow: CI
- Conclusion: cancelled
- Branch: main
- Commit: `26e69e5f00ddbe196a9e00e0dc40506948edf08c`
- Run ID: 22973095097
- Run attempt: 1
- Event: push
- Updated at: 2026-03-11T20:34:00Z
- Run URL: https://github.com/Halfblood-Prince/tosm-os/actions/runs/22973095097

## Job results

- Format: success
- Clippy: success
- Tests: success
- Build: success
- Smoke: failure

## Codex handoff

Codex should read `docs/status/ci-feedback.json`, the reports under `docs/status/reports/`, and the excerpts under `docs/status/logs/` before writing more code.

Recommended next action: **fix the smallest concrete failure from the latest CI excerpts before adding new scope**.

## Failure excerpt summary

- `tests`: [1m[92m     Running[0m `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name kernel --edition=2021 kernel/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="default"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default"))' -C metadata=2a4871c6d9b9677f -C extra-filename=-9e1bdf8d8c0d7a05 --out-dir /home/runner/work/tosm-os/tosm-os/target/debug/deps -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps`
- `tests`: [1m[92m     Running[0m `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name kernel --edition=2021 kernel/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --cfg 'feature="default"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default"))' -C metadata=15d82e99752eb17b -C extra-filename=-5ba93148c5fbe0c7 --out-dir /home/runner/work/tosm-os/tosm-os/target/debug/deps -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps`
- `tests`: test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
- `tests`: [1m[92m     Running[0m `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustdoc --edition=2021 --crate-type lib --color always --crate-name kernel --test kernel/src/lib.rs --test-run-directory /home/runner/work/tosm-os/tosm-os/kernel --extern kernel=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libkernel-9e1bdf8d8c0d7a05.rlib -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps -C embed-bitcode=no --cfg 'feature="default"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default"))' --error-format human`
- `tests`: test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stored reports

- `docs/status/reports/rust-checks.md`
- `docs/status/reports/boot-smoke.md`
- `docs/status/logs/fmt.log`
- `docs/status/logs/clippy.log`
- `docs/status/logs/test.log`
- `docs/status/logs/build.log`
- `docs/status/logs/smoke.log`
