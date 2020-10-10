//! <p id="core_io-show-docblock"></p>
//! This is just a listing of the functionality available in this crate. See
//! the [std documentation](https://doc.rust-lang.org/nightly/std/io/index.html)
//! for a full description of the functionality.
#![allow(stable_features,unused_features)]
#![feature(question_mark,const_fn,copy_from_slice,
	try_from,str_internals,align_offset,doc_spotlight,slice_internals)]
#![no_std]


extern crate alloc;
#[cfg(rustc_unicode)]
extern crate rustc_unicode;
#[cfg(std_unicode)]
extern crate std_unicode;

#[path="io/mod.rs"] mod io;

pub use io::*;
