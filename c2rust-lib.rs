#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(thread_local)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate num_traits;
extern crate libc;
pub mod src {
pub mod blkdev;
pub mod buffer;
pub mod c_strtod;
pub mod canonicalize;
pub mod caputils;
pub mod color_names;
pub mod cpuset;
pub mod crc32;
pub mod crc32c;
pub mod crc64;
pub mod encode;
pub mod env;
pub mod exec_shell;
pub mod fileutils;
pub mod idcache;
pub mod jsonwrt;
pub mod linux_version;
pub mod loopdev;
pub mod mangle;
pub mod mbsalign;
pub mod mbsedit;
pub mod md5;
pub mod path;
pub mod procfs;
pub mod pwdutils;
pub mod r#match;
pub mod randutils;
pub mod sha1;
pub mod sha256;
pub mod signames;
pub mod strutils;
pub mod strv;
pub mod sysfs;
pub mod timeutils;
pub mod ttyutils;
pub mod xxhash;
} // mod lib
