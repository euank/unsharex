use crate::exec_shell::exec_shell;
use ::c2rust_bitfields::BitfieldStruct;
use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    static mut program_invocation_short_name: *mut libc::c_char;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    fn unshare(__flags: libc::c_int) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vasprintf(
        __ptr: *mut *mut libc::c_char,
        __f: *const libc::c_char,
        __arg: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn strtoul(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn execvp(__file: *const libc::c_char, __argv: *const *mut libc::c_char) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn getpid() -> __pid_t;
    fn geteuid() -> __uid_t;
    fn getegid() -> __gid_t;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn setgid(__gid: __gid_t) -> libc::c_int;
    fn fork() -> __pid_t;
    fn chroot(__path: *const libc::c_char) -> libc::c_int;
    fn syscall(__sysno: libc::c_long, _: ...) -> libc::c_long;
    fn eventfd(__count: libc::c_uint, __flags: libc::c_int) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int, __options: libc::c_int) -> __pid_t;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn mount(
        __special_file: *const libc::c_char,
        __dir: *const libc::c_char,
        __fstype: *const libc::c_char,
        __rwflag: libc::c_ulong,
        __data: *const libc::c_void,
    ) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn prctl(__option: libc::c_int, _: ...) -> libc::c_int;
    fn setgroups(__n: size_t, __groups: *const __gid_t) -> libc::c_int;
    fn setlocale(__category: libc::c_int, __locale: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn nanosleep(__requested_time: *const timespec, __remaining: *mut timespec) -> libc::c_int;
    fn err(__status: libc::c_int, __format: *const libc::c_char, _: ...) -> !;
    fn warnx(__format: *const libc::c_char, _: ...);
    fn warn(__format: *const libc::c_char, _: ...);
    fn errx(__status: libc::c_int, __format: *const libc::c_char, _: ...) -> !;
    fn cap_last_cap() -> libc::c_int;
    fn cap_permitted_to_ambient();
    fn signame_to_signum(sig: *const libc::c_char) -> libc::c_int;
    fn str2num_or_err(
        str: *const libc::c_char,
        base: libc::c_int,
        errmesg: *const libc::c_char,
        low: int64_t,
        up: int64_t,
    ) -> int64_t;
    fn str2unum_or_err(
        str: *const libc::c_char,
        base: libc::c_int,
        errmesg: *const libc::c_char,
        up: uint64_t,
    ) -> uint64_t;
    fn xgetpwnam(username: *const libc::c_char, pwdbuf: *mut *mut libc::c_char) -> *mut passwd;
    fn xgetgrnam(groupname: *const libc::c_char, grpbuf: *mut *mut libc::c_char) -> *mut group;
    fn xgetpwuid(uid: uid_t, pwdbuf: *mut *mut libc::c_char) -> *mut passwd;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __clockid_t = libc::c_int;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type size_t = libc::c_ulong;
pub type pid_t = __pid_t;
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
pub type ino_t = __ino_t;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type clockid_t = __clockid_t;
pub type useconds_t = __useconds_t;
pub type int64_t = __int64_t;
pub type sigset_t = __sigset_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type __sighandler_t = Option<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type C2RustUnnamed = libc::c_int;
pub const MS_NOUSER: C2RustUnnamed = -2147483648;
pub const MS_ACTIVE: C2RustUnnamed = 1073741824;
pub const MS_LAZYTIME: C2RustUnnamed = 33554432;
pub const MS_STRICTATIME: C2RustUnnamed = 16777216;
pub const MS_I_VERSION: C2RustUnnamed = 8388608;
pub const MS_KERNMOUNT: C2RustUnnamed = 4194304;
pub const MS_RELATIME: C2RustUnnamed = 2097152;
pub const MS_SHARED: C2RustUnnamed = 1048576;
pub const MS_SLAVE: C2RustUnnamed = 524288;
pub const MS_PRIVATE: C2RustUnnamed = 262144;
pub const MS_UNBINDABLE: C2RustUnnamed = 131072;
pub const MS_POSIXACL: C2RustUnnamed = 65536;
pub const MS_SILENT: C2RustUnnamed = 32768;
pub const MS_REC: C2RustUnnamed = 16384;
pub const MS_MOVE: C2RustUnnamed = 8192;
pub const MS_BIND: C2RustUnnamed = 4096;
pub const MS_NODIRATIME: C2RustUnnamed = 2048;
pub const MS_NOATIME: C2RustUnnamed = 1024;
pub const MS_NOSYMFOLLOW: C2RustUnnamed = 256;
pub const MS_DIRSYNC: C2RustUnnamed = 128;
pub const MS_MANDLOCK: C2RustUnnamed = 64;
pub const MS_REMOUNT: C2RustUnnamed = 32;
pub const MS_SYNCHRONOUS: C2RustUnnamed = 16;
pub const MS_NOEXEC: C2RustUnnamed = 8;
pub const MS_NODEV: C2RustUnnamed = 4;
pub const MS_NOSUID: C2RustUnnamed = 2;
pub const MS_RDONLY: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct namespace_file {
    pub type_0: libc::c_int,
    pub name: *const libc::c_char,
    pub target: *const libc::c_char,
}
pub type C2RustUnnamed_0 = libc::c_int;
pub const SETGROUPS_ALLOW: C2RustUnnamed_0 = 1;
pub const SETGROUPS_DENY: C2RustUnnamed_0 = 0;
pub const SETGROUPS_NONE: C2RustUnnamed_0 = -1;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_1 {}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_2 {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct prop_opts {
    pub name: *const libc::c_char,
    pub flag: libc::c_ulong,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_3 {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct map_range {
    pub outer: libc::c_uint,
    pub inner: libc::c_uint,
    pub count: libc::c_uint,
}
pub const OPT_BOOTTIME: C2RustUnnamed_4 = 134;
pub const OPT_MONOTONIC: C2RustUnnamed_4 = 133;
pub const OPT_KEEPCAPS: C2RustUnnamed_4 = 132;
pub const OPT_KILLCHILD: C2RustUnnamed_4 = 131;
pub const OPT_PROPAGATION: C2RustUnnamed_4 = 129;
pub const OPT_SETGROUPS: C2RustUnnamed_4 = 130;
pub const OPT_MAPAUTO: C2RustUnnamed_4 = 139;
pub const OPT_MAPGROUPS: C2RustUnnamed_4 = 138;
pub const OPT_MAPUSERS: C2RustUnnamed_4 = 136;
pub const OPT_MAPGROUP: C2RustUnnamed_4 = 137;
pub const OPT_MAPUSER: C2RustUnnamed_4 = 135;
pub const OPT_MOUNTPROC: C2RustUnnamed_4 = 128;
pub type C2RustUnnamed_4 = libc::c_uint;
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut libc::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
#[inline]
unsafe extern "C" fn xusleep(usec: useconds_t) -> libc::c_int {
    let mut waittime: timespec = {
        let init = timespec {
            tv_sec: usec as libc::c_long / 1000000 as libc::c_long,
            tv_nsec: usec as libc::c_long % 1000000 as libc::c_long
                * 1000 as libc::c_int as libc::c_long,
        };
        init
    };
    return nanosleep(&mut waittime, 0 as *mut timespec);
}
#[inline]
unsafe extern "C" fn flush_standard_stream(stream: *mut FILE) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    *__errno_location() = 0 as libc::c_int;
    if !(ferror(stream) != 0 as libc::c_int || fflush(stream) != 0 as libc::c_int) {
        fd = fileno(stream);
        if !(fd < 0 as libc::c_int
            || {
                fd = dup(fd);
                fd < 0 as libc::c_int
            }
            || close(fd) != 0 as libc::c_int)
        {
            return 0 as libc::c_int;
        }
    }
    return if *__errno_location() == 9 as libc::c_int {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
#[inline]
unsafe extern "C" fn close_stdout() {
    if flush_standard_stream(stdout) != 0 as libc::c_int
        && !(*__errno_location() == 32 as libc::c_int)
    {
        if *__errno_location() != 0 {
            warn(dcgettext(
                0 as *const libc::c_char,
                b"write error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ));
        } else {
            warnx(dcgettext(
                0 as *const libc::c_char,
                b"write error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ));
        }
        _exit(1 as libc::c_int);
    }
    if flush_standard_stream(stderr) != 0 as libc::c_int {
        _exit(1 as libc::c_int);
    }
}
#[inline]
unsafe extern "C" fn close_stdout_atexit() {
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
}
#[inline]
unsafe extern "C" fn pidfd_open(pid: pid_t, flags: libc::c_uint) -> libc::c_int {
    return syscall(434 as libc::c_int as libc::c_long, pid, flags) as libc::c_int;
}
#[inline]
unsafe extern "C" fn xmalloc(size: size_t) -> *mut libc::c_void {
    let ret: *mut libc::c_void = malloc(size);
    if ret.is_null() && size != 0 {
        err(
            1 as libc::c_int,
            b"cannot allocate %zu bytes\0" as *const u8 as *const libc::c_char,
            size,
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    if !str.is_null() {
    } else {
        __assert_fail(
            b"str\0" as *const u8 as *const libc::c_char,
            b"./include/xalloc.h\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"char *xstrdup(const char *)\0",
            ))
            .as_ptr(),
        );
    }
    'c_7543: {
        if !str.is_null() {
        } else {
            __assert_fail(
                b"str\0" as *const u8 as *const libc::c_char,
                b"./include/xalloc.h\0" as *const u8 as *const libc::c_char,
                80 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                    b"char *xstrdup(const char *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    ret = strdup(str);
    if ret.is_null() {
        err(
            1 as libc::c_int,
            b"cannot duplicate string\0" as *const u8 as *const libc::c_char,
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn xasprintf(
    strp: *mut *mut libc::c_char,
    fmt: *const libc::c_char,
    args: ...
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    ret = vasprintf(strp, fmt, args_0.as_va_list());
    if ret < 0 as libc::c_int {
        err(
            1 as libc::c_int,
            b"cannot allocate string\0" as *const u8 as *const libc::c_char,
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn write_all(
    fd: libc::c_int,
    mut buf: *const libc::c_void,
    mut count: size_t,
) -> libc::c_int {
    while count != 0 {
        let mut tmp: ssize_t = 0;
        *__errno_location() = 0 as libc::c_int;
        tmp = write(fd, buf, count);
        if tmp > 0 as libc::c_int as libc::c_long {
            count = (count as libc::c_ulong).wrapping_sub(tmp as libc::c_ulong) as size_t as size_t;
            if count != 0 {
                buf = (buf as *const libc::c_char).offset(tmp as isize) as *const libc::c_void;
            }
        } else if *__errno_location() != 4 as libc::c_int
            && *__errno_location() != 11 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        if *__errno_location() == 11 as libc::c_int {
            xusleep(250000 as libc::c_int as useconds_t);
        }
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn read_all(
    fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut count: size_t,
) -> ssize_t {
    let mut ret: ssize_t = 0;
    let mut c: ssize_t = 0 as libc::c_int as ssize_t;
    let mut tries: libc::c_int = 0 as libc::c_int;
    memset(buf as *mut libc::c_void, 0 as libc::c_int, count);
    while count > 0 as libc::c_int as libc::c_ulong {
        ret = read(fd, buf as *mut libc::c_void, count);
        if ret < 0 as libc::c_int as libc::c_long {
            if (*__errno_location() == 11 as libc::c_int || *__errno_location() == 4 as libc::c_int)
                && {
                    let fresh0 = tries;
                    tries = tries + 1;
                    fresh0 < 5 as libc::c_int
                }
            {
                xusleep(250000 as libc::c_int as useconds_t);
            } else {
                return if c != 0 {
                    c
                } else {
                    -(1 as libc::c_int) as libc::c_long
                };
            }
        } else {
            if ret == 0 as libc::c_int as libc::c_long {
                return c;
            }
            tries = 0 as libc::c_int;
            count = (count as libc::c_ulong).wrapping_sub(ret as libc::c_ulong) as size_t as size_t;
            buf = buf.offset(ret as isize);
            c += ret;
        }
    }
    return c;
}
static mut namespace_files: [namespace_file; 9] = [
    {
        let init = namespace_file {
            type_0: 0x10000000 as libc::c_int,
            name: b"ns/user\0" as *const u8 as *const libc::c_char,
            target: 0 as *const libc::c_char,
        };
        init
    },
    {
        let init = namespace_file {
            type_0: 0x2000000 as libc::c_int,
            name: b"ns/cgroup\0" as *const u8 as *const libc::c_char,
            target: 0 as *const libc::c_char,
        };
        init
    },
    {
        let init = namespace_file {
            type_0: 0x8000000 as libc::c_int,
            name: b"ns/ipc\0" as *const u8 as *const libc::c_char,
            target: 0 as *const libc::c_char,
        };
        init
    },
    {
        let init = namespace_file {
            type_0: 0x4000000 as libc::c_int,
            name: b"ns/uts\0" as *const u8 as *const libc::c_char,
            target: 0 as *const libc::c_char,
        };
        init
    },
    {
        let init = namespace_file {
            type_0: 0x40000000 as libc::c_int,
            name: b"ns/net\0" as *const u8 as *const libc::c_char,
            target: 0 as *const libc::c_char,
        };
        init
    },
    {
        let init = namespace_file {
            type_0: 0x20000000 as libc::c_int,
            name: b"ns/pid_for_children\0" as *const u8 as *const libc::c_char,
            target: 0 as *const libc::c_char,
        };
        init
    },
    {
        let init = namespace_file {
            type_0: 0x20000 as libc::c_int,
            name: b"ns/mnt\0" as *const u8 as *const libc::c_char,
            target: 0 as *const libc::c_char,
        };
        init
    },
    {
        let init = namespace_file {
            type_0: 0x80 as libc::c_int,
            name: b"ns/time_for_children\0" as *const u8 as *const libc::c_char,
            target: 0 as *const libc::c_char,
        };
        init
    },
    {
        let init = namespace_file {
            type_0: 0,
            name: 0 as *const libc::c_char,
            target: 0 as *const libc::c_char,
        };
        init
    },
];
static mut npersists: libc::c_int = 0;
static mut setgroups_strings: [*const libc::c_char; 2] = [
    b"deny\0" as *const u8 as *const libc::c_char,
    b"allow\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn setgroups_str2id(str: *const libc::c_char) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i
        < (::core::mem::size_of::<[*const libc::c_char; 2]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong)
    {
        if strcmp(str, setgroups_strings[i as usize]) == 0 as libc::c_int {
            return i as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    errx(
        1 as libc::c_int,
        dcgettext(
            0 as *const libc::c_char,
            b"unsupported --setgroups argument '%s'\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        str,
    );
}
unsafe extern "C" fn setgroups_control(action: libc::c_int) {
    let file: *const libc::c_char = b"/proc/self/setgroups\0" as *const u8 as *const libc::c_char;
    let mut cmd: *const libc::c_char = 0 as *const libc::c_char;
    let mut fd: libc::c_int = 0;
    if action < 0 as libc::c_int
        || action as size_t
            >= (::core::mem::size_of::<[*const libc::c_char; 2]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong)
    {
        return;
    }
    cmd = setgroups_strings[action as usize];
    fd = open(file, 0o1 as libc::c_int);
    if fd < 0 as libc::c_int {
        if *__errno_location() == 2 as libc::c_int {
            return;
        }
        err(
            1 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"cannot open %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            file,
        );
    }
    if write_all(fd, cmd as *const libc::c_void, strlen(cmd)) != 0 {
        err(
            1 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"write failed %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            file,
        );
    }
    close(fd);
}
unsafe extern "C" fn map_id(file: *const libc::c_char, from: uint32_t, to: uint32_t) {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fd: libc::c_int = 0;
    fd = open(file, 0o1 as libc::c_int);
    if fd < 0 as libc::c_int {
        err(
            1 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"cannot open %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            file,
        );
    }
    xasprintf(
        &mut buf as *mut *mut libc::c_char,
        b"%u %u 1\0" as *const u8 as *const libc::c_char,
        from,
        to,
    );
    if write_all(fd, buf as *const libc::c_void, strlen(buf)) != 0 {
        err(
            1 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"write failed %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            file,
        );
    }
    free(buf as *mut libc::c_void);
    close(fd);
}
unsafe extern "C" fn parse_propagation(str: *const libc::c_char) -> libc::c_ulong {
    let mut i: size_t = 0;
    static mut opts: [prop_opts; 4] = [
        {
            let init = prop_opts {
                name: b"slave\0" as *const u8 as *const libc::c_char,
                flag: (MS_REC as libc::c_int | MS_SLAVE as libc::c_int) as libc::c_ulong,
            };
            init
        },
        {
            let init = prop_opts {
                name: b"private\0" as *const u8 as *const libc::c_char,
                flag: (MS_REC as libc::c_int | MS_PRIVATE as libc::c_int) as libc::c_ulong,
            };
            init
        },
        {
            let init = prop_opts {
                name: b"shared\0" as *const u8 as *const libc::c_char,
                flag: (MS_REC as libc::c_int | MS_SHARED as libc::c_int) as libc::c_ulong,
            };
            init
        },
        {
            let init = prop_opts {
                name: b"unchanged\0" as *const u8 as *const libc::c_char,
                flag: 0 as libc::c_int as libc::c_ulong,
            };
            init
        },
    ];
    i = 0 as libc::c_int as size_t;
    while i
        < (::core::mem::size_of::<[prop_opts; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<prop_opts>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong)
    {
        if strcmp(opts[i as usize].name, str) == 0 as libc::c_int {
            return opts[i as usize].flag;
        }
        i = i.wrapping_add(1);
        i;
    }
    errx(
        1 as libc::c_int,
        dcgettext(
            0 as *const libc::c_char,
            b"unsupported propagation mode: %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        str,
    );
}
unsafe extern "C" fn set_propagation(flags: libc::c_ulong) {
    if flags == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    if mount(
        b"none\0" as *const u8 as *const libc::c_char,
        b"/\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        flags,
        0 as *const libc::c_void,
    ) != 0 as libc::c_int
    {
        err(
            1 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"cannot change root filesystem propagation\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
unsafe extern "C" fn set_ns_target(type_0: libc::c_int, path: *const libc::c_char) -> libc::c_int {
    let mut ns: *mut namespace_file = 0 as *mut namespace_file;
    ns = namespace_files.as_mut_ptr();
    while !((*ns).name).is_null() {
        if (*ns).type_0 != type_0 {
            ns = ns.offset(1);
            ns;
        } else {
            (*ns).target = path;
            npersists += 1;
            npersists;
            return 0 as libc::c_int;
        }
    }
    return -(22 as libc::c_int);
}
unsafe extern "C" fn bind_ns_files(pid: pid_t) -> libc::c_int {
    let mut ns: *mut namespace_file = 0 as *mut namespace_file;
    let mut src: [libc::c_char; 4096] = [0; 4096];
    ns = namespace_files.as_mut_ptr();
    while !((*ns).name).is_null() {
        if !((*ns).target).is_null() {
            snprintf(
                src.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
                b"/proc/%u/%s\0" as *const u8 as *const libc::c_char,
                pid as libc::c_uint,
                (*ns).name,
            );
            if mount(
                src.as_mut_ptr(),
                (*ns).target,
                0 as *const libc::c_char,
                MS_BIND as libc::c_int as libc::c_ulong,
                0 as *const libc::c_void,
            ) != 0 as libc::c_int
            {
                err(
                    1 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"mount %s on %s failed\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    src.as_mut_ptr(),
                    (*ns).target,
                );
            }
        }
        ns = ns.offset(1);
        ns;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_mnt_ino(pid: pid_t) -> ino_t {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut path: [libc::c_char; 4096] = [0; 4096];
    snprintf(
        path.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        b"/proc/%u/ns/mnt\0" as *const u8 as *const libc::c_char,
        pid as libc::c_uint,
    );
    if stat(path.as_mut_ptr(), &mut st) != 0 as libc::c_int {
        err(
            1 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"stat of %s failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            path.as_mut_ptr(),
        );
    }
    return st.st_ino;
}
unsafe extern "C" fn settime(offset: int64_t, clk_id: clockid_t) {
    let mut buf: [libc::c_char; 96] = [0; 96];
    let mut fd: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    len = snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 96]>() as libc::c_ulong,
        b"%d %ld 0\0" as *const u8 as *const libc::c_char,
        clk_id,
        offset,
    );
    fd = open(
        b"/proc/self/timens_offsets\0" as *const u8 as *const libc::c_char,
        0o1 as libc::c_int,
    );
    if fd < 0 as libc::c_int {
        err(
            1 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"failed to open /proc/self/timens_offsets\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if write(fd, buf.as_mut_ptr() as *const libc::c_void, len as size_t) != len as libc::c_long {
        err(
            1 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"failed to write to /proc/self/timens_offsets\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    close(fd);
}
unsafe extern "C" fn waitchild(pid: libc::c_int) {
    let mut rc: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    loop {
        rc = waitpid(pid, &mut status, 0 as libc::c_int);
        if rc < 0 as libc::c_int {
            if !(*__errno_location() == 4 as libc::c_int) {
                err(
                    1 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"waitpid failed\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        } else if status & 0x7f as libc::c_int == 0 as libc::c_int
            && (status & 0xff00 as libc::c_int) >> 8 as libc::c_int != 0 as libc::c_int
        {
            exit((status & 0xff00 as libc::c_int) >> 8 as libc::c_int);
        }
        if !(rc < 0 as libc::c_int) {
            break;
        }
    }
}
unsafe extern "C" fn sync_with_child(pid: pid_t, fd: libc::c_int) {
    let mut ch: uint64_t = 0x6 as libc::c_int as uint64_t;
    write_all(
        fd,
        &mut ch as *mut uint64_t as *const libc::c_void,
        ::core::mem::size_of::<uint64_t>() as libc::c_ulong,
    );
    close(fd);
    waitchild(pid);
}
unsafe extern "C" fn fork_and_wait(fd: *mut libc::c_int) -> pid_t {
    let mut pid: pid_t = 0;
    let mut ch: uint64_t = 0;
    *fd = eventfd(0 as libc::c_int as libc::c_uint, 0 as libc::c_int);
    if *fd < 0 as libc::c_int {
        err(
            1 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"eventfd failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    pid = fork();
    if pid < 0 as libc::c_int {
        err(
            1 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"fork failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if pid == 0 {
        if read_all(
            *fd,
            &mut ch as *mut uint64_t as *mut libc::c_char,
            ::core::mem::size_of::<uint64_t>() as libc::c_ulong,
        ) as libc::c_ulong
            != ::core::mem::size_of::<uint64_t>() as libc::c_ulong
            || ch != 0x6 as libc::c_int as libc::c_ulong
        {
            err(
                1 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to read eventfd\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        close(*fd);
    }
    return pid;
}
unsafe extern "C" fn bind_ns_files_from_child(fd: *mut libc::c_int) -> pid_t {
    let mut child: pid_t = 0;
    let ppid: pid_t = getpid();
    let ino: ino_t = get_mnt_ino(ppid);
    child = fork_and_wait(fd);
    if child != 0 {
        return child;
    }
    if get_mnt_ino(ppid) == ino {
        exit(1 as libc::c_int);
    }
    bind_ns_files(ppid);
    exit(0 as libc::c_int);
}
unsafe extern "C" fn get_user(s: *const libc::c_char, err_0: *const libc::c_char) -> uid_t {
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: uid_t = 0;
    pw = xgetpwnam(s, &mut buf);
    if !pw.is_null() {
        ret = (*pw).pw_uid;
        free(pw as *mut libc::c_void);
        free(buf as *mut libc::c_void);
    } else {
        ret = str2unum_or_err(
            s,
            10 as libc::c_int,
            err_0,
            (9223372036854775807 as libc::c_long as libc::c_ulong)
                .wrapping_mul(2 as libc::c_ulong)
                .wrapping_add(1 as libc::c_ulong),
        ) as uid_t;
    }
    return ret;
}
unsafe extern "C" fn get_group(s: *const libc::c_char, err_0: *const libc::c_char) -> gid_t {
    let mut gr: *mut group = 0 as *mut group;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: gid_t = 0;
    gr = xgetgrnam(s, &mut buf);
    if !gr.is_null() {
        ret = (*gr).gr_gid;
        free(gr as *mut libc::c_void);
        free(buf as *mut libc::c_void);
    } else {
        ret = str2unum_or_err(
            s,
            10 as libc::c_int,
            err_0,
            (9223372036854775807 as libc::c_long as libc::c_ulong)
                .wrapping_mul(2 as libc::c_ulong)
                .wrapping_add(1 as libc::c_ulong),
        ) as gid_t;
    }
    return ret;
}
unsafe extern "C" fn get_map_range(s: *const libc::c_char) -> *mut map_range {
    let mut end: libc::c_int = 0;
    let mut ret: *mut map_range = 0 as *mut map_range;
    ret = xmalloc(::core::mem::size_of::<map_range>() as libc::c_ulong) as *mut map_range;
    if sscanf(
        s,
        b"%u:%u:%u%n\0" as *const u8 as *const libc::c_char,
        &mut (*ret).inner as *mut libc::c_uint,
        &mut (*ret).outer as *mut libc::c_uint,
        &mut (*ret).count as *mut libc::c_uint,
        &mut end as *mut libc::c_int,
    ) >= 3 as libc::c_int
        && *s.offset(end as isize) == 0
    {
        return ret;
    }
    if sscanf(
        s,
        b"%u,%u,%u%n\0" as *const u8 as *const libc::c_char,
        &mut (*ret).outer as *mut libc::c_uint,
        &mut (*ret).inner as *mut libc::c_uint,
        &mut (*ret).count as *mut libc::c_uint,
        &mut end as *mut libc::c_int,
    ) >= 3 as libc::c_int
        && *s.offset(end as isize) == 0
    {
        return ret;
    }
    errx(
        1 as libc::c_int,
        dcgettext(
            0 as *const libc::c_char,
            b"invalid mapping '%s'\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        s,
    );
}
unsafe extern "C" fn read_subid_range(filename: *mut libc::c_char, uid: uid_t) -> *mut map_range {
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pwbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut idmap: *mut FILE = 0 as *mut FILE;
    let mut n: size_t = 0 as libc::c_int as size_t;
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut map: *mut map_range = 0 as *mut map_range;
    map = xmalloc(::core::mem::size_of::<map_range>() as libc::c_ulong) as *mut map_range;
    (*map).inner = -(1 as libc::c_int) as libc::c_uint;
    pw = xgetpwuid(uid, &mut pwbuf);
    if pw.is_null() {
        errx(
            1 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"you (user %d) don't exist.\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            uid,
        );
    }
    idmap = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if idmap.is_null() {
        err(
            1 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"could not open '%s'\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
        );
    }
    while getline(&mut line, &mut n, idmap) != -(1 as libc::c_int) as libc::c_long {
        let mut rest: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        rest = strchr(line, ':' as i32);
        if rest.is_null() {
            continue;
        }
        *rest = '\0' as i32 as libc::c_char;
        if strcmp(line, (*pw).pw_name) != 0
            && strtoul(line, 0 as *mut *mut libc::c_char, 10 as libc::c_int)
                != (*pw).pw_uid as libc::c_ulong
        {
            continue;
        }
        s = rest.offset(1 as libc::c_int as isize);
        rest = strchr(s, ':' as i32);
        if rest.is_null() {
            continue;
        }
        *rest = '\0' as i32 as libc::c_char;
        (*map).outer = str2unum_or_err(
            s,
            10 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"failed to parse subid map\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (9223372036854775807 as libc::c_long as libc::c_ulong)
                .wrapping_mul(2 as libc::c_ulong)
                .wrapping_add(1 as libc::c_ulong),
        ) as libc::c_uint;
        s = rest.offset(1 as libc::c_int as isize);
        rest = strchr(s, '\n' as i32);
        if !rest.is_null() {
            *rest = '\0' as i32 as libc::c_char;
        }
        (*map).count = str2unum_or_err(
            s,
            10 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"failed to parse subid map\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (9223372036854775807 as libc::c_long as libc::c_ulong)
                .wrapping_mul(2 as libc::c_ulong)
                .wrapping_add(1 as libc::c_ulong),
        ) as libc::c_uint;
        fclose(idmap);
        free(pw as *mut libc::c_void);
        free(pwbuf as *mut libc::c_void);
        return map;
    }
    errx(
        1 as libc::c_int,
        dcgettext(
            0 as *const libc::c_char,
            b"no line matching user \"%s\" in %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        (*pw).pw_name,
        filename,
    );
}
unsafe extern "C" fn map_ids(
    idmapper: *const libc::c_char,
    ppid: libc::c_int,
    outer: libc::c_uint,
    inner: libc::c_uint,
    map: *mut map_range,
) -> ! {
    let mut argv: [*mut libc::c_char; 15] = [0 as *mut libc::c_char; 15];
    let mut args: [[libc::c_char; 32]; 12] = [[0; 32]; 12];
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut lo: map_range = map_range {
        outer: 0,
        inner: 0,
        count: 0,
    };
    let mut mid: map_range = map_range {
        outer: 0,
        inner: 0,
        count: 0,
    };
    let mut hi: map_range = map_range {
        outer: 0,
        inner: 0,
        count: 0,
    };
    let mut inner_offset: libc::c_uint = 0;
    let mut outer_offset: libc::c_uint = 0;
    let fresh1 = i;
    i = i + 1;
    argv[fresh1 as usize] = xstrdup(idmapper);
    snprintf(
        (args[j as usize]).as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        b"%u\0" as *const u8 as *const libc::c_char,
        ppid,
    );
    let fresh2 = j;
    j = j + 1;
    let fresh3 = i;
    i = i + 1;
    argv[fresh3 as usize] = (args[fresh2 as usize]).as_mut_ptr();
    if inner as libc::c_int == -(1 as libc::c_int) {
        snprintf(
            (args[j as usize]).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%u\0" as *const u8 as *const libc::c_char,
            if ((*map).inner).wrapping_add(1 as libc::c_int as libc::c_uint) != 0 {
                (*map).inner
            } else {
                0 as libc::c_int as libc::c_uint
            },
        );
        let fresh4 = j;
        j = j + 1;
        let fresh5 = i;
        i = i + 1;
        argv[fresh5 as usize] = (args[fresh4 as usize]).as_mut_ptr();
        snprintf(
            (args[j as usize]).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%u\0" as *const u8 as *const libc::c_char,
            (*map).outer,
        );
        let fresh6 = j;
        j = j + 1;
        let fresh7 = i;
        i = i + 1;
        argv[fresh7 as usize] = (args[fresh6 as usize]).as_mut_ptr();
        snprintf(
            (args[j as usize]).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%u\0" as *const u8 as *const libc::c_char,
            (*map).count,
        );
        let fresh8 = j;
        j = j + 1;
        let fresh9 = i;
        i = i + 1;
        argv[fresh9 as usize] = (args[fresh8 as usize]).as_mut_ptr();
        let fresh10 = i;
        i = i + 1;
        argv[fresh10 as usize] = 0 as *mut libc::c_char;
        execvp(idmapper, argv.as_mut_ptr() as *const *mut libc::c_char);
        err(
            if *__errno_location() == 2 as libc::c_int {
                127 as libc::c_int
            } else {
                126 as libc::c_int
            },
            dcgettext(
                0 as *const libc::c_char,
                b"failed to execute %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            idmapper,
        );
    }
    if ((*map).inner).wrapping_add(1 as libc::c_int as libc::c_uint)
        == 0 as libc::c_int as libc::c_uint
    {
        (*map).inner = 0 as libc::c_int as libc::c_uint;
    } else if outer >= (*map).outer && outer <= ((*map).outer).wrapping_add((*map).count)
        || inner >= (*map).inner && inner <= ((*map).inner).wrapping_add((*map).count)
    {
        (*map).count = ((*map).count).wrapping_sub(1);
        (*map).count;
    }
    outer_offset = {
        let mut _min1: libc::c_uint = if outer > (*map).outer {
            outer.wrapping_sub((*map).outer)
        } else {
            0 as libc::c_int as libc::c_uint
        };
        let mut _min2: libc::c_uint = (*map).count;
        &mut _min1 as *mut libc::c_uint;
        &mut _min2 as *mut libc::c_uint;
        if _min1 < _min2 {
            _min1
        } else {
            _min2
        }
    };
    inner_offset = {
        let mut _min1: libc::c_uint = if inner > (*map).inner {
            inner.wrapping_sub((*map).inner)
        } else {
            0 as libc::c_int as libc::c_uint
        };
        let mut _min2: libc::c_uint = (*map).count;
        &mut _min1 as *mut libc::c_uint;
        &mut _min2 as *mut libc::c_uint;
        if _min1 < _min2 {
            _min1
        } else {
            _min2
        }
    };
    lo.outer = (*map).outer;
    lo.inner = (*map).inner;
    lo.count = {
        let mut _min1: libc::c_uint = inner_offset;
        let mut _min2: libc::c_uint = outer_offset;
        &mut _min1 as *mut libc::c_uint;
        &mut _min2 as *mut libc::c_uint;
        if _min1 < _min2 {
            _min1
        } else {
            _min2
        }
    };
    mid.outer = (lo.outer).wrapping_add(lo.count);
    mid.outer = (mid.outer).wrapping_add((mid.outer == outer) as libc::c_int as libc::c_uint);
    mid.inner = (lo.inner).wrapping_add(lo.count);
    mid.inner = (mid.inner).wrapping_add((mid.inner == inner) as libc::c_int as libc::c_uint);
    mid.count = {
        let mut _a: libc::c_uint = outer_offset;
        let mut _b: libc::c_uint = inner_offset;
        &mut _a as *mut libc::c_uint;
        &mut _b as *mut libc::c_uint;
        if _a > _b {
            _a.wrapping_sub(_b)
        } else {
            _b.wrapping_sub(_a)
        }
    };
    hi.outer = (mid.outer).wrapping_add(mid.count);
    hi.outer = (hi.outer).wrapping_add((hi.outer == outer) as libc::c_int as libc::c_uint);
    hi.inner = (mid.inner).wrapping_add(mid.count);
    hi.inner = (hi.inner).wrapping_add((hi.inner == inner) as libc::c_int as libc::c_uint);
    hi.count = ((*map).count)
        .wrapping_sub(lo.count)
        .wrapping_sub(mid.count);
    snprintf(
        (args[j as usize]).as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        b"%u\0" as *const u8 as *const libc::c_char,
        inner,
    );
    let fresh11 = j;
    j = j + 1;
    let fresh12 = i;
    i = i + 1;
    argv[fresh12 as usize] = (args[fresh11 as usize]).as_mut_ptr();
    snprintf(
        (args[j as usize]).as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        b"%u\0" as *const u8 as *const libc::c_char,
        outer,
    );
    let fresh13 = j;
    j = j + 1;
    let fresh14 = i;
    i = i + 1;
    argv[fresh14 as usize] = (args[fresh13 as usize]).as_mut_ptr();
    let fresh15 = i;
    i = i + 1;
    argv[fresh15 as usize] = b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if lo.count != 0 {
        snprintf(
            (args[j as usize]).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%u\0" as *const u8 as *const libc::c_char,
            lo.inner,
        );
        let fresh16 = j;
        j = j + 1;
        let fresh17 = i;
        i = i + 1;
        argv[fresh17 as usize] = (args[fresh16 as usize]).as_mut_ptr();
        snprintf(
            (args[j as usize]).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%u\0" as *const u8 as *const libc::c_char,
            lo.outer,
        );
        let fresh18 = j;
        j = j + 1;
        let fresh19 = i;
        i = i + 1;
        argv[fresh19 as usize] = (args[fresh18 as usize]).as_mut_ptr();
        snprintf(
            (args[j as usize]).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%u\0" as *const u8 as *const libc::c_char,
            lo.count,
        );
        let fresh20 = j;
        j = j + 1;
        let fresh21 = i;
        i = i + 1;
        argv[fresh21 as usize] = (args[fresh20 as usize]).as_mut_ptr();
    }
    if mid.count != 0 {
        snprintf(
            (args[j as usize]).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%u\0" as *const u8 as *const libc::c_char,
            mid.inner,
        );
        let fresh22 = j;
        j = j + 1;
        let fresh23 = i;
        i = i + 1;
        argv[fresh23 as usize] = (args[fresh22 as usize]).as_mut_ptr();
        snprintf(
            (args[j as usize]).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%u\0" as *const u8 as *const libc::c_char,
            mid.outer,
        );
        let fresh24 = j;
        j = j + 1;
        let fresh25 = i;
        i = i + 1;
        argv[fresh25 as usize] = (args[fresh24 as usize]).as_mut_ptr();
        snprintf(
            (args[j as usize]).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%u\0" as *const u8 as *const libc::c_char,
            mid.count,
        );
        let fresh26 = j;
        j = j + 1;
        let fresh27 = i;
        i = i + 1;
        argv[fresh27 as usize] = (args[fresh26 as usize]).as_mut_ptr();
    }
    if hi.count != 0 {
        snprintf(
            (args[j as usize]).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%u\0" as *const u8 as *const libc::c_char,
            hi.inner,
        );
        let fresh28 = j;
        j = j + 1;
        let fresh29 = i;
        i = i + 1;
        argv[fresh29 as usize] = (args[fresh28 as usize]).as_mut_ptr();
        snprintf(
            (args[j as usize]).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%u\0" as *const u8 as *const libc::c_char,
            hi.outer,
        );
        let fresh30 = j;
        j = j + 1;
        let fresh31 = i;
        i = i + 1;
        argv[fresh31 as usize] = (args[fresh30 as usize]).as_mut_ptr();
        snprintf(
            (args[j as usize]).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%u\0" as *const u8 as *const libc::c_char,
            hi.count,
        );
        let fresh32 = j;
        j = j + 1;
        let fresh33 = i;
        i = i + 1;
        argv[fresh33 as usize] = (args[fresh32 as usize]).as_mut_ptr();
    }
    let fresh34 = i;
    i = i + 1;
    argv[fresh34 as usize] = 0 as *mut libc::c_char;
    execvp(idmapper, argv.as_mut_ptr() as *const *mut libc::c_char);
    err(
        if *__errno_location() == 2 as libc::c_int {
            127 as libc::c_int
        } else {
            126 as libc::c_int
        },
        dcgettext(
            0 as *const libc::c_char,
            b"failed to execute %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        idmapper,
    );
}
unsafe extern "C" fn map_ids_from_child(
    fd: *mut libc::c_int,
    mapuser: uid_t,
    usermap: *mut map_range,
    mapgroup: gid_t,
    groupmap: *mut map_range,
) -> pid_t {
    let mut child: pid_t = 0;
    let mut pid: pid_t = 0 as libc::c_int;
    let ppid: pid_t = getpid();
    child = fork_and_wait(fd);
    if child != 0 {
        return child;
    }
    if !usermap.is_null() && !groupmap.is_null() {
        pid = fork();
        if pid < 0 as libc::c_int {
            err(
                1 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"fork failed\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        if pid != 0 {
            waitchild(pid);
        }
    }
    if pid == 0 && !usermap.is_null() {
        map_ids(
            b"newuidmap\0" as *const u8 as *const libc::c_char,
            ppid,
            geteuid(),
            mapuser,
            usermap,
        );
    }
    if !groupmap.is_null() {
        map_ids(
            b"newgidmap\0" as *const u8 as *const libc::c_char,
            ppid,
            getegid(),
            mapgroup,
            groupmap,
        );
    }
    exit(0 as libc::c_int);
}
unsafe extern "C" fn usage() -> ! {
    let out: *mut FILE = stdout;
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\nUsage:\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fprintf(
        out,
        dcgettext(
            0 as *const libc::c_char,
            b" %s [options] [<program> [<argument>...]]\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        program_invocation_short_name,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, out);
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"Run a program with some namespaces unshared from the parent.\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\nOptions:\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" -m, --mount[=<file>]      unshare mounts namespace\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" -u, --uts[=<file>]        unshare UTS namespace (hostname etc)\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" -i, --ipc[=<file>]        unshare System V IPC namespace\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" -n, --net[=<file>]        unshare network namespace\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" -p, --pid[=<file>]        unshare pid namespace\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" -U, --user[=<file>]       unshare user namespace\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" -C, --cgroup[=<file>]     unshare cgroup namespace\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" -T, --time[=<file>]       unshare time namespace\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, out);
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" -f, --fork                fork before launching <program>\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" --map-user=<uid>|<name>   map current user to uid (implies --user)\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" --map-group=<gid>|<name>  map current group to gid (implies --user)\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" -r, --map-root-user       map current user to root (implies --user)\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" -c, --map-current-user    map current user to itself (implies --user)\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" --map-auto                map users and groups automatically (implies --user)\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" --map-users=<inneruid>:<outeruid>:<count>\n                           map count users from outeruid to inneruid (implies --user)\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" --map-groups=<innergid>:<outergid>:<count>\n                           map count groups from outergid to innergid (implies --user)\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, out);
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" --kill-child[=<signame>]  when dying, kill the forked child (implies --fork)\n                             defaults to SIGKILL\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" --mount-proc[=<dir>]      mount proc filesystem first (implies --mount)\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" --propagation slave|shared|private|unchanged\n                           modify mount propagation in mount namespace\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" --setgroups allow|deny    control the setgroups syscall in user namespaces\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" --keep-caps               retain capabilities granted in user namespaces\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, out);
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" -R, --root=<dir>          run the command with root directory set to <dir>\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" -w, --wd=<dir>            change working directory to <dir>\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" -S, --setuid <uid>        set uid in entered namespace\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" -G, --setgid <gid>        set gid in entered namespace\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" --monotonic <offset>      set clock monotonic offset (seconds) in time namespaces\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" --boottime <offset>       set clock boottime offset (seconds) in time namespaces\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        out,
    );
    fputs(b"\n\0" as *const u8 as *const libc::c_char, out);
    printf(
        b"%-27s%s\n%-27s%s\n\0" as *const u8 as *const libc::c_char,
        b" -h, --help\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"display this help\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        b" -V, --version\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"display version\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"\nFor more details see %s.\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        b"unshare(1)\0" as *const u8 as *const libc::c_char,
    );
    exit(0 as libc::c_int);
}

unsafe fn main_0(argc: libc::c_int, argv: *mut *mut libc::c_char) -> i32 {
    static mut longopts: [option; 30] = [
        {
            let init = option {
                name: b"help\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'h' as i32,
            };
            init
        },
        {
            let init = option {
                name: b"version\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'V' as i32,
            };
            init
        },
        {
            let init = option {
                name: b"mount\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'm' as i32,
            };
            init
        },
        {
            let init = option {
                name: b"uts\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'u' as i32,
            };
            init
        },
        {
            let init = option {
                name: b"ipc\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'i' as i32,
            };
            init
        },
        {
            let init = option {
                name: b"net\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'n' as i32,
            };
            init
        },
        {
            let init = option {
                name: b"pid\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'p' as i32,
            };
            init
        },
        {
            let init = option {
                name: b"user\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'U' as i32,
            };
            init
        },
        {
            let init = option {
                name: b"cgroup\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'C' as i32,
            };
            init
        },
        {
            let init = option {
                name: b"time\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'T' as i32,
            };
            init
        },
        {
            let init = option {
                name: b"fork\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'f' as i32,
            };
            init
        },
        {
            let init = option {
                name: b"kill-child\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPT_KILLCHILD as libc::c_int,
            };
            init
        },
        {
            let init = option {
                name: b"mount-proc\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPT_MOUNTPROC as libc::c_int,
            };
            init
        },
        {
            let init = option {
                name: b"map-user\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPT_MAPUSER as libc::c_int,
            };
            init
        },
        {
            let init = option {
                name: b"map-users\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPT_MAPUSERS as libc::c_int,
            };
            init
        },
        {
            let init = option {
                name: b"map-group\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPT_MAPGROUP as libc::c_int,
            };
            init
        },
        {
            let init = option {
                name: b"map-groups\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPT_MAPGROUPS as libc::c_int,
            };
            init
        },
        {
            let init = option {
                name: b"map-root-user\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'r' as i32,
            };
            init
        },
        {
            let init = option {
                name: b"map-current-user\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'c' as i32,
            };
            init
        },
        {
            let init = option {
                name: b"map-auto\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPT_MAPAUTO as libc::c_int,
            };
            init
        },
        {
            let init = option {
                name: b"propagation\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPT_PROPAGATION as libc::c_int,
            };
            init
        },
        {
            let init = option {
                name: b"setgroups\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPT_SETGROUPS as libc::c_int,
            };
            init
        },
        {
            let init = option {
                name: b"keep-caps\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPT_KEEPCAPS as libc::c_int,
            };
            init
        },
        {
            let init = option {
                name: b"setuid\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'S' as i32,
            };
            init
        },
        {
            let init = option {
                name: b"setgid\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'G' as i32,
            };
            init
        },
        {
            let init = option {
                name: b"root\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'R' as i32,
            };
            init
        },
        {
            let init = option {
                name: b"wd\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'w' as i32,
            };
            init
        },
        {
            let init = option {
                name: b"monotonic\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPT_MONOTONIC as libc::c_int,
            };
            init
        },
        {
            let init = option {
                name: b"boottime\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPT_BOOTTIME as libc::c_int,
            };
            init
        },
        {
            let init = option {
                name: 0 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
    ];
    let mut setgrpcmd: libc::c_int = SETGROUPS_NONE as libc::c_int;
    let mut unshare_flags: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    let mut forkit: libc::c_int = 0 as libc::c_int;
    let mut mapuser: uid_t = -(1 as libc::c_int) as uid_t;
    let mut mapgroup: gid_t = -(1 as libc::c_int) as gid_t;
    let mut usermap: *mut map_range = 0 as *mut map_range;
    let mut groupmap: *mut map_range = 0 as *mut map_range;
    let mut kill_child_signo: libc::c_int = 0 as libc::c_int;
    let mut procmnt: *const libc::c_char = 0 as *const libc::c_char;
    let mut newroot: *const libc::c_char = 0 as *const libc::c_char;
    let mut newdir: *const libc::c_char = 0 as *const libc::c_char;
    let mut pid_bind: pid_t = 0 as libc::c_int;
    let mut pid_idmap: pid_t = 0 as libc::c_int;
    let mut pid: pid_t = 0 as libc::c_int;
    let mut fd_parent_pid: libc::c_int = -(1 as libc::c_int);
    let mut fd_idmap: libc::c_int = 0;
    let mut fd_bind: libc::c_int = -(1 as libc::c_int);
    let mut sigset: sigset_t = sigset_t { __val: [0; 16] };
    let mut oldsigset: sigset_t = sigset_t { __val: [0; 16] };
    let mut status: libc::c_int = 0;
    let mut propagation: libc::c_ulong =
        (MS_REC as libc::c_int | MS_PRIVATE as libc::c_int) as libc::c_ulong;
    let mut force_uid: libc::c_int = 0 as libc::c_int;
    let mut force_gid: libc::c_int = 0 as libc::c_int;
    let mut uid: uid_t = 0 as libc::c_int as uid_t;
    let real_euid: uid_t = unsafe { geteuid() };
    let mut gid: gid_t = 0 as libc::c_int as gid_t;
    let real_egid: gid_t = unsafe { getegid() };
    let mut keepcaps: libc::c_int = 0 as libc::c_int;
    let mut monotonic: int64_t = 0 as libc::c_int as int64_t;
    let mut boottime: int64_t = 0 as libc::c_int as int64_t;
    let mut force_monotonic: libc::c_int = 0 as libc::c_int;
    let mut force_boottime: libc::c_int = 0 as libc::c_int;
    unsafe { setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char) };
    unsafe {
        bindtextdomain(
            b"util-linux\0" as *const u8 as *const libc::c_char,
            b"/usr/share/locale\0" as *const u8 as *const libc::c_char,
        );
    }
    unsafe {
        textdomain(b"util-linux\0" as *const u8 as *const libc::c_char);
    }
    unsafe {
        close_stdout_atexit();
    }
    loop {
        c = unsafe {
            getopt_long(
                argc,
                argv as *const *mut libc::c_char,
                b"+fhVmuinpCTUrR:w:S:G:c\0" as *const u8 as *const libc::c_char,
                longopts.as_ptr(),
                0 as *mut libc::c_int,
            )
        };
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            102 => {
                forkit = 1 as libc::c_int;
            }
            109 => {
                unshare_flags |= 0x20000 as libc::c_int;
                if !optarg.is_null() {
                    set_ns_target(0x20000 as libc::c_int, optarg);
                }
            }
            117 => {
                unshare_flags |= 0x4000000 as libc::c_int;
                if !optarg.is_null() {
                    set_ns_target(0x4000000 as libc::c_int, optarg);
                }
            }
            105 => {
                unshare_flags |= 0x8000000 as libc::c_int;
                if !optarg.is_null() {
                    set_ns_target(0x8000000 as libc::c_int, optarg);
                }
            }
            110 => {
                unshare_flags |= 0x40000000 as libc::c_int;
                if !optarg.is_null() {
                    set_ns_target(0x40000000 as libc::c_int, optarg);
                }
            }
            112 => {
                unshare_flags |= 0x20000000 as libc::c_int;
                if !optarg.is_null() {
                    set_ns_target(0x20000000 as libc::c_int, optarg);
                }
            }
            85 => {
                unshare_flags |= 0x10000000 as libc::c_int;
                if !optarg.is_null() {
                    set_ns_target(0x10000000 as libc::c_int, optarg);
                }
            }
            67 => {
                unshare_flags |= 0x2000000 as libc::c_int;
                if !optarg.is_null() {
                    set_ns_target(0x2000000 as libc::c_int, optarg);
                }
            }
            84 => {
                unshare_flags |= 0x80 as libc::c_int;
                if !optarg.is_null() {
                    set_ns_target(0x80 as libc::c_int, optarg);
                }
            }
            128 => {
                unshare_flags |= 0x20000 as libc::c_int;
                procmnt = if !optarg.is_null() {
                    optarg as *const libc::c_char
                } else {
                    b"/proc\0" as *const u8 as *const libc::c_char
                };
            }
            135 => {
                unshare_flags |= 0x10000000 as libc::c_int;
                mapuser = get_user(
                    optarg,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to parse uid\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            137 => {
                unshare_flags |= 0x10000000 as libc::c_int;
                mapgroup = get_group(
                    optarg,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to parse gid\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            114 => {
                unshare_flags |= 0x10000000 as libc::c_int;
                mapuser = 0 as libc::c_int as uid_t;
                mapgroup = 0 as libc::c_int as gid_t;
            }
            99 => {
                unshare_flags |= 0x10000000 as libc::c_int;
                mapuser = real_euid;
                mapgroup = real_egid;
            }
            136 => {
                unshare_flags |= 0x10000000 as libc::c_int;
                if strcmp(optarg, b"auto\0" as *const u8 as *const libc::c_char) == 0 {
                    usermap = read_subid_range(
                        b"/etc/subuid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        real_euid,
                    );
                } else {
                    usermap = get_map_range(optarg);
                }
            }
            138 => {
                unshare_flags |= 0x10000000 as libc::c_int;
                if strcmp(optarg, b"auto\0" as *const u8 as *const libc::c_char) == 0 {
                    groupmap = read_subid_range(
                        b"/etc/subgid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        real_euid,
                    );
                } else {
                    groupmap = get_map_range(optarg);
                }
            }
            139 => {
                unshare_flags |= 0x10000000 as libc::c_int;
                usermap = read_subid_range(
                    b"/etc/subuid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    real_euid,
                );
                groupmap = read_subid_range(
                    b"/etc/subgid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    real_euid,
                );
            }
            130 => {
                setgrpcmd = setgroups_str2id(optarg);
            }
            129 => {
                propagation = parse_propagation(optarg);
            }
            131 => {
                forkit = 1 as libc::c_int;
                if !optarg.is_null() {
                    kill_child_signo = signame_to_signum(optarg);
                    if kill_child_signo < 0 as libc::c_int {
                        errx(
                            1 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"unknown signal: %s\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            optarg,
                        );
                    }
                } else {
                    kill_child_signo = 9 as libc::c_int;
                }
            }
            132 => {
                keepcaps = 1 as libc::c_int;
                cap_last_cap();
            }
            83 => {
                uid = str2unum_or_err(
                    optarg,
                    10 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to parse uid\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_ulong)
                        .wrapping_add(1 as libc::c_ulong),
                ) as uid_t;
                force_uid = 1 as libc::c_int;
            }
            71 => {
                gid = str2unum_or_err(
                    optarg,
                    10 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to parse gid\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_ulong)
                        .wrapping_add(1 as libc::c_ulong),
                ) as gid_t;
                force_gid = 1 as libc::c_int;
            }
            82 => {
                newroot = optarg;
            }
            119 => {
                newdir = optarg;
            }
            133 => {
                monotonic = str2num_or_err(
                    optarg,
                    10 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to parse monotonic offset\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    0 as libc::c_int as int64_t,
                    0 as libc::c_int as int64_t,
                );
                force_monotonic = 1 as libc::c_int;
            }
            134 => {
                boottime = str2num_or_err(
                    optarg,
                    10 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to parse boottime offset\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    0 as libc::c_int as int64_t,
                    0 as libc::c_int as int64_t,
                );
                force_boottime = 1 as libc::c_int;
            }
            104 => {
                usage();
            }
            86 => {
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s from %s\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    program_invocation_short_name,
                    b"util-linux 2.39\0" as *const u8 as *const libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Try '%s --help' for more information.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    program_invocation_short_name,
                );
                exit(1 as libc::c_int);
            }
        }
    }
    if (force_monotonic != 0 || force_boottime != 0) && unshare_flags & 0x80 as libc::c_int == 0 {
        errx(
            1 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"options --monotonic and --boottime require unsharing of a time namespace (-T)\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    signal(17 as libc::c_int, None);
    if npersists != 0 && unshare_flags & 0x20000 as libc::c_int != 0 {
        pid_bind = bind_ns_files_from_child(&mut fd_bind);
    }
    if !usermap.is_null() || !groupmap.is_null() {
        pid_idmap = map_ids_from_child(&mut fd_idmap, mapuser, usermap, mapgroup, groupmap);
    }
    if -(1 as libc::c_int) == unshare(unshare_flags) {
        err(
            1 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"unshare failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if !usermap.is_null() || !groupmap.is_null() {
        sync_with_child(pid_idmap, fd_idmap);
    }
    if force_boottime != 0 {
        settime(boottime, 7 as libc::c_int);
    }
    if force_monotonic != 0 {
        settime(monotonic, 1 as libc::c_int);
    }
    if forkit != 0 {
        if sigemptyset(&mut sigset) != 0 as libc::c_int
            || sigaddset(&mut sigset, 2 as libc::c_int) != 0 as libc::c_int
            || sigaddset(&mut sigset, 15 as libc::c_int) != 0 as libc::c_int
            || sigprocmask(0 as libc::c_int, &mut sigset, &mut oldsigset) != 0 as libc::c_int
        {
            err(
                1 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"sigprocmask block failed\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        if kill_child_signo != 0 as libc::c_int {
            fd_parent_pid = pidfd_open(getpid(), 0 as libc::c_int as libc::c_uint);
            if 0 as libc::c_int > fd_parent_pid {
                err(
                    1 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"pidfd_open failed\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        }
        pid = fork();
        match pid {
            -1 => {
                err(
                    1 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"fork failed\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            0 => {
                if sigprocmask(2 as libc::c_int, &mut oldsigset, 0 as *mut sigset_t) != 0 {
                    err(
                        1 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"sigprocmask restore failed\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                if npersists != 0 && unshare_flags & 0x20000 as libc::c_int != 0 {
                    close(fd_bind);
                }
            }
            _ => {}
        }
    }
    if npersists != 0 && (pid != 0 || forkit == 0) {
        if pid_bind != 0 && unshare_flags & 0x20000 as libc::c_int != 0 {
            sync_with_child(pid_bind, fd_bind);
        } else {
            bind_ns_files(getpid());
        }
    }
    if pid != 0 {
        if waitpid(pid, &mut status, 0 as libc::c_int) == -(1 as libc::c_int) {
            err(
                1 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"waitpid failed\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        if status & 0x7f as libc::c_int == 0 as libc::c_int {
            return (status & 0xff00 as libc::c_int) >> 8 as libc::c_int;
        }
        if ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar as libc::c_int
            >> 1 as libc::c_int
            > 0 as libc::c_int
        {
            let termsig: libc::c_int = status & 0x7f as libc::c_int;
            if signal(termsig, None)
                == ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
                    -(1 as libc::c_int) as libc::intptr_t,
                )
                || sigemptyset(&mut sigset) != 0 as libc::c_int
                || sigaddset(&mut sigset, termsig) != 0 as libc::c_int
                || sigprocmask(1 as libc::c_int, &mut sigset, 0 as *mut sigset_t)
                    != 0 as libc::c_int
            {
                err(
                    1 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"sigprocmask unblock failed\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            kill(getpid(), termsig);
        }
        err(
            1 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"child exit failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if kill_child_signo != 0 as libc::c_int {
        if prctl(1 as libc::c_int, kill_child_signo) < 0 as libc::c_int {
            err(
                1 as libc::c_int,
                b"prctl failed\0" as *const u8 as *const libc::c_char,
            );
        }
        if fd_parent_pid != -(1 as libc::c_int) {
            let mut pollfds: [pollfd; 1] = [{
                let init = pollfd {
                    fd: fd_parent_pid,
                    events: 0x1 as libc::c_int as libc::c_short,
                    revents: 0,
                };
                init
            }];
            let nfds: libc::c_int = poll(
                pollfds.as_mut_ptr(),
                1 as libc::c_int as nfds_t,
                0 as libc::c_int,
            );
            if 0 as libc::c_int > nfds {
                err(
                    1 as libc::c_int,
                    b"poll parent pidfd failed\0" as *const u8 as *const libc::c_char,
                );
            }
            if nfds != 0 {
                exit(1 as libc::c_int);
            }
            close(fd_parent_pid);
            fd_parent_pid = -(1 as libc::c_int);
        }
    }
    if mapuser != -(1 as libc::c_int) as uid_t && usermap.is_null() {
        map_id(
            b"/proc/self/uid_map\0" as *const u8 as *const libc::c_char,
            mapuser,
            real_euid,
        );
    }
    if mapgroup != -(1 as libc::c_int) as gid_t && groupmap.is_null() {
        if setgrpcmd == SETGROUPS_ALLOW as libc::c_int {
            errx(
                1 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"options --setgroups=allow and --map-group are mutually exclusive\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        setgroups_control(SETGROUPS_DENY as libc::c_int);
        map_id(
            b"/proc/self/gid_map\0" as *const u8 as *const libc::c_char,
            mapgroup,
            real_egid,
        );
    }
    if setgrpcmd != SETGROUPS_NONE as libc::c_int {
        setgroups_control(setgrpcmd);
    }
    if unshare_flags & 0x20000 as libc::c_int != 0 && propagation != 0 {
        set_propagation(propagation);
    }
    if !newroot.is_null() {
        if chroot(newroot) != 0 as libc::c_int {
            err(
                1 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot change root directory to '%s'\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                newroot,
            );
        }
        newdir = if !newdir.is_null() {
            newdir
        } else {
            b"/\0" as *const u8 as *const libc::c_char
        };
    }
    if !newdir.is_null() && chdir(newdir) != 0 {
        err(
            1 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"cannot chdir to '%s'\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            newdir,
        );
    }
    if !procmnt.is_null() {
        if newroot.is_null()
            && propagation != (MS_PRIVATE as libc::c_int | MS_REC as libc::c_int) as libc::c_ulong
        {
            let rc: libc::c_int = mount(
                b"none\0" as *const u8 as *const libc::c_char,
                procmnt,
                0 as *const libc::c_char,
                (MS_PRIVATE as libc::c_int | MS_REC as libc::c_int) as libc::c_ulong,
                0 as *const libc::c_void,
            );
            if rc != 0 as libc::c_int && *__errno_location() != 22 as libc::c_int {
                err(
                    1 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot change %s filesystem propagation\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    procmnt,
                );
            }
        }
        if mount(
            b"proc\0" as *const u8 as *const libc::c_char,
            procmnt,
            b"proc\0" as *const u8 as *const libc::c_char,
            (MS_NOSUID as libc::c_int | MS_NOEXEC as libc::c_int | MS_NODEV as libc::c_int)
                as libc::c_ulong,
            0 as *const libc::c_void,
        ) != 0 as libc::c_int
        {
            err(
                1 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"mount %s failed\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                procmnt,
            );
        }
    }
    if force_gid != 0 {
        if setgroups(0 as libc::c_int as size_t, 0 as *const __gid_t) != 0 as libc::c_int {
            err(
                1 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"setgroups failed\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        if setgid(gid) < 0 as libc::c_int {
            err(
                1 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"setgid failed\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    if force_uid != 0 && setuid(uid) < 0 as libc::c_int {
        err(
            1 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"setuid failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if keepcaps != 0 && unshare_flags & 0x10000000 as libc::c_int != 0 {
        cap_permitted_to_ambient();
    }
    if optind < argc {
        execvp(
            *argv.offset(optind as isize),
            argv.offset(optind as isize) as *const *mut libc::c_char,
        );
        err(
            if *__errno_location() == 2 as libc::c_int {
                127 as libc::c_int
            } else {
                126 as libc::c_int
            },
            dcgettext(
                0 as *const libc::c_char,
                b"failed to execute %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            *argv.offset(optind as isize),
        );
    }
    exec_shell()
}

pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    std::process::exit(unsafe {
        main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        )
    });
}
