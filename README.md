# rulibc
rulibc is a portable POSIX C standard library written in Rust. It is under heavy development, and currently supports Windows, Linux and Redox.
This is the fork of relibc(https://github.com/redox-os/relibc) for more focusing on win32/linux support.
Other platform may also considerte if anybody interested to contribute.

glibc/musl alternative on linux world.

redox support will alomost unchanged with the upstream  https://github.com/redox-os/relibc

The motivation for this project is:
 - Create a safer alternative to a C standard library written in C.
 - On Windows, as a alternative to msvcrt/mingw32 crt, but also support for possible posix api, it's can be used
   cl/gcc/clang on windows
 - On Linux, remove the need of musl/glibc, and create glibc compatiable api set. It supports linux syscalls via the [sc](https://crates.io/crates/sc) crate.
 - Reduce issues the redox crew was having with newlib,

### [Contributing](CONTRIBUTING.md)

## Supported OSes

 - Windows
 - Linux
 - Redox OS

## Supported architectures

 - x86\_64
 - Aarch64

### Notice
 The main branch are not stable, will rebase on the upstream perdically
 use `git fetch --force` to sync with the main branch.
 The release branch will keep stable

## Developing
```
cargo install --force cargo-make
cargo make
```