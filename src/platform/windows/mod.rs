use core::ptr;
use core_io::Write;

use super::{errno, types::*, Pal};
use crate::{
    c_str::CStr,
    header::{dirent::dirent, signal::SIGCHLD, sys_stat::S_IFIFO},
};
// use header::sys_resource::rusage;
use crate::header::{
    sys_resource::rlimit,
    sys_stat::stat,
    sys_statvfs::statvfs,
    sys_time::{timeval, timezone},
};
// use header::sys_times::tms;
use crate::header::{sys_utsname::utsname, time::timespec};

mod epoll;
mod ptrace;
mod signal;
mod socket;

pub fn e(sys: usize) -> usize {
    if (sys as isize) < 0 && (sys as isize) >= -256 {
        unsafe {
            errno = -(sys as isize) as c_int;
        }
        !0
    } else {
        sys
    }
}

pub struct Sys;

impl Sys {
    // fn getrusage(who: c_int, r_usage: *mut rusage) -> c_int {
    //     e(unsafe { syscall!(GETRUSAGE, who, r_usage) }) as c_int
    // }

    pub unsafe fn ioctl(fd: c_int, request: c_ulong, out: *mut c_void) -> c_int {
        // TODO: Somehow support varargs to syscall??
        unimplemented!()
    }

    // fn times(out: *mut tms) -> clock_t {
    //     unsafe { syscall!(TIMES, out) as clock_t }
    // }
}

impl Pal for Sys {
    fn access(path: &CStr, mode: c_int) -> c_int {
        unimplemented!()
    }

    fn brk(addr: *mut c_void) -> *mut c_void {
        unimplemented!()
    }

    fn chdir(path: &CStr) -> c_int {
        unimplemented!()
    }

    fn chmod(path: &CStr, mode: mode_t) -> c_int {
        unimplemented!()
    }

    fn chown(path: &CStr, owner: uid_t, group: gid_t) -> c_int {
        unimplemented!()
    }

    fn clock_gettime(clk_id: clockid_t, tp: *mut timespec) -> c_int {
        unimplemented!()
    }

    fn close(fildes: c_int) -> c_int {
        unimplemented!()
    }

    fn dup(fildes: c_int) -> c_int {
        unimplemented!()
    }

    fn dup2(fildes: c_int, fildes2: c_int) -> c_int {
        unimplemented!()
    }

    unsafe fn execve(path: &CStr, argv: *const *mut c_char, envp: *const *mut c_char) -> c_int {
        unimplemented!()
    }

    fn exit(status: c_int) -> ! {
        unsafe { unimplemented!() }
        loop {}
    }

    fn fchdir(fildes: c_int) -> c_int {
        unimplemented!()
    }

    fn fchmod(fildes: c_int, mode: mode_t) -> c_int {
        unimplemented!()
    }

    fn fchown(fildes: c_int, owner: uid_t, group: gid_t) -> c_int {
        unimplemented!()
    }

    fn flock(fd: c_int, operation: c_int) -> c_int {
        unimplemented!()
    }

    fn fstat(fildes: c_int, buf: *mut stat) -> c_int {
        let empty = b"\0";
        let empty_ptr = empty.as_ptr() as *const c_char;
        unimplemented!()
    }

    fn fstatvfs(fildes: c_int, buf: *mut statvfs) -> c_int {
        unimplemented!()
    }

    fn fcntl(fildes: c_int, cmd: c_int, arg: c_int) -> c_int {
        unimplemented!()
    }

    fn fork() -> pid_t {
        unimplemented!()
    }

    fn fpath(fildes: c_int, out: &mut [u8]) -> ssize_t {
        unimplemented!()
    }

    fn fsync(fildes: c_int) -> c_int {
        unimplemented!()
    }

    fn ftruncate(fildes: c_int, length: off_t) -> c_int {
        unimplemented!()
    }

    fn futex(addr: *mut c_int, op: c_int, val: c_int) -> c_int {
        unimplemented!()
    }

    fn futimens(fd: c_int, times: *const timespec) -> c_int {
        unimplemented!()
    }

    fn utimens(path: &CStr, times: *const timespec) -> c_int {
        unimplemented!()
    }

    fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char {
        unimplemented!()
    }

