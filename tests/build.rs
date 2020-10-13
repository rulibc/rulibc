extern crate cc;

use std::env;
extern crate encoding;
use encoding::{all::GBK, DecoderTrap, Encoding};
use std::{fs, fs::DirEntry, path::Path};

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let cargo_profile = std::env::var("PROFILE").unwrap();
    let cargo_target = std::env::var("TARGET").unwrap();

    let mut build = cc::Build::new();
    let compiler = build.get_compiler();

    let mut cmd = compiler.to_command();
    let target_dir = format!(
        "{}/../target/dist/{}-{}",
        crate_dir, cargo_target, cargo_profile
    );
    let dist_tets_dir = format!("{}/tests", target_dir);
    fs::create_dir(Path::new(&dist_tets_dir));
    let executable_path_arg = format!("-out:{}/tests/args.exe", target_dir);
    if compiler.is_like_msvc() {
        cmd.arg(&format!("-I{}/../include", crate_dir))
            .arg(&format!("-I{}/../target/include", crate_dir))
            .arg("args.c")
            .args(&["-link", "-NODEFAULTLIB"])
            .arg(&format!("-LIBPATH:{}/lib", target_dir))
            .args(&["rulibc.lib", "kernel32.lib"])
            .arg(executable_path_arg);
    }
    let status = cmd.status();
    let code = status.unwrap().code().unwrap();
    if (code != 0) {
        let output = cmd.output();
        let s = output.unwrap().stdout;
        let val = GBK.decode(&s, DecoderTrap::Replace);
        panic!(
            "process exited with: exit code:{} {} {:?} {:?}",
            code, target_dir, val, cmd
        );
    }
}
