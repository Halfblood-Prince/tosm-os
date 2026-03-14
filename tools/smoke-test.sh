#!/usr/bin/env bash
set -euo pipefail

expected_banner='tosm-os: kernel entry reached'
expected_panic='tosm-os: panic in uefi-entry'
expected_interrupt_init='tosm-os: idt skeleton initialized'
expected_exception_page_fault='tosm-os: exception vector 14 page fault'
expected_exception_unknown='tosm-os: exception vector unknown'
expected_entry_done='tosm-os: efi_main completed'
expected_memory_init='tosm-os: memory init usable=0x3f790000 reserved=0x00811000 regions=5'
expected_paging_plan='tosm-os: paging plan frames=4 window=0x3f7ed000-0x3f7f1000 map4k=512'
expected_paging_install='tosm-os: paging install root=0x3f7ed000 span=0x40000000 entries=514'
expected_heap_bootstrap='tosm-os: heap bootstrap start=0x00400000 size=0x00004000 frames=4'
expected_heap_alloc_cycle='tosm-os: heap alloc cycle allocs=2 frees=2 cursor=0x00400000'
expected_global_allocator_ready='tosm-os: global allocator ready heap=0x00400000-0x00404000'
expected_global_allocator_probe='tosm-os: global allocator probe entries=4 checksum=0x000000000000002a'
expected_timer_init='tosm-os: timer init source=pit hz=100 divisor=11931 irq=0x20'
expected_timer_first_tick='tosm-os: timer tick irq=0x20 count=1 uptime_ns=10000000'
expected_timer_third_tick='tosm-os: timer tick irq=0x20 count=3 uptime_ns=30000000'
expected_timer_ack='tosm-os: timer ack irq=0x20 pic=0x20 eoi=0x20'
expected_timer_handoff='tosm-os: timer handoff ticks=3 delta=3 quantum=1 uptime_ns=30000000'
expected_scheduler_handoff='tosm-os: scheduler handoff reason=timer runq=2 selected=1 idle=0 delta=3'
expected_thread_enqueue='tosm-os: thread enqueue task=2 runq=3 selected=1'
expected_thread_dequeue='tosm-os: thread dequeue task=2 runq=2 selected=1'
expected_thread_context_save='tosm-os: thread ctx save from=1 to=2 rip=0x100200 rsp=0x401f00'
expected_thread_context_restore='tosm-os: thread ctx restore to=2 rip=0x200000 rsp=0x402000'
expected_thread_context_meta='tosm-os: thread ctx meta reason=yield tick=3 runq=3 watermark=3'
expected_thread_state_blocked='tosm-os: thread state task=2 ready->blocked runq=2 selected=1'
expected_thread_state_ready='tosm-os: thread state task=2 blocked->ready runq=3 selected=1'
expected_thread_wake='tosm-os: thread wake task=2 reason=timer wait=0x2000 runq=3 sel=1'
expected_thread_wait_ownership='tosm-os: thread wait owner=1 task=2 wait=0x2000 claim=1'
expected_thread_wake_timeout='tosm-os: thread wake timeout task=2 deadline=3 now=3 expired=1'
expected_thread_wait_contention='tosm-os: thread wait contend wait=0x3000 winner=3 loser=2 pri=signal>timer'
expected_thread_wake_order='tosm-os: thread wake order first=3 second=2 wait=0x3000 claims=2,3'
expected_thread_wake_fairness='tosm-os: thread wake fairness first=4 wait=0x5000 age=5 second=3 age=3 rotate=1'
expected_scheduler_rebalance='tosm-os: scheduler rebalance winner=2 age=4 decayed=6 floor=4 boost=1'
expected_scheduler_carryover='tosm-os: scheduler carryover task=2 rem=2 carry=1 thresh=3 preempt=0 next=2'
expected_thread_state_terminated='tosm-os: thread state task=2 ready->terminated runq=1 selected=0'
expected_scheduler_edge_blocked='tosm-os: scheduler edge case=blocked-selected task=1 runq=2 selected=0'
expected_scheduler_edge_terminated='tosm-os: scheduler edge case=terminated-dequeue task=2 err=task-not-found runq=1 selected=0'
expected_banner_line='tosm-os: kernel entry reached\r\n'
expected_panic_line='tosm-os: panic in uefi-entry\r\n'
expected_interrupt_init_line='tosm-os: idt skeleton initialized\r\n'
expected_entry_done_line='tosm-os: efi_main completed\r\n'
expected_memory_init_line='tosm-os: memory init usable=0x3f790000 reserved=0x00811000 regions=5\r\n'
expected_paging_plan_line='tosm-os: paging plan frames=4 window=0x3f7ed000-0x3f7f1000 map4k=512\r\n'
expected_paging_install_line='tosm-os: paging install root=0x3f7ed000 span=0x40000000 entries=514\r\n'
expected_heap_bootstrap_line='tosm-os: heap bootstrap start=0x00400000 size=0x00004000 frames=4\r\n'
expected_heap_alloc_cycle_line='tosm-os: heap alloc cycle allocs=2 frees=2 cursor=0x00400000\r\n'
expected_global_allocator_ready_line='tosm-os: global allocator ready heap=0x00400000-0x00404000\r\n'
expected_global_allocator_probe_line='tosm-os: global allocator probe entries=4 checksum=0x000000000000002a\r\n'
expected_timer_init_line='tosm-os: timer init source=pit hz=100 divisor=11931 irq=0x20\r\n'
expected_timer_first_tick_line='tosm-os: timer tick irq=0x20 count=1 uptime_ns=10000000\r\n'
expected_timer_third_tick_line='tosm-os: timer tick irq=0x20 count=3 uptime_ns=30000000\r\n'
expected_timer_ack_line='tosm-os: timer ack irq=0x20 pic=0x20 eoi=0x20\r\n'
expected_timer_handoff_line='tosm-os: timer handoff ticks=3 delta=3 quantum=1 uptime_ns=30000000\r\n'
expected_scheduler_handoff_line='tosm-os: scheduler handoff reason=timer runq=2 selected=1 idle=0 delta=3\r\n'
expected_thread_enqueue_line='tosm-os: thread enqueue task=2 runq=3 selected=1\r\n'
expected_thread_dequeue_line='tosm-os: thread dequeue task=2 runq=2 selected=1\r\n'
expected_thread_context_save_line='tosm-os: thread ctx save from=1 to=2 rip=0x100200 rsp=0x401f00\r\n'
expected_thread_context_restore_line='tosm-os: thread ctx restore to=2 rip=0x200000 rsp=0x402000\r\n'
expected_thread_context_meta_line='tosm-os: thread ctx meta reason=yield tick=3 runq=3 watermark=3\r\n'
expected_thread_state_blocked_line='tosm-os: thread state task=2 ready->blocked runq=2 selected=1\r\n'
expected_thread_state_ready_line='tosm-os: thread state task=2 blocked->ready runq=3 selected=1\r\n'
expected_thread_wake_line='tosm-os: thread wake task=2 reason=timer wait=0x2000 runq=3 sel=1\r\n'
expected_thread_wait_ownership_line='tosm-os: thread wait owner=1 task=2 wait=0x2000 claim=1\r\n'
expected_thread_wake_timeout_line='tosm-os: thread wake timeout task=2 deadline=3 now=3 expired=1\r\n'
expected_thread_wait_contention_line='tosm-os: thread wait contend wait=0x3000 winner=3 loser=2 pri=signal>timer\r\n'
expected_thread_wake_order_line='tosm-os: thread wake order first=3 second=2 wait=0x3000 claims=2,3\r\n'
expected_thread_wake_fairness_line='tosm-os: thread wake fairness first=4 wait=0x5000 age=5 second=3 age=3 rotate=1\r\n'
expected_scheduler_rebalance_line='tosm-os: scheduler rebalance winner=2 age=4 decayed=6 floor=4 boost=1\r\n'
expected_scheduler_carryover_line='tosm-os: scheduler carryover task=2 rem=2 carry=1 thresh=3 preempt=0 next=2\r\n'
expected_thread_state_terminated_line='tosm-os: thread state task=2 ready->terminated runq=1 selected=0\r\n'
expected_scheduler_edge_blocked_line='tosm-os: scheduler edge case=blocked-selected task=1 runq=2 selected=0\r\n'
expected_scheduler_edge_terminated_line='tosm-os: scheduler edge case=terminated-dequeue task=2 err=task-not-found runq=1 selected=0\r\n'
expected_exception_page_fault_line='tosm-os: exception vector 14 page fault\r\n'

