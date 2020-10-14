use super::{
    super::{types::*, PalEpoll},
    e, Sys,
};
use crate::header::{signal::sigset_t, sys_epoll::epoll_event};

impl PalEpoll for Sys {
    fn epoll_create1(flags: c_int) -> c_int {
        unimplemented!()
    }

    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int {
        unimplemented!()
    }

    fn epoll_pwait(
        epfd: c_int,
        events: *mut epoll_event,
        maxevents: c_int,
        timeout: c_int,
        sigmask: *const sigset_t,
    ) -> c_int {
        unimplemented!()
    }
}
