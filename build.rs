use cc;

use std::{env, fs};

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let mut build = cc::Build::new();
    let compiler = build.get_compiler();
    if compiler.is_like_msvc() {
        build.ar_flag("-NODEFAULTLIB"); // Skip all default lib
    } else {
        build
            .flag("-nostdinc")
            .flag("-nostdlib")
            .flag("-fno-stack-protector")
            .flag("-Wno-expansion-to-defined")
            .include(&format!("{}/include", crate_dir))
            .include(&format!("{}/target/include", crate_dir));
    }
    build
        .files(
            fs::read_dir("src/c")
                .expect("src/c directory missing")
                .map(|res| res.expect("read_dir error").path()),
        )
        .compile("rulibc_c");

    println!("cargo:rustc-link-lib=static=rulibc_c");
}

//
// cargo b -Zbuild-std=core --target x86_64-pc-windows-msvc
// cargo b
// cl /MD kernel32.lib target\debug\rulibc.lib examples\test.c -X -Iinclude -Itarget\include -link  -NODEFAULTLIB
