mod call;
mod errno;
mod sysno;

#[cfg(not(nightly))]
use crate::syscalls;
pub use call::*;
pub use errno::*;
pub use sysno::*;

const MAX_ERRNO: i32 = 4095;

#[inline(always)]
pub fn check_errno(ret: usize) -> Result<usize, Errno> {
    let reti = ret as isize;
    if reti < 0 && reti >= (-MAX_ERRNO) as isize {
        let reti = (-reti) as Errno;
        Err(reti)
    } else {
        Ok(ret)
    }
}

#[cfg(not(nightly))]
#[inline(always)]
pub unsafe fn syscall0(n: Sysno) -> Result<usize, Errno> {
    check_errno(syscalls::__syscall0(n))
}

#[cfg(not(nightly))]
#[inline(always)]
pub unsafe fn syscall1(n: Sysno, a1: usize) -> Result<usize, Errno> {
    check_errno(syscalls::__syscall1(n, a1))
}

#[cfg(not(nightly))]
#[inline(always)]
pub unsafe fn syscall2(n: Sysno, a1: usize, a2: usize) -> Result<usize, Errno> {
    check_errno(syscalls::__syscall2(n, a1, a2))
}

#[cfg(not(nightly))]
#[inline(always)]
pub unsafe fn syscall3(n: Sysno, a1: usize, a2: usize, a3: usize) -> Result<usize, Errno> {
    check_errno(syscalls::__syscall3(n, a1, a2, a3))
}

#[cfg(not(nightly))]
#[inline(always)]
pub unsafe fn syscall4(
    n: Sysno,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
) -> Result<usize, Errno> {
    check_errno(syscalls::__syscall4(n, a1, a2, a3, a4))
}

#[cfg(not(nightly))]
#[inline(always)]
pub unsafe fn syscall5(
    n: Sysno,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> Result<usize, Errno> {
    check_errno(syscalls::__syscall5(n, a1, a2, a3, a4, a5))
}

#[cfg(not(nightly))]
#[inline(always)]
pub unsafe fn syscall6(
    n: Sysno,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> Result<usize, Errno> {
    check_errno(syscalls::__syscall6(n, a1, a2, a3, a4, a5, a6))
}

#[cfg(nightly)]
#[inline(always)]
pub unsafe fn syscall0(n: Sysno) -> Result<usize, Errno> {
    let ret: usize;
    asm!("swi $$0"
         : "={r0}"(ret)
         : "{r7}"(n)
         : "memory" "cc"
         : "volatile");
    check_errno(ret)
}

#[cfg(nightly)]
#[inline(always)]
pub unsafe fn syscall1(n: Sysno, a1: usize) -> Result<usize, Errno> {
    let ret: usize;
    asm!("swi $$0"
         : "={r0}"(ret)
         : "{r7}"(n),
           "{r0}"(a1)
         : "memory" "cc"
         : "volatile");
    check_errno(ret)
}

#[cfg(nightly)]
#[inline(always)]
pub unsafe fn syscall2(n: Sysno, a1: usize, a2: usize) -> Result<usize, Errno> {
    let ret: usize;
    asm!("swi $$0"
         : "={r0}"(ret)
         : "{r7}"(n),
           "{r0}"(a1),
           "{r1}"(a2)
         : "memory" "cc"
         : "volatile");
    check_errno(ret)
}

#[inline(always)]
pub unsafe fn syscall3(n: Sysno, a1: usize, a2: usize, a3: usize) -> Result<usize, Errno> {
    let ret: usize;
    asm!("swi $$0"
         : "={r0}"(ret)
         : "{r7}"(n),
           "{r0}"(a1),
           "{r1}"(a2),
           "{r2}"(a3)
         : "memory" "cc"
         : "volatile");
    check_errno(ret)
}

#[cfg(nightly)]
#[inline(always)]
pub unsafe fn syscall4(
    n: Sysno,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
) -> Result<usize, Errno> {
    let ret: usize;
    asm!("swi $$0"
         : "={r0}"(ret)
         : "{r7}"(n),
           "{r0}"(a1),
           "{r1}"(a2),
           "{r2}"(a3),
           "{r3}"(a4)
         : "memory" "cc"
         : "volatile");
    check_errno(ret)
}

#[cfg(nightly)]
#[inline(always)]
pub unsafe fn syscall5(
    n: Sysno,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> Result<usize, Errno> {
    let ret: usize;
    asm!("swi $$0"
         : "={r0}"(ret)
         : "{r7}"(n),
           "{r0}"(a1),
           "{r1}"(a2),
           "{r2}"(a3),
           "{r3}"(a4),
           "{r4}"(a5)
         : "memory" "cc"
         : "volatile");
    check_errno(ret)
}

#[cfg(nightly)]
#[inline(always)]
pub unsafe fn syscall6(
    n: Sysno,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> Result<usize, Errno> {
    let ret: usize;
    asm!("swi $$0"
         : "={r0}"(ret)
         : "{r7}"(n),
           "{r0}"(a1),
           "{r1}"(a2),
           "{r2}"(a3),
           "{r3}"(a4),
           "{r4}"(a5),
           "{r5}"(a6)
         : "memory" "cc"
         : "volatile");
    check_errno(ret)
}
