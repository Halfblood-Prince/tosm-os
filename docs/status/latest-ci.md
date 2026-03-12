# Latest CI feedback

- Workflow: CI
- Conclusion: failure
- Branch: main
- Commit: `5fe8e4ec5c215b7886fe4eabe2f7fd919610cca2`
- Run ID: 23007240241
- Run attempt: 1
- Event: push
- Updated at: 2026-03-12T14:36:07Z
- Run URL: https://github.com/Halfblood-Prince/tosm-os/actions/runs/23007240241

## Job results

- Format: failure
- Clippy: success
- Tests: failure
- Build: success
- Smoke: success

## Codex handoff

Codex should read `docs/status/ci-feedback.json`, the reports under `docs/status/reports/`, and the excerpts under `docs/status/logs/` before writing more code.

Recommended next action: **fix the smallest concrete failure from the latest CI excerpts before adding new scope**.

## Failure excerpt summary

- `fmt`: assert_eq!(model.row, 1);
- `fmt`: assert_eq!(model.column, 0);
- `fmt`: assert_eq!(model.row_text_prefix(0), b"tosm-os: panic in uefi-entry");
- `fmt`: -        assert_ne!(
- `fmt`: +        assert_ne!(model.row_text_prefix(0), b"tosm-os: kernel entry reached");
- `fmt`: assert_ne!(model.row_text_prefix(0), b"tosm-os: efi_main completed");
- `fmt`: assert_eq!(
- `tests`: [1m[92m     Running[0m `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name kernel --edition=2021 kernel/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="default"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default"))' -C metadata=2a4871c6d9b9677f -C extra-filename=-9e1bdf8d8c0d7a05 --out-dir /home/runner/work/tosm-os/tosm-os/target/debug/deps -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps`
- `tests`: [1m[92m     Running[0m `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name kernel --edition=2021 kernel/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --cfg 'feature="default"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default"))' -C metadata=15d82e99752eb17b -C extra-filename=-5ba93148c5fbe0c7 --out-dir /home/runner/work/tosm-os/tosm-os/target/debug/deps -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps`
- `tests`: [1m[92m     Running[0m `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name uefi_entry --edition=2021 boot/uefi-entry/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=e6a49e033c94928f -C extra-filename=-672de0481a9d44c1 --out-dir /home/runner/work/tosm-os/tosm-os/target/debug/deps -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps --extern kernel=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libkernel-9e1bdf8d8c0d7a05.rmeta`
- `tests`: [1m[92m     Running[0m `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name uefi_entry --edition=2021 boot/uefi-entry/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=bd8e6a3497160a4a -C extra-filename=-df6ca0fed293499f --out-dir /home/runner/work/tosm-os/tosm-os/target/debug/deps -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps --extern kernel=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libkernel-9e1bdf8d8c0d7a05.rlib`
- `tests`: [1m[92m     Running[0m `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name bootx64 --edition=2021 boot/uefi-entry/src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=f8ed4a52099efd2f -C extra-filename=-60f3d71e13d89e2f --out-dir /home/runner/work/tosm-os/tosm-os/target/debug/deps -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps --extern kernel=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libkernel-9e1bdf8d8c0d7a05.rlib --extern uefi_entry=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libuefi_entry-672de0481a9d44c1.rlib`
- `tests`: test tests::boot_panic_line_bytes_include_crlf ... ok
- `tests`: test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
- `tests`: test tests::model_boot_transcript_renders_banner_then_done_on_distinct_rows ... FAILED
- `tests`: test tests::model_panic_line_renders_on_first_row_after_init ... ok
- `tests`: test tests::model_panic_transcript_reinitializes_screen_and_removes_old_boot_lines ... FAILED
- `tests`: test tests::panic_message_line_matches_kernel_canonical_panic_line ... ok
- `tests`: thread 'tests::model_boot_transcript_renders_banner_then_done_on_distinct_rows' (2617) panicked at boot/uefi-entry/src/lib.rs:463:9:
- `tests`: assertion `left == right` failed
- `tests`: ---- tests::model_panic_transcript_reinitializes_screen_and_removes_old_boot_lines stdout ----
- `tests`: thread 'tests::model_panic_transcript_reinitializes_screen_and_removes_old_boot_lines' (2621) panicked at boot/uefi-entry/src/lib.rs:483:9:
- `tests`: assertion `left == right` failed
- `tests`: tests::model_panic_transcript_reinitializes_screen_and_removes_old_boot_lines
- `tests`: test result: FAILED. 12 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
- `tests`: [1m[91merror[0m: test failed, to rerun pass `-p uefi-entry --lib`
- `build`: [1m[92m     Running[0m `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name bootx64 --edition=2021 boot/uefi-entry/src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=6b5032e417c7913e -C extra-filename=-cf7c0a985f3d483b --out-dir /home/runner/work/tosm-os/tosm-os/target/debug/deps -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps --extern kernel=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libkernel-9e1bdf8d8c0d7a05.rlib --extern uefi_entry=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libuefi_entry-672de0481a9d44c1.rlib`

## Stored reports

- `docs/status/reports/rust-checks.md`
- `docs/status/reports/boot-smoke.md`
- `docs/status/logs/fmt.log`
- `docs/status/logs/clippy.log`
- `docs/status/logs/test.log`
- `docs/status/logs/build.log`
- `docs/status/logs/smoke.log`
