#![feature(c_variadic, extern_types, core_intrinsics, thread_local)]
mod caputils;
mod exec_shell;
mod idcache;
mod procfs;
mod pwdutils;
mod signames;
mod strutils;

mod unshare;

pub fn main() {
    unsafe { unshare::main() }
}
