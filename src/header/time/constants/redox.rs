use crate::platform::types::*;

pub const CLOCK_REALTIME: c_int = 1;
pub const CLOCK_MONOTONIC: c_int = 4;
pub const CLOCK_PROCESS_CPUTIME_ID: clockid_t = 2;