    fn getdents(fd: c_int, dirents: *mut dirent, bytes: usize) -> c_int {
        unimplemented!()
    }

    fn getegid() -> gid_t {
        unimplemented!()
    }

    fn geteuid() -> uid_t {
        unimplemented!()
    }

    fn getgid() -> gid_t {
        unimplemented!()
    }

    fn getpgid(pid: pid_t) -> pid_t {
        unimplemented!()
    }

    fn getpid() -> pid_t {
        unimplemented!()
    }

    fn getppid() -> pid_t {
        unimplemented!()
    }

    fn getrandom(buf: &mut [u8], flags: c_uint) -> ssize_t {
        unimplemented!()
    }

    unsafe fn getrlimit(resource: c_int, rlim: *mut rlimit) -> c_int {
        unimplemented!()
    }

    fn gettid() -> pid_t {
        unimplemented!()
    }

    fn gettimeofday(tp: *mut timeval, tzp: *mut timezone) -> c_int {
        unimplemented!()
    }

    fn getuid() -> uid_t {
        unimplemented!()
    }

    fn link(path1: &CStr, path2: &CStr) -> c_int {
        unimplemented!()
    }

    fn lseek(fildes: c_int, offset: off_t, whence: c_int) -> off_t {
        unimplemented!()
    }

    fn mkdir(path: &CStr, mode: mode_t) -> c_int {
        unimplemented!()
    }

    fn mkfifo(path: &CStr, mode: mode_t) -> c_int {
        unimplemented!()
    }

    unsafe fn mmap(
        addr: *mut c_void,
        len: usize,
        prot: c_int,
        flags: c_int,
        fildes: c_int,
        off: off_t,
    ) -> *mut c_void {
        unimplemented!()
    }

    unsafe fn mprotect(addr: *mut c_void, len: usize, prot: c_int) -> c_int {
        unimplemented!()
    }

    unsafe fn msync(addr: *mut c_void, len: usize, flags: c_int) -> c_int {
        unimplemented!()
    }

    unsafe fn munmap(addr: *mut c_void, len: usize) -> c_int {
        unimplemented!()
    }

    fn nanosleep(rqtp: *const timespec, rmtp: *mut timespec) -> c_int {
        unimplemented!()
    }

    fn open(path: &CStr, oflag: c_int, mode: mode_t) -> c_int {
        unimplemented!()
    }

    fn pipe2(fildes: &mut [c_int], flags: c_int) -> c_int {
        unimplemented!()
    }

    #[cfg(target_arch = "x86_64")]
    unsafe fn pte_clone(stack: *mut usize) -> pid_t {
        unimplemented!()
    }

    fn read(fildes: c_int, buf: &mut [u8]) -> ssize_t {
        unimplemented!()
    }

    fn readlink(pathname: &CStr, out: &mut [u8]) -> ssize_t {
        unimplemented!()
    }

    fn rename(old: &CStr, new: &CStr) -> c_int {
        unimplemented!()
    }

    fn rmdir(path: &CStr) -> c_int {
        unimplemented!()
    }

    fn sched_yield() -> c_int {
        unimplemented!()
    }

    fn setpgid(pid: pid_t, pgid: pid_t) -> c_int {
        unimplemented!()
    }

    fn setregid(rgid: gid_t, egid: gid_t) -> c_int {
        unimplemented!()
    }

    fn setreuid(ruid: uid_t, euid: uid_t) -> c_int {
        unimplemented!()
    }

    fn symlink(path1: &CStr, path2: &CStr) -> c_int {
        unimplemented!()
    }

    fn umask(mask: mode_t) -> mode_t {
        unimplemented!()
    }

    fn uname(utsname: *mut utsname) -> c_int {
        unimplemented!()
    }

    fn unlink(path: &CStr) -> c_int {
        unimplemented!()
    }

    fn waitpid(pid: pid_t, stat_loc: *mut c_int, options: c_int) -> pid_t {
        unimplemented!()
    }

    fn write(fildes: c_int, buf: &[u8]) -> ssize_t {
        unimplemented!()
    }

    fn verify() -> bool {
        // GETPID on Linux is 39, which does not exist on Redox
        unimplemented!()
    }
}