contract_check() {
  if ! grep --fixed-strings --quiet -- "${expected_banner}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected boot banner not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_interrupt_init}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected interrupt-init line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_exception_page_fault}" kernel/src/lib.rs; then
    echo "smoke: expected page-fault exception log line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_exception_unknown}" kernel/src/lib.rs; then
    echo "smoke: expected unknown exception log line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_memory_init}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected memory-init line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_paging_install}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected paging-install line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_heap_bootstrap}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected heap-bootstrap line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_heap_alloc_cycle}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected heap-alloc-cycle line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_global_allocator_ready}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected global-allocator-ready line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_global_allocator_probe}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected global-allocator-probe line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_timer_init}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected timer-init line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_timer_first_tick}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected timer-first-tick line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_timer_third_tick}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected timer-third-tick line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_timer_ack}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected timer-ack line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_entry_done}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected efi_main completion line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_scheduler_handoff}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected scheduler-handoff line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_enqueue}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-enqueue line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_dequeue}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-dequeue line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_context_save}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-ctx-save line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_context_restore}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-ctx-restore line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_context_meta}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-ctx-meta line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_state_blocked}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-state-blocked line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_state_ready}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-state-ready line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_wake}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-wake line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_wait_ownership}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-wait-ownership line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_wake_timeout}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-wake-timeout line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_wait_contention}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-wait-contention line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_wake_order}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-wake-order line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_wake_fairness}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-wake-fairness line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_scheduler_rebalance}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected scheduler-rebalance line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_scheduler_carryover}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected scheduler-carryover line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_state_terminated}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-state-terminated line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_scheduler_edge_blocked}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected scheduler-edge-blocked line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_scheduler_edge_terminated}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected scheduler-edge-terminated line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_paging_plan}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected paging-plan line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_exception_page_fault}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected exception vector 14 line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_panic}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected panic line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_banner_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected boot banner CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_panic_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected panic CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_interrupt_init_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected interrupt-init CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_memory_init_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected memory-init CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_paging_plan_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected paging-plan CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_paging_install_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected paging-install CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_heap_bootstrap_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected heap-bootstrap CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_heap_alloc_cycle_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected heap-alloc-cycle CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_global_allocator_ready_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected global-allocator-ready CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_global_allocator_probe_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected global-allocator-probe CRLF line contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_timer_init_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected timer-init CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_timer_first_tick_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected timer-first-tick CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_timer_third_tick_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected timer-third-tick CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_timer_ack_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected timer-ack CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_timer_handoff_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected timer-handoff CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_scheduler_handoff_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected scheduler-handoff CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_enqueue_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-enqueue CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_dequeue_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-dequeue CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_context_save_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-ctx-save CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_context_restore_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-ctx-restore CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_context_meta_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-ctx-meta CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_state_blocked_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-state-blocked CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_state_ready_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-state-ready CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_wake_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-wake CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_wait_ownership_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-wait-ownership CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_wake_timeout_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-wake-timeout CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_wait_contention_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-wait-contention CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_wake_order_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-wake-order CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_wake_fairness_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-wake-fairness CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_scheduler_rebalance_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected scheduler-rebalance CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_scheduler_carryover_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected scheduler-carryover CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_state_terminated_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected thread-state-terminated CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_scheduler_edge_blocked_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected scheduler-edge-blocked CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_scheduler_edge_terminated_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected scheduler-edge-terminated CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_entry_done_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected efi_main completion CRLF contract not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_exception_page_fault_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected exception vector 14 CRLF contract not found"
    exit 1
  fi

  echo "smoke: serial message contracts present"
}

