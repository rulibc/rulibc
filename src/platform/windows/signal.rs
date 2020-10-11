use core::mem;

use super::{
    super::{types::*, PalSignal},
    e, Sys,
};
use crate::header::{
    signal::{sigaction, sigset_t, stack_t},
    sys_time::itimerval,
};

impl PalSignal for Sys {
    fn getitimer(which: c_int, out: *mut itimerval) -> c_int {
        unimplemented!()
    }

    fn kill(pid: pid_t, sig: c_int) -> c_int {
        unimplemented!()
    }

    fn killpg(pgrp: pid_t, sig: c_int) -> c_int {
        unimplemented!()
    }

    fn raise(sig: c_int) -> c_int {
        unimplemented!()
    }

    fn setitimer(which: c_int, new: *const itimerval, old: *mut itimerval) -> c_int {
        unimplemented!()
    }

    fn sigaction(sig: c_int, act: Option<&sigaction>, oact: Option<&mut sigaction>) -> c_int {
        unimplemented!()
    }

    fn sigaltstack(ss: *const stack_t, old_ss: *mut stack_t) -> c_int {
        unimplemented!()
    }

    fn sigprocmask(how: c_int, set: *const sigset_t, oset: *mut sigset_t) -> c_int {
        unimplemented!()
    }
}
