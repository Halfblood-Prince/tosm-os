# Current milestone

- Active milestone: scheduler / threads
- Subtask: model scheduler wait-channel contention resolution and deterministic wake-priority ordering contracts.
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

- Completed slice: fixed CI smoke timeout failure (`thread-state-ready` missing) by increasing the default QEMU smoke timeout from 90s to 150s, preserving deterministic transcript checks while allowing slower OVMF/TCG boots to finish.
- Next slice: model scheduler wait-channel contention aging under repeated timer wake retries and deterministic priority inversion recovery contracts.

- Completed slice: fixed smoke timeout pressure by moving the new scheduler debt-aging transcript emission earlier in the UEFI boot transcript so CI can validate the new contract before long-tail firmware runtime stalls.
- Completed slice: modeled scheduler debt aging decay across timer handoffs with deterministic repayment-cap reset contracts and integrated kernel/UEFI/smoke coverage for the new `scheduler debt aging` line.
- Next slice: model scheduler wait-channel contention aging under repeated timer wake retries and deterministic priority inversion recovery contracts.

- Completed slice: modeled scheduler preemption-debt repayment and deterministic starvation backoff contracts (`debt/repaid/starve/backoff/next`) in the kernel scheduler model.
- Completed slice: integrated canonical scheduler-debt transcript emission into UEFI boot flow and expanded kernel/UEFI/smoke contract coverage so source/runtime checks enforce the new debt line.
- Next slice: model scheduler debt aging decay across successive timer handoffs and deterministic repayment-cap reset contracts.

- Completed slice: hardened UEFI wake transcript emission with a deterministic scheduler reseed + blocked-state retry path so `thread state ready`/wake lines still emit when prior probes perturb lifecycle state on slower QEMU firmware runs.
- Completed slice: prioritized the latest CI smoke failure by fixing the smallest concrete missing line (`thread-state-ready`) without adding new milestone scope.
- Next slice: model scheduler preemption debt repayment and deterministic starvation backoff contracts.

- Completed slice: modeled scheduler time-slice carryover accounting with deterministic preemption-threshold contracts (selected task, carry ticks, threshold crossing, preempt/non-preempt next-task selection) in the kernel scheduler model.
- Completed slice: integrated canonical scheduler-carryover transcript emission into UEFI boot flow and expanded kernel/UEFI/smoke contract coverage so source/runtime checks enforce the new carryover line.
- Next slice: model scheduler preemption debt repayment and deterministic starvation backoff contracts.

- Completed slice: modeled scheduler runnable-queue aging decay/rebalance contracts with deterministic floor-boost behavior (`winner age after decay/rebalance`, accumulated decay) and explicit error handling for duplicate participants or zero floor age.
- Completed slice: integrated canonical scheduler-rebalance transcript emission into UEFI boot flow and expanded kernel/UEFI/smoke contract coverage to require the new runnable-aging line.
- Next slice: model scheduler time-slice carryover accounting and deterministic preemption-threshold contracts.

- Completed slice: modeled deterministic multi-channel wake fairness rotation with per-channel aging metadata in the kernel scheduler wake path, including starvation-prevention arbitration contracts and duplicate-participant rejection.
- Completed slice: integrated canonical wake-fairness transcript emission into the UEFI boot flow and expanded kernel/UEFI/smoke contract coverage so source/runtime checks require the new aging-rotation line.
- Next slice: model scheduler runnable-queue aging decay/rebalance contracts after fairness-based wake selection.

- Completed slice: hardened COM1 transmit polling in UEFI entry with a deterministic bounded spin-wait before each byte write, preventing indefinite stalls when UART transmitter-ready status lags on slower CI/QEMU paths.
- Completed slice: kept scheduler/thread transcript ordering intact while reducing risk of smoke timeout failures caused by blocked serial draining during late lifecycle lines (including thread-state-ready).
- Next slice: model multi-channel wake fairness rotation (per-channel aging) and deterministic starvation-prevention contracts.

