extern crate cc;

use std::env;
extern crate encoding;
use encoding::{all::GBK, DecoderTrap, Encoding};
use std::{fs, fs::DirEntry, path::Path};

// include src/header directories that don't start with '_'
fn include_dir(d: &DirEntry) -> bool {
    d.metadata().map(|m| m.is_dir()).unwrap_or(false)
        && d.path()
            .iter()
            .nth(2)
            .map_or(false, |c| c.to_str().map_or(false, |x| !x.starts_with("_")))
}

fn generate_bindings(cbindgen_config_path: &Path) {
    let relative_path = cbindgen_config_path
        .strip_prefix("../src/header")
        .ok()
        .and_then(|p| p.parent())
        .and_then(|p| p.to_str())
        .unwrap()
        .replace("_", "/");
    let header_path = Path::new("../target/dist/include")
        .join(&relative_path)
        .with_extension("h");
    let mod_path = cbindgen_config_path.with_file_name("mod.rs");
    let config = cbindgen::Config::from_file(cbindgen_config_path).unwrap();
    let header_path_str = header_path.to_str().unwrap();
    println!("{}", header_path_str);
    if !header_path_str.starts_with("/") {
        cbindgen::Builder::new()
        .with_config(config)
        .with_src(mod_path)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(header_path);
    } else {
        // panic!("{}", header_path_str);
    }
}

fn main() {
    // Generate C includes
    // - based on contents of src/header/**
    // - headers written to target/include
    fs::read_dir(&Path::new("../src/header"))
        .unwrap()
        .into_iter()
        .filter_map(Result::ok)
        .filter(|d| include_dir(d))
        .map(|d| d.path().as_path().join("cbindgen.toml"))
        .filter(|p| p.exists())
        .for_each(|p| {
            println!("cargo:rerun-if-changed={:?}", p.parent().unwrap());
            println!("cargo:rerun-if-changed={:?}", p);
            println!("cargo:rerun-if-changed={:?}", p.with_file_name("mod.rs"));
            generate_bindings(&p);
    });

    let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let cargo_profile = std::env::var("PROFILE").unwrap();
    let cargo_target = std::env::var("TARGET").unwrap();

    let build = cc::Build::new();
    let compiler = build.get_compiler();

    let mut cmd = compiler.to_command();
    let target_dir = format!(
        "{}/../target/dist/{}-{}",
        crate_dir, cargo_target, cargo_profile
    );
    let dist_tets_dir = format!("{}/tests", target_dir);
    let _result = fs::create_dir(Path::new(&dist_tets_dir));
    let executable_path_arg = format!("-out:{}/tests/args.exe", target_dir);
    if compiler.is_like_msvc() {
        cmd.arg(&format!("-I{}/../include", crate_dir))
            .arg(&format!("-I{}/../target/dist/include", crate_dir))
            .arg("args.c")
            .args(&["-link", "-NODEFAULTLIB"])
            .arg(&format!("-LIBPATH:{}/lib", target_dir))
            .args(&["rulibc.lib", "kernel32.lib", "User32.lib"])
            .arg(executable_path_arg);
    }
    let status = cmd.status();
    let code = status.unwrap().code().unwrap();
    if code != 0 {
        let output = cmd.output();
        let s = output.unwrap().stdout;
        let val = GBK.decode(&s, DecoderTrap::Replace);
        panic!(
            "process exited with: exit code:{} {} {:?} {:?}",
            code, target_dir, val, cmd
        );
    }
}
