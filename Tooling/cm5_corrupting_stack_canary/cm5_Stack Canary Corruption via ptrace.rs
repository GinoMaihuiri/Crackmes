// cm5_canary_corrupt — Scenario 1: Stack Canary Corruption
//
// Corrupts the stack canary after it is stored, then resumes execution.
// When the program reaches the canary check at function exit,
// the comparison fails and __stack_chk_fail is called.
//
// Author: Aldair Maihuiri
// Target: cm5_transform
// Technique: PTRACE_POKEDATA to corrupt stack memory

use nix::sys::ptrace;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{fork, ForkResult, execv, Pid};
use std::ffi::CString;
use std::fs;

// Offset of the canary store instruction in the binary:
//   main+20: mov %rax, -0x8(%rbp)
const CANARY_STORE_OFFSET: u64 = 0x171a;

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
    println!("[*] load base         : {:#x}", base);
    println!("[*] canary store at   : {:#x}", store_addr);

    // Place breakpoint at the canary store instruction
    let orig_store = ptrace::read(child, store_addr as *mut _)
        .expect("PEEKDATA failed");
    ptrace::write(child, store_addr as *mut _, (orig_store & !0xff) | 0xCC)
        .expect("POKEDATA (breakpoint) failed");

    // Resume — the crackme will prompt for the password
    ptrace::cont(child, None).expect("cont failed");

    // Wait for breakpoint hit at the canary store
    match waitpid(child, None).expect("waitpid failed") {
        WaitStatus::Stopped(_, _) => {
            let mut regs = ptrace::getregs(child).expect("GETREGS failed");
            regs.rip -= 1;

            // Restore the original store instruction
            ptrace::write(child, store_addr as *mut _, orig_store as i64)
                .expect("restore store failed");
            ptrace::setregs(child, regs).expect("SETREGS failed");

            // Single-step to execute the store — canary is now on the stack
            ptrace::step(child, None).expect("singlestep failed");
            waitpid(child, None).ok();

            // Read rbp to compute the canary's stack address
            let regs = ptrace::getregs(child).expect("GETREGS failed");
            let canary_addr = regs.rbp - 8;
            let canary_value = ptrace::read(child, canary_addr as *mut _)
                .expect("read canary failed");

            println!("[*] rbp               : {:#x}", regs.rbp);
            println!("[*] canary on stack at: {:#x}", canary_addr);
            println!("[*] canary value      : {:#x}", canary_value as u64);

            // Corrupt the canary with a recognizable pattern
            ptrace::write(child, canary_addr as *mut _, 0xDEADBEEFCAFEBABEu64 as i64)
                .expect("corrupt canary failed");
            println!("[*] canary corrupted with 0xDEADBEEFCAFEBABE");
            println!("[*] resuming — expect stack smashing detected...");

            // Resume — the program runs normally until the canary check fails
            ptrace::cont(child, None).expect("final cont failed");
            waitpid(child, None).ok();
        }
        other => println!("unexpected stop status: {:?}", other),
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