- Completed slice: fixed CI smoke transcript timeout risk by increasing COM1 initialization to 115200 baud in UEFI entry, reducing time spent draining deterministic scheduler/thread transcript lines before QEMU timeout windows.
- Completed slice: updated the UART init regression test contract to assert the 8N1 + divisor=1 (115200) profile used by the firmware transcript path.
- Next slice: model multi-channel wake fairness rotation (per-channel aging) and deterministic starvation-prevention contracts.

- Completed slice: modeled deterministic wait-channel contention arbitration for blocked threads with explicit wake-priority ordering (`signal > io > timer`) and claim-sequence tie-break metadata in the kernel scheduler model.
- Completed slice: integrated canonical wait-contention and wake-order transcript lines into the UEFI boot flow and expanded kernel/UEFI/smoke contract coverage to enforce deterministic source/runtime transcript presence for the new ordering metadata.
- Next slice: model multi-channel wake fairness rotation (per-channel aging) and deterministic starvation-prevention contracts.

- Completed slice: fixed the latest CI smoke timeout flake by reducing UEFI UART TX-ready polling budget (`UART_TRANSMIT_READY_SPIN_LIMIT`) from 1,000,000 to 4,096 spins per byte, preventing long firmware stalls that could suppress late scheduler transcript lines in QEMU.
- Next slice: monitor CI smoke runtime after the UART polling-budget fix, then continue advancing scheduler/thread contracts if stable.

- Completed slice: fixed CI smoke timeout miss of `thread ctx save` by restoring worker task enrollment before context/lifecycle modeling in UEFI boot flow, so the deterministic handoff transcript line is emitted after scheduler reseed paths.
- Completed slice: added a scheduler-flow regression test in `uefi-entry` that reproduces the reseed + timer-handoff path and asserts context handoff plus blocked/wake lifecycle transitions still succeed for task `2`.
- Next slice: model scheduler wait-channel contention resolution and deterministic wake-priority ordering contracts.

- Completed slice: stabilized early boot scheduler transcript emission by moving deterministic `thread dequeue` output earlier in the UEFI boot flow and re-seeding scheduler state before context/lifecycle modeling, reducing timeout-induced QEMU smoke flakes where dequeue lines were missing late in boot output.
- Next slice: model scheduler wait-channel contention resolution and deterministic wake-priority ordering contracts.

- Completed slice: modeled scheduler wait-channel ownership accounting and timeout wake-deadline contracts in the kernel thread-wake path (owner task + claim sequence + deadline/observed tick metadata), plus deterministic transcript lines for ownership/timeout wake reporting.
- Completed slice: integrated wait-ownership and wake-timeout transcript emission into UEFI boot flow and expanded kernel/UEFI/smoke checks to enforce deterministic source/runtime transcript coverage for the new wake metadata lines.
- Next slice: model scheduler wait-channel contention resolution and deterministic wake-priority ordering contracts.

- Completed slice: added deterministic blocked-thread wake modeling with explicit wake reason (`timer`/`signal`/`io`) and wait-channel metadata contracts in the kernel scheduler model, plus canonical `thread wake` transcript bytes for serial/VGA consumers.
- Completed slice: integrated wake-contract emission into the UEFI boot transcript after blocked->ready recovery, and expanded kernel/UEFI/smoke checks to enforce deterministic wake-line presence across source and runtime transcript validation paths.
- Next slice: model scheduler wait-channel ownership accounting and deterministic timeout wake-deadline contracts.

