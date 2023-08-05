#![feature(c_variadic, extern_types, label_break_value, core_intrinsics, thread_local)]
mod caputils;
mod exec_shell;
mod idcache;
mod mbsedit;
mod procfs;
mod pwdutils;
mod signames;
mod strutils;

mod unshare;

pub fn main() {
    unsafe {
        unshare::main()
    }
}