screen_transcript_contract_check() {
  local transcript_tests=(
    model_boot_transcript_renders_banner_then_interrupt_then_done_on_distinct_rows
    model_panic_transcript_reinitializes_screen_and_removes_old_boot_lines
    model_init_clears_screen_and_resets_cursor
    model_newline_clears_destination_row
    model_carriage_return_resets_column_and_overwrites_in_place
    model_width_boundary_wrap_advances_to_next_row_and_clears_it
    model_scroll_moves_rows_up_and_clears_last_row
  )

  local test_name
  for test_name in "${transcript_tests[@]}"; do
    cargo test --package uefi-entry --lib "${test_name}"
  done

  if ! grep --fixed-strings --quiet -- "tosm-os: timer init source=pit hz=100 divisor=11931 irq=0x20" boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected VGA transcript row for timer init line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "tosm-os: timer tick irq=0x20 count=1 uptime_ns=10000000" boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected VGA transcript row for timer first tick line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "tosm-os: timer tick irq=0x20 count=3 uptime_ns=30000000" boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected VGA transcript row for timer third tick line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "tosm-os: timer ack irq=0x20 pic=0x20 eoi=0x20" boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected VGA transcript row for timer ack line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "tosm-os: timer handoff ticks=3 delta=3 quantum=1 uptime_ns=30000000" boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected VGA transcript row for timer handoff line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "tosm-os: scheduler handoff reason=timer runq=2 selected=1 idle=0 delta=3" boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected VGA transcript row for scheduler handoff line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "tosm-os: thread enqueue task=2 runq=3 selected=1" boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected VGA transcript row for thread enqueue line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "tosm-os: thread dequeue task=2 runq=2 selected=1" boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected VGA transcript row for thread dequeue line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "tosm-os: thread ctx save from=1 to=2 rip=0x100200 rsp=0x401f00" boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected VGA transcript row for thread ctx save line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "tosm-os: thread ctx restore to=2 rip=0x200000 rsp=0x402000" boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected VGA transcript row for thread ctx restore line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "tosm-os: thread ctx meta reason=yield tick=3 runq=3 watermark=3" boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected VGA transcript row for thread ctx meta line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "tosm-os: thread state task=2 ready->blocked runq=2 selected=1" boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected VGA transcript row for thread state blocked line not found"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "tosm-os: thread state task=2 blocked->ready runq=3 selected=1" boot/uefi-entry/src/lib.rs; then
    echo "smoke: expected VGA transcript row for thread state ready line not found"
    exit 1
  fi

  echo "smoke: VGA transcript init/newline/carriage-return/wrap/ordering/interrupt-ordering/memory-reporting/paging-plan-reporting/timer-reporting/scheduler-thread-reporting/context-handoff-reporting/lifecycle-reporting/scrolling contracts present"
}

