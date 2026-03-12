# CODEX.md
Guidelines for Codex when contributing to this repository.

This repository contains a hobby operating system. Codex should prioritize **forward progress** and **meaningful milestones**, not micro-optimizations.

---

# Core Principle

Always **continue development from the current repository state** and push the OS toward the **next functional milestone**.

Do not stall on tiny improvements.

---

# Development Behaviour

Codex must:

1. Read the current repository structure.
2. Continue development logically from the current code.
3. Implement meaningful features in **coherent chunks**, not tiny edits.

Avoid changes that add only a few lines unless absolutely necessary.

Target **substantial progress per run**.

---

# Expected Patch Size

Codex should prefer:

- multi-file improvements
- coherent feature additions
- logical subsystem implementations

Typical acceptable contribution size:


50–300+ lines per run


Do **not artificially limit changes** to small patches.

---

# Milestone Driven Development

Development should follow **milestones typical for hobby OS projects**, such as:

- Bootloader
- Basic kernel
- Screen output
- Memory setup
- Interrupt handling
- Keyboard input
- Simple shell
- Filesystem support
- Process/task system

---

# Milestone Completion Rule

When a milestone is **functional and usable**, Codex should:

1. Mark the milestone as **complete**
2. Move to the **next milestone**

Do NOT repeatedly optimize the same milestone.

Example:

Bad behavior:

keep rewriting VGA printing code repeatedly


Correct behavior:

VGA printing works → move to interrupts


---

# Optimization Policy

Optimization is **not the priority**.

Only optimize when:

- a milestone is already complete
- performance blocks further development

Otherwise prioritize **new functionality**.

---

# Implementation Strategy

When implementing features:

1. Create minimal working implementation.
2. Ensure it builds successfully.
3. Ensure the system still boots.
4. Continue building the next subsystem.

---

# Repository Awareness

Codex should examine:

- Makefiles
- build scripts
- existing kernel modules
- bootloader configuration
- architecture folders

Then extend the system logically.

---

# Goal

Gradually evolve this project into a **fully working hobby OS** with:

- booting kernel
- interrupt support
- memory management
- device input
- basic shell
- filesystem
- multitasking

Progress matters more than perfection.
