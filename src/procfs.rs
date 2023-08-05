use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn getpid() -> __pid_t;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getegid() -> __gid_t;
    fn __errno_location() -> *mut libc::c_int;
    fn fstatfs(__fildes: libc::c_int, __buf: *mut statfs) -> libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
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
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn openat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __oflag: libc::c_int,
        _: ...
    ) -> libc::c_int;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn dirfd(__dirp: *mut DIR) -> libc::c_int;
    fn ul_new_path(dir: *const libc::c_char, _: ...) -> *mut path_cxt;
    fn ul_unref_path(pc: *mut path_cxt);
    fn ul_path_set_prefix(pc: *mut path_cxt, prefix: *const libc::c_char) -> libc::c_int;
    fn ul_path_set_dir(pc: *mut path_cxt, dir: *const libc::c_char) -> libc::c_int;
    fn ul_path_set_dialect(
        pc: *mut path_cxt,
        data: *mut libc::c_void,
        free_data: Option::<unsafe extern "C" fn(*mut path_cxt) -> ()>,
    ) -> libc::c_int;
    fn ul_path_get_dialect(pc: *mut path_cxt) -> *mut libc::c_void;
    fn ul_path_get_dirfd(pc: *mut path_cxt) -> libc::c_int;
    fn ul_path_stat(
        pc: *mut path_cxt,
        sb: *mut stat,
        flags: libc::c_int,
        path: *const libc::c_char,
    ) -> libc::c_int;
    fn ul_path_open(
        pc: *mut path_cxt,
        flags: libc::c_int,
        path: *const libc::c_char,
    ) -> libc::c_int;
    fn ul_path_opendir(pc: *mut path_cxt, path: *const libc::c_char) -> *mut DIR;
    fn fstatat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn ul_strtou64(
        str: *const libc::c_char,
        num: *mut uint64_t,
        base: libc::c_int,
    ) -> libc::c_int;
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
pub type __uint64_t = libc::c_ulong;
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __fsid_t {
    pub __val: [libc::c_int; 2],
}
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __fsblkcnt_t = libc::c_ulong;
pub type __fsfilcnt_t = libc::c_ulong;
pub type __fsword_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type uid_t = __uid_t;
pub type useconds_t = __useconds_t;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statfs {
    pub f_type: __fsword_t,
    pub f_bsize: __fsword_t,
    pub f_blocks: __fsblkcnt_t,
    pub f_bfree: __fsblkcnt_t,
    pub f_bavail: __fsblkcnt_t,
    pub f_files: __fsfilcnt_t,
    pub f_ffree: __fsfilcnt_t,
    pub f_fsid: __fsid_t,
    pub f_namelen: __fsword_t,
    pub f_frsize: __fsword_t,
    pub f_flags: __fsword_t,
    pub f_spare: [__fsword_t; 4],
}
pub type uint64_t = __uint64_t;
pub type uintmax_t = __uintmax_t;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const DT_WHT: C2RustUnnamed_0 = 14;
pub const DT_SOCK: C2RustUnnamed_0 = 12;
pub const DT_LNK: C2RustUnnamed_0 = 10;
pub const DT_REG: C2RustUnnamed_0 = 8;
pub const DT_BLK: C2RustUnnamed_0 = 6;
pub const DT_DIR: C2RustUnnamed_0 = 4;
pub const DT_CHR: C2RustUnnamed_0 = 2;
pub const DT_FIFO: C2RustUnnamed_0 = 1;
pub const DT_UNKNOWN: C2RustUnnamed_0 = 0;
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
pub struct procfs_process {
    pub pid: pid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ul_debug_maskname {
    pub name: *const libc::c_char,
    pub mask: libc::c_int,
    pub help: *const libc::c_char,
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
unsafe extern "C" fn fopen_at(
    dir: libc::c_int,
    filename: *const libc::c_char,
    flags: libc::c_int,
    mode: *const libc::c_char,
) -> *mut FILE {
    let fd: libc::c_int = openat(dir, filename, flags);
    let mut ret: *mut FILE = 0 as *mut FILE;
    if fd < 0 as libc::c_int {
        return 0 as *mut FILE;
    }
    ret = fdopen(fd, mode);
    if ret.is_null() {
        close(fd);
    }
    return ret;
}
#[inline]
unsafe extern "C" fn xreaddir(dp: *mut DIR) -> *mut dirent {
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
unsafe extern "C" fn ul_debug_parse_mask(
    flagnames: *const ul_debug_maskname,
    mask: *const libc::c_char,
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
static mut ulprocfs_debug_mask: libc::c_int = 0;
static mut ulprocfs_masknames: [ul_debug_maskname; 1] = [
    {
        let init = ul_debug_maskname {
            name: 0 as *const libc::c_char,
            mask: 0 as libc::c_int,
            help: 0 as *const libc::c_char,
        };
        init
    },
];
#[inline]
unsafe extern "C" fn ul_debugobj(
    handler: *const libc::c_void,
    mesg: *const libc::c_char,
    args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    if !handler.is_null()
        && ulprocfs_debug_mask & (1 as libc::c_int) << 24 as libc::c_int == 0
    {
        fprintf(stderr, b"[%p]: \0" as *const u8 as *const libc::c_char, handler);
    }
    ap = args.clone();
    vfprintf(stderr, mesg, ap.as_va_list());
    fputc('\n' as i32, stderr);
}
#[no_mangle]
pub unsafe extern "C" fn ul_procfs_init_debug() {
    if ulprocfs_debug_mask != 0 {
        return;
    }
    let envstr: *const libc::c_char = if 0 as libc::c_int != 0 {
        0 as *mut libc::c_char
    } else {
        getenv(b"ULPROCFS_DEBUG\0" as *const u8 as *const libc::c_char)
    };
    if !(ulprocfs_debug_mask & (1 as libc::c_int) << 1 as libc::c_int != 0) {
        if 0 as libc::c_int == 0 && !envstr.is_null() {
            ulprocfs_debug_mask = ul_debug_parse_mask(
                ulprocfs_masknames.as_ptr(),
                envstr,
            );
        } else {
            ulprocfs_debug_mask = 0 as libc::c_int;
        }
    }
    if ulprocfs_debug_mask != 0 {
        if getuid() != geteuid() || getgid() != getegid() {
            ulprocfs_debug_mask |= (1 as libc::c_int) << 24 as libc::c_int;
            fprintf(
                stderr,
                b"%d: %s: don't print memory addresses (SUID executable).\n\0"
                    as *const u8 as *const libc::c_char,
                getpid(),
                b"ulprocfs\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    ulprocfs_debug_mask |= (1 as libc::c_int) << 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_new_procfs_path(
    pid: pid_t,
    prefix: *const libc::c_char,
) -> *mut path_cxt {
    let pc: *mut path_cxt = ul_new_path(0 as *const libc::c_char);
    if pc.is_null() {
        return 0 as *mut path_cxt;
    }
    if !prefix.is_null() {
        ul_path_set_prefix(pc, prefix);
    }
    if procfs_process_init_path(pc, pid) != 0 as libc::c_int {
        ul_unref_path(pc);
        return 0 as *mut path_cxt;
    }
    if (1 as libc::c_int) << 2 as libc::c_int & ulprocfs_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"ulprocfs\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            pc as *const libc::c_void,
            b"alloc\0" as *const u8 as *const libc::c_char,
        );
    }
    return pc;
}
#[no_mangle]
pub unsafe extern "C" fn procfs_process_init_path(
    pc: *mut path_cxt,
    pid: pid_t,
) -> libc::c_int {
    let mut prc: *mut procfs_process = 0 as *mut procfs_process;
    let mut rc: libc::c_int = 0;
    let mut buf: [libc::c_char; 22] = [0; 22];
    snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong,
        b"/proc/%zu\0" as *const u8 as *const libc::c_char,
        pid as size_t,
    );
    rc = ul_path_set_dir(pc, buf.as_mut_ptr());
    if rc != 0 {
        return rc;
    }
    rc = ul_path_get_dirfd(pc);
    if rc < 0 as libc::c_int {
        return rc;
    }
    prc = ul_path_get_dialect(pc) as *mut procfs_process;
    if prc.is_null() {
        if (1 as libc::c_int) << 2 as libc::c_int & ulprocfs_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"ulprocfs\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                pc as *const libc::c_void,
                b"alloc new procfs handler\0" as *const u8 as *const libc::c_char,
            );
        }
        prc = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::core::mem::size_of::<procfs_process>() as libc::c_ulong,
        ) as *mut procfs_process;
        if prc.is_null() {
            return -(12 as libc::c_int);
        }
        ul_path_set_dialect(
            pc,
            prc as *mut libc::c_void,
            Some(procfs_process_deinit_path as unsafe extern "C" fn(*mut path_cxt) -> ()),
        );
    }
    if (1 as libc::c_int) << 2 as libc::c_int & ulprocfs_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"ulprocfs\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            pc as *const libc::c_void,
            b"init procfs stuff\0" as *const u8 as *const libc::c_char,
        );
    }
    (*prc).pid = pid;
    return 0 as libc::c_int;
}
unsafe extern "C" fn procfs_process_deinit_path(pc: *mut path_cxt) {
    let mut prc: *mut procfs_process = 0 as *mut procfs_process;
    if pc.is_null() {
        return;
    }
    if (1 as libc::c_int) << 2 as libc::c_int & ulprocfs_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"ulprocfs\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            pc as *const libc::c_void,
            b"deinit\0" as *const u8 as *const libc::c_char,
        );
    }
    prc = ul_path_get_dialect(pc) as *mut procfs_process;
    if prc.is_null() {
        return;
    }
    free(prc as *mut libc::c_void);
    ul_path_set_dialect(pc, 0 as *mut libc::c_void, None);
}
unsafe extern "C" fn read_procfs_file(
    fd: libc::c_int,
    buf: *mut libc::c_char,
    bufsz: size_t,
) -> ssize_t {
    let mut sz: ssize_t = 0 as libc::c_int as ssize_t;
    let mut i: size_t = 0;
    if fd < 0 as libc::c_int {
        return -(22 as libc::c_int) as ssize_t;
    }
    sz = read_all(fd, buf, bufsz);
    if sz <= 0 as libc::c_int as libc::c_long {
        return sz;
    }
    i = 0 as libc::c_int as size_t;
    while i < sz as size_t {
        if *buf.offset(i as isize) as libc::c_int == '\0' as i32 {
            *buf.offset(i as isize) = ' ' as i32 as libc::c_char;
        }
        i = i.wrapping_add(1);
        i;
    }
    *buf
        .offset(
            (sz - 1 as libc::c_int as libc::c_long) as isize,
        ) = '\0' as i32 as libc::c_char;
    return sz;
}
unsafe extern "C" fn procfs_process_get_data_for(
    pc: *mut path_cxt,
    buf: *mut libc::c_char,
    bufsz: size_t,
    fname: *const libc::c_char,
) -> ssize_t {
    let fd: libc::c_int = ul_path_open(
        pc,
        0 as libc::c_int | 0o2000000 as libc::c_int,
        fname,
    );
    if fd >= 0 as libc::c_int {
        let sz: ssize_t = read_procfs_file(fd, buf, bufsz);
        close(fd);
        return sz;
    }
    return -*__errno_location() as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn procfs_process_get_cmdline(
    pc: *mut path_cxt,
    buf: *mut libc::c_char,
    bufsz: size_t,
) -> ssize_t {
    return procfs_process_get_data_for(
        pc,
        buf,
        bufsz,
        b"cmdline\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn procfs_process_get_cmdname(
    pc: *mut path_cxt,
    buf: *mut libc::c_char,
    bufsz: size_t,
) -> ssize_t {
    return procfs_process_get_data_for(
        pc,
        buf,
        bufsz,
        b"comm\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn procfs_process_get_stat(
    pc: *mut path_cxt,
    buf: *mut libc::c_char,
    bufsz: size_t,
) -> ssize_t {
    return procfs_process_get_data_for(
        pc,
        buf,
        bufsz,
        b"stat\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn procfs_process_get_stat_nth(
    pc: *mut path_cxt,
    n: libc::c_int,
    re: *mut uintmax_t,
) -> libc::c_int {
    let mut rc: ssize_t = 0;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tok: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut i: libc::c_int = 0;
    if n == 2 as libc::c_int || n == 3 as libc::c_int {
        return -(22 as libc::c_int);
    }
    rc = procfs_process_get_data_for(
        pc,
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
        b"stat\0" as *const u8 as *const libc::c_char,
    );
    if rc < 0 as libc::c_int as libc::c_long {
        return rc as libc::c_int;
    }
    i = 0 as libc::c_int;
    tok = strtok_r(
        buf.as_mut_ptr(),
        b" \0" as *const u8 as *const libc::c_char,
        &mut key,
    );
    while !tok.is_null() {
        i += 1;
        i;
        if i == n {
            return ul_strtou64(tok, re, 10 as libc::c_int);
        }
        if i == 2 as libc::c_int
            && {
                p = strrchr(key, ')' as i32);
                !p.is_null()
            }
        {
            key = p.offset(2 as libc::c_int as isize);
        }
        tok = strtok_r(
            0 as *mut libc::c_char,
            b" \0" as *const u8 as *const libc::c_char,
            &mut key,
        );
    }
    return -(22 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn procfs_process_get_uid(
    pc: *mut path_cxt,
    uid: *mut uid_t,
) -> libc::c_int {
    let mut sb: stat = stat {
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
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut rc: libc::c_int = 0;
    rc = ul_path_stat(pc, &mut sb, 0 as libc::c_int, 0 as *const libc::c_char);
    if rc == 0 as libc::c_int {
        *uid = sb.st_uid;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn procfs_process_next_tid(
    pc: *mut path_cxt,
    sub: *mut *mut DIR,
    tid: *mut pid_t,
) -> libc::c_int {
    let mut d: *mut dirent = 0 as *mut dirent;
    if pc.is_null() || sub.is_null() || tid.is_null() {
        return -(22 as libc::c_int);
    }
    if (*sub).is_null() {
        *sub = ul_path_opendir(pc, b"task\0" as *const u8 as *const libc::c_char);
        if (*sub).is_null() {
            return -*__errno_location();
        }
    }
    loop {
        d = xreaddir(*sub);
        if d.is_null() {
            break;
        }
        if procfs_dirent_get_pid(d, tid) == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    closedir(*sub);
    *sub = 0 as *mut DIR;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn procfs_process_next_fd(
    pc: *mut path_cxt,
    sub: *mut *mut DIR,
    fd: *mut libc::c_int,
) -> libc::c_int {
    let mut d: *mut dirent = 0 as *mut dirent;
    if pc.is_null() || sub.is_null() || fd.is_null() {
        return -(22 as libc::c_int);
    }
    if (*sub).is_null() {
        *sub = ul_path_opendir(pc, b"fd\0" as *const u8 as *const libc::c_char);
        if (*sub).is_null() {
            return -*__errno_location();
        }
    }
    loop {
        d = xreaddir(*sub);
        if d.is_null() {
            break;
        }
        let mut num: uint64_t = 0;
        if (*d).d_type as libc::c_int != DT_LNK as libc::c_int
            && (*d).d_type as libc::c_int != DT_UNKNOWN as libc::c_int
        {
            continue;
        }
        if ul_strtou64(((*d).d_name).as_mut_ptr(), &mut num, 10 as libc::c_int)
            < 0 as libc::c_int
        {
            continue;
        }
        *fd = num as libc::c_int;
        return 0 as libc::c_int;
    }
    closedir(*sub);
    *sub = 0 as *mut DIR;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn procfs_dirent_is_process(d: *mut dirent) -> libc::c_int {
    if (*d).d_type as libc::c_int != DT_DIR as libc::c_int
        && (*d).d_type as libc::c_int != DT_UNKNOWN as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if *(*__ctype_b_loc())
        .offset(*((*d).d_name).as_mut_ptr() as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn procfs_dirent_get_pid(
    d: *mut dirent,
    pid: *mut pid_t,
) -> libc::c_int {
    let mut num: uint64_t = 0;
    if procfs_dirent_is_process(d) == 0 {
        return -(22 as libc::c_int);
    }
    if ul_strtou64(((*d).d_name).as_mut_ptr(), &mut num, 10 as libc::c_int)
        < 0 as libc::c_int
    {
        return -(22 as libc::c_int);
    }
    *pid = num as pid_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn procfs_dirent_get_uid(
    procfs: *mut DIR,
    d: *mut dirent,
    uid: *mut uid_t,
) -> libc::c_int {
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
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if procfs_dirent_is_process(d) == 0 {
        return -(22 as libc::c_int);
    }
    if fstatat(dirfd(procfs), ((*d).d_name).as_mut_ptr(), &mut st, 0 as libc::c_int) != 0
    {
        return -(22 as libc::c_int);
    }
    *uid = st.st_uid;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn procfs_dirent_match_uid(
    procfs: *mut DIR,
    d: *mut dirent,
    uid: uid_t,
) -> libc::c_int {
    let mut x: uid_t = 0;
    if procfs_dirent_get_uid(procfs, d, &mut x) == 0 as libc::c_int {
        return (x == uid) as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn procfs_dirent_get_name(
    procfs: *mut DIR,
    d: *mut dirent,
    buf: *mut libc::c_char,
    bufsz: size_t,
) -> libc::c_int {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut sz: size_t = 0;
    let mut tmp: [libc::c_char; 1024] = [0; 1024];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    if bufsz < 32 as libc::c_int as libc::c_ulong {
        return -(22 as libc::c_int);
    }
    if procfs_dirent_is_process(d) == 0 {
        return -(22 as libc::c_int);
    }
    snprintf(
        tmp.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        b"%s/stat\0" as *const u8 as *const libc::c_char,
        ((*d).d_name).as_mut_ptr(),
    );
    f = fopen_at(
        dirfd(procfs),
        tmp.as_mut_ptr(),
        0o2000000 as libc::c_int | 0 as libc::c_int,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if f.is_null() {
        return -*__errno_location();
    }
    p = fgets(
        tmp.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        f,
    );
    fclose(f);
    if p.is_null() {
        return -*__errno_location();
    }
    while *p as libc::c_int != 0 && *p as libc::c_int != '(' as i32 {
        p = p.offset(1);
        p;
    }
    while *p as libc::c_int != 0 && *p as libc::c_int == '(' as i32 {
        p = p.offset(1);
        p;
    }
    end = p;
    while *end as libc::c_int != 0 && *end as libc::c_int != ')' as i32 {
        end = end.offset(1);
        end;
    }
    sz = end.offset_from(p) as libc::c_long as size_t;
    if sz >= bufsz {
        sz = bufsz.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    }
    memcpy(buf as *mut libc::c_void, p as *const libc::c_void, sz);
    *buf.offset(sz as isize) = '\0' as i32 as libc::c_char;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn procfs_dirent_match_name(
    procfs: *mut DIR,
    d: *mut dirent,
    name: *const libc::c_char,
) -> libc::c_int {
    let mut buf: [libc::c_char; 33] = [0; 33];
    if procfs_dirent_get_name(
        procfs,
        d,
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return (strcmp(name, buf.as_mut_ptr()) == 0 as libc::c_int) as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fd_is_procfs(fd: libc::c_int) -> libc::c_int {
    let mut st: statfs = statfs {
        f_type: 0,
        f_bsize: 0,
        f_blocks: 0,
        f_bfree: 0,
        f_bavail: 0,
        f_files: 0,
        f_ffree: 0,
        f_fsid: __fsid_t { __val: [0; 2] },
        f_namelen: 0,
        f_frsize: 0,
        f_flags: 0,
        f_spare: [0; 4],
    };
    let mut ret: libc::c_int = 0;
    loop {
        *__errno_location() = 0 as libc::c_int;
        ret = fstatfs(fd, &mut st);
        if ret < 0 as libc::c_int {
            if *__errno_location() != 4 as libc::c_int
                && *__errno_location() != 11 as libc::c_int
            {
                return 0 as libc::c_int;
            }
            xusleep(250000 as libc::c_int as useconds_t);
        }
        if !(ret != 0 as libc::c_int) {
            break;
        }
    }
    return (st.f_type == 0x9fa0 as libc::c_int as libc::c_long) as libc::c_int;
}
unsafe extern "C" fn strdup_procfs_file(
    pid: pid_t,
    name: *const libc::c_char,
) -> *mut libc::c_char {
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut re: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fd: libc::c_int = 0;
    snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
        b"/proc/%d/%s\0" as *const u8 as *const libc::c_char,
        pid,
        name,
    );
    fd = open(buf.as_mut_ptr(), 0o2000000 as libc::c_int | 0 as libc::c_int);
    if fd < 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    if read_procfs_file(
        fd,
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
    ) > 0 as libc::c_int as libc::c_long
    {
        re = strdup(buf.as_mut_ptr());
    }
    close(fd);
    return re;
}
#[no_mangle]
pub unsafe extern "C" fn pid_get_cmdname(pid: pid_t) -> *mut libc::c_char {
    return strdup_procfs_file(pid, b"comm\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn pid_get_cmdline(pid: pid_t) -> *mut libc::c_char {
    return strdup_procfs_file(pid, b"cmdline\0" as *const u8 as *const libc::c_char);
}
