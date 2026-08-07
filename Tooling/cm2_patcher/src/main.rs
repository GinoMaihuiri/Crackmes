use nix::sys::ptrace;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{fork, ForkResult, execv, Pid};
use std::ffi::CString;
use std::fs;

const JNE_OFFSET: u64 = 0x1790;

fn get_load_base(pid: i32) -> u64 {
    let maps = fs::read_to_string(format!("/proc/{}/maps", pid)).unwrap();
    for line in maps.lines() {
        if line.contains("crackme") {
            let base = line.split('-').next().unwrap();
            return u64::from_str_radix(base, 16).unwrap();
        }
    }
    panic!("no encontré la base de carga");
}

fn run_tracer(child: Pid) {
    waitpid(child, None).expect("waitpid inicial falló");

    let base = get_load_base(child.as_raw());
    let jne_addr = base + JNE_OFFSET;
    println!("[*] base de carga: {:#x}", base);
    println!("[*] jne en:        {:#x}", jne_addr);

    let original = ptrace::read(child, jne_addr as *mut _).expect("peek falló");
    let with_bp = (original & !0xff) | 0xCC;

    ptrace::write(child, jne_addr as *mut _, with_bp as i64).expect("poke falló");
    println!("[*] breakpoint colocado en el jne");

    ptrace::cont(child, None).expect("cont falló");

    match waitpid(child, None).expect("waitpid falló") {
        WaitStatus::Stopped(_, _) => {
            println!("[*] breakpoint alcanzado — forzando ZF=1");
            let mut regs = ptrace::getregs(child).expect("getregs falló");
            regs.eflags |= 0x40;
            regs.rip -= 1;
            ptrace::write(child, jne_addr as *mut _, original as i64)
                .expect("restaurar falló");
            ptrace::setregs(child, regs).expect("setregs falló");
            ptrace::cont(child, None).expect("cont final falló");
            waitpid(child, None).ok();
            println!("[*] hecho");
        }
        other => println!("estado inesperado: {:?}", other),
    }
}

fn main() {
    match unsafe { fork() }.expect("fork failed") {
        ForkResult::Child => {
            ptrace::traceme().expect("traceme failed");
            let path = CString::new("./crackme").unwrap();
            execv(&path, &[] as &[CString]).expect("execv failed");
            unreachable!();
        }
        ForkResult::Parent { child } => {
            run_tracer(child);
        }
    }
}