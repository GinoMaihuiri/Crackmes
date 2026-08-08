# Crackmes
Aldair Maihuiri Security  — Crackmes
# Crackmes

Original crackme challenges written by **Gino Aldair Maihuiri Romero** (Aldair Maihuiri).

Each challenge targets a specific reverse engineering technique — starting from
hardcoded comparisons and progressing toward obfuscated checks, custom algorithms,
and anti-debug protections.

Built for anyone learning RE who wants deliberate, focused practice on one concept
at a time.

---

⚠️ **The `solutions/` folder contains source code.** Open it only after attempting
the challenge — it spoils the answer.

---

## Challenges

| Name | Level | Technique | Platform | Writeup |
|---|---|---|---|---|
| cm1_strcmp | 1 | Hardcoded strcmp | Linux x86_64 (ELF) | [Writeup](https://ginomaihuiri.github.io/crackmes/cm1-strcmp) |
| cm2_numeric | 1 | Numeric serial + live patching via ptrace | Linux x86_64 (ELF) | [Writeup](https://ginomaihuiri.github.io/crackmes/cm2-numeric) |
| cm3_xor | 1 | XOR encrypted immediates in instruction stream | Linux x86_64 (ELF) | [Writeup](https://ginomaihuiri.github.io/crackmes/cm3-xor) |
---

## How to run

```bash
cd <challenge_folder>
chmod +x crackme
./crackme
```

Each challenge folder contains the binary and a `solutions/` subfolder with the source.

## Suggested tools

GDB, Ghidra, radare2, objdump, strings — whatever you're comfortable with.
The point is understanding the mechanism, not the specific tool.

---

## Writeups

Detailed solutions with full assembly analysis:
[ginomaihuiri.github.io/crackmes](https://ginomaihuiri.github.io/crackmes/)

---

## Author

**Gino Aldair Maihuiri Romero** — security researcher
[GitHub](https://github.com/GinoMaihuiri) · [LinkedIn](https://www.linkedin.com/in/AldairMaihuiri) · [X](https://x.com/AldairMaihuiri)

---

© 2026 Gino Aldair Maihuiri Romero. Challenges are free to use for learning purposes.
