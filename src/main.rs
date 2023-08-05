#![feature(c_variadic, extern_types, label_break_value, core_intrinsics, thread_local)]
mod blkdev;
mod buffer;
mod canonicalize;
mod caputils;
mod color_names;
mod cpuset;
mod crc32c;
mod crc32;
mod crc64;
mod c_strtod;
mod encode;
mod env;
mod exec_shell;
mod fileutils;
mod idcache;
mod jsonwrt;
mod loopdev;
mod mangle;
mod mbsalign;
mod mbsedit;
mod md5;
mod path;
mod procfs;
mod pwdutils;
mod randutils;
mod sha1;
mod sha256;
mod signames;
mod strutils;
mod strv;
mod sysfs;
mod timeutils;
mod ttyutils;
mod xxhash;

mod unshare;

pub fn main() {
    unsafe {
        unshare::main()
    }
}
