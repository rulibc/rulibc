//! wchar implementation for Redox, following http://pubs.opengroup.org/onlinepubs/7908799/xsh/wchar.h.html

use crate::platform::types::{uint32_t, wint_t};

mod casecmp;
use casecmp::casemap;

#[no_mangle]
pub extern "C" fn towlower(wc: wint_t) -> wint_t {
    casemap(wc as uint32_t, 0) as wint_t
}

#[no_mangle]
pub extern "C" fn towupper(wc: wint_t) -> wint_t {
    casemap(wc as uint32_t, 1) as wint_t
}
