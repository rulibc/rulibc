//! crt0

#![no_std]
#![feature(asm)]
#![feature(linkage)]
#![feature(llvm_asm)]
#![feature(naked_functions)]

#[no_mangle]
#[naked]
pub unsafe extern "C" fn _start() {
    #[cfg(target_arch = "x86_64")]
    llvm_asm!("mov rdi, rsp
        and rsp, 0xFFFFFFFFFFFFFFF0
        call rulibc_start"
        :
        :
        :
        : "intel", "volatile"
    );
    #[cfg(target_arch = "aarch64")]
    llvm_asm!("mov x0, sp
        bl rulibc_start"
        :
        :
        :
        : "volatile"
    );
}

#[panic_handler]
#[linkage = "weak"]
#[no_mangle]
pub unsafe fn rust_begin_unwind(pi: &::core::panic::PanicInfo) -> ! {
    extern "C" {
        fn rulibc_panic(pi: &::core::panic::PanicInfo) -> !;
    }
    rulibc_panic(pi)
}
