use super::{
    super::{types::*, PalPtrace},
    Sys,
};

impl PalPtrace for Sys {
    fn ptrace(request: c_int, pid: pid_t, addr: *mut c_void, data: *mut c_void) -> c_int {
        unimplemented!()
    }
}
