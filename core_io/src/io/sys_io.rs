use alloc::slice;
use core::marker::PhantomData;

use platform_types::{c_void, iov_len_t, iovec};

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct IoSlice<'a> {
    vec: iovec,
    _p: PhantomData<&'a [u8]>,
}

impl<'a> IoSlice<'a> {
    #[inline]
    pub fn new(buf: &'a [u8]) -> IoSlice<'a> {
        IoSlice {
            vec: iovec {
                iov_base: buf.as_ptr() as *mut u8 as *mut c_void,
                iov_len: buf.len() as iov_len_t,
            },
            _p: PhantomData,
        }
    }

    #[inline]
    pub fn advance(&mut self, n: usize) {
        if (self.vec.iov_len as usize) < n {
            panic!("advancing IoSlice beyond its length");
        }

        unsafe {
            self.vec.iov_len -= n as iov_len_t;
            self.vec.iov_base = self.vec.iov_base.add(n);
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.vec.iov_base as *mut u8, self.vec.iov_len as usize) }
    }
}

#[repr(transparent)]
pub struct IoSliceMut<'a> {
    vec: iovec,
    _p: PhantomData<&'a mut [u8]>,
}

impl<'a> IoSliceMut<'a> {
    #[inline]
    pub fn new(buf: &'a mut [u8]) -> IoSliceMut<'a> {
        IoSliceMut {
            vec: iovec {
                iov_base: buf.as_mut_ptr() as *mut c_void,
                iov_len: buf.len() as iov_len_t,
            },
            _p: PhantomData,
        }
    }

    #[inline]
    pub fn advance(&mut self, n: usize) {
        if (self.vec.iov_len as usize) < n {
            panic!("advancing IoSliceMut beyond its length");
        }

        unsafe {
            self.vec.iov_len -= n as iov_len_t;
            self.vec.iov_base = self.vec.iov_base.add(n);
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.vec.iov_base as *mut u8, self.vec.iov_len as usize) }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self.vec.iov_base as *mut u8, self.vec.iov_len as usize)
        }
    }
}
