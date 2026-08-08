// cm5_canary_bypass — Scenario 2: Stack Canary Bypass
//
// Corrupts the stack canary after it is stored, then intercepts
// execution at the canary check and restores the original value
// both on the stack and in the %rdx register (which has already
// loaded the corrupted value before our breakpoint fires).
//
// The program exits cleanly — no crash, SSP bypassed.
//
// Author: Aldair Maihuiri
// Target: cm5_transform
// Technique: dual PTRACE_POKEDATA + PTRACE_SETREGS to bypass SSP

use nix::sys::ptrace;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{fork, ForkResult, execv, Pid};
use std::ffi::CString;
use std::fs;

// Offset of the canary store instruction:
//   main+20: mov %rax, -0x8(%rbp)
const CANARY_STORE_OFFSET: u64 = 0x171a;

// Offset of the canary check instruction:
//   main+284: sub %fs:0x28, %rdx
// Note: at this point, the previous instruction (main+280) has already
// loaded the (corrupted) canary from -0x8(%rbp) into %rdx. Both the
// stack and %rdx must be restored.
const CANARY_CHECK_OFFSET: u64 = 0x1822;

/// Resolves the binary's load base from /proc/<pid>/maps.
/// Required because the binary is PIE.
fn get_load_base(pid: i32) -> u64 {
    let maps = fs::read_to_string(format!("/proc/{}/maps", pid)).unwrap();
    for line in maps.lines() {
        if line.contains("crackme") {
            let base = line.split('-').next().unwrap();
            return u64::from_str_radix(base, 16).unwrap();
        }
    }
    panic!("load base not found in /proc/{}/maps", pid);
}

fn run_tracer(child: Pid) {
    waitpid(child, None).expect("initial waitpid failed");

    let base = get_load_base(child.as_raw());
    let store_addr = base + CANARY_STORE_OFFSET;
    let check_addr = base + CANARY_CHECK_OFFSET;

    println!("[*] load base         : {:#x}", base);
    println!("[*] canary store at   : {:#x}", store_addr);
    println!("[*] canary check at   : {:#x}", check_addr);

    // Place breakpoint 1 — canary store
    let orig_store = ptrace::read(child, store_addr as *mut _)
        .expect("PEEKDATA (store) failed");
    ptrace::write(child, store_addr as *mut _, (orig_store & !0xff) | 0xCC)
        .expect("POKEDATA (bp1) failed");

    // Place breakpoint 2 — canary check
    let orig_check = ptrace::read(child, check_addr as *mut _)
        .expect("PEEKDATA (check) failed");
    ptrace::write(child, check_addr as *mut _, (orig_check & !0xff) | 0xCC)
        .expect("POKEDATA (bp2) failed");

    // Resume — the crackme will prompt for the password
    ptrace::cont(child, None).expect("cont failed");

    // --- Breakpoint 1 fires: the canary store ---
    match waitpid(child, None).expect("waitpid (bp1) failed") {
        WaitStatus::Stopped(_, _) => {
            let mut regs = ptrace::getregs(child).expect("GETREGS (bp1) failed");
            regs.rip -= 1;

            // Restore the original store instruction and execute it
            ptrace::write(child, store_addr as *mut _, orig_store as i64)
                .expect("restore store failed");
            ptrace::setregs(child, regs).expect("SETREGS (bp1) failed");
            ptrace::step(child, None).expect("singlestep failed");
            waitpid(child, None).ok();

            // Compute canary address from rbp
            let regs = ptrace::getregs(child).expect("GETREGS (after step) failed");
            let canary_addr = regs.rbp - 8;
            let canary_original = ptrace::read(child, canary_addr as *mut _)
                .expect("read canary failed");

            println!("[*] rbp               : {:#x}", regs.rbp);
            println!("[*] canary on stack at: {:#x}", canary_addr);
            println!("[*] canary value      : {:#x}", canary_original as u64);

            // Corrupt the canary
            ptrace::write(child, canary_addr as *mut _, 0xDEADBEEFCAFEBABEu64 as i64)
                .expect("corrupt canary failed");
            println!("[*] canary corrupted with 0xDEADBEEFCAFEBABE");

            // Resume — the program runs until breakpoint 2
            ptrace::cont(child, None).expect("cont (to check) failed");

            // --- Breakpoint 2 fires: the canary check ---
            match waitpid(child, None).expect("waitpid (bp2) failed") {
                WaitStatus::Stopped(_, _) => {
                    let mut regs = ptrace::getregs(child)
                        .expect("GETREGS (bp2) failed");
                    regs.rip -= 1;

                    // Restore the original check instruction
                    ptrace::write(child, check_addr as *mut _, orig_check as i64)
                        .expect("restore check failed");

                    // Restore the canary on the stack
                    ptrace::write(child, canary_addr as *mut _, canary_original)
                        .expect("restore canary on stack failed");

                    // The instruction at main+280 (mov -0x8(%rbp), %rdx) already ran
                    // and loaded the corrupted value into %rdx. The check at main+284
                    // (sub %fs:0x28, %rdx) uses %rdx, not the stack directly.
                    // We must restore %rdx as well, or the subtraction will not give
                    // zero and __stack_chk_fail will still be called.
                    regs.rdx = canary_original as u64;

                    println!("[*] canary restored on stack and in %%rdx");
                    println!("[*] bypassing SSP — resuming");

                    ptrace::setregs(child, regs).expect("SETREGS (bp2) failed");
                    ptrace::cont(child, None).expect("final cont failed");
                    waitpid(child, None).ok();
                    println!("[*] done — process exited cleanly, no crash");
                }
                other => println!("unexpected at check bp: {:?}", other),
            }
        }
        other => println!("unexpected at store bp: {:?}", other),
    }
}

fn main() {
    match unsafe { fork() }.expect("fork failed") {
        ForkResult::Child => {
            ptrace::traceme().expect("TRACEME failed");
            let path = CString::new("./crackme").unwrap();
            execv(&path, &[] as &[CString]).expect("execv failed");
        }
        ForkResult::Parent { child } => {
            run_tracer(child);
        }
    }
}