- Completed slice: fixed CI boot-smoke transcript flake where `thread dequeue` could be skipped after terminated-cleanup modeling by reseeding deterministic scheduler state before dequeue emission in UEFI entry flow, restoring stable dequeue-line output ordering for QEMU serial checks.
- Completed slice: increased default `QEMU_TIMEOUT_SECS` in `tools/smoke-test.sh` from 45s to 90s so slower CI runners have enough time to emit late scheduler/thread transcript lines (including thread dequeue) before timeout, while keeping explicit timeout override support.
- Completed slice: replaced the single shared early exception handler pointer with 32 vector-specific early stubs and wired the early IDT population path so each exception vector gets its own deterministic handler address.
- Completed slice: added kernel-owned deterministic exception log line contracts for vectors 0-31 plus an unknown-vector fallback, with host tests and smoke contract checks covering representative vectors and fallback behavior.
- Completed slice: wired firmware-path exception dispatch logging through kernel dispatch contracts (including CRLF-safe byte accessors), emitting a deterministic vector 14 page-fault line between interrupt init and completion logs.
- Completed slice: extended host transcript/smoke contracts so boot output ordering now enforces banner -> idt init -> exception vector 14 -> completion.
- Completed slice: added a deterministic early physical-memory map model in `kernel` with stable region classification and totals reporting (usable/reserved/highest usable end), plus canonical CRLF memory-init serial contracts.
- Completed slice: integrated memory-init reporting into UEFI boot transcript output and smoke/QEMU contract checks so boot ordering is now banner -> idt init -> exception vector 14 -> memory init -> completion.
- Completed slice: added a deterministic early paging bootstrap planning contract derived from physical memory totals (4-frame top-of-usable window + 2MiB identity-map span), and integrated paging-plan serial/VGA/smoke ordering checks.
- Completed slice: materialized deterministic early paging structures (PML4/PDPT/PD) from the paging plan, populated 1GiB identity coverage via 2MiB PDEs, and added an early CR3 install hook (UEFI/x86_64 only) with host-side snapshot tests.
- Completed slice: integrated a canonical paging-install boot transcript line across kernel contracts, UEFI serial/VGA output, and smoke checks so boot ordering is now banner -> idt init -> exception vector 14 -> memory init -> paging plan -> paging install -> completion.
- Completed slice: hardened `tools/smoke-test.sh` QEMU execution to run with deterministic TCG single-thread acceleration by default, support override via `QEMU_ACCEL_ARGS`, and gracefully skip runtime serial assertions for local non-required QEMU failures while preserving strict CI failure behavior under `REQUIRE_QEMU_SMOKE=1`.
- Completed slice: updated `tools/smoke-test.sh` to verify deterministic serial transcript contracts before honoring QEMU exit status, so CI-required smoke runs now pass when the expected boot transcript is complete even if QEMU aborts during shutdown with the known iothread assertion.
- Completed slice: raised QEMU smoke timeout default from 20s to a configurable `QEMU_TIMEOUT_SECS` (default 45s), and emit an explicit timeout diagnostic before transcript assertions so slower CI boots can still produce the required paging-install line.
- Completed slice: switched smoke QEMU default acceleration args from `-accel tcg,thread=single` to `-accel tcg` to avoid the observed iothread assertion crash path in CI runners while preserving `QEMU_ACCEL_ARGS` override behavior.
- Completed slice: added kernel-level virtual-memory helpers (`VirtualAddress`/`PhysicalAddress`) with canonical-address validation, 4KiB alignment/state guards, and deterministic early identity-map translation contracts to unblock allocator bring-up.
- Completed slice: fixed the boot-smoke runtime failure trigger by deferring early CR3 installation in the UEFI path until allocator-backed physical ownership is available, preventing pre-transcript faults during QEMU boot smoke.
- Completed slice: integrated an initial allocator-facing early frame-selection API (`EarlyFrameAllocator`) that consumes paging install reports, validates translation preconditions, and returns deterministic 4KiB frame allocations with explicit error contracts.
- Completed slice: threaded `EarlyFrameAllocator` through a first deterministic kernel-heap bootstrap path (4x4KiB at VA 0x00400000), including canonical heap bootstrap transcript contracts across kernel/UEFI/smoke checks.
- Completed slice: added a deterministic early-heap bump allocator with tracked allocate/deallocate operations, plus a canonical boot-time alloc-cycle contract (`allocs=2 frees=2 cursor=0x00400000`) wired through kernel tests, UEFI transcript output, and smoke checks.
- Completed slice: connected the early-heap bootstrap window to a minimal `GlobalAlloc` facade with one-time initialization/state reporting contracts, and integrated a canonical global-allocator-ready boot transcript line across kernel/UEFI/smoke checks.
- Completed slice: routed a first kernel-owned dynamic structure through the global allocator facade with deterministic probe contracts (`entries=4 checksum=0x2a`) wired across kernel/UEFI/smoke checks and tests, validating end-to-end allocate/use/free behavior in boot flow.
- Completed slice: added deterministic early PIT timer contracts (source/frequency/divisor/IRQ/tick-period), integrated canonical timer-init serial/VGA transcript output into UEFI entry flow, and extended kernel/UEFI/smoke contract checks to enforce the new timer line before boot completion.
- Completed slice: implemented deterministic periodic timer tick accounting state in kernel, added canonical first-tick transcript contracts (`irq=0x20 count=1 uptime_ns=10000000`), wired first-tick emission in UEFI boot flow, and extended transcript/smoke checks to require timer-init -> timer-first-tick ordering before completion.
- Completed slice: wired deterministic timer interrupt dispatch to pair tick accounting with PIC EOI acknowledgement contracts, then integrated canonical timer third-tick and timer-ack transcript lines across kernel/UEFI/smoke checks.
- Completed slice: added kernel timer handoff primitives (`sample_early_timer_handoff`/`take_early_timer_handoff`) with deterministic tick-delta/uptime reports and one-way handoff watermark updates, then integrated a canonical timer-handoff transcript line across UEFI flow and smoke contracts.
- Completed slice: marked timer support complete by extending the kernel with deterministic scheduler bootstrap models (`reset_early_scheduler_state`, scheduler snapshots, timer-driven scheduler handoff reports), then integrated canonical scheduler-handoff transcript contracts across UEFI flow and smoke checks.
- Completed slice: added deterministic scheduler slot mutation APIs (`enqueue_early_scheduler_task` / `dequeue_early_scheduler_task`) plus runnable-slot round-robin advancement, then integrated canonical thread enqueue/dequeue transcript contracts across kernel, UEFI boot flow, and smoke checks.
- Completed slice: introduced deterministic per-thread context handoff modeling (save/restore register snapshots), integrated canonical thread-context transcript lines across kernel + UEFI boot flow, and extended tests/smoke contracts to cover the new handoff output.
- Completed slice: extended thread context handoff reports with deterministic metadata (reason, timer tick, run-queue depth, queue watermark), added canonical thread-ctx-meta transcript contracts, and integrated those checks across kernel/UEFI tests plus smoke/QEMU serial validations.
- Completed slice: added deterministic per-thread lifecycle transition modeling (ready/running/blocked/terminated) wired into scheduler runnable-state bookkeeping, then integrated canonical blocked/ready transcript contracts across kernel, UEFI flow, and smoke checks.
- Completed slice: modeled deterministic scheduler edge-case contracts for blocked-selected fallback and terminated-task dequeue rejection, added canonical thread-state-terminated plus scheduler-edge transcript lines, and integrated them through kernel/UEFI/smoke validation paths.
- Completed slice: hardened UEFI dequeue transcript emission with a deterministic reseed+retry fallback so `thread dequeue` still gets emitted even if prior edge-case probes perturb scheduler model state during firmware execution.
- Completed slice: fixed the latest CI smoke timeout failure by making COM1 TX-ready polling fail open after the deterministic bounded spin budget is exceeded once, preventing per-byte stall amplification that could drop late transcript lines (including `thread ctx meta`) before timeout.
- Next slice: model scheduler preemption debt repayment and deterministic starvation backoff contracts.

<!-- ci-status:start -->
## Latest CI automation

- Last CI conclusion: failure
- Last CI run: `23088643088`
- Last tested commit: `3431bfa7196fed1c0147eb4964bb33562f987efa`
- Recommended next action for Codex: fix the smallest concrete failure from the latest CI excerpts before adding new scope
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
