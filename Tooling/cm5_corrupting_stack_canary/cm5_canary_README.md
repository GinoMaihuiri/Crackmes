# cm5_canary — Stack Canary Corruption and SSP Bypass via ptrace

Two Rust programs demonstrating GCC's Stack Smashing Protector (SSP) behavior
from outside the process using `ptrace` — corruption, detection, and bypass.

## Background

The stack canary is a random value placed on the stack at function entry and
verified at function exit. If a buffer overflow modifies it, the program aborts
before `ret` executes. GCC inserts this mechanism automatically when compiling
with `-fstack-protector`.

The canary is loaded from thread-local storage (`%fs:0x28`) and stored at
`%rbp - 8`. At function exit, it is loaded back into a register, the TLS value
is subtracted, and if the result is not zero, `__stack_chk_fail` is called.

Key design details:
- The canary is randomized on every execution
- Its least significant byte is always `0x00` — a null byte that stops string-based
  overflows before they can overwrite the rest of the canary
- Only one tracer can be attached to a process at a time (ptrace limitation)

## Scenarios

### Scenario 1 — Corruption (`scenario1_corrupt/`)

Corrupts the canary after it is stored on the stack, then resumes execution.
When the program reaches the canary check at function exit, the comparison
fails and `__stack_chk_fail` is called — even if the password was correct.

Demonstrates: the canary detects corruption regardless of program logic.

```
[*] canary value      : 0x8964659be4698200
[*] canary corrupted with 0xDEADBEEFCAFEBABE
  [+] Password valido!
*** stack smashing detected ***: terminated
```

### Scenario 2 — Bypass (`scenario2_bypass/`)

Corrupts the canary after it is stored, then intercepts execution at the check
instruction and restores the original value — both on the stack and in the `%rdx`
register, which has already loaded the corrupted value before the breakpoint fires.

The program exits cleanly with no crash.

Demonstrates: SSP protects against attackers who do not know the canary value.
An attacker with read access to the process can leak the canary and bypass the check.

```
[*] canary value      : 0x6d89dc5a86b68f00
[*] canary corrupted with 0xDEADBEEFCAFEBABE
  [+] Password valido!
[*] canary restored on stack and in %rdx
[*] done — process exited cleanly, no crash
```

## Why %rdx must also be restored

The canary check sequence in the binary is:

```asm
main+280: mov  -0x8(%rbp), %rdx    ; loads canary from stack into %rdx
main+284: sub  %fs:0x28, %rdx      ; subtracts TLS canary from %rdx
main+293: je   main+300            ; if zero, return normally
main+295: call __stack_chk_fail    ; if non-zero, abort
```

The breakpoint is placed at `main+284`. By the time it fires, `main+280` has
already executed — `%rdx` already holds the corrupted value. Restoring only the
stack is not enough: `%rdx` must also be set to the original canary so that the
subtraction produces zero.

## Usage

```bash
# Build and run either scenario
cd scenario1_corrupt   # or scenario2_bypass
cargo build
cp ../../crackme .
./target/debug/cm5_canary
# Enter the password when prompted
```

## Target

- **Binary:** cm5_transform (cm5_xor)
- **Architecture:** x86-64 (Linux, ELF PIE)
- **Canary store offset:** `0x171a`
- **Canary check offset:** `0x1822`

## Writeup

Full analysis: [ginomaihuiri.github.io/crackmes/cm5-transform](https://ginomaihuiri.github.io/crackmes/cm5-transform)

---

© 2026 Aldair Maihuiri
