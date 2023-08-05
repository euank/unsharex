use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn faccessat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __type: libc::c_int,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getegid() -> __gid_t;
    fn readlink(
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn readlinkat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn getpid() -> __pid_t;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn vasprintf(
        __ptr: *mut *mut libc::c_char,
        __f: *const libc::c_char,
        __arg: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn vfscanf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn openat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __oflag: libc::c_int,
        _: ...
    ) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn fstatat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn dup_fd_cloexec(oldfd: libc::c_int, lowfd: libc::c_int) -> libc::c_int;
    fn fdopendir(__fd: libc::c_int) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn rewinddir(__dirp: *mut DIR);
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn cpumask_parse(
        str: *const libc::c_char,
        set: *mut cpu_set_t,
        setsize: size_t,
    ) -> libc::c_int;
    fn cpulist_parse(
        str: *const libc::c_char,
        set: *mut cpu_set_t,
        setsize: size_t,
        fail: libc::c_int,
    ) -> libc::c_int;
    fn cpuset_free(set: *mut cpu_set_t);
    fn cpuset_alloc(
        ncpus: libc::c_int,
        setsize: *mut size_t,
        nbits: *mut size_t,
    ) -> *mut cpu_set_t;
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
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
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
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type useconds_t = __useconds_t;
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
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type dev_t = __dev_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct path_cxt {
    pub dir_fd: libc::c_int,
    pub dir_path: *mut libc::c_char,
    pub refcount: libc::c_int,
    pub prefix: *mut libc::c_char,
    pub path_buffer: [libc::c_char; 4096],
    pub dialect: *mut libc::c_void,
    pub free_dialect: Option::<unsafe extern "C" fn(*mut path_cxt) -> ()>,
    pub redirect_on_enoent: Option::<
        unsafe extern "C" fn(
            *mut path_cxt,
            *const libc::c_char,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ul_debug_maskname {
    pub name: *const libc::c_char,
    pub mask: libc::c_int,
    pub help: *const libc::c_char,
}
pub type __cpu_mask = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16],
}
#[inline]
unsafe extern "C" fn xusleep(mut usec: useconds_t) -> libc::c_int {
    let mut waittime: timespec = {
        let mut init = timespec {
            tv_sec: usec as libc::c_long / 1000000 as libc::c_long,
            tv_nsec: usec as libc::c_long % 1000000 as libc::c_long
                * 1000 as libc::c_int as libc::c_long,
        };
        init
    };
    return nanosleep(&mut waittime, 0 as *mut timespec);
}
#[inline]
unsafe extern "C" fn gnu_dev_makedev(
    mut __major: libc::c_uint,
    mut __minor: libc::c_uint,
) -> __dev_t {
    let mut __dev: __dev_t = 0;
    __dev = ((__major & 0xfff as libc::c_uint) as __dev_t) << 8 as libc::c_int;
    __dev |= ((__major & 0xfffff000 as libc::c_uint) as __dev_t) << 32 as libc::c_int;
    __dev |= ((__minor & 0xff as libc::c_uint) as __dev_t) << 0 as libc::c_int;
    __dev |= ((__minor & 0xffffff00 as libc::c_uint) as __dev_t) << 12 as libc::c_int;
    return __dev;
}
#[inline]
unsafe extern "C" fn xreaddir(mut dp: *mut DIR) -> *mut dirent {
    let mut d: *mut dirent = 0 as *mut dirent;
    loop {
        d = readdir(dp);
        if d.is_null() {
            break;
        }
        if !(strcmp(
            ((*d).d_name).as_mut_ptr(),
            b".\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                ((*d).d_name).as_mut_ptr(),
                b"..\0" as *const u8 as *const libc::c_char,
            ) == 0)
        {
            break;
        }
    }
    return d;
}
#[inline]
unsafe extern "C" fn write_all(
    mut fd: libc::c_int,
    mut buf: *const libc::c_void,
    mut count: size_t,
) -> libc::c_int {
    while count != 0 {
        let mut tmp: ssize_t = 0;
        *__errno_location() = 0 as libc::c_int;
        tmp = write(fd, buf, count);
        if tmp > 0 as libc::c_int as libc::c_long {
            count = (count as libc::c_ulong).wrapping_sub(tmp as libc::c_ulong) as size_t
                as size_t;
            if count != 0 {
                buf = (buf as *const libc::c_char).offset(tmp as isize)
                    as *const libc::c_void;
            }
        } else if *__errno_location() != 4 as libc::c_int
            && *__errno_location() != 11 as libc::c_int
        {
            return -(1 as libc::c_int)
        }
        if *__errno_location() == 11 as libc::c_int {
            xusleep(250000 as libc::c_int as useconds_t);
        }
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn read_all(
    mut fd: libc::c_int,
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
            if (*__errno_location() == 11 as libc::c_int
                || *__errno_location() == 4 as libc::c_int)
                && {
                    let fresh0 = tries;
                    tries = tries + 1;
                    fresh0 < 5 as libc::c_int
                }
            {
                xusleep(250000 as libc::c_int as useconds_t);
            } else {
                return if c != 0 { c } else { -(1 as libc::c_int) as libc::c_long }
            }
        } else {
            if ret == 0 as libc::c_int as libc::c_long {
                return c;
            }
            tries = 0 as libc::c_int;
            count = (count as libc::c_ulong).wrapping_sub(ret as libc::c_ulong) as size_t
                as size_t;
            buf = buf.offset(ret as isize);
            c += ret;
        }
    }
    return c;
}
#[inline]
unsafe extern "C" fn ul_debug(mut mesg: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    vfprintf(stderr, mesg, ap.as_va_list());
    fputc('\n' as i32, stderr);
}
#[inline]
unsafe extern "C" fn ul_debug_parse_mask(
    mut flagnames: *const ul_debug_maskname,
    mut mask: *const libc::c_char,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    res = strtoul(mask, &mut ptr, 0 as libc::c_int) as libc::c_int;
    if !ptr.is_null() && *ptr as libc::c_int != 0 && !flagnames.is_null()
        && !((*flagnames.offset(0 as libc::c_int as isize)).name).is_null()
    {
        let mut msbuf: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ms: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        res = 0 as libc::c_int;
        msbuf = strdup(mask);
        ms = msbuf;
        if ms.is_null() {
            return res;
        }
        loop {
            name = strtok_r(ms, b",\0" as *const u8 as *const libc::c_char, &mut ptr);
            if name.is_null() {
                break;
            }
            let mut d: *const ul_debug_maskname = 0 as *const ul_debug_maskname;
            ms = ptr;
            d = flagnames;
            while !d.is_null() && !((*d).name).is_null() {
                if strcmp(name, (*d).name) == 0 as libc::c_int {
                    res |= (*d).mask;
                    break;
                } else {
                    d = d.offset(1);
                    d;
                }
            }
            if res == 0xffff as libc::c_int {
                break;
            }
        }
        free(msbuf as *mut libc::c_void);
    } else if !ptr.is_null()
        && strcmp(ptr, b"all\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        res = 0xffff as libc::c_int;
    }
    return res;
}
#[inline]
unsafe extern "C" fn xstrncpy(
    mut dest: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut n: size_t,
) {
    let mut len: size_t = if !src.is_null() {
        strlen(src)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    if len == 0 {
        return;
    }
    len = ({
        let mut _min1: size_t = len;
        let mut _min2: libc::c_ulong = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        &mut _min1 as *mut size_t;
        &mut _min2 as *mut libc::c_ulong;
        if _min1 < _min2 { _min1 } else { _min2 }
    });
    memcpy(dest as *mut libc::c_void, src as *const libc::c_void, len);
    *dest.offset(len as isize) = 0 as libc::c_int as libc::c_char;
}
static mut ulpath_debug_mask: libc::c_int = 0;
static mut ulpath_masknames: [ul_debug_maskname; 1] = [
    {
        let mut init = ul_debug_maskname {
            name: 0 as *const libc::c_char,
            mask: 0 as libc::c_int,
            help: 0 as *const libc::c_char,
        };
        init
    },
];
#[inline]
unsafe extern "C" fn ul_debugobj(
    mut handler: *const libc::c_void,
    mut mesg: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    if !handler.is_null()
        && ulpath_debug_mask & (1 as libc::c_int) << 24 as libc::c_int == 0
    {
        fprintf(stderr, b"[%p]: \0" as *const u8 as *const libc::c_char, handler);
    }
    ap = args.clone();
    vfprintf(stderr, mesg, ap.as_va_list());
    fputc('\n' as i32, stderr);
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_init_debug() {
    if ulpath_debug_mask != 0 {
        return;
    }
    let mut envstr: *const libc::c_char = if 0 as libc::c_int != 0 {
        0 as *mut libc::c_char
    } else {
        getenv(b"ULPATH_DEBUG\0" as *const u8 as *const libc::c_char)
    };
    if !(ulpath_debug_mask & (1 as libc::c_int) << 1 as libc::c_int != 0) {
        if 0 as libc::c_int == 0 && !envstr.is_null() {
            ulpath_debug_mask = ul_debug_parse_mask(ulpath_masknames.as_ptr(), envstr);
        } else {
            ulpath_debug_mask = 0 as libc::c_int;
        }
    }
    if ulpath_debug_mask != 0 {
        if getuid() != geteuid() || getgid() != getegid() {
            ulpath_debug_mask |= (1 as libc::c_int) << 24 as libc::c_int;
            fprintf(
                stderr,
                b"%d: %s: don't print memory addresses (SUID executable).\n\0"
                    as *const u8 as *const libc::c_char,
                getpid(),
                b"ulpath\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    ulpath_debug_mask |= (1 as libc::c_int) << 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_new_path(
    mut dir: *const libc::c_char,
    mut args: ...
) -> *mut path_cxt {
    let mut pc: *mut path_cxt = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<path_cxt>() as libc::c_ulong,
    ) as *mut path_cxt;
    if pc.is_null() {
        return 0 as *mut path_cxt;
    }
    if (1 as libc::c_int) << 2 as libc::c_int & ulpath_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"ulpath\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            pc as *const libc::c_void,
            b"alloc\0" as *const u8 as *const libc::c_char,
        );
    }
    (*pc).refcount = 1 as libc::c_int;
    (*pc).dir_fd = -(1 as libc::c_int);
    if !dir.is_null() {
        let mut rc: libc::c_int = 0;
        let mut ap: ::core::ffi::VaListImpl;
        ap = args.clone();
        rc = vasprintf(&mut (*pc).dir_path, dir, ap.as_va_list());
        if rc < 0 as libc::c_int || ((*pc).dir_path).is_null() {
            ul_unref_path(pc);
            return 0 as *mut path_cxt;
        }
    }
    return pc;
}
#[no_mangle]
pub unsafe extern "C" fn ul_ref_path(mut pc: *mut path_cxt) {
    if !pc.is_null() {
        (*pc).refcount += 1;
        (*pc).refcount;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ul_unref_path(mut pc: *mut path_cxt) {
    if pc.is_null() {
        return;
    }
    (*pc).refcount -= 1;
    (*pc).refcount;
    if (*pc).refcount <= 0 as libc::c_int {
        if (1 as libc::c_int) << 2 as libc::c_int & ulpath_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"ulpath\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                pc as *const libc::c_void,
                b"dealloc\0" as *const u8 as *const libc::c_char,
            );
        }
        if !((*pc).dialect).is_null() {
            ((*pc).free_dialect).expect("non-null function pointer")(pc);
        }
        ul_path_close_dirfd(pc);
        free((*pc).dir_path as *mut libc::c_void);
        free((*pc).prefix as *mut libc::c_void);
        free(pc as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_set_prefix(
    mut pc: *mut path_cxt,
    mut prefix: *const libc::c_char,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*pc).dir_fd < 0 as libc::c_int {} else {
        __assert_fail(
            b"pc->dir_fd < 0\0" as *const u8 as *const libc::c_char,
            b"lib/path.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"int ul_path_set_prefix(struct path_cxt *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_6415: {
        if (*pc).dir_fd < 0 as libc::c_int {} else {
            __assert_fail(
                b"pc->dir_fd < 0\0" as *const u8 as *const libc::c_char,
                b"lib/path.c\0" as *const u8 as *const libc::c_char,
                110 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 56],
                    &[libc::c_char; 56],
                >(b"int ul_path_set_prefix(struct path_cxt *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !prefix.is_null() {
        p = strdup(prefix);
        if p.is_null() {
            return -(12 as libc::c_int);
        }
    }
    free((*pc).prefix as *mut libc::c_void);
    (*pc).prefix = p;
    if (1 as libc::c_int) << 2 as libc::c_int & ulpath_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"ulpath\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            pc as *const libc::c_void,
            b"new prefix: '%s'\0" as *const u8 as *const libc::c_char,
            p,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_get_prefix(
    mut pc: *mut path_cxt,
) -> *const libc::c_char {
    return if !pc.is_null() { (*pc).prefix } else { 0 as *mut libc::c_char };
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_set_dir(
    mut pc: *mut path_cxt,
    mut dir: *const libc::c_char,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if !dir.is_null() {
        p = strdup(dir);
        if p.is_null() {
            return -(12 as libc::c_int);
        }
    }
    if (*pc).dir_fd >= 0 as libc::c_int {
        close((*pc).dir_fd);
        (*pc).dir_fd = -(1 as libc::c_int);
    }
    free((*pc).dir_path as *mut libc::c_void);
    (*pc).dir_path = p;
    if (1 as libc::c_int) << 2 as libc::c_int & ulpath_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"ulpath\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            pc as *const libc::c_void,
            b"new dir: '%s'\0" as *const u8 as *const libc::c_char,
            p,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_get_dir(mut pc: *mut path_cxt) -> *const libc::c_char {
    return if !pc.is_null() { (*pc).dir_path } else { 0 as *mut libc::c_char };
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_set_dialect(
    mut pc: *mut path_cxt,
    mut data: *mut libc::c_void,
    mut free_data: Option::<unsafe extern "C" fn(*mut path_cxt) -> ()>,
) -> libc::c_int {
    (*pc).dialect = data;
    (*pc).free_dialect = free_data;
    if (1 as libc::c_int) << 2 as libc::c_int & ulpath_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"ulpath\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            pc as *const libc::c_void,
            b"(re)set dialect\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_get_dialect(
    mut pc: *mut path_cxt,
) -> *mut libc::c_void {
    return if !pc.is_null() { (*pc).dialect } else { 0 as *mut libc::c_void };
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_set_enoent_redirect(
    mut pc: *mut path_cxt,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut path_cxt,
            *const libc::c_char,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
) -> libc::c_int {
    (*pc).redirect_on_enoent = func;
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_absdir(mut pc: *mut path_cxt) -> *const libc::c_char {
    let mut rc: libc::c_int = 0;
    let mut dirpath: *const libc::c_char = 0 as *const libc::c_char;
    if ((*pc).prefix).is_null() {
        return (*pc).dir_path;
    }
    dirpath = (*pc).dir_path;
    if dirpath.is_null() {
        return (*pc).prefix;
    }
    if *dirpath as libc::c_int == '/' as i32 {
        dirpath = dirpath.offset(1);
        dirpath;
    }
    rc = snprintf(
        ((*pc).path_buffer).as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        b"%s/%s\0" as *const u8 as *const libc::c_char,
        (*pc).prefix,
        dirpath,
    );
    if rc < 0 as libc::c_int {
        return 0 as *const libc::c_char;
    }
    if rc as size_t >= ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong {
        *__errno_location() = 36 as libc::c_int;
        return 0 as *const libc::c_char;
    }
    return ((*pc).path_buffer).as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_is_accessible(mut pc: *mut path_cxt) -> libc::c_int {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    if !pc.is_null() {} else {
        __assert_fail(
            b"pc\0" as *const u8 as *const libc::c_char,
            b"lib/path.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"int ul_path_is_accessible(struct path_cxt *)\0"))
                .as_ptr(),
        );
    }
    'c_7020: {
        if !pc.is_null() {} else {
            __assert_fail(
                b"pc\0" as *const u8 as *const libc::c_char,
                b"lib/path.c\0" as *const u8 as *const libc::c_char,
                202 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"int ul_path_is_accessible(struct path_cxt *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*pc).dir_fd >= 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    path = get_absdir(pc);
    if path.is_null() {
        return 0 as libc::c_int;
    }
    return (access(path, 0 as libc::c_int) == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_get_dirfd(mut pc: *mut path_cxt) -> libc::c_int {
    if !pc.is_null() {} else {
        __assert_fail(
            b"pc\0" as *const u8 as *const libc::c_char,
            b"lib/path.c\0" as *const u8 as *const libc::c_char,
            215 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"int ul_path_get_dirfd(struct path_cxt *)\0"))
                .as_ptr(),
        );
    }
    'c_6935: {
        if !pc.is_null() {} else {
            __assert_fail(
                b"pc\0" as *const u8 as *const libc::c_char,
                b"lib/path.c\0" as *const u8 as *const libc::c_char,
                215 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"int ul_path_get_dirfd(struct path_cxt *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*pc).dir_path).is_null() {} else {
        __assert_fail(
            b"pc->dir_path\0" as *const u8 as *const libc::c_char,
            b"lib/path.c\0" as *const u8 as *const libc::c_char,
            216 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"int ul_path_get_dirfd(struct path_cxt *)\0"))
                .as_ptr(),
        );
    }
    'c_6898: {
        if !((*pc).dir_path).is_null() {} else {
            __assert_fail(
                b"pc->dir_path\0" as *const u8 as *const libc::c_char,
                b"lib/path.c\0" as *const u8 as *const libc::c_char,
                216 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"int ul_path_get_dirfd(struct path_cxt *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*pc).dir_fd < 0 as libc::c_int {
        let mut path: *const libc::c_char = get_absdir(pc);
        if path.is_null() {
            return -*__errno_location();
        }
        if (1 as libc::c_int) << 2 as libc::c_int & ulpath_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"ulpath\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                pc as *const libc::c_void,
                b"opening dir: '%s'\0" as *const u8 as *const libc::c_char,
                path,
            );
        }
        (*pc).dir_fd = open(path, 0 as libc::c_int | 0o2000000 as libc::c_int);
    }
    return (*pc).dir_fd;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_close_dirfd(mut pc: *mut path_cxt) {
    if !pc.is_null() {} else {
        __assert_fail(
            b"pc\0" as *const u8 as *const libc::c_char,
            b"lib/path.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void ul_path_close_dirfd(struct path_cxt *)\0"))
                .as_ptr(),
        );
    }
    'c_5818: {
        if !pc.is_null() {} else {
            __assert_fail(
                b"pc\0" as *const u8 as *const libc::c_char,
                b"lib/path.c\0" as *const u8 as *const libc::c_char,
                233 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"void ul_path_close_dirfd(struct path_cxt *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*pc).dir_fd >= 0 as libc::c_int {
        if (1 as libc::c_int) << 2 as libc::c_int & ulpath_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"ulpath\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                pc as *const libc::c_void,
                b"closing dir\0" as *const u8 as *const libc::c_char,
            );
        }
        close((*pc).dir_fd);
        (*pc).dir_fd = -(1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_isopen_dirfd(mut pc: *mut path_cxt) -> libc::c_int {
    return (!pc.is_null() && (*pc).dir_fd >= 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn ul_path_mkpath(
    mut pc: *mut path_cxt,
    mut path: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) -> *const libc::c_char {
    let mut rc: libc::c_int = 0;
    *__errno_location() = 0 as libc::c_int;
    rc = vsnprintf(
        ((*pc).path_buffer).as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        path,
        ap.as_va_list(),
    );
    if rc < 0 as libc::c_int {
        if *__errno_location() == 0 {
            *__errno_location() = 22 as libc::c_int;
        }
        return 0 as *const libc::c_char;
    }
    if rc as size_t >= ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong {
        *__errno_location() = 36 as libc::c_int;
        return 0 as *const libc::c_char;
    }
    return ((*pc).path_buffer).as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_get_abspath(
    mut pc: *mut path_cxt,
    mut buf: *mut libc::c_char,
    mut bufsz: size_t,
    mut path: *const libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    if !path.is_null() {
        let mut rc: libc::c_int = 0;
        let mut ap: ::core::ffi::VaListImpl;
        let mut tail: *const libc::c_char = 0 as *const libc::c_char;
        let mut dirpath: *const libc::c_char = (*pc).dir_path;
        ap = args.clone();
        tail = ul_path_mkpath(pc, path, ap.as_va_list());
        if !dirpath.is_null() && *dirpath as libc::c_int == '/' as i32 {
            dirpath = dirpath.offset(1);
            dirpath;
        }
        if !tail.is_null() && *tail as libc::c_int == '/' as i32 {
            tail = tail.offset(1);
            tail;
        }
        rc = snprintf(
            buf,
            bufsz,
            b"%s/%s/%s\0" as *const u8 as *const libc::c_char,
            if !((*pc).prefix).is_null() {
                (*pc).prefix as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            if !dirpath.is_null() {
                dirpath
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            if !tail.is_null() {
                tail
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        if rc as size_t >= bufsz {
            *__errno_location() = 36 as libc::c_int;
            return 0 as *mut libc::c_char;
        }
    } else {
        let mut tmp: *const libc::c_char = get_absdir(pc);
        if tmp.is_null() {
            return 0 as *mut libc::c_char;
        }
        xstrncpy(buf, tmp, bufsz);
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_access(
    mut pc: *mut path_cxt,
    mut mode: libc::c_int,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    if pc.is_null() {
        rc = access(path, mode);
        if (1 as libc::c_int) << 2 as libc::c_int & ulpath_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"ulpath\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debug(
                b"access '%s' [no context, rc=%d]\0" as *const u8 as *const libc::c_char,
                path,
                rc,
            );
        }
    } else {
        let mut dir: libc::c_int = ul_path_get_dirfd(pc);
        if dir < 0 as libc::c_int {
            return dir;
        }
        if *path as libc::c_int == '/' as i32 {
            path = path.offset(1);
            path;
        }
        rc = faccessat(dir, path, mode, 0 as libc::c_int);
        if rc != 0 && *__errno_location() == 2 as libc::c_int
            && ((*pc).redirect_on_enoent).is_some()
            && ((*pc).redirect_on_enoent)
                .expect("non-null function pointer")(pc, path, &mut dir)
                == 0 as libc::c_int
        {
            rc = faccessat(dir, path, mode, 0 as libc::c_int);
        }
        if (1 as libc::c_int) << 2 as libc::c_int & ulpath_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"ulpath\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                pc as *const libc::c_void,
                b"access: '%s' [rc=%d]\0" as *const u8 as *const libc::c_char,
                path,
                rc,
            );
        }
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_accessf(
    mut pc: *mut path_cxt,
    mut mode: libc::c_int,
    mut path: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    ap = args.clone();
    p = ul_path_mkpath(pc, path, ap.as_va_list());
    return if p.is_null() { -*__errno_location() } else { ul_path_access(pc, mode, p) };
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_stat(
    mut pc: *mut path_cxt,
    mut sb: *mut stat,
    mut flags: libc::c_int,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    if pc.is_null() {
        rc = if !path.is_null() { stat(path, sb) } else { -(22 as libc::c_int) };
        if (1 as libc::c_int) << 2 as libc::c_int & ulpath_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"ulpath\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debug(
                b"stat '%s' [no context, rc=%d]\0" as *const u8 as *const libc::c_char,
                path,
                rc,
            );
        }
    } else {
        let mut dir: libc::c_int = ul_path_get_dirfd(pc);
        if dir < 0 as libc::c_int {
            return dir;
        }
        if !path.is_null() {
            if *path as libc::c_int == '/' as i32 {
                path = path.offset(1);
                path;
            }
            rc = fstatat(dir, path, sb, flags);
        } else {
            rc = fstat(dir, sb);
        }
        if rc != 0 && *__errno_location() == 2 as libc::c_int && !path.is_null()
            && ((*pc).redirect_on_enoent).is_some()
            && ((*pc).redirect_on_enoent)
                .expect("non-null function pointer")(pc, path, &mut dir)
                == 0 as libc::c_int
        {
            rc = fstatat(dir, path, sb, 0 as libc::c_int);
        }
        if (1 as libc::c_int) << 2 as libc::c_int & ulpath_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"ulpath\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                pc as *const libc::c_void,
                b"stat '%s' [rc=%d]\0" as *const u8 as *const libc::c_char,
                path,
                rc,
            );
        }
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_open(
    mut pc: *mut path_cxt,
    mut flags: libc::c_int,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    if path.is_null() {
        return -(22 as libc::c_int);
    }
    if pc.is_null() {
        fd = open(path, flags);
        if (1 as libc::c_int) << 2 as libc::c_int & ulpath_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"ulpath\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debug(
                b"opening '%s' [no context]\0" as *const u8 as *const libc::c_char,
                path,
            );
        }
    } else {
        let mut fdx: libc::c_int = 0;
        let mut dir: libc::c_int = ul_path_get_dirfd(pc);
        if dir < 0 as libc::c_int {
            return dir;
        }
        if *path as libc::c_int == '/' as i32 {
            path = path.offset(1);
            path;
        }
        fd = openat(dir, path, flags);
        fdx = fd;
        if fd < 0 as libc::c_int && *__errno_location() == 2 as libc::c_int
            && ((*pc).redirect_on_enoent).is_some()
            && ((*pc).redirect_on_enoent)
                .expect("non-null function pointer")(pc, path, &mut dir)
                == 0 as libc::c_int
        {
            fd = openat(dir, path, flags);
        }
        if (1 as libc::c_int) << 2 as libc::c_int & ulpath_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"ulpath\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                pc as *const libc::c_void,
                b"opening '%s'%s\0" as *const u8 as *const libc::c_char,
                path,
                if fdx != fd {
                    b" [redirected]\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
        }
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_vopenf(
    mut pc: *mut path_cxt,
    mut flags: libc::c_int,
    mut path: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) -> libc::c_int {
    let mut p: *const libc::c_char = ul_path_mkpath(pc, path, ap.as_va_list());
    return if p.is_null() { -*__errno_location() } else { ul_path_open(pc, flags, p) };
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_openf(
    mut pc: *mut path_cxt,
    mut flags: libc::c_int,
    mut path: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    let mut rc: libc::c_int = 0;
    ap = args.clone();
    rc = ul_path_vopenf(pc, flags, path, ap.as_va_list());
    return rc;
}
unsafe extern "C" fn mode2flags(mut mode: *const libc::c_char) -> libc::c_int {
    let mut flags: libc::c_int = 0 as libc::c_int;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = mode;
    while !p.is_null() && *p as libc::c_int != 0 {
        if *p as libc::c_int == 'r' as i32
            && *p.offset(1 as libc::c_int as isize) as libc::c_int == '+' as i32
        {
            flags |= 0o2 as libc::c_int;
        } else if *p as libc::c_int == 'r' as i32 {
            flags |= 0 as libc::c_int;
        } else if *p as libc::c_int == 'w' as i32
            && *p.offset(1 as libc::c_int as isize) as libc::c_int == '+' as i32
        {
            flags |= 0o2 as libc::c_int | 0o1000 as libc::c_int;
        } else if *p as libc::c_int == 'w' as i32 {
            flags |= 0o1 as libc::c_int | 0o1000 as libc::c_int;
        } else if *p as libc::c_int == 'a' as i32
            && *p.offset(1 as libc::c_int as isize) as libc::c_int == '+' as i32
        {
            flags |= 0o2 as libc::c_int | 0o2000 as libc::c_int;
        } else if *p as libc::c_int == 'a' as i32 {
            flags |= 0o1 as libc::c_int | 0o2000 as libc::c_int;
        } else if *p as libc::c_int
            == *(b"e\0" as *const u8 as *const libc::c_char) as libc::c_int
        {
            flags |= 0o2000000 as libc::c_int;
        }
        p = p.offset(1);
        p;
    }
    return flags;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_fopen(
    mut pc: *mut path_cxt,
    mut mode: *const libc::c_char,
    mut path: *const libc::c_char,
) -> *mut FILE {
    let mut flags: libc::c_int = mode2flags(mode);
    let mut fd: libc::c_int = ul_path_open(pc, flags, path);
    if fd < 0 as libc::c_int {
        return 0 as *mut FILE;
    }
    return fdopen(fd, mode);
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_vfopenf(
    mut pc: *mut path_cxt,
    mut mode: *const libc::c_char,
    mut path: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) -> *mut FILE {
    let mut p: *const libc::c_char = ul_path_mkpath(pc, path, ap.as_va_list());
    return if p.is_null() { 0 as *mut FILE } else { ul_path_fopen(pc, mode, p) };
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_fopenf(
    mut pc: *mut path_cxt,
    mut mode: *const libc::c_char,
    mut path: *const libc::c_char,
    mut args: ...
) -> *mut FILE {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    f = ul_path_vfopenf(pc, mode, path, ap.as_va_list());
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_opendir(
    mut pc: *mut path_cxt,
    mut path: *const libc::c_char,
) -> *mut DIR {
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut fd: libc::c_int = -(1 as libc::c_int);
    if !path.is_null() {
        fd = ul_path_open(pc, 0 as libc::c_int | 0o2000000 as libc::c_int, path);
    } else if !((*pc).dir_path).is_null() {
        let mut dirfd: libc::c_int = 0;
        if (1 as libc::c_int) << 2 as libc::c_int & ulpath_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"ulpath\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                pc as *const libc::c_void,
                b"duplicate dir path\0" as *const u8 as *const libc::c_char,
            );
        }
        dirfd = ul_path_get_dirfd(pc);
        if dirfd >= 0 as libc::c_int {
            fd = dup_fd_cloexec(dirfd, 2 as libc::c_int + 1 as libc::c_int);
        }
    }
    if fd < 0 as libc::c_int {
        return 0 as *mut DIR;
    }
    dir = fdopendir(fd);
    if dir.is_null() {
        close(fd);
        return 0 as *mut DIR;
    }
    if path.is_null() {
        rewinddir(dir);
    }
    return dir;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_vopendirf(
    mut pc: *mut path_cxt,
    mut path: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) -> *mut DIR {
    let mut p: *const libc::c_char = ul_path_mkpath(pc, path, ap.as_va_list());
    return if p.is_null() { 0 as *mut DIR } else { ul_path_opendir(pc, p) };
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_opendirf(
    mut pc: *mut path_cxt,
    mut path: *const libc::c_char,
    mut args: ...
) -> *mut DIR {
    let mut ap: ::core::ffi::VaListImpl;
    let mut dir: *mut DIR = 0 as *mut DIR;
    ap = args.clone();
    dir = ul_path_vopendirf(pc, path, ap.as_va_list());
    return dir;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_readlink(
    mut pc: *mut path_cxt,
    mut buf: *mut libc::c_char,
    mut bufsiz: size_t,
    mut path: *const libc::c_char,
) -> ssize_t {
    let mut dirfd: libc::c_int = 0;
    let mut ssz: ssize_t = 0;
    if path.is_null() {
        let mut p: *const libc::c_char = get_absdir(pc);
        if p.is_null() {
            return -*__errno_location() as ssize_t;
        }
        ssz = readlink(p, buf, bufsiz.wrapping_sub(1 as libc::c_int as libc::c_ulong));
    } else {
        dirfd = ul_path_get_dirfd(pc);
        if dirfd < 0 as libc::c_int {
            return dirfd as ssize_t;
        }
        if *path as libc::c_int == '/' as i32 {
            path = path.offset(1);
            path;
        }
        ssz = readlinkat(
            dirfd,
            path,
            buf,
            bufsiz.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    if ssz >= 0 as libc::c_int as libc::c_long {
        *buf.offset(ssz as isize) = '\0' as i32 as libc::c_char;
    }
    return ssz;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_readlinkf(
    mut pc: *mut path_cxt,
    mut buf: *mut libc::c_char,
    mut bufsiz: size_t,
    mut path: *const libc::c_char,
    mut args: ...
) -> ssize_t {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    p = ul_path_mkpath(pc, path, ap.as_va_list());
    return if p.is_null() {
        -*__errno_location() as libc::c_long
    } else {
        ul_path_readlink(pc, buf, bufsiz, p)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_read(
    mut pc: *mut path_cxt,
    mut buf: *mut libc::c_char,
    mut len: size_t,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut errsv: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    fd = ul_path_open(pc, 0 as libc::c_int | 0o2000000 as libc::c_int, path);
    if fd < 0 as libc::c_int {
        return -*__errno_location();
    }
    if (1 as libc::c_int) << 2 as libc::c_int & ulpath_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"ulpath\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debug(b" reading '%s'\0" as *const u8 as *const libc::c_char, path);
    }
    rc = read_all(fd, buf, len) as libc::c_int;
    errsv = *__errno_location();
    close(fd);
    *__errno_location() = errsv;
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_vreadf(
    mut pc: *mut path_cxt,
    mut buf: *mut libc::c_char,
    mut len: size_t,
    mut path: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) -> libc::c_int {
    let mut p: *const libc::c_char = ul_path_mkpath(pc, path, ap.as_va_list());
    return if p.is_null() {
        -*__errno_location()
    } else {
        ul_path_read(pc, buf, len, p)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_readf(
    mut pc: *mut path_cxt,
    mut buf: *mut libc::c_char,
    mut len: size_t,
    mut path: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    let mut rc: libc::c_int = 0;
    ap = args.clone();
    rc = ul_path_vreadf(pc, buf, len, path, ap.as_va_list());
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_read_string(
    mut pc: *mut path_cxt,
    mut str: *mut *mut libc::c_char,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut rc: libc::c_int = 0;
    if str.is_null() {
        return -(22 as libc::c_int);
    }
    *str = 0 as *mut libc::c_char;
    rc = ul_path_read(
        pc,
        buf.as_mut_ptr(),
        (::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        path,
    );
    if rc < 0 as libc::c_int {
        return rc;
    }
    if rc > 0 as libc::c_int
        && *buf.as_mut_ptr().offset(rc as isize).offset(-(1 as libc::c_int as isize))
            as libc::c_int == '\n' as i32
    {
        rc -= 1;
        rc;
    }
    if rc == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    buf[rc as usize] = '\0' as i32 as libc::c_char;
    *str = strdup(buf.as_mut_ptr());
    if (*str).is_null() {
        rc = -(12 as libc::c_int);
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_readf_string(
    mut pc: *mut path_cxt,
    mut str: *mut *mut libc::c_char,
    mut path: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    p = ul_path_mkpath(pc, path, ap.as_va_list());
    return if p.is_null() {
        -*__errno_location()
    } else {
        ul_path_read_string(pc, str, p)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_read_buffer(
    mut pc: *mut path_cxt,
    mut buf: *mut libc::c_char,
    mut bufsz: size_t,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut rc: libc::c_int = ul_path_read(
        pc,
        buf,
        bufsz.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        path,
    );
    if rc == 0 as libc::c_int {
        *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    } else if rc > 0 as libc::c_int {
        if *buf.offset(rc as isize).offset(-(1 as libc::c_int as isize)) as libc::c_int
            == '\n' as i32
        {
            rc -= 1;
            *buf.offset(rc as isize) = '\0' as i32 as libc::c_char;
        } else {
            *buf.offset((rc - 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        }
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_readf_buffer(
    mut pc: *mut path_cxt,
    mut buf: *mut libc::c_char,
    mut bufsz: size_t,
    mut path: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    p = ul_path_mkpath(pc, path, ap.as_va_list());
    return if p.is_null() {
        -*__errno_location()
    } else {
        ul_path_read_buffer(pc, buf, bufsz, p)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_scanf(
    mut pc: *mut path_cxt,
    mut path: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut fmt_ap: ::core::ffi::VaListImpl;
    let mut rc: libc::c_int = 0;
    f = ul_path_fopen(pc, b"re\0" as *const u8 as *const libc::c_char, path);
    if f.is_null() {
        return -(22 as libc::c_int);
    }
    if (1 as libc::c_int) << 2 as libc::c_int & ulpath_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"ulpath\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debug(b" fscanf [%s] '%s'\0" as *const u8 as *const libc::c_char, fmt, path);
    }
    fmt_ap = args.clone();
    rc = vfscanf(f, fmt, fmt_ap.as_va_list());
    fclose(f);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_scanff(
    mut pc: *mut path_cxt,
    mut path: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut fmt_ap: ::core::ffi::VaListImpl;
    let mut rc: libc::c_int = 0;
    f = ul_path_vfopenf(
        pc,
        b"re\0" as *const u8 as *const libc::c_char,
        path,
        ap.as_va_list(),
    );
    if f.is_null() {
        return -(22 as libc::c_int);
    }
    fmt_ap = args.clone();
    rc = vfscanf(f, fmt, fmt_ap.as_va_list());
    fclose(f);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_read_s64(
    mut pc: *mut path_cxt,
    mut res: *mut int64_t,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut x: int64_t = 0 as libc::c_int as int64_t;
    let mut rc: libc::c_int = 0;
    rc = ul_path_scanf(
        pc,
        path,
        b"%ld\0" as *const u8 as *const libc::c_char,
        &mut x as *mut int64_t,
    );
    if rc != 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if !res.is_null() {
        *res = x;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_readf_s64(
    mut pc: *mut path_cxt,
    mut res: *mut int64_t,
    mut path: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    p = ul_path_mkpath(pc, path, ap.as_va_list());
    return if p.is_null() { -*__errno_location() } else { ul_path_read_s64(pc, res, p) };
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_read_u64(
    mut pc: *mut path_cxt,
    mut res: *mut uint64_t,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut x: uint64_t = 0 as libc::c_int as uint64_t;
    let mut rc: libc::c_int = 0;
    rc = ul_path_scanf(
        pc,
        path,
        b"%lu\0" as *const u8 as *const libc::c_char,
        &mut x as *mut uint64_t,
    );
    if rc != 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if !res.is_null() {
        *res = x;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_readf_u64(
    mut pc: *mut path_cxt,
    mut res: *mut uint64_t,
    mut path: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    p = ul_path_mkpath(pc, path, ap.as_va_list());
    return if p.is_null() { -*__errno_location() } else { ul_path_read_u64(pc, res, p) };
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_read_s32(
    mut pc: *mut path_cxt,
    mut res: *mut libc::c_int,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut x: libc::c_int = 0 as libc::c_int;
    rc = ul_path_scanf(
        pc,
        path,
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut x as *mut libc::c_int,
    );
    if rc != 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if !res.is_null() {
        *res = x;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_readf_s32(
    mut pc: *mut path_cxt,
    mut res: *mut libc::c_int,
    mut path: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    p = ul_path_mkpath(pc, path, ap.as_va_list());
    return if p.is_null() { -*__errno_location() } else { ul_path_read_s32(pc, res, p) };
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_read_u32(
    mut pc: *mut path_cxt,
    mut res: *mut libc::c_uint,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut x: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    rc = ul_path_scanf(
        pc,
        path,
        b"%u\0" as *const u8 as *const libc::c_char,
        &mut x as *mut libc::c_uint,
    );
    if rc != 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if !res.is_null() {
        *res = x;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_readf_u32(
    mut pc: *mut path_cxt,
    mut res: *mut libc::c_uint,
    mut path: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    p = ul_path_mkpath(pc, path, ap.as_va_list());
    return if p.is_null() { -*__errno_location() } else { ul_path_read_u32(pc, res, p) };
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_read_majmin(
    mut pc: *mut path_cxt,
    mut res: *mut dev_t,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut maj: libc::c_int = 0 as libc::c_int;
    let mut min: libc::c_int = 0 as libc::c_int;
    rc = ul_path_scanf(
        pc,
        path,
        b"%d:%d\0" as *const u8 as *const libc::c_char,
        &mut maj as *mut libc::c_int,
        &mut min as *mut libc::c_int,
    );
    if rc != 2 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if !res.is_null() {
        *res = gnu_dev_makedev(maj as libc::c_uint, min as libc::c_uint);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_readf_majmin(
    mut pc: *mut path_cxt,
    mut res: *mut dev_t,
    mut path: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    p = ul_path_mkpath(pc, path, ap.as_va_list());
    return if p.is_null() {
        -*__errno_location()
    } else {
        ul_path_read_majmin(pc, res, p)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_write_string(
    mut pc: *mut path_cxt,
    mut str: *const libc::c_char,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut errsv: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    fd = ul_path_open(pc, 0o1 as libc::c_int | 0o2000000 as libc::c_int, path);
    if fd < 0 as libc::c_int {
        return -*__errno_location();
    }
    rc = write_all(fd, str as *const libc::c_void, strlen(str));
    errsv = *__errno_location();
    close(fd);
    *__errno_location() = errsv;
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_writef_string(
    mut pc: *mut path_cxt,
    mut str: *const libc::c_char,
    mut path: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    p = ul_path_mkpath(pc, path, ap.as_va_list());
    return if p.is_null() {
        -*__errno_location()
    } else {
        ul_path_write_string(pc, str, p)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_write_s64(
    mut pc: *mut path_cxt,
    mut num: int64_t,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut buf: [libc::c_char; 22] = [0; 22];
    let mut rc: libc::c_int = 0;
    let mut errsv: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    fd = ul_path_open(pc, 0o1 as libc::c_int | 0o2000000 as libc::c_int, path);
    if fd < 0 as libc::c_int {
        return -*__errno_location();
    }
    len = snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong,
        b"%ld\0" as *const u8 as *const libc::c_char,
        num,
    );
    if len < 0 as libc::c_int
        || len as size_t >= ::core::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
    {
        rc = if len < 0 as libc::c_int {
            -*__errno_location()
        } else {
            -(7 as libc::c_int)
        };
    } else {
        rc = write_all(fd, buf.as_mut_ptr() as *const libc::c_void, len as size_t);
    }
    errsv = *__errno_location();
    close(fd);
    *__errno_location() = errsv;
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_write_u64(
    mut pc: *mut path_cxt,
    mut num: uint64_t,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut buf: [libc::c_char; 34] = [0; 34];
    let mut rc: libc::c_int = 0;
    let mut errsv: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    fd = ul_path_open(pc, 0o1 as libc::c_int | 0o2000000 as libc::c_int, path);
    if fd < 0 as libc::c_int {
        return -*__errno_location();
    }
    len = snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 34]>() as libc::c_ulong,
        b"%lu\0" as *const u8 as *const libc::c_char,
        num,
    );
    if len < 0 as libc::c_int
        || len as size_t >= ::core::mem::size_of::<[libc::c_char; 34]>() as libc::c_ulong
    {
        rc = if len < 0 as libc::c_int {
            -*__errno_location()
        } else {
            -(7 as libc::c_int)
        };
    } else {
        rc = write_all(fd, buf.as_mut_ptr() as *const libc::c_void, len as size_t);
    }
    errsv = *__errno_location();
    close(fd);
    *__errno_location() = errsv;
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_writef_u64(
    mut pc: *mut path_cxt,
    mut num: uint64_t,
    mut path: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    p = ul_path_mkpath(pc, path, ap.as_va_list());
    return if p.is_null() {
        -*__errno_location()
    } else {
        ul_path_write_u64(pc, num, p)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_count_dirents(
    mut pc: *mut path_cxt,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut r: libc::c_int = 0 as libc::c_int;
    dir = ul_path_opendir(pc, path);
    if dir.is_null() {
        return 0 as libc::c_int;
    }
    while !(xreaddir(dir)).is_null() {
        r += 1;
        r;
    }
    closedir(dir);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_countf_dirents(
    mut pc: *mut path_cxt,
    mut path: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    p = ul_path_mkpath(pc, path, ap.as_va_list());
    return if p.is_null() { -*__errno_location() } else { ul_path_count_dirents(pc, p) };
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_next_dirent(
    mut pc: *mut path_cxt,
    mut sub: *mut *mut DIR,
    mut dirname: *const libc::c_char,
    mut d: *mut *mut dirent,
) -> libc::c_int {
    if pc.is_null() || sub.is_null() || d.is_null() {
        return -(22 as libc::c_int);
    }
    if (*sub).is_null() {
        *sub = ul_path_opendir(pc, dirname);
        if (*sub).is_null() {
            return -*__errno_location();
        }
    }
    *d = xreaddir(*sub);
    if !(*d).is_null() {
        return 0 as libc::c_int;
    }
    closedir(*sub);
    *sub = 0 as *mut DIR;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_prefix_fopen(
    mut prefix: *const libc::c_char,
    mut path: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> *mut FILE {
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    if path.is_null() {
        return 0 as *mut FILE;
    }
    if prefix.is_null() {
        return fopen(path, mode);
    }
    if *path as libc::c_int == '/' as i32 {
        path = path.offset(1);
        path;
    }
    snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        b"%s/%s\0" as *const u8 as *const libc::c_char,
        prefix,
        path,
    );
    return fopen(buf.as_mut_ptr(), mode);
}
unsafe extern "C" fn ul_path_cpuparse(
    mut pc: *mut path_cxt,
    mut set: *mut *mut cpu_set_t,
    mut maxcpus: libc::c_int,
    mut islist: libc::c_int,
    mut path: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) -> libc::c_int {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut setsize: size_t = 0;
    let mut len: size_t = (maxcpus * 7 as libc::c_int) as size_t;
    let vla = len as usize;
    let mut buf: Vec::<libc::c_char> = ::std::vec::from_elem(0, vla);
    let mut rc: libc::c_int = 0;
    *set = 0 as *mut cpu_set_t;
    f = ul_path_vfopenf(
        pc,
        b"re\0" as *const u8 as *const libc::c_char,
        path,
        ap.as_va_list(),
    );
    if f.is_null() {
        return -*__errno_location();
    }
    rc = if (fgets(buf.as_mut_ptr(), len as libc::c_int, f)).is_null() {
        -(5 as libc::c_int)
    } else {
        0 as libc::c_int
    };
    fclose(f);
    if rc != 0 {
        return rc;
    }
    len = strlen(buf.as_mut_ptr());
    if *buf
        .as_mut_ptr()
        .offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int == '\n' as i32
    {
        *buf
            .as_mut_ptr()
            .offset(
                len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
    }
    *set = cpuset_alloc(maxcpus, &mut setsize, 0 as *mut size_t);
    if (*set).is_null() {
        return -(12 as libc::c_int);
    }
    if islist != 0 {
        if cpulist_parse(buf.as_mut_ptr(), *set, setsize, 0 as libc::c_int) != 0 {
            cpuset_free(*set);
            return -(22 as libc::c_int);
        }
    } else if cpumask_parse(buf.as_mut_ptr(), *set, setsize) != 0 {
        cpuset_free(*set);
        return -(22 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_readf_cpuset(
    mut pc: *mut path_cxt,
    mut set: *mut *mut cpu_set_t,
    mut maxcpus: libc::c_int,
    mut path: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    let mut rc: libc::c_int = 0 as libc::c_int;
    ap = args.clone();
    rc = ul_path_cpuparse(pc, set, maxcpus, 0 as libc::c_int, path, ap.as_va_list());
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn ul_path_readf_cpulist(
    mut pc: *mut path_cxt,
    mut set: *mut *mut cpu_set_t,
    mut maxcpus: libc::c_int,
    mut path: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    let mut rc: libc::c_int = 0 as libc::c_int;
    ap = args.clone();
    rc = ul_path_cpuparse(pc, set, maxcpus, 1 as libc::c_int, path, ap.as_va_list());
    return rc;
}
