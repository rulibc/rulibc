use alloc::vec::Vec;
use core::{intrinsics};
use core::ptr::null_mut;
use winapi::um::winuser::{MB_OK, MessageBoxW};

use crate::{
    header::{stdio, stdlib},
    platform::{self, new_mspace, types::*},
    ALLOCATOR,
};

fn alloc_init() {
    /* TODO: how to init alloc on windows properly */
    if ALLOCATOR.get_book_keeper() == 0 {
        ALLOCATOR.set_book_keeper(new_mspace());
    }
}

fn io_init() {
    unsafe {
        // Initialize stdin/stdout/stderr, see https://github.com/rust-lang/rust/issues/51718
        stdio::stdin = stdio::default_stdin.get();
        stdio::stdout = stdio::default_stdout.get();
        stdio::stderr = stdio::default_stderr.get();
    }
}

#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn relibc_start() -> ! {
    extern "C" {
        fn main(argc: isize, argv: *mut *mut c_char, envp: *mut *mut c_char) -> c_int;
    }

    // Set up the right allocator...
    // if any memory rust based memory allocation happen before this step .. we are doomed.
    alloc_init();
    io_init();

    print_message("Hello, the world");

    let argc: isize = 0;
    // Set up argc and argv
    // platform::inner_argv = copy_string_array(argv, argc as usize);
    platform::argv = platform::inner_argv.as_mut_ptr();

    // Set up envp
    // platform::inner_environ = copy_string_array(envp, len);
    platform::environ = platform::inner_environ.as_mut_ptr();

    // not argv or envp, because programs like bash try to modify this *const* pointer :|
    stdlib::exit(main(argc, platform::argv, platform::environ));

    unreachable!();
}

fn print_message(msg: &str) {
    let wide: Vec<u16> = msg.encode_utf16().collect();
    let ret = unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
    };
}

#[no_mangle]
pub unsafe extern "C" fn mainCRTStartup() -> ! {
    relibc_start();
    unreachable!();
}

// TODO:
#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn __CxxFrameHandler3() {}

// TODO:
#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn __GSHandlerCheck() {}

// Define _fltused, since we're not linking against the MS C runtime, but use
// floats.
#[linkage = "weak"]
#[used]
#[no_mangle]
pub static _fltused: i32 = 0;

// NOTE These functions are implemented using assembly because they using a custom
// calling convention which can't be implemented using a normal Rust function

// NOTE These functions are never mangled as they are not tested against compiler-rt
// and mangling ___chkstk would break the `jmp ___chkstk` instruction in __alloca

#[naked]
#[no_mangle]
pub unsafe fn ___chkstk_ms() {
    llvm_asm!("
        push   %rcx
        push   %rax
        cmp    $$0x1000,%rax
        lea    24(%rsp),%rcx
        jb     1f
    2:
        sub    $$0x1000,%rcx
        test   %rcx,(%rcx)
        sub    $$0x1000,%rax
        cmp    $$0x1000,%rax
        ja     2b
    1:
        sub    %rax,%rcx
        test   %rcx,(%rcx)
        pop    %rax
        pop    %rcx
        ret" ::: "memory" : "volatile");
    intrinsics::unreachable();
}

#[naked]
#[no_mangle]
pub unsafe fn __alloca() {
    llvm_asm!("mov    %rcx,%rax  // x64 _alloca is a normal function with parameter in rcx
          jmp    __chkstk  // Jump to __chkstk since fallthrough may be unreliable"
         ::: "memory" : "volatile");
    intrinsics::unreachable();
}

#[naked]
#[no_mangle]
pub unsafe fn __chkstk() {
    llvm_asm!(
        "
        push   %rcx
        cmp    $$0x1000,%rax
        lea    16(%rsp),%rcx  // rsp before calling this routine -> rcx
        jb     1f
    2:
        sub    $$0x1000,%rcx
        test   %rcx,(%rcx)
        sub    $$0x1000,%rax
        cmp    $$0x1000,%rax
        ja     2b
    1:
        sub    %rax,%rcx
        test   %rcx,(%rcx)
        lea    8(%rsp),%rax   // load pointer to the return address into rax
        mov    %rcx,%rsp      // install the new top of stack pointer into rsp
        mov    -8(%rax),%rcx  // restore rcx
        push   (%rax)         // push return address onto the stack
        sub    %rsp,%rax      // restore the original value in rax
        ret"
        ::: "memory" : "volatile"
    );
    intrinsics::unreachable();
}
