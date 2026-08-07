# cm2_patcher — Zero Flag Hijacking via CPU State Manipulation

A Rust process that attaches to cm2_numeric as a ptrace tracer and forces
"Serial válido" with any input — without touching the binary on disk and
without knowing the serial.

## Technique

`ptrace` is the syscall GDB is built on. This tool uses it directly.

The patcher resolves the `jne` address at runtime from `/proc/pid/maps`
(the binary is PIE), writes `0xCC` at that address to insert a CPU-level
breakpoint, and when the process stops, manipulates the processor state
directly: one bit in EFLAGS, the Zero Flag (ZF).

The `jne` reads ZF=1, interprets the comparison as successful, and does
not jump. Any input passes.

That is CPU state manipulation via ptrace — Zero Flag hijacking.

## How it works

| Step | ptrace request | What happens |
|---|---|---|
| 1 | `PTRACE_TRACEME` | Child offers itself to be traced |
| 2 | `PTRACE_PEEKDATA` | Read the word at the jne address |
| 3 | `PTRACE_POKEDATA` | Write `0xCC` — insert CPU-level breakpoint |
| 4 | `PTRACE_CONT` | Resume — crackme prompts for serial |
| 5 | `PTRACE_GETREGS` | Read all registers when breakpoint hits |
| 6 | — | Force ZF=1 in EFLAGS, correct RIP |
| 7 | `PTRACE_POKEDATA` | Restore original jne byte |
| 8 | `PTRACE_SETREGS` | Write modified registers back |
| 9 | `PTRACE_CONT` | Resume — jne does not jump |

## Usage

```bash
cp ../../cm2_numeric/crackme .
cargo build
./target/debug/cm2_patcher
```

Enter any input when prompted.

## Target

- **Binary:** cm2_numeric
- **Architecture:** x86-64 (Linux, ELF PIE)
- **jne file offset:** `0x1790`
- **Technique:** Zero Flag hijacking via ptrace

## Writeup

Full analysis: [ginomaihuiri.github.io/crackmes/cm2-numeric](https://ginomaihuiri.github.io/crackmes/cm2-numeric)

---

© 2026 Aldair Maihuiri
