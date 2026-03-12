# Latest CI feedback

- Workflow: CI
- Conclusion: failure
- Branch: main
- Commit: `d96cc268d968385f11eabf9dbc27fe18ac849b4f`
- Run ID: 22998863757
- Run attempt: 1
- Event: push
- Updated at: 2026-03-12T11:06:08Z
- Run URL: https://github.com/Halfblood-Prince/tosm-os/actions/runs/22998863757

## Job results

- Format: success
- Clippy: failure
- Tests: failure
- Build: failure
- Smoke: success

## Codex handoff

Codex should read `docs/status/ci-feedback.json`, the reports under `docs/status/reports/`, and the excerpts under `docs/status/logs/` before writing more code.

Recommended next action: **fix the smallest concrete failure from the latest CI excerpts before adding new scope**.

## Failure excerpt summary

- `clippy`: [1m[91merror[0m[1m: unwinding panics are not supported without std[0m
- `clippy`: [1m[94m= [0m[1mhelp[0m: using nightly cargo, use -Zbuild-std with panic="abort" to avoid unwinding
- `clippy`: [1m[94m= [0m[1mnote[0m: since the core library is usually precompiled with panic="unwind", rebuilding your crate with panic="abort" may not be enough to fix the problem
- `clippy`: [1m[91merror[E0152][0m[1m: duplicate lang item in crate `std` (which `test` depends on): `panic_impl`[0m
- `clippy`: [1mFor more information about this error, try `rustc --explain E0152`.[0m
- `clippy`: [1m[91merror[0m: could not compile `uefi-entry` (bin "bootx64") due to 1 previous error
- `clippy`: [1m[33mwarning[0m: build failed, waiting for other jobs to finish...
- `clippy`: [1m[91merror[0m: could not compile `uefi-entry` (bin "bootx64" test) due to 1 previous error
- `tests`: [1m[92m     Running[0m `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name kernel --edition=2021 kernel/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --cfg 'feature="default"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default"))' -C metadata=15d82e99752eb17b -C extra-filename=-5ba93148c5fbe0c7 --out-dir /home/runner/work/tosm-os/tosm-os/target/debug/deps -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps`
- `tests`: [1m[92m     Running[0m `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name kernel --edition=2021 kernel/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="default"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default"))' -C metadata=2a4871c6d9b9677f -C extra-filename=-9e1bdf8d8c0d7a05 --out-dir /home/runner/work/tosm-os/tosm-os/target/debug/deps -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps`
- `tests`: [1m[92m     Running[0m `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name uefi_entry --edition=2021 boot/uefi-entry/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=e6a49e033c94928f -C extra-filename=-672de0481a9d44c1 --out-dir /home/runner/work/tosm-os/tosm-os/target/debug/deps -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps --extern kernel=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libkernel-9e1bdf8d8c0d7a05.rmeta`
- `tests`: [1m[92m     Running[0m `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name uefi_entry --edition=2021 boot/uefi-entry/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=bd8e6a3497160a4a -C extra-filename=-df6ca0fed293499f --out-dir /home/runner/work/tosm-os/tosm-os/target/debug/deps -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps --extern kernel=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libkernel-9e1bdf8d8c0d7a05.rlib`
- `tests`: [1m[92m     Running[0m `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name bootx64 --edition=2021 boot/uefi-entry/src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=f8ed4a52099efd2f -C extra-filename=-60f3d71e13d89e2f --out-dir /home/runner/work/tosm-os/tosm-os/target/debug/deps -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps --extern kernel=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libkernel-9e1bdf8d8c0d7a05.rlib --extern uefi_entry=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libuefi_entry-672de0481a9d44c1.rlib`
- `tests`: [1m[91merror[E0152][0m[1m: duplicate lang item in crate `std` (which `test` depends on): `panic_impl`[0m
- `tests`: [1mFor more information about this error, try `rustc --explain E0152`.[0m
- `tests`: [1m[91merror[0m: could not compile `uefi-entry` (bin "bootx64" test) due to 1 previous error
- `tests`: process didn't exit successfully: `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name bootx64 --edition=2021 boot/uefi-entry/src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=f8ed4a52099efd2f -C extra-filename=-60f3d71e13d89e2f --out-dir /home/runner/work/tosm-os/tosm-os/target/debug/deps -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps --extern kernel=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libkernel-9e1bdf8d8c0d7a05.rlib --extern uefi_entry=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libuefi_entry-672de0481a9d44c1.rlib` (exit status: 1)
- `tests`: [1m[33mwarning[0m: build failed, waiting for other jobs to finish...
- `build`: [1m[92m     Running[0m `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name bootx64 --edition=2021 boot/uefi-entry/src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=6b5032e417c7913e -C extra-filename=-cf7c0a985f3d483b --out-dir /home/runner/work/tosm-os/tosm-os/target/debug/deps -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps --extern kernel=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libkernel-9e1bdf8d8c0d7a05.rlib --extern uefi_entry=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libuefi_entry-672de0481a9d44c1.rlib`
- `build`: [1m[91merror[0m[1m: unwinding panics are not supported without std[0m
- `build`: [1m[94m= [0m[1mhelp[0m: using nightly cargo, use -Zbuild-std with panic="abort" to avoid unwinding
- `build`: [1m[94m= [0m[1mnote[0m: since the core library is usually precompiled with panic="unwind", rebuilding your crate with panic="abort" may not be enough to fix the problem
- `build`: [1m[91merror[0m: could not compile `uefi-entry` (bin "bootx64") due to 1 previous error
- `build`: process didn't exit successfully: `/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name bootx64 --edition=2021 boot/uefi-entry/src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=6b5032e417c7913e -C extra-filename=-cf7c0a985f3d483b --out-dir /home/runner/work/tosm-os/tosm-os/target/debug/deps -L dependency=/home/runner/work/tosm-os/tosm-os/target/debug/deps --extern kernel=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libkernel-9e1bdf8d8c0d7a05.rlib --extern uefi_entry=/home/runner/work/tosm-os/tosm-os/target/debug/deps/libuefi_entry-672de0481a9d44c1.rlib` (exit status: 1)

## Stored reports

- `docs/status/reports/rust-checks.md`
- `docs/status/reports/boot-smoke.md`
- `docs/status/logs/fmt.log`
- `docs/status/logs/clippy.log`
- `docs/status/logs/test.log`
- `docs/status/logs/build.log`
- `docs/status/logs/smoke.log`
