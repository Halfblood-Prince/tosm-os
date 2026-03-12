# Latest CI feedback

- Workflow: CI
- Conclusion: failure
- Branch: main
- Commit: `3fa919970f82a9e4bb5f8f2897405329a6dbab70`
- Run ID: 23005707534
- Run attempt: 1
- Event: push
- Updated at: 2026-03-12T14:02:21Z
- Run URL: https://github.com/Halfblood-Prince/tosm-os/actions/runs/23005707534

## Job results

- Format: failure
- Clippy: success
- Tests: success
- Build: success
- Smoke: success

## Codex handoff

Codex should read `docs/status/ci-feedback.json`, the reports under `docs/status/reports/`, and the excerpts under `docs/status/logs/` before writing more code.

Recommended next action: **fix the smallest concrete failure from the latest CI excerpts before adding new scope**.

## Failure excerpt summary

- `fmt`: assert_eq!(model.row, 0);
- `fmt`: assert_eq!(model.column, 0);
- `fmt`: -            assert_eq!(model.row_bytes(row), [VgaWriterModel::BLANK; VGA_TEXT_COLUMNS]);
- `fmt`: +            assert_eq!(
- `fmt`: assert_eq!(model.row, 1);
- `fmt`: assert_eq!(model.column, 0);
- `fmt`: assert_eq!(model.row_bytes(0)[0..2], [b'A', b'B']);
- `fmt`: -        assert_eq!(model.row_bytes(1), [VgaWriterModel::BLANK; VGA_TEXT_COLUMNS]);
- `fmt`: +        assert_eq!(
- `fmt`: assert_eq!(model.row, VGA_TEXT_ROWS - 1);
- `fmt`: assert_eq!(model.column, 0);
- `fmt`: assert_eq!(model.row_bytes(VGA_TEXT_ROWS - 2)[0], b'Z');
- `fmt`: -        assert_eq!(model.row_bytes(VGA_TEXT_ROWS - 1), [VgaWriterModel::BLANK; VGA_TEXT_COLUMNS]);
- `fmt`: +        assert_eq!(
- `tests`: [1m[92m     Running[0m `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name kernel --edition=2021 kernel/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="default"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default"))' -C metadata=2a4871c6d9b9677f -C extra-filename=-9e1bdf8d8c0d7a05 --out-dir /home/runner/work/tosm-os/tosm-os/target/debug/deps -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps`
- `tests`: [1m[92m     Running[0m `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name kernel --edition=2021 kernel/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --cfg 'feature="default"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default"))' -C metadata=15d82e99752eb17b -C extra-filename=-5ba93148c5fbe0c7 --out-dir /home/runner/work/tosm-os/tosm-os/target/debug/deps -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps`
- `tests`: [1m[92m     Running[0m `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name uefi_entry --edition=2021 boot/uefi-entry/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=e6a49e033c94928f -C extra-filename=-672de0481a9d44c1 --out-dir /home/runner/work/tosm-os/tosm-os/target/debug/deps -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps --extern kernel=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libkernel-9e1bdf8d8c0d7a05.rmeta`
- `tests`: [1m[92m     Running[0m `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name uefi_entry --edition=2021 boot/uefi-entry/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=bd8e6a3497160a4a -C extra-filename=-df6ca0fed293499f --out-dir /home/runner/work/tosm-os/tosm-os/target/debug/deps -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps --extern kernel=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libkernel-9e1bdf8d8c0d7a05.rlib`
- `tests`: [1m[92m     Running[0m `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name bootx64 --edition=2021 boot/uefi-entry/src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=f8ed4a52099efd2f -C extra-filename=-60f3d71e13d89e2f --out-dir /home/runner/work/tosm-os/tosm-os/target/debug/deps -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps --extern kernel=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libkernel-9e1bdf8d8c0d7a05.rlib --extern uefi_entry=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libuefi_entry-672de0481a9d44c1.rlib`
- `tests`: test tests::boot_panic_line_bytes_include_crlf ... ok
- `tests`: test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
- `tests`: test tests::panic_message_line_matches_kernel_canonical_panic_line ... ok
- `tests`: test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
- `tests`: test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
- `tests`: [1m[92m     Running[0m `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustdoc --edition=2021 --crate-type lib --color always --crate-name kernel --test kernel/src/lib.rs --test-run-directory /home/runner/work/tosm-os/tosm-os/kernel --extern kernel=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libkernel-9e1bdf8d8c0d7a05.rlib -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps -C embed-bitcode=no --cfg 'feature="default"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default"))' --error-format human`
- `tests`: test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
- `tests`: [1m[92m     Running[0m `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustdoc --edition=2021 --crate-type lib --color always --crate-name uefi_entry --test boot/uefi-entry/src/lib.rs --test-run-directory /home/runner/work/tosm-os/tosm-os/boot/uefi-entry --extern kernel=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libkernel-9e1bdf8d8c0d7a05.rlib --extern uefi_entry=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libuefi_entry-672de0481a9d44c1.rlib -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps -C embed-bitcode=no --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' --error-format human`
- `tests`: test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
- `build`: [1m[92m     Running[0m `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name bootx64 --edition=2021 boot/uefi-entry/src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=6b5032e417c7913e -C extra-filename=-cf7c0a985f3d483b --out-dir /home/runner/work/tosm-os/tosm-os/target/debug/deps -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps --extern kernel=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libkernel-9e1bdf8d8c0d7a05.rlib --extern uefi_entry=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libuefi_entry-672de0481a9d44c1.rlib`

## Stored reports

- `docs/status/reports/rust-checks.md`
- `docs/status/reports/boot-smoke.md`
- `docs/status/logs/fmt.log`
- `docs/status/logs/clippy.log`
- `docs/status/logs/test.log`
- `docs/status/logs/build.log`
- `docs/status/logs/smoke.log`