find_ovmf_code() {
  local candidate
  for candidate in \
    "${OVMF_CODE_PATH:-}" \
    /usr/share/OVMF/OVMF_CODE_4M.fd \
    /usr/share/OVMF/OVMF_CODE.fd \
    /usr/share/edk2/ovmf/OVMF_CODE.fd \
    /usr/share/edk2/ovmf/OVMF_CODE_4M.fd \
    /usr/share/ovmf/OVMF.fd \
    /usr/share/edk2/x64/OVMF_CODE.fd; do
    if [[ -n "${candidate}" && -f "${candidate}" ]]; then
      printf '%s\n' "${candidate}"
      return 0
    fi
  done
  return 1
}

find_ovmf_vars() {
  local candidate
  for candidate in \
    "${OVMF_VARS_PATH:-}" \
    /usr/share/OVMF/OVMF_VARS_4M.fd \
    /usr/share/OVMF/OVMF_VARS.fd \
    /usr/share/edk2/ovmf/OVMF_VARS.fd \
    /usr/share/edk2/ovmf/OVMF_VARS_4M.fd \
    /usr/share/edk2/x64/OVMF_VARS.fd; do
    if [[ -n "${candidate}" && -f "${candidate}" ]]; then
      printf '%s\n' "${candidate}"
      return 0
    fi
  done
  return 1
}


