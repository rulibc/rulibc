use super::linker::{Linker, Symbol, DSO};
use alloc::boxed::Box;

pub struct LinkerCallbacks {
    pub unload: Box<dyn Fn(&mut Linker, usize)>,
    pub load_library: Box<dyn Fn(&mut Linker, Option<&str>) -> Result<usize>>,
    pub link:
        Box<dyn Fn(&mut Linker, Option<&str>, Option<DSO>, Option<usize>) -> Result<Option<usize>>>,
    pub get_sym: Box<dyn Fn(&Linker, &str, Option<usize>) -> Option<Symbol>>,
    pub run_init: Box<dyn Fn(&Linker, Option<usize>) -> Result<()>>,
    pub run_fini: Box<dyn Fn(&Linker, Option<usize>) -> Result<()>>,
}

impl LinkerCallbacks {
    pub fn new() -> LinkerCallbacks {
        LinkerCallbacks {
            unload: Box::new(unload),
            load_library: Box::new(load_library),
            link: Box::new(link),
            get_sym: Box::new(get_sym),
            run_init: Box::new(run_init),
            run_fini: Box::new(run_fini),
        }
    }
}

fn unload(linker: &mut Linker, libspace: usize) {
    linker.unload(libspace)
}

fn load_library(linker: &mut Linker, name: Option<&str>) -> Result<usize> {
    linker.load_library(name)
}

fn link(
    linker: &mut Linker,
    primary_opt: Option<&str>,
    dso: Option<DSO>,
    libspace: Option<usize>,
) -> Result<Option<usize>> {
    linker.link(primary_opt, dso, libspace)
}

fn get_sym(linker: &Linker, name: &str, libspace: Option<usize>) -> Option<Symbol> {
    linker.get_sym(name, libspace)
}
fn run_init(linker: &Linker, libspace: Option<usize>) -> Result<()> {
    linker.run_init(libspace)
}
fn run_fini(linker: &Linker, libspace: Option<usize>) -> Result<()> {
    linker.run_fini(libspace)
}
