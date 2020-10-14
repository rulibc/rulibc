use super::{
    super::{types::*, PalSocket},
    Sys,
};
use crate::header::sys_socket::{sockaddr, socklen_t};

impl PalSocket for Sys {
    unsafe fn accept(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int {
        unimplemented!()
    }

    unsafe fn bind(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int {
        unimplemented!()
    }

    unsafe fn connect(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int {
        unimplemented!()
    }

    unsafe fn getpeername(
        socket: c_int,
        address: *mut sockaddr,
        address_len: *mut socklen_t,
    ) -> c_int {
        unimplemented!()
    }

    unsafe fn getsockname(
        socket: c_int,
        address: *mut sockaddr,
        address_len: *mut socklen_t,
    ) -> c_int {
        unimplemented!()
    }

    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut socklen_t,
    ) -> c_int {
        unimplemented!()
    }

    fn listen(socket: c_int, backlog: c_int) -> c_int {
        unimplemented!()
    }

    unsafe fn recvfrom(
        socket: c_int,
        buf: *mut c_void,
        len: size_t,
        flags: c_int,
        address: *mut sockaddr,
        address_len: *mut socklen_t,
    ) -> ssize_t {
        unimplemented!()
    }

    unsafe fn sendto(
        socket: c_int,
        buf: *const c_void,
        len: size_t,
        flags: c_int,
        dest_addr: *const sockaddr,
        dest_len: socklen_t,
    ) -> ssize_t {
        unimplemented!()
    }

    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int {
        unimplemented!()
    }

    fn shutdown(socket: c_int, how: c_int) -> c_int {
        unimplemented!()
    }

    unsafe fn socket(domain: c_int, kind: c_int, protocol: c_int) -> c_int {
        unimplemented!()
    }

    fn socketpair(domain: c_int, kind: c_int, protocol: c_int, sv: &mut [c_int; 2]) -> c_int {
        unimplemented!()
    }
}
