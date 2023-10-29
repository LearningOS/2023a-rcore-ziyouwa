//! SBI call wrappers
#![allow(unused)]
use core::arch::asm;
// system reset extension
// https://github.com/riscv-non-isa/riscv-sbi-doc/blob/master/riscv-sbi.adoc#system-reset-extension-eid-0x53525354-srst
const SRST_EXTENSION: usize = 0x53525354;
const SYSTEM_RESET_FUNCTION: usize = 0;
enum SystemResetType {
    Shutdown = 0,
    ColdReboot = 1,
    WarmReboot = 2,
}
enum SystemResetReason {
    NoReason = 0,
    SystemFailure = 1,
}

const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
// const SBI_SHUTDOWN: usize = 8;

/// general sbi call
#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "li x16, 0",
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x17") which,
        );
    }
    ret
}

/// export sbi_call_N macro, sbi_call_N!(eid, fid, arg0, arg1, arg2...), up to 6 parameters
// #[macro_export]
// macro_rules! sbi_call_N {
//     ($eid:expr, $fid:expr, $($arg:expr),* $(,)?    ) => {{
//         let args = [$($arg),*];
//         if args.len() > 6 || args.len() < 2 { panic!("Low to 2 and up to 6 parameters: eid, fid, a0 ~ a5") }

//         let mut ret;
//         unsafe {
//             asm!(
//                 "ecall",
//                 $( in(concat!("a", stringify!($arg))) $arg, )*
//                 in("a6") $fid,
//                 in("a7") $eid,
//                 lateout("a0") _,
//                 lateout("a1") ret,
//             );
//         }
//         ret
//     }};
// }


#[inline(always)]
fn sbi_call_4(eid: usize, fid: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "ecall",
            inlateout("a0") arg0 => _,
            inlateout("a1") arg1 => ret,
            in("a2") arg2,
            in("a6") fid,
            in("a7") eid,
        );
    }
    ret
}
/// use sbi call to set timer
pub fn set_timer(timer: usize) {
    sbi_call(SBI_SET_TIMER, timer, 0, 0);
}

/// use sbi call to putchar in console (qemu uart handler)
pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

/// use sbi call to shutdown the kernel
pub fn shutdown() -> ! {
    sbi_call_4(
        SRST_EXTENSION,
        SYSTEM_RESET_FUNCTION,
        SystemResetType::Shutdown as usize,
        SystemResetReason::NoReason as usize,
        0,
    );
    panic!("It should shutdown!");
}
