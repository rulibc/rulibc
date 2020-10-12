pub use platform_types::*;

//===================================================================
// TODO: Not implement yet
// These types are should compatiable with exist compiler and os
// and they have no design flow.
// And different platform may have different type size
//===================================================================

pub type regoff_t = size_t;
pub type mode_t = c_int;
pub type pid_t = c_int;
pub type id_t = c_uint;
pub type gid_t = c_int;
pub type uid_t = c_int;
pub type clockid_t = c_int;

//===================================================================
// These types are should compatiable with exist compiler and os
// and they have no design flow. And seems shared across all
// compiler/os
//===================================================================

pub type useconds_t = c_uint;
pub type suseconds_t = c_int;

//===================================================================
// These type are force to int64_t/uint64_t to get rid of the history
// definition's limitation, these may incompatiable with old native
// compiler and platform, but also remove the limitation of those
// compiler and platform.
// And those issue always need to be fixed.
//===================================================================

pub type off_t = int64_t;
pub type time_t = int64_t;
pub type dev_t = int64_t;
pub type ino_t = uint64_t;

pub type nlink_t = uint64_t;
pub type blksize_t = int64_t;
pub type blkcnt_t = uint64_t;

pub type fsblkcnt_t = uint64_t;
pub type fsfilcnt_t = uint64_t;

pub type clock_t = int64_t;
pub type timer_t = *mut c_void;
