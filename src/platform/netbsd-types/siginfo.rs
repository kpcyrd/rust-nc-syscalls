// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

//! From `/usr/include/sys/siginfo.h`

#[repr(C)]
pub union sigval_t {
	sival_int: i32,
	sival_ptr: usize,
}

/// signal caused by trap
pub const KSI_TRAP: i32 = 0x01;
/// no additional information
pub const KSI_EMPTY: i32 = 0x02;
/// on a sigpend_t queue
pub const KSI_QUEUED: i32 = 0x04;
/// allocated from the ksiginfo pool
pub const KSI_FROMPOOL: i32 = 0x08;

#[repr(C)]
pub union siginfo_t {
    /// Total size; for future expansion
	si_pad: [u8; 128],
	//_info: _ksiginfo_t,
}

/// si_code
///
/// SIGILL
/// Illegal opcode
pub const ILL_ILLOPC: i32 = 1;
/// Illegal operand
pub const ILL_ILLOPN: i32 = 2;
/// Illegal addressing mode
pub const ILL_ILLADR: i32 = 3;
/// Illegal trap
pub const ILL_ILLTRP: i32 = 4;
/// Privileged opcode
pub const ILL_PRVOPC: i32 = 5;
/// Privileged register
pub const ILL_PRVREG: i32 = 6;
/// Coprocessor error
pub const ILL_COPROC: i32 = 7;
/// Internal stack error
pub const ILL_BADSTK: i32 = 8;

/// SIGFPE
/// Integer divide by zero
pub const FPE_INTDIV: i32 = 1;
/// Integer overflow
pub const FPE_INTOVF: i32 = 2;
/// Floating point divide by zero
pub const FPE_FLTDIV: i32 = 3;
/// Floating point overflow
pub const FPE_FLTOVF: i32 = 4;
/// Floating point underflow
pub const FPE_FLTUND: i32 = 5;
/// Floating point inexact result
pub const FPE_FLTRES: i32 = 6;
/// Invalid Floating point operation
pub const FPE_FLTINV: i32 = 7;
/// Subscript out of range
pub const FPE_FLTSUB: i32 = 8;

/// SIGSEGV
/// Address not mapped to object
pub const SEGV_MAPERR: i32 = 1;
/// Invalid permissions for mapped object
pub const SEGV_ACCERR: i32 = 2;

/// SIGBUS
/// Invalid address alignment
pub const BUS_ADRALN: i32 = 1;
/// Non-existent physical address
pub const BUS_ADRERR: i32 = 2;
/// Object specific hardware error
pub const BUS_OBJERR: i32 = 3;

/// SIGTRAP
/// Process breakpoint
pub const TRAP_BRKPT: i32 = 1;
/// Process trace trap
pub const TRAP_TRACE: i32 = 2;
/// Process exec trap
pub const TRAP_EXEC: i32 = 3;
/// Process child trap
pub const TRAP_CHLD: i32 = 4;
/// Process lwp trap
pub const TRAP_LWP: i32 = 5;
/// Process hardware debug register trap
pub const TRAP_DBREG: i32 = 6;
/// Process syscall entry trap
pub const TRAP_SCE: i32 = 7;
/// Process syscall exit trap
pub const TRAP_SCX: i32 = 8;

/// SIGCHLD
/// Child has exited
pub const CLD_EXITED: i32 = 1;
/// Child has terminated abnormally but
/// did not create a core file
pub const CLD_KILLED: i32 = 2;
/// Child has terminated abnormally and
/// created a core file
pub const CLD_DUMPED: i32 = 3;
/// Traced child has trapped
pub const CLD_TRAPPED: i32 = 4;
/// Child has stopped
pub const CLD_STOPPED: i32 = 5;
/// Stopped child has continued
pub const CLD_CONTINUED: i32 = 6;

/// SIGIO
/// Data input available
pub const POLL_IN: i32 = 1;
/// Output buffers available
pub const POLL_OUT: i32 = 2;
/// Input message available
pub const POLL_MSG: i32 = 3;
/// I/O Error
pub const POLL_ERR: i32 = 4;
/// High priority input available
pub const POLL_PRI: i32 = 5;
/// Device disconnected
pub const POLL_HUP: i32 = 6;

/// * si_code
/// Sent by kill(2)
pub const SI_USER: i32 = 0;
/// Sent by the sigqueue(2)
pub const SI_QUEUE: i32 = -1;
/// Generated by expiration of a timer
/// set by timer_settime(2)
pub const SI_TIMER: i32 = -2;
/// Generated by completion of an
/// asynchronous I/O signal
pub const SI_ASYNCIO: i32 = -3;
/// Generated by arrival of a message on
/// an empty message queue
pub const SI_MESGQ: i32 = -4;
/// Generated by _lwp_kill(2)
pub const SI_LWP: i32 = -5;
/// No signal specific info available
pub const SI_NOINFO: i32 = 32767;