ensure_uefi_target_installed() {
  local target="x86_64-unknown-uefi"

  if ! command -v rustup >/dev/null 2>&1; then
    echo "smoke: rustup unavailable; cannot provision ${target} target"
    return 1
  fi

  if rustup target list --installed | grep --fixed-strings --quiet -- "${target}"; then
    return 0
  fi

  echo "smoke: installing missing Rust target ${target}"
  rustup target add "${target}"
}

run_qemu_smoke() {
  local qemu_bin="${QEMU_BIN:-qemu-system-x86_64}"
  if ! command -v "${qemu_bin}" >/dev/null 2>&1; then
    if [[ "${REQUIRE_QEMU_SMOKE:-0}" -eq 1 ]]; then
      echo "smoke: ${qemu_bin} unavailable but REQUIRE_QEMU_SMOKE=1"
      exit 1
    fi
    echo "smoke: ${qemu_bin} unavailable, skipping QEMU execution"
    return 2
  fi

  local ovmf_code ovmf_vars
  if ! ovmf_code="$(find_ovmf_code)"; then
    if [[ "${REQUIRE_QEMU_SMOKE:-0}" -eq 1 ]]; then
      echo "smoke: OVMF code firmware unavailable but REQUIRE_QEMU_SMOKE=1"
      exit 1
    fi
    echo "smoke: OVMF code firmware unavailable, skipping QEMU execution"
    return 2
  fi
  if ! ovmf_vars="$(find_ovmf_vars)"; then
    if [[ "${REQUIRE_QEMU_SMOKE:-0}" -eq 1 ]]; then
      echo "smoke: OVMF vars firmware unavailable but REQUIRE_QEMU_SMOKE=1"
      exit 1
    fi
    echo "smoke: OVMF vars firmware unavailable, skipping QEMU execution"
    return 2
  fi

  ensure_uefi_target_installed

  cargo build --package uefi-entry --bin bootx64 --target x86_64-unknown-uefi

  local efi_path="target/x86_64-unknown-uefi/debug/bootx64.efi"
  if [[ ! -f "${efi_path}" ]]; then
    echo "smoke: expected EFI image missing at ${efi_path}"
    exit 1
  fi

  SMOKE_RUN_DIR="$(mktemp -d)"
  trap 'rm -rf "${SMOKE_RUN_DIR}"' EXIT
  mkdir -p "${SMOKE_RUN_DIR}/EFI/BOOT"
  cp "${efi_path}" "${SMOKE_RUN_DIR}/EFI/BOOT/BOOTX64.EFI"

  # OVMF variable stores are mutable. Always use a temp copy so each run is deterministic
  # and never mutates global firmware state in CI workers.
  local ovmf_vars_runtime="${SMOKE_RUN_DIR}/OVMF_VARS.fd"
  cp "${ovmf_vars}" "${ovmf_vars_runtime}"

  local serial_log="${SMOKE_RUN_DIR}/serial.log"
  local qemu_accel_args=("-accel" "tcg")
  if [[ -n "${QEMU_ACCEL_ARGS:-}" ]]; then
    # shellcheck disable=SC2206
    qemu_accel_args=(${QEMU_ACCEL_ARGS})
  fi

  # CI runners without hardware acceleration can take noticeably longer to
  # reach the final scheduler/thread transcript lines, so keep a conservative
  # default timeout while still allowing explicit overrides.
  local qemu_timeout_secs="${QEMU_TIMEOUT_SECS:-90}"
  local qemu_status=0
  set +e
  timeout "${qemu_timeout_secs}s" "${qemu_bin}" \
    -nodefaults \
    -nographic \
    "${qemu_accel_args[@]}" \
    -serial file:"${serial_log}" \
    -drive if=pflash,format=raw,readonly=on,file="${ovmf_code}" \
    -drive if=pflash,format=raw,file="${ovmf_vars_runtime}" \
    -drive format=raw,file=fat:rw:"${SMOKE_RUN_DIR}"
  qemu_status=$?
  set -e

  if [[ "${qemu_status}" -eq 124 ]]; then
    echo "smoke: QEMU timed out after ${qemu_timeout_secs}s"
  fi

  if ! grep --fixed-strings --quiet -- "${expected_banner}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing banner"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_interrupt_init}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing interrupt-init line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_exception_page_fault}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing exception vector 14 line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_memory_init}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing memory-init line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_paging_plan}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing paging-plan line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_paging_install}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing paging-install line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_heap_bootstrap}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing heap-bootstrap line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_heap_alloc_cycle}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing heap-alloc-cycle line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_global_allocator_ready}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing global-allocator-ready line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_global_allocator_probe}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing global-allocator-probe line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_timer_init}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing timer-init line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_timer_first_tick}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing timer-first-tick line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_timer_third_tick}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing timer-third-tick line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_timer_ack}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing timer-ack line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_timer_handoff}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing timer-handoff line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_scheduler_handoff}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing scheduler-handoff line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_enqueue}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing thread-enqueue line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_dequeue}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing thread-dequeue line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_context_save}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing thread-ctx-save line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_context_restore}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing thread-ctx-restore line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_context_meta}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing thread-ctx-meta line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_state_blocked}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing thread-state-blocked line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_state_ready}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing thread-state-ready line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_wake}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing thread-wake line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_wait_ownership}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing thread-wait-ownership line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_wake_timeout}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing thread-wake-timeout line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_wait_contention}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing thread-wait-contention line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_wake_order}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing thread-wake-order line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_wake_fairness}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing thread-wake-fairness line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_scheduler_rebalance}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing scheduler-rebalance line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_scheduler_carryover}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing scheduler-carryover line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_scheduler_edge_blocked}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing scheduler-edge-blocked line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_thread_state_terminated}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing thread-state-terminated line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_scheduler_edge_terminated}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing scheduler-edge-terminated line"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_entry_done}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing completion line"
    exit 1
  fi

  if [[ "${qemu_status}" -ne 0 ]]; then
    if [[ "${REQUIRE_QEMU_SMOKE:-0}" -eq 1 ]]; then
      echo "smoke: QEMU exited with status ${qemu_status} after producing complete serial transcript"
      return 0
    fi
    echo "smoke: QEMU exited with status ${qemu_status} after producing complete serial transcript"
    return 2
  fi

  echo "smoke: QEMU boot output includes banner, interrupt-init, exception, memory-init, paging-plan, paging-install, heap-bootstrap, heap-alloc-cycle, global-allocator-ready, global-allocator-probe, timer-init, timer-first-tick, timer-third-tick, timer-ack, timer-handoff, scheduler-handoff, thread-enqueue, thread-dequeue, thread-ctx-save, thread-ctx-restore, thread-ctx-meta, thread-state-blocked, thread-state-ready, thread-wake, thread-wait-contention, thread-wake-order, thread-wake-fairness, scheduler-carryover, scheduler-edge-blocked, thread-state-terminated, scheduler-edge-terminated, and completion lines"
}

contract_check
screen_transcript_contract_check
qemu_status=0
run_qemu_smoke || qemu_status=$?
if [[ "${qemu_status}" -ne 0 && "${qemu_status}" -ne 2 ]]; then
  exit "${qemu_status}"
fi
