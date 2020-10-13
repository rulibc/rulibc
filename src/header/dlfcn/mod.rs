//! dlfcn implementation for Redox, following http://pubs.opengroup.org/onlinepubs/7908799/xsh/dlfcn.h.html

use core::{
    ptr, str,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::platform::types::*;

#[cfg(not(target_os = "windows"))]
use crate::{c_str::CStr, ld_so::tcb::Tcb};

pub const RTLD_LAZY: c_int = 0x0001;
pub const RTLD_NOW: c_int = 0x0002;
pub const RTLD_GLOBAL: c_int = 0x0100;
pub const RTLD_LOCAL: c_int = 0x0000;

#[cfg(not(target_os = "windows"))]
static ERROR_NOT_SUPPORTED: &'static CStr = c_str!("dlfcn not supported");

#[thread_local]
static ERROR: AtomicUsize = AtomicUsize::new(0);

#[repr(C)]
pub struct Dl_info {
    dli_fname: *const c_char,
    dli_fbase: *mut c_void,
    dli_sname: *const c_char,
    dli_saddr: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn dladdr(addr: *mut c_void, info: *mut Dl_info) -> c_int {
    //TODO
    (*info).dli_fname = ptr::null();
    (*info).dli_fbase = ptr::null_mut();
    (*info).dli_sname = ptr::null();
    (*info).dli_saddr = ptr::null_mut();
    0
}

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub unsafe extern "C" fn dlopen(cfilename: *const c_char, flags: c_int) -> *mut c_void {
    //TODO support all sort of flags

    let filename = if cfilename.is_null() {
        None
    } else {
        Some(str::from_utf8_unchecked(
            CStr::from_ptr(cfilename).to_bytes(),
        ))
    };

    let tcb = match Tcb::current() {
        Some(tcb) => tcb,
        None => {
            eprintln!("dlopen: tcb not found");
            ERROR.store(ERROR_NOT_SUPPORTED.as_ptr() as usize, Ordering::SeqCst);
            return ptr::null_mut();
        }
    };
    if tcb.linker_ptr.is_null() {
        eprintln!("dlopen: linker not found");
        ERROR.store(ERROR_NOT_SUPPORTED.as_ptr() as usize, Ordering::SeqCst);
        return ptr::null_mut();
    }
    let mut linker = (&*tcb.linker_ptr).lock();

    let cbs_c = linker.cbs.clone();
    let cbs = cbs_c.borrow();

    let id = match (cbs.load_library)(&mut linker, filename) {
        Err(err) => {
            eprintln!("dlopen: failed to load {:?}", filename);
            ERROR.store(ERROR_NOT_SUPPORTED.as_ptr() as usize, Ordering::SeqCst);
            return ptr::null_mut();
        }
        Ok(id) => id,
    };

    if let Some(fname) = filename {
        if let Err(err) = (cbs.link)(&mut linker, None, None, Some(id)) {
            (cbs.unload)(&mut linker, id);
            eprintln!("dlopen: failed to link '{}': {}", fname, err);
            ERROR.store(ERROR_NOT_SUPPORTED.as_ptr() as usize, Ordering::SeqCst);
            return ptr::null_mut();
        };

        if let Err(err) = (cbs.run_init)(&mut linker, Some(id)) {
            (cbs.unload)(&mut linker, id);
            eprintln!("dlopen: failed to link '{}': {}", fname, err);
            ERROR.store(ERROR_NOT_SUPPORTED.as_ptr() as usize, Ordering::SeqCst);
            return ptr::null_mut();
        };
    }
    id as *mut c_void
}

#[cfg(target_os = "windows")]
#[no_mangle]
pub unsafe extern "C" fn dlopen(cfilename: *const c_char, flags: c_int) -> *mut c_void {
    unimplemented!();
}

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub unsafe extern "C" fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void {
    if symbol.is_null() {
        ERROR.store(ERROR_NOT_SUPPORTED.as_ptr() as usize, Ordering::SeqCst);
        return ptr::null_mut();
    }

    let symbol_str = str::from_utf8_unchecked(CStr::from_ptr(symbol).to_bytes());

    let tcb = match Tcb::current() {
        Some(tcb) => tcb,
        None => {
            eprintln!("dlsym: tcb not found");
            ERROR.store(ERROR_NOT_SUPPORTED.as_ptr() as usize, Ordering::SeqCst);
            return ptr::null_mut();
        }
    };

    if tcb.linker_ptr.is_null() {
        eprintln!("dlsym: linker not found");
        ERROR.store(ERROR_NOT_SUPPORTED.as_ptr() as usize, Ordering::SeqCst);
        return ptr::null_mut();
    }

    let linker = (&*tcb.linker_ptr).lock();
    let cbs_c = linker.cbs.clone();
    let cbs = cbs_c.borrow();
    if let Some(global) = (cbs.get_sym)(&linker, symbol_str, Some(handle as usize)) {
        global.as_ptr()
    } else {
        eprintln!("dlsym: symbol not found");
        ERROR.store(ERROR_NOT_SUPPORTED.as_ptr() as usize, Ordering::SeqCst);
        ptr::null_mut()
    }
}

#[cfg(target_os = "windows")]
#[no_mangle]
pub unsafe extern "C" fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void {
    unimplemented!();
}

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub unsafe extern "C" fn dlclose(handle: *mut c_void) -> c_int {
    let tcb = match Tcb::current() {
        Some(tcb) => tcb,
        None => {
            eprintln!("dlclose: tcb not found");
            ERROR.store(ERROR_NOT_SUPPORTED.as_ptr() as usize, Ordering::SeqCst);
            return -1;
        }
    };

    if tcb.linker_ptr.is_null() {
        eprintln!("dlclose: linker not found");
        ERROR.store(ERROR_NOT_SUPPORTED.as_ptr() as usize, Ordering::SeqCst);
        return -1;
    };
    let mut linker = (&*tcb.linker_ptr).lock();
    let cbs_c = linker.cbs.clone();
    let cbs = cbs_c.borrow();
    if let Err(err) = (cbs.run_fini)(&mut linker, Some(handle as usize)) {
        ERROR.store(ERROR_NOT_SUPPORTED.as_ptr() as usize, Ordering::SeqCst);
        return -1;
    };
    (cbs.unload)(&mut linker, handle as usize);
    0
}

#[cfg(target_os = "windows")]
#[no_mangle]
pub unsafe extern "C" fn dlclose(handle: *mut c_void) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn dlerror() -> *mut c_char {
    ERROR.swap(0, Ordering::SeqCst) as *mut c_char
}
