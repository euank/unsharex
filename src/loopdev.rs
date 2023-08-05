use ::libc;
use ::c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
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
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
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
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getegid() -> __gid_t;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn fstatat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn dirfd(__dirp: *mut DIR) -> libc::c_int;
    fn get_linux_version() -> libc::c_int;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn ul_unref_path(pc: *mut path_cxt);
    fn ul_new_sysfs_path(
        devno: dev_t,
        parent: *mut path_cxt,
        prefix: *const libc::c_char,
    ) -> *mut path_cxt;
    fn ul_path_read_s32(
        pc: *mut path_cxt,
        res: *mut int32_t,
        path: *const libc::c_char,
    ) -> libc::c_int;
    fn ul_path_read_string(
        pc: *mut path_cxt,
        str: *mut *mut libc::c_char,
        path: *const libc::c_char,
    ) -> libc::c_int;
    fn ul_path_read_u64(
        pc: *mut path_cxt,
        res: *mut uint64_t,
        path: *const libc::c_char,
    ) -> libc::c_int;
    fn sysfs_devno_is_wholedisk(devno: dev_t) -> libc::c_int;
    fn sysfs_devname_to_devno(name: *const libc::c_char) -> dev_t;
    fn canonicalize_path(path: *const libc::c_char) -> *mut libc::c_char;
    fn blkdev_get_size(fd: libc::c_int, bytes: *mut libc::c_ulonglong) -> libc::c_int;
    fn blkdev_get_sector_size(
        fd: libc::c_int,
        sector_size: *mut libc::c_int,
    ) -> libc::c_int;
    fn stripoff_last_component(path: *mut libc::c_char) -> *mut libc::c_char;
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
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
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
pub type __syscall_slong_t = libc::c_long;
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
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type useconds_t = __useconds_t;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type C2RustUnnamed = libc::c_uint;
pub const DT_WHT: C2RustUnnamed = 14;
pub const DT_SOCK: C2RustUnnamed = 12;
pub const DT_LNK: C2RustUnnamed = 10;
pub const DT_REG: C2RustUnnamed = 8;
pub const DT_BLK: C2RustUnnamed = 6;
pub const DT_DIR: C2RustUnnamed = 4;
pub const DT_CHR: C2RustUnnamed = 2;
pub const DT_FIFO: C2RustUnnamed = 1;
pub const DT_UNKNOWN: C2RustUnnamed = 0;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const LO_FLAGS_DIRECT_IO: C2RustUnnamed_0 = 16;
pub const LO_FLAGS_PARTSCAN: C2RustUnnamed_0 = 8;
pub const LO_FLAGS_AUTOCLEAR: C2RustUnnamed_0 = 4;
pub const LO_FLAGS_USE_AOPS: C2RustUnnamed_0 = 2;
pub const LO_FLAGS_READ_ONLY: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_info64 {
    pub lo_device: uint64_t,
    pub lo_inode: uint64_t,
    pub lo_rdevice: uint64_t,
    pub lo_offset: uint64_t,
    pub lo_sizelimit: uint64_t,
    pub lo_number: uint32_t,
    pub lo_encrypt_type: uint32_t,
    pub lo_encrypt_key_size: uint32_t,
    pub lo_flags: uint32_t,
    pub lo_file_name: [uint8_t; 64],
    pub lo_crypt_name: [uint8_t; 64],
    pub lo_encrypt_key: [uint8_t; 32],
    pub lo_init: [uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_config {
    pub fd: uint32_t,
    pub block_size: uint32_t,
    pub info: loop_info64,
    pub __reserved: [uint64_t; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct loopdev_iter {
    pub proc_0: *mut FILE,
    pub sysblock: *mut DIR,
    pub ncur: libc::c_int,
    pub minors: *mut libc::c_int,
    pub nminors: libc::c_int,
    pub ct_perm: libc::c_int,
    pub ct_succ: libc::c_int,
    #[bitfield(name = "done", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "default_check", ty = "libc::c_uint", bits = "1..=1")]
    pub done_default_check: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub flags: libc::c_int,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const LOOPITER_FL_USED: C2RustUnnamed_1 = 2;
pub const LOOPITER_FL_FREE: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct loopdev_cxt {
    pub device: [libc::c_char; 128],
    pub filename: *mut libc::c_char,
    pub fd: libc::c_int,
    pub mode: libc::c_int,
    pub blocksize: uint64_t,
    pub flags: libc::c_int,
    #[bitfield(name = "has_info", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "extra_check", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "info_failed", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "control_ok", ty = "libc::c_uint", bits = "3..=3")]
    pub has_info_extra_check_info_failed_control_ok: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub sysfs: *mut path_cxt,
    pub config: loop_config,
    pub iter: loopdev_iter,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const LOOPDEV_FL_SIZELIMIT: C2RustUnnamed_2 = 512;
pub const LOOPDEV_FL_CONTROL: C2RustUnnamed_2 = 256;
pub const LOOPDEV_FL_DEVSUBDIR: C2RustUnnamed_2 = 128;
pub const LOOPDEV_FL_NOIOCTL: C2RustUnnamed_2 = 64;
pub const LOOPDEV_FL_NOSYSFS: C2RustUnnamed_2 = 32;
pub const LOOPDEV_FL_OFFSET: C2RustUnnamed_2 = 16;
pub const LOOPDEV_FL_RDWR: C2RustUnnamed_2 = 2;
pub const LOOPDEV_FL_RDONLY: C2RustUnnamed_2 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ul_debug_maskname {
    pub name: *const libc::c_char,
    pub mask: libc::c_int,
    pub help: *const libc::c_char,
}
#[inline]
unsafe extern "C" fn gnu_dev_major(mut __dev: __dev_t) -> libc::c_uint {
    let mut __major: libc::c_uint = 0;
    __major = ((__dev & 0xfff00 as libc::c_uint as __dev_t) >> 8 as libc::c_int)
        as libc::c_uint;
    __major = (__major as libc::c_ulong
        | (__dev & 0xfffff00000000000 as libc::c_ulong) >> 32 as libc::c_int)
        as libc::c_uint;
    return __major;
}
#[inline]
unsafe extern "C" fn gnu_dev_minor(mut __dev: __dev_t) -> libc::c_uint {
    let mut __minor: libc::c_uint = 0;
    __minor = ((__dev & 0xff as libc::c_uint as __dev_t) >> 0 as libc::c_int)
        as libc::c_uint;
    __minor = (__minor as libc::c_ulong
        | (__dev & 0xffffff00000 as libc::c_ulong) >> 12 as libc::c_int) as libc::c_uint;
    return __minor;
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
#[inline]
unsafe extern "C" fn startswith(
    mut s: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> *const libc::c_char {
    let mut sz: size_t = if !prefix.is_null() {
        strlen(prefix)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    if !s.is_null() && sz != 0 && strncmp(s, prefix, sz) == 0 as libc::c_int {
        return s.offset(sz as isize);
    }
    return 0 as *const libc::c_char;
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
static mut loopdev_debug_mask: libc::c_int = 0;
static mut loopdev_masknames: [ul_debug_maskname; 1] = [
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
        && loopdev_debug_mask & (1 as libc::c_int) << 24 as libc::c_int == 0
    {
        fprintf(stderr, b"[%p]: \0" as *const u8 as *const libc::c_char, handler);
    }
    ap = args.clone();
    vfprintf(stderr, mesg, ap.as_va_list());
    fputc('\n' as i32, stderr);
}
unsafe extern "C" fn loopdev_init_debug() {
    if loopdev_debug_mask != 0 {
        return;
    }
    let mut envstr: *const libc::c_char = if 0 as libc::c_int != 0 {
        0 as *mut libc::c_char
    } else {
        getenv(b"LOOPDEV_DEBUG\0" as *const u8 as *const libc::c_char)
    };
    if !(loopdev_debug_mask & (1 as libc::c_int) << 1 as libc::c_int != 0) {
        if 0 as libc::c_int == 0 && !envstr.is_null() {
            loopdev_debug_mask = ul_debug_parse_mask(loopdev_masknames.as_ptr(), envstr);
        } else {
            loopdev_debug_mask = 0 as libc::c_int;
        }
    }
    if loopdev_debug_mask != 0 {
        if getuid() != geteuid() || getgid() != getegid() {
            loopdev_debug_mask |= (1 as libc::c_int) << 24 as libc::c_int;
            fprintf(
                stderr,
                b"%d: %s: don't print memory addresses (SUID executable).\n\0"
                    as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    loopdev_debug_mask |= (1 as libc::c_int) << 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_set_device(
    mut lc: *mut loopdev_cxt,
    mut device: *const libc::c_char,
) -> libc::c_int {
    if lc.is_null() {
        return -(22 as libc::c_int);
    }
    if (*lc).fd >= 0 as libc::c_int {
        close((*lc).fd);
        if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"closing old open fd\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    (*lc).fd = -(1 as libc::c_int);
    (*lc).mode = 0 as libc::c_int;
    (*lc).blocksize = 0 as libc::c_int as uint64_t;
    (*lc).set_has_info(0 as libc::c_int as libc::c_uint);
    (*lc).set_info_failed(0 as libc::c_int as libc::c_uint);
    *((*lc).device).as_mut_ptr() = '\0' as i32 as libc::c_char;
    memset(
        &mut (*lc).config as *mut loop_config as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<loop_config>() as libc::c_ulong,
    );
    if !device.is_null() {
        if *device as libc::c_int != '/' as i32 {
            let mut dir: *const libc::c_char = b"/dev/\0" as *const u8
                as *const libc::c_char;
            if (*lc).flags & LOOPDEV_FL_DEVSUBDIR as libc::c_int != 0 {
                if strlen(device) < 5 as libc::c_int as libc::c_ulong {
                    return -(1 as libc::c_int);
                }
                device = device.offset(4 as libc::c_int as isize);
                dir = b"/dev/loop/\0" as *const u8 as *const libc::c_char;
            }
            snprintf(
                ((*lc).device).as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                b"%s%s\0" as *const u8 as *const libc::c_char,
                dir,
                device,
            );
        } else {
            xstrncpy(
                ((*lc).device).as_mut_ptr(),
                device,
                ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            );
        }
        if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"%s name assigned\0" as *const u8 as *const libc::c_char,
                device,
            );
        }
    }
    ul_unref_path((*lc).sysfs);
    (*lc).sysfs = 0 as *mut path_cxt;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_has_device(mut lc: *mut loopdev_cxt) -> libc::c_int {
    return (!lc.is_null() && *((*lc).device).as_mut_ptr() as libc::c_int != 0)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_init(
    mut lc: *mut loopdev_cxt,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
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
    let mut dummy: loopdev_cxt = {
        let mut init = loopdev_cxt {
            has_info_extra_check_info_failed_control_ok: [0; 1],
            c2rust_padding: [0; 3],
            device: [0; 128],
            filename: 0 as *mut libc::c_char,
            fd: -(1 as libc::c_int),
            mode: 0,
            blocksize: 0,
            flags: 0,
            sysfs: 0 as *mut path_cxt,
            config: loop_config {
                fd: 0,
                block_size: 0,
                info: loop_info64 {
                    lo_device: 0,
                    lo_inode: 0,
                    lo_rdevice: 0,
                    lo_offset: 0,
                    lo_sizelimit: 0,
                    lo_number: 0,
                    lo_encrypt_type: 0,
                    lo_encrypt_key_size: 0,
                    lo_flags: 0,
                    lo_file_name: [0; 64],
                    lo_crypt_name: [0; 64],
                    lo_encrypt_key: [0; 32],
                    lo_init: [0; 2],
                },
                __reserved: [0; 8],
            },
            iter: loopdev_iter {
                proc_0: 0 as *mut FILE,
                sysblock: 0 as *mut DIR,
                ncur: 0,
                minors: 0 as *mut libc::c_int,
                nminors: 0,
                ct_perm: 0,
                ct_succ: 0,
                done_default_check: [0; 1],
                c2rust_padding: [0; 3],
                flags: 0,
            },
        };
        init.set_has_info(0);
        init.set_extra_check(0);
        init.set_info_failed(0);
        init.set_control_ok(0);
        init
    };
    if lc.is_null() {
        return -(22 as libc::c_int);
    }
    loopdev_init_debug();
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"initialize context\0" as *const u8 as *const libc::c_char,
        );
    }
    memcpy(
        lc as *mut libc::c_void,
        &mut dummy as *mut loopdev_cxt as *const libc::c_void,
        ::core::mem::size_of::<loopdev_cxt>() as libc::c_ulong,
    );
    (*lc).flags = flags;
    rc = loopcxt_set_device(lc, 0 as *const libc::c_char);
    if rc != 0 {
        return rc;
    }
    if stat(b"/sys/block\0" as *const u8 as *const libc::c_char, &mut st) != 0
        || !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint)
    {
        (*lc).flags |= LOOPDEV_FL_NOSYSFS as libc::c_int;
        (*lc).flags &= !(LOOPDEV_FL_NOIOCTL as libc::c_int);
        if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"init: disable /sys usage\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if (*lc).flags & LOOPDEV_FL_NOSYSFS as libc::c_int == 0
        && get_linux_version()
            >= ((2 as libc::c_int) << 16 as libc::c_int)
                + ((6 as libc::c_int) << 8 as libc::c_int)
                + (if 37 as libc::c_int > 255 as libc::c_int {
                    255 as libc::c_int
                } else {
                    37 as libc::c_int
                })
    {
        (*lc).flags |= LOOPDEV_FL_NOIOCTL as libc::c_int;
        if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"init: ignore ioctls\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if (*lc).flags & LOOPDEV_FL_CONTROL as libc::c_int == 0
        && stat(b"/dev/loop-control\0" as *const u8 as *const libc::c_char, &mut st) == 0
    {
        (*lc).flags |= LOOPDEV_FL_CONTROL as libc::c_int;
        if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"init: loop-control detected \0" as *const u8 as *const libc::c_char,
            );
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_deinit(mut lc: *mut loopdev_cxt) {
    let mut errsv: libc::c_int = *__errno_location();
    if lc.is_null() {
        return;
    }
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"de-initialize\0" as *const u8 as *const libc::c_char,
        );
    }
    free((*lc).filename as *mut libc::c_void);
    (*lc).filename = 0 as *mut libc::c_char;
    let mut __dummy: libc::c_int = loopcxt_set_device(lc, 0 as *const libc::c_char);
    loopcxt_deinit_iterator(lc);
    *__errno_location() = errsv;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_strdup_device(
    mut lc: *mut loopdev_cxt,
) -> *mut libc::c_char {
    if lc.is_null() || *((*lc).device).as_mut_ptr() == 0 {
        return 0 as *mut libc::c_char;
    }
    return strdup(((*lc).device).as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_get_device(
    mut lc: *mut loopdev_cxt,
) -> *const libc::c_char {
    return if !lc.is_null() && *((*lc).device).as_mut_ptr() as libc::c_int != 0 {
        ((*lc).device).as_mut_ptr()
    } else {
        0 as *mut libc::c_char
    };
}
unsafe extern "C" fn loopcxt_get_sysfs(mut lc: *mut loopdev_cxt) -> *mut path_cxt {
    if lc.is_null() || *((*lc).device).as_mut_ptr() == 0
        || (*lc).flags & LOOPDEV_FL_NOSYSFS as libc::c_int != 0
    {
        return 0 as *mut path_cxt;
    }
    if ((*lc).sysfs).is_null() {
        let mut devno: dev_t = sysfs_devname_to_devno(((*lc).device).as_mut_ptr());
        if devno == 0 {
            if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
                fprintf(
                    stderr,
                    b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                    getpid(),
                    b"loopdev\0" as *const u8 as *const libc::c_char,
                    b"CXT\0" as *const u8 as *const libc::c_char,
                );
                ul_debugobj(
                    lc as *const libc::c_void,
                    b"sysfs: failed devname to devno\0" as *const u8
                        as *const libc::c_char,
                );
            }
            return 0 as *mut path_cxt;
        }
        (*lc)
            .sysfs = ul_new_sysfs_path(
            devno,
            0 as *mut path_cxt,
            0 as *const libc::c_char,
        );
        if ((*lc).sysfs).is_null() {
            if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
                fprintf(
                    stderr,
                    b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                    getpid(),
                    b"loopdev\0" as *const u8 as *const libc::c_char,
                    b"CXT\0" as *const u8 as *const libc::c_char,
                );
                ul_debugobj(
                    lc as *const libc::c_void,
                    b"sysfs: init failed\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    return (*lc).sysfs;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_get_fd(mut lc: *mut loopdev_cxt) -> libc::c_int {
    if lc.is_null() || *((*lc).device).as_mut_ptr() == 0 {
        return -(22 as libc::c_int);
    }
    if (*lc).fd < 0 as libc::c_int {
        (*lc)
            .mode = if (*lc).flags & LOOPDEV_FL_RDWR as libc::c_int != 0 {
            0o2 as libc::c_int
        } else {
            0 as libc::c_int
        };
        (*lc)
            .fd = open(
            ((*lc).device).as_mut_ptr(),
            (*lc).mode | 0o2000000 as libc::c_int,
        );
        if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"open %s [%s]: %m\0" as *const u8 as *const libc::c_char,
                ((*lc).device).as_mut_ptr(),
                if (*lc).flags & LOOPDEV_FL_RDWR as libc::c_int != 0 {
                    b"rw\0" as *const u8 as *const libc::c_char
                } else {
                    b"ro\0" as *const u8 as *const libc::c_char
                },
            );
        }
    }
    return (*lc).fd;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_set_fd(
    mut lc: *mut loopdev_cxt,
    mut fd: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    if lc.is_null() {
        return -(22 as libc::c_int);
    }
    (*lc).fd = fd;
    (*lc).mode = mode;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_init_iterator(
    mut lc: *mut loopdev_cxt,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut iter: *mut loopdev_iter = 0 as *mut loopdev_iter;
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
    if lc.is_null() {
        return -(22 as libc::c_int);
    }
    iter = &mut (*lc).iter;
    if (1 as libc::c_int) << 3 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"ITER\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            iter as *const libc::c_void,
            b"initialize\0" as *const u8 as *const libc::c_char,
        );
    }
    memset(
        iter as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<loopdev_iter>() as libc::c_ulong,
    );
    (*iter).ncur = -(1 as libc::c_int);
    (*iter).flags = flags;
    (*iter).set_default_check(1 as libc::c_int as libc::c_uint);
    if (*lc).extra_check() == 0 {
        if (*lc).flags & LOOPDEV_FL_DEVSUBDIR as libc::c_int == 0
            && stat(b"/dev/loop\0" as *const u8 as *const libc::c_char, &mut st)
                == 0 as libc::c_int
            && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint
        {
            (*lc).flags |= LOOPDEV_FL_DEVSUBDIR as libc::c_int;
        }
        (*lc).set_extra_check(1 as libc::c_int as libc::c_uint);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_deinit_iterator(
    mut lc: *mut loopdev_cxt,
) -> libc::c_int {
    let mut iter: *mut loopdev_iter = 0 as *mut loopdev_iter;
    if lc.is_null() {
        return -(22 as libc::c_int);
    }
    iter = &mut (*lc).iter;
    if (1 as libc::c_int) << 3 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"ITER\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            iter as *const libc::c_void,
            b"de-initialize\0" as *const u8 as *const libc::c_char,
        );
    }
    free((*iter).minors as *mut libc::c_void);
    if !((*iter).proc_0).is_null() {
        fclose((*iter).proc_0);
    }
    if !((*iter).sysblock).is_null() {
        closedir((*iter).sysblock);
    }
    memset(
        iter as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<loopdev_iter>() as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn loopiter_set_device(
    mut lc: *mut loopdev_cxt,
    mut device: *const libc::c_char,
) -> libc::c_int {
    let mut rc: libc::c_int = loopcxt_set_device(lc, device);
    let mut used: libc::c_int = 0;
    if rc != 0 {
        return rc;
    }
    if (*lc).iter.flags & LOOPITER_FL_USED as libc::c_int == 0
        && (*lc).iter.flags & LOOPITER_FL_FREE as libc::c_int == 0
    {
        return 0 as libc::c_int;
    }
    if is_loopdev(((*lc).device).as_mut_ptr()) == 0 {
        if (1 as libc::c_int) << 3 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"ITER\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                &mut (*lc).iter as *mut loopdev_iter as *const libc::c_void,
                b"%s does not exist\0" as *const u8 as *const libc::c_char,
                ((*lc).device).as_mut_ptr(),
            );
        }
        return -*__errno_location();
    }
    if (1 as libc::c_int) << 3 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"ITER\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            &mut (*lc).iter as *mut loopdev_iter as *const libc::c_void,
            b"%s exist\0" as *const u8 as *const libc::c_char,
            ((*lc).device).as_mut_ptr(),
        );
    }
    used = (loopcxt_get_offset(lc, 0 as *mut uint64_t) == 0 as libc::c_int)
        as libc::c_int;
    if (*lc).iter.flags & LOOPITER_FL_USED as libc::c_int != 0 && used != 0 {
        return 0 as libc::c_int;
    }
    if (*lc).iter.flags & LOOPITER_FL_FREE as libc::c_int != 0 && used == 0 {
        return 0 as libc::c_int;
    }
    if (1 as libc::c_int) << 3 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"ITER\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            &mut (*lc).iter as *mut loopdev_iter as *const libc::c_void,
            b"failed to use %s device\0" as *const u8 as *const libc::c_char,
            ((*lc).device).as_mut_ptr(),
        );
    }
    let mut __dummy: libc::c_int = loopcxt_set_device(lc, 0 as *const libc::c_char);
    return 1 as libc::c_int;
}
unsafe extern "C" fn cmpnum(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    return (*(p1 as *const libc::c_int) > *(p2 as *const libc::c_int)) as libc::c_int
        - (*(p1 as *const libc::c_int) < *(p2 as *const libc::c_int)) as libc::c_int;
}
unsafe extern "C" fn loop_scandir(
    mut dirname: *const libc::c_char,
    mut ary: *mut *mut libc::c_int,
    mut hasprefix: libc::c_int,
) -> libc::c_int {
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut d: *mut dirent = 0 as *mut dirent;
    let mut n: libc::c_uint = 0;
    let mut count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut arylen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if dirname.is_null() || ary.is_null() {
        return 0 as libc::c_int;
    }
    if (1 as libc::c_int) << 3 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"ITER\0" as *const u8 as *const libc::c_char,
        );
        ul_debug(b"scan dir: %s\0" as *const u8 as *const libc::c_char, dirname);
    }
    dir = opendir(dirname);
    if dir.is_null() {
        return 0 as libc::c_int;
    }
    free(*ary as *mut libc::c_void);
    *ary = 0 as *mut libc::c_int;
    loop {
        d = readdir(dir);
        if d.is_null() {
            break;
        }
        if (*d).d_type as libc::c_int != DT_BLK as libc::c_int
            && (*d).d_type as libc::c_int != DT_UNKNOWN as libc::c_int
            && (*d).d_type as libc::c_int != DT_LNK as libc::c_int
        {
            continue;
        }
        if strcmp(((*d).d_name).as_mut_ptr(), b".\0" as *const u8 as *const libc::c_char)
            == 0
            || strcmp(
                ((*d).d_name).as_mut_ptr(),
                b"..\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            continue;
        }
        if hasprefix != 0 {
            if sscanf(
                ((*d).d_name).as_mut_ptr(),
                b"loop%u\0" as *const u8 as *const libc::c_char,
                &mut n as *mut libc::c_uint,
            ) != 1 as libc::c_int
            {
                continue;
            }
        } else {
            let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
            *__errno_location() = 0 as libc::c_int;
            n = strtol(((*d).d_name).as_mut_ptr(), &mut end, 10 as libc::c_int)
                as libc::c_uint;
            if ((*d).d_name).as_mut_ptr() == end
                || !end.is_null() && *end as libc::c_int != 0 || *__errno_location() != 0
            {
                continue;
            }
        }
        if n < 8 as libc::c_int as libc::c_uint {
            continue;
        }
        if count.wrapping_add(1 as libc::c_int as libc::c_uint) > arylen {
            let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
            arylen = arylen.wrapping_add(1 as libc::c_int as libc::c_uint);
            tmp = realloc(
                *ary as *mut libc::c_void,
                (arylen as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            ) as *mut libc::c_int;
            if tmp.is_null() {
                free(*ary as *mut libc::c_void);
                *ary = 0 as *mut libc::c_int;
                closedir(dir);
                return -(1 as libc::c_int);
            }
            *ary = tmp;
        }
        if !(*ary).is_null() {
            let fresh0 = count;
            count = count.wrapping_add(1);
            *(*ary).offset(fresh0 as isize) = n as libc::c_int;
        }
    }
    if count != 0 && !(*ary).is_null() {
        qsort(
            *ary as *mut libc::c_void,
            count as size_t,
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
            Some(
                cmpnum
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
    }
    closedir(dir);
    return count as libc::c_int;
}
unsafe extern "C" fn loopcxt_next_from_proc(mut lc: *mut loopdev_cxt) -> libc::c_int {
    let mut iter: *mut loopdev_iter = &mut (*lc).iter;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    if (1 as libc::c_int) << 3 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"ITER\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            iter as *const libc::c_void,
            b"scan /proc/partitions\0" as *const u8 as *const libc::c_char,
        );
    }
    if ((*iter).proc_0).is_null() {
        (*iter)
            .proc_0 = fopen(
            b"/proc/partitions\0" as *const u8 as *const libc::c_char,
            b"re\0" as *const u8 as *const libc::c_char,
        );
    }
    if ((*iter).proc_0).is_null() {
        return 1 as libc::c_int;
    }
    while !(fgets(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as libc::c_int,
        (*iter).proc_0,
    ))
        .is_null()
    {
        let mut m: libc::c_uint = 0;
        let mut name: [libc::c_char; 129] = [0; 129];
        if sscanf(
            buf.as_mut_ptr(),
            b" %u %*s %*s %128[^\n ]\0" as *const u8 as *const libc::c_char,
            &mut m as *mut libc::c_uint,
            name.as_mut_ptr(),
        ) != 2 as libc::c_int || m != 7 as libc::c_int as libc::c_uint
        {
            continue;
        }
        if (1 as libc::c_int) << 3 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"ITER\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                iter as *const libc::c_void,
                b"checking %s\0" as *const u8 as *const libc::c_char,
                name.as_mut_ptr(),
            );
        }
        if loopiter_set_device(lc, name.as_mut_ptr()) == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn loopcxt_next_from_sysfs(mut lc: *mut loopdev_cxt) -> libc::c_int {
    let mut iter: *mut loopdev_iter = &mut (*lc).iter;
    let mut d: *mut dirent = 0 as *mut dirent;
    let mut fd: libc::c_int = 0;
    if (1 as libc::c_int) << 3 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"ITER\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            iter as *const libc::c_void,
            b"scanning /sys/block\0" as *const u8 as *const libc::c_char,
        );
    }
    if ((*iter).sysblock).is_null() {
        (*iter).sysblock = opendir(b"/sys/block\0" as *const u8 as *const libc::c_char);
    }
    if ((*iter).sysblock).is_null() {
        return 1 as libc::c_int;
    }
    fd = dirfd((*iter).sysblock);
    loop {
        d = readdir((*iter).sysblock);
        if d.is_null() {
            break;
        }
        let mut name: [libc::c_char; 274] = [0; 274];
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
        if (1 as libc::c_int) << 3 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"ITER\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                iter as *const libc::c_void,
                b"check %s\0" as *const u8 as *const libc::c_char,
                ((*d).d_name).as_mut_ptr(),
            );
        }
        if strcmp(((*d).d_name).as_mut_ptr(), b".\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(
                ((*d).d_name).as_mut_ptr(),
                b"..\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            || strncmp(
                ((*d).d_name).as_mut_ptr(),
                b"loop\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) != 0 as libc::c_int
        {
            continue;
        }
        snprintf(
            name.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 274]>() as libc::c_ulong,
            b"%s/loop/backing_file\0" as *const u8 as *const libc::c_char,
            ((*d).d_name).as_mut_ptr(),
        );
        if fstatat(fd, name.as_mut_ptr(), &mut st, 0 as libc::c_int) != 0 as libc::c_int
        {
            continue;
        }
        if loopiter_set_device(lc, ((*d).d_name).as_mut_ptr()) == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_next(mut lc: *mut loopdev_cxt) -> libc::c_int {
    let mut iter: *mut loopdev_iter = 0 as *mut loopdev_iter;
    if lc.is_null() {
        return -(22 as libc::c_int);
    }
    iter = &mut (*lc).iter;
    if (*iter).done() != 0 {
        return 1 as libc::c_int;
    }
    if (1 as libc::c_int) << 3 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"ITER\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            iter as *const libc::c_void,
            b"next\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*iter).flags & LOOPITER_FL_USED as libc::c_int != 0 {
        let mut rc: libc::c_int = 0;
        if (*lc).flags & LOOPDEV_FL_NOSYSFS as libc::c_int == 0
            && (*lc).flags & LOOPDEV_FL_NOIOCTL as libc::c_int != 0
        {
            rc = loopcxt_next_from_sysfs(lc);
        } else {
            rc = loopcxt_next_from_proc(lc);
        }
        if rc == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    } else {
        if (*iter).default_check() != 0 {
            if (1 as libc::c_int) << 3 as libc::c_int & loopdev_debug_mask != 0 {
                fprintf(
                    stderr,
                    b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                    getpid(),
                    b"loopdev\0" as *const u8 as *const libc::c_char,
                    b"ITER\0" as *const u8 as *const libc::c_char,
                );
                ul_debugobj(
                    iter as *const libc::c_void,
                    b"next: default check\0" as *const u8 as *const libc::c_char,
                );
            }
            (*iter).ncur += 1;
            (*iter).ncur;
            while (*iter).ncur < 8 as libc::c_int {
                let mut name: [libc::c_char; 16] = [0; 16];
                snprintf(
                    name.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                    b"loop%d\0" as *const u8 as *const libc::c_char,
                    (*iter).ncur,
                );
                if loopiter_set_device(lc, name.as_mut_ptr()) == 0 as libc::c_int {
                    return 0 as libc::c_int;
                }
                (*iter).ncur += 1;
                (*iter).ncur;
            }
            (*iter).set_default_check(0 as libc::c_int as libc::c_uint);
        }
        if ((*iter).minors).is_null() {
            if (1 as libc::c_int) << 3 as libc::c_int & loopdev_debug_mask != 0 {
                fprintf(
                    stderr,
                    b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                    getpid(),
                    b"loopdev\0" as *const u8 as *const libc::c_char,
                    b"ITER\0" as *const u8 as *const libc::c_char,
                );
                ul_debugobj(
                    iter as *const libc::c_void,
                    b"next: scanning /dev\0" as *const u8 as *const libc::c_char,
                );
            }
            (*iter)
                .nminors = if (*lc).flags & LOOPDEV_FL_DEVSUBDIR as libc::c_int != 0 {
                loop_scandir(
                    b"/dev/loop\0" as *const u8 as *const libc::c_char,
                    &mut (*iter).minors,
                    0 as libc::c_int,
                )
            } else {
                loop_scandir(
                    b"/dev/\0" as *const u8 as *const libc::c_char,
                    &mut (*iter).minors,
                    1 as libc::c_int,
                )
            };
            (*iter).ncur = -(1 as libc::c_int);
        }
        (*iter).ncur += 1;
        (*iter).ncur;
        while (*iter).ncur < (*iter).nminors {
            let mut name_0: [libc::c_char; 16] = [0; 16];
            snprintf(
                name_0.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                b"loop%d\0" as *const u8 as *const libc::c_char,
                *((*iter).minors).offset((*iter).ncur as isize),
            );
            if loopiter_set_device(lc, name_0.as_mut_ptr()) == 0 as libc::c_int {
                return 0 as libc::c_int;
            }
            (*iter).ncur += 1;
            (*iter).ncur;
        }
    }
    loopcxt_deinit_iterator(lc);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn is_loopdev(mut device: *const libc::c_char) -> libc::c_int {
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
    let mut rc: libc::c_int = 0 as libc::c_int;
    if device.is_null() || stat(device, &mut st) != 0 as libc::c_int
        || !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o60000 as libc::c_int as libc::c_uint)
    {
        rc = 0 as libc::c_int;
    } else if gnu_dev_major(st.st_rdev) == 7 as libc::c_int as libc::c_uint {
        rc = 1 as libc::c_int;
    } else if sysfs_devno_is_wholedisk(st.st_rdev) != 0 {
        let mut name: [libc::c_char; 4096] = [0; 4096];
        let mut cn: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        snprintf(
            name.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
            b"/sys/dev/block/%d:%d\0" as *const u8 as *const libc::c_char,
            gnu_dev_major(st.st_rdev),
            gnu_dev_minor(st.st_rdev),
        );
        cn = canonicalize_path(name.as_mut_ptr());
        if !cn.is_null() {
            p = stripoff_last_component(cn);
        }
        rc = (!p.is_null()
            && !(startswith(p, b"loop\0" as *const u8 as *const libc::c_char)).is_null())
            as libc::c_int;
        free(cn as *mut libc::c_void);
    }
    if rc == 0 as libc::c_int {
        *__errno_location() = 19 as libc::c_int;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_get_info(mut lc: *mut loopdev_cxt) -> *mut loop_info64 {
    let mut fd: libc::c_int = 0;
    if lc.is_null() || (*lc).info_failed() as libc::c_int != 0 {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut loop_info64;
    }
    *__errno_location() = 0 as libc::c_int;
    if (*lc).has_info() != 0 {
        return &mut (*lc).config.info;
    }
    fd = loopcxt_get_fd(lc);
    if fd < 0 as libc::c_int {
        return 0 as *mut loop_info64;
    }
    if ioctl(
        fd,
        0x4c05 as libc::c_int as libc::c_ulong,
        &mut (*lc).config.info as *mut loop_info64,
    ) == 0 as libc::c_int
    {
        (*lc).set_has_info(1 as libc::c_int as libc::c_uint);
        (*lc).set_info_failed(0 as libc::c_int as libc::c_uint);
        if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"reading loop_info64 OK\0" as *const u8 as *const libc::c_char,
            );
        }
        return &mut (*lc).config.info;
    }
    (*lc).set_info_failed(1 as libc::c_int as libc::c_uint);
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"reading loop_info64 FAILED\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as *mut loop_info64;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_get_backing_file(
    mut lc: *mut loopdev_cxt,
) -> *mut libc::c_char {
    let mut sysfs: *mut path_cxt = loopcxt_get_sysfs(lc);
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    if !sysfs.is_null() {
        ul_path_read_string(
            sysfs,
            &mut res,
            b"loop/backing_file\0" as *const u8 as *const libc::c_char,
        );
    }
    if res.is_null() && (*lc).flags & LOOPDEV_FL_NOIOCTL as libc::c_int == 0 {
        let mut lo: *mut loop_info64 = loopcxt_get_info(lc);
        if !lo.is_null() {
            (*lo)
                .lo_file_name[(64 as libc::c_int - 2 as libc::c_int)
                as usize] = '*' as i32 as uint8_t;
            (*lo)
                .lo_file_name[(64 as libc::c_int - 1 as libc::c_int)
                as usize] = '\0' as i32 as uint8_t;
            res = strdup(((*lo).lo_file_name).as_mut_ptr() as *mut libc::c_char);
        }
    }
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"get_backing_file [%s]\0" as *const u8 as *const libc::c_char,
            res,
        );
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_get_refname(
    mut lc: *mut loopdev_cxt,
) -> *mut libc::c_char {
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lo: *mut loop_info64 = loopcxt_get_info(lc);
    if !lo.is_null() {
        (*lo)
            .lo_file_name[(64 as libc::c_int - 1 as libc::c_int)
            as usize] = '\0' as i32 as uint8_t;
        res = strdup(((*lo).lo_file_name).as_mut_ptr() as *mut libc::c_char);
    }
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"get_refname [%s]\0" as *const u8 as *const libc::c_char,
            res,
        );
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_get_offset(
    mut lc: *mut loopdev_cxt,
    mut offset: *mut uint64_t,
) -> libc::c_int {
    let mut sysfs: *mut path_cxt = loopcxt_get_sysfs(lc);
    let mut rc: libc::c_int = -(22 as libc::c_int);
    if !sysfs.is_null() {
        if ul_path_read_u64(
            sysfs,
            offset,
            b"loop/offset\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            rc = 0 as libc::c_int;
        }
    }
    if rc != 0 && (*lc).flags & LOOPDEV_FL_NOIOCTL as libc::c_int == 0 {
        let mut lo: *mut loop_info64 = loopcxt_get_info(lc);
        if !lo.is_null() {
            if !offset.is_null() {
                *offset = (*lo).lo_offset;
            }
            rc = 0 as libc::c_int;
        } else {
            rc = -*__errno_location();
        }
    }
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"get_offset [rc=%d]\0" as *const u8 as *const libc::c_char,
            rc,
        );
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_get_blocksize(
    mut lc: *mut loopdev_cxt,
    mut blocksize: *mut uint64_t,
) -> libc::c_int {
    let mut sysfs: *mut path_cxt = loopcxt_get_sysfs(lc);
    let mut rc: libc::c_int = -(22 as libc::c_int);
    if !sysfs.is_null() {
        if ul_path_read_u64(
            sysfs,
            blocksize,
            b"queue/logical_block_size\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            rc = 0 as libc::c_int;
        }
    }
    if rc != 0 {
        let mut fd: libc::c_int = loopcxt_get_fd(lc);
        let mut sz: libc::c_int = 0 as libc::c_int;
        if fd < 0 as libc::c_int {
            return -(22 as libc::c_int);
        }
        rc = blkdev_get_sector_size(fd, &mut sz);
        if rc != 0 {
            return rc;
        }
        *blocksize = sz as uint64_t;
    }
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"get_blocksize [rc=%d]\0" as *const u8 as *const libc::c_char,
            rc,
        );
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_get_sizelimit(
    mut lc: *mut loopdev_cxt,
    mut size: *mut uint64_t,
) -> libc::c_int {
    let mut sysfs: *mut path_cxt = loopcxt_get_sysfs(lc);
    let mut rc: libc::c_int = -(22 as libc::c_int);
    if !sysfs.is_null() {
        if ul_path_read_u64(
            sysfs,
            size,
            b"loop/sizelimit\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            rc = 0 as libc::c_int;
        }
    }
    if rc != 0 && (*lc).flags & LOOPDEV_FL_NOIOCTL as libc::c_int == 0 {
        let mut lo: *mut loop_info64 = loopcxt_get_info(lc);
        if !lo.is_null() {
            if !size.is_null() {
                *size = (*lo).lo_sizelimit;
            }
            rc = 0 as libc::c_int;
        } else {
            rc = -*__errno_location();
        }
    }
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"get_sizelimit [rc=%d]\0" as *const u8 as *const libc::c_char,
            rc,
        );
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_get_encrypt_type(
    mut lc: *mut loopdev_cxt,
    mut type_0: *mut uint32_t,
) -> libc::c_int {
    let mut lo: *mut loop_info64 = loopcxt_get_info(lc);
    let mut rc: libc::c_int = 0;
    if !lo.is_null() {
        if !type_0.is_null() {
            *type_0 = (*lo).lo_encrypt_type;
        }
        rc = 0 as libc::c_int;
    } else {
        rc = -*__errno_location();
    }
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"get_encrypt_type [rc=%d]\0" as *const u8 as *const libc::c_char,
            rc,
        );
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_get_crypt_name(
    mut lc: *mut loopdev_cxt,
) -> *const libc::c_char {
    let mut lo: *mut loop_info64 = loopcxt_get_info(lc);
    if !lo.is_null() {
        return ((*lo).lo_crypt_name).as_mut_ptr() as *mut libc::c_char;
    }
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"get_crypt_name failed\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_get_backing_devno(
    mut lc: *mut loopdev_cxt,
    mut devno: *mut dev_t,
) -> libc::c_int {
    let mut lo: *mut loop_info64 = loopcxt_get_info(lc);
    let mut rc: libc::c_int = 0;
    if !lo.is_null() {
        if !devno.is_null() {
            *devno = (*lo).lo_device;
        }
        rc = 0 as libc::c_int;
    } else {
        rc = -*__errno_location();
    }
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"get_backing_devno [rc=%d]\0" as *const u8 as *const libc::c_char,
            rc,
        );
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_get_backing_inode(
    mut lc: *mut loopdev_cxt,
    mut ino: *mut ino_t,
) -> libc::c_int {
    let mut lo: *mut loop_info64 = loopcxt_get_info(lc);
    let mut rc: libc::c_int = 0;
    if !lo.is_null() {
        if !ino.is_null() {
            *ino = (*lo).lo_inode;
        }
        rc = 0 as libc::c_int;
    } else {
        rc = -*__errno_location();
    }
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"get_backing_inode [rc=%d]\0" as *const u8 as *const libc::c_char,
            rc,
        );
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn loopmod_supports_partscan() -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut f: *mut FILE = 0 as *mut FILE;
    if get_linux_version()
        >= ((3 as libc::c_int) << 16 as libc::c_int)
            + ((2 as libc::c_int) << 8 as libc::c_int)
            + (if 0 as libc::c_int > 255 as libc::c_int {
                255 as libc::c_int
            } else {
                0 as libc::c_int
            })
    {
        return 1 as libc::c_int;
    }
    f = fopen(
        b"/sys/module/loop/parameters/max_part\0" as *const u8 as *const libc::c_char,
        b"re\0" as *const u8 as *const libc::c_char,
    );
    if f.is_null() {
        return 0 as libc::c_int;
    }
    rc = fscanf(
        f,
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut ret as *mut libc::c_int,
    );
    fclose(f);
    return if rc == 1 as libc::c_int { ret } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_is_partscan(mut lc: *mut loopdev_cxt) -> libc::c_int {
    let mut sysfs: *mut path_cxt = loopcxt_get_sysfs(lc);
    if !sysfs.is_null() {
        let mut fl: libc::c_int = 0;
        if ul_path_read_s32(
            sysfs,
            &mut fl,
            b"loop/partscan\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            return fl;
        }
    }
    return loopmod_supports_partscan();
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_is_autoclear(mut lc: *mut loopdev_cxt) -> libc::c_int {
    let mut sysfs: *mut path_cxt = loopcxt_get_sysfs(lc);
    if !sysfs.is_null() {
        let mut fl: libc::c_int = 0;
        if ul_path_read_s32(
            sysfs,
            &mut fl,
            b"loop/autoclear\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            return fl;
        }
    }
    if (*lc).flags & LOOPDEV_FL_NOIOCTL as libc::c_int == 0 {
        let mut lo: *mut loop_info64 = loopcxt_get_info(lc);
        if !lo.is_null() {
            return ((*lo).lo_flags & LO_FLAGS_AUTOCLEAR as libc::c_int as libc::c_uint)
                as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_is_readonly(mut lc: *mut loopdev_cxt) -> libc::c_int {
    let mut sysfs: *mut path_cxt = loopcxt_get_sysfs(lc);
    if !sysfs.is_null() {
        let mut fl: libc::c_int = 0;
        if ul_path_read_s32(sysfs, &mut fl, b"ro\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            return fl;
        }
    }
    if (*lc).flags & LOOPDEV_FL_NOIOCTL as libc::c_int == 0 {
        let mut lo: *mut loop_info64 = loopcxt_get_info(lc);
        if !lo.is_null() {
            return ((*lo).lo_flags & LO_FLAGS_READ_ONLY as libc::c_int as libc::c_uint)
                as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_is_dio(mut lc: *mut loopdev_cxt) -> libc::c_int {
    let mut sysfs: *mut path_cxt = loopcxt_get_sysfs(lc);
    if !sysfs.is_null() {
        let mut fl: libc::c_int = 0;
        if ul_path_read_s32(
            sysfs,
            &mut fl,
            b"loop/dio\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            return fl;
        }
    }
    if (*lc).flags & LOOPDEV_FL_NOIOCTL as libc::c_int == 0 {
        let mut lo: *mut loop_info64 = loopcxt_get_info(lc);
        if !lo.is_null() {
            return ((*lo).lo_flags & LO_FLAGS_DIRECT_IO as libc::c_int as libc::c_uint)
                as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_is_used(
    mut lc: *mut loopdev_cxt,
    mut st: *mut stat,
    mut backing_file: *const libc::c_char,
    mut offset: uint64_t,
    mut sizelimit: uint64_t,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ino: ino_t = 0 as libc::c_int as ino_t;
    let mut dev: dev_t = 0 as libc::c_int as dev_t;
    if lc.is_null() {
        return 0 as libc::c_int;
    }
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"checking %s vs. %s\0" as *const u8 as *const libc::c_char,
            loopcxt_get_device(lc),
            backing_file,
        );
    }
    if !st.is_null() && loopcxt_get_backing_inode(lc, &mut ino) == 0 as libc::c_int
        && loopcxt_get_backing_devno(lc, &mut dev) == 0 as libc::c_int
    {
        if !(ino == (*st).st_ino && dev == (*st).st_dev) {
            return 0 as libc::c_int;
        }
    } else {
        if !backing_file.is_null() {
            let mut name: *mut libc::c_char = loopcxt_get_backing_file(lc);
            let mut rc: libc::c_int = (!name.is_null()
                && strcmp(name, backing_file) == 0 as libc::c_int) as libc::c_int;
            free(name as *mut libc::c_void);
            if rc != 0 {
                current_block = 12403753975413079746;
            } else {
                current_block = 12800627514080957624;
            }
        } else {
            current_block = 12800627514080957624;
        }
        match current_block {
            12403753975413079746 => {}
            _ => return 0 as libc::c_int,
        }
    }
    if flags & LOOPDEV_FL_OFFSET as libc::c_int != 0 {
        let mut off: uint64_t = 0 as libc::c_int as uint64_t;
        let mut rc_0: libc::c_int = (loopcxt_get_offset(lc, &mut off) == 0 as libc::c_int
            && off == offset) as libc::c_int;
        if rc_0 != 0 && flags & LOOPDEV_FL_SIZELIMIT as libc::c_int != 0 {
            let mut sz: uint64_t = 0 as libc::c_int as uint64_t;
            return (loopcxt_get_sizelimit(lc, &mut sz) == 0 as libc::c_int
                && sz == sizelimit) as libc::c_int;
        }
        return rc_0;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_set_offset(
    mut lc: *mut loopdev_cxt,
    mut offset: uint64_t,
) -> libc::c_int {
    if lc.is_null() {
        return -(22 as libc::c_int);
    }
    (*lc).config.info.lo_offset = offset;
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"set offset=%jd\0" as *const u8 as *const libc::c_char,
            offset,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_set_sizelimit(
    mut lc: *mut loopdev_cxt,
    mut sizelimit: uint64_t,
) -> libc::c_int {
    if lc.is_null() {
        return -(22 as libc::c_int);
    }
    (*lc).config.info.lo_sizelimit = sizelimit;
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"set sizelimit=%jd\0" as *const u8 as *const libc::c_char,
            sizelimit,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_set_blocksize(
    mut lc: *mut loopdev_cxt,
    mut blocksize: uint64_t,
) -> libc::c_int {
    if lc.is_null() {
        return -(22 as libc::c_int);
    }
    (*lc).blocksize = blocksize;
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"set blocksize=%jd\0" as *const u8 as *const libc::c_char,
            blocksize,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_set_flags(
    mut lc: *mut loopdev_cxt,
    mut flags: uint32_t,
) -> libc::c_int {
    if lc.is_null() {
        return -(22 as libc::c_int);
    }
    (*lc).config.info.lo_flags = flags;
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"set flags=%u\0" as *const u8 as *const libc::c_char,
            flags,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_set_refname(
    mut lc: *mut loopdev_cxt,
    mut refname: *const libc::c_char,
) -> libc::c_int {
    if lc.is_null() {
        return -(22 as libc::c_int);
    }
    memset(
        ((*lc).config.info.lo_file_name).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong,
    );
    if !refname.is_null() {
        xstrncpy(
            ((*lc).config.info.lo_file_name).as_mut_ptr() as *mut libc::c_char,
            refname,
            64 as libc::c_int as size_t,
        );
    }
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"set refname=%s\0" as *const u8 as *const libc::c_char,
            ((*lc).config.info.lo_file_name).as_mut_ptr() as *mut libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_set_backing_file(
    mut lc: *mut loopdev_cxt,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    if lc.is_null() {
        return -(22 as libc::c_int);
    }
    (*lc).filename = canonicalize_path(filename);
    if ((*lc).filename).is_null() {
        return -*__errno_location();
    }
    if (*lc).config.info.lo_file_name[0 as libc::c_int as usize] == 0 {
        loopcxt_set_refname(lc, (*lc).filename);
    }
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"set backing file=%s\0" as *const u8 as *const libc::c_char,
            (*lc).filename,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn loopcxt_check_size(
    mut lc: *mut loopdev_cxt,
    mut file_fd: libc::c_int,
) -> libc::c_int {
    let mut size: uint64_t = 0;
    let mut expected_size: uint64_t = 0;
    let mut dev_fd: libc::c_int = 0;
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
    if (*lc).config.info.lo_offset == 0 && (*lc).config.info.lo_sizelimit == 0 {
        return 0 as libc::c_int;
    }
    if fstat(file_fd, &mut st) != 0 {
        if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"failed to fstat backing file\0" as *const u8 as *const libc::c_char,
            );
        }
        return -*__errno_location();
    }
    if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o60000 as libc::c_int as libc::c_uint
    {
        if blkdev_get_size(
            file_fd,
            &mut expected_size as *mut uint64_t as *mut libc::c_ulonglong,
        ) != 0
        {
            if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
                fprintf(
                    stderr,
                    b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                    getpid(),
                    b"loopdev\0" as *const u8 as *const libc::c_char,
                    b"CXT\0" as *const u8 as *const libc::c_char,
                );
                ul_debugobj(
                    lc as *const libc::c_void,
                    b"failed to determine device size\0" as *const u8
                        as *const libc::c_char,
                );
            }
            return -*__errno_location();
        }
    } else {
        expected_size = st.st_size as uint64_t;
    }
    if expected_size == 0 as libc::c_int as libc::c_ulong
        || expected_size <= (*lc).config.info.lo_offset
    {
        if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"failed to determine expected size\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    if (*lc).config.info.lo_offset > 0 as libc::c_int as libc::c_ulong {
        expected_size = (expected_size as libc::c_ulong)
            .wrapping_sub((*lc).config.info.lo_offset) as uint64_t as uint64_t;
    }
    if (*lc).config.info.lo_sizelimit > 0 as libc::c_int as libc::c_ulong
        && (*lc).config.info.lo_sizelimit < expected_size
    {
        expected_size = (*lc).config.info.lo_sizelimit;
    }
    dev_fd = loopcxt_get_fd(lc);
    if dev_fd < 0 as libc::c_int {
        if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"failed to get loop FD\0" as *const u8 as *const libc::c_char,
            );
        }
        return -*__errno_location();
    }
    if blkdev_get_size(dev_fd, &mut size as *mut uint64_t as *mut libc::c_ulonglong) != 0
    {
        if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"failed to determine loopdev size\0" as *const u8 as *const libc::c_char,
            );
        }
        return -*__errno_location();
    }
    if expected_size.wrapping_rem(512 as libc::c_int as libc::c_ulong) != 0 {
        if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"expected size misaligned to 512-byte sectors\0" as *const u8
                    as *const libc::c_char,
            );
        }
        expected_size = (expected_size >> 9 as libc::c_int) << 9 as libc::c_int;
    }
    if expected_size != size {
        if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"warning: loopdev and expected size mismatch (%ju/%ju)\0" as *const u8
                    as *const libc::c_char,
                size,
                expected_size,
            );
        }
        if loopcxt_ioctl_capacity(lc) != 0 {
            if *__errno_location() == 25 as libc::c_int
                || *__errno_location() == 22 as libc::c_int
            {
                *__errno_location() = 34 as libc::c_int;
            }
            return -*__errno_location();
        }
        if blkdev_get_size(dev_fd, &mut size as *mut uint64_t as *mut libc::c_ulonglong)
            != 0
        {
            return -*__errno_location();
        }
        if expected_size != size {
            *__errno_location() = 34 as libc::c_int;
            if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
                fprintf(
                    stderr,
                    b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                    getpid(),
                    b"loopdev\0" as *const u8 as *const libc::c_char,
                    b"CXT\0" as *const u8 as *const libc::c_char,
                );
                ul_debugobj(
                    lc as *const libc::c_void,
                    b"failed to set loopdev size, size: %ju, expected: %ju\0"
                        as *const u8 as *const libc::c_char,
                    size,
                    expected_size,
                );
            }
            return -*__errno_location();
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_setup_device(mut lc: *mut loopdev_cxt) -> libc::c_int {
    let mut current_block: u64;
    let mut file_fd: libc::c_int = 0;
    let mut dev_fd: libc::c_int = 0;
    let mut mode: libc::c_int = 0o2 as libc::c_int;
    let mut flags: libc::c_int = 0o2000000 as libc::c_int;
    let mut rc: libc::c_int = -(1 as libc::c_int);
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut errsv: libc::c_int = 0 as libc::c_int;
    let mut fallback: libc::c_int = 0 as libc::c_int;
    if lc.is_null() || *((*lc).device).as_mut_ptr() == 0 || ((*lc).filename).is_null() {
        return -(22 as libc::c_int);
    }
    if (1 as libc::c_int) << 4 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"SETUP\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"device setup requested\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*lc).config.info.lo_flags & LO_FLAGS_READ_ONLY as libc::c_int as libc::c_uint
        != 0
    {
        mode = 0 as libc::c_int;
    }
    if (*lc).config.info.lo_flags & LO_FLAGS_DIRECT_IO as libc::c_int as libc::c_uint
        != 0
    {
        flags |= 0o40000 as libc::c_int;
    }
    file_fd = open((*lc).filename, mode | flags);
    if file_fd < 0 as libc::c_int {
        if mode != 0 as libc::c_int
            && (*__errno_location() == 30 as libc::c_int
                || *__errno_location() == 13 as libc::c_int)
        {
            mode = 0 as libc::c_int;
            file_fd = open((*lc).filename, mode | flags);
        }
        if file_fd < 0 as libc::c_int {
            if (1 as libc::c_int) << 4 as libc::c_int & loopdev_debug_mask != 0 {
                fprintf(
                    stderr,
                    b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                    getpid(),
                    b"loopdev\0" as *const u8 as *const libc::c_char,
                    b"SETUP\0" as *const u8 as *const libc::c_char,
                );
                ul_debugobj(
                    lc as *const libc::c_void,
                    b"open backing file failed: %m\0" as *const u8 as *const libc::c_char,
                );
            }
            return -*__errno_location();
        }
    }
    if (1 as libc::c_int) << 4 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"SETUP\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"backing file open: OK\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*lc).fd != -(1 as libc::c_int) && (*lc).mode != mode {
        if (1 as libc::c_int) << 4 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"SETUP\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"closing already open device (mode mismatch)\0" as *const u8
                    as *const libc::c_char,
            );
        }
        close((*lc).fd);
        (*lc).fd = -(1 as libc::c_int);
        (*lc).mode = 0 as libc::c_int;
    }
    if mode == 0 as libc::c_int {
        (*lc).flags |= LOOPDEV_FL_RDONLY as libc::c_int;
        (*lc).config.info.lo_flags |= LO_FLAGS_READ_ONLY as libc::c_int as libc::c_uint;
    } else {
        (*lc).flags |= LOOPDEV_FL_RDWR as libc::c_int;
        (*lc).config.info.lo_flags
            &= !(LO_FLAGS_READ_ONLY as libc::c_int) as libc::c_uint;
        (*lc).flags &= !(LOOPDEV_FL_RDONLY as libc::c_int);
    }
    loop {
        *__errno_location() = 0 as libc::c_int;
        dev_fd = loopcxt_get_fd(lc);
        if dev_fd >= 0 as libc::c_int
            || (*lc).control_ok() as libc::c_int == 0 as libc::c_int
        {
            break;
        }
        if *__errno_location() != 13 as libc::c_int
            && *__errno_location() != 2 as libc::c_int
        {
            break;
        }
        xusleep(25000 as libc::c_int as useconds_t);
        let fresh1 = cnt;
        cnt = cnt + 1;
        if !(fresh1 < 16 as libc::c_int) {
            break;
        }
    }
    if dev_fd < 0 as libc::c_int {
        rc = -*__errno_location();
    } else {
        if (1 as libc::c_int) << 4 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"SETUP\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"device open: OK\0" as *const u8 as *const libc::c_char,
            );
        }
        (*lc).config.fd = file_fd as uint32_t;
        if (*lc).blocksize > 0 as libc::c_int as libc::c_ulong {
            (*lc).config.block_size = (*lc).blocksize as uint32_t;
        }
        rc = ({
            let mut _c: libc::c_int = 0 as libc::c_int;
            let mut _e: libc::c_int = 0;
            loop {
                *__errno_location() = 0 as libc::c_int;
                _e = ioctl(
                    dev_fd,
                    0x4c0a as libc::c_int as libc::c_ulong,
                    &mut (*lc).config as *mut loop_config,
                );
                if _e == 0 as libc::c_int || *__errno_location() != 11 as libc::c_int {
                    break;
                }
                if _c >= 10 as libc::c_int {
                    break;
                }
                xusleep(250000 as libc::c_int as useconds_t);
                _c += 1;
                _c;
            }
            if _e == 0 as libc::c_int {
                0 as libc::c_int
            } else if *__errno_location() != 0 {
                -*__errno_location()
            } else {
                -(1 as libc::c_int)
            }
        });
        if rc != 0 as libc::c_int {
            errsv = *__errno_location();
            if *__errno_location() != 22 as libc::c_int
                && *__errno_location() != 25 as libc::c_int
                && *__errno_location() != 38 as libc::c_int
            {
                if (1 as libc::c_int) << 4 as libc::c_int & loopdev_debug_mask != 0 {
                    fprintf(
                        stderr,
                        b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                        getpid(),
                        b"loopdev\0" as *const u8 as *const libc::c_char,
                        b"SETUP\0" as *const u8 as *const libc::c_char,
                    );
                    ul_debugobj(
                        lc as *const libc::c_void,
                        b"LOOP_CONFIGURE failed: %m\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                current_block = 10781277991799156595;
            } else {
                fallback = 1 as libc::c_int;
                current_block = 15462640364611497761;
            }
        } else {
            if (1 as libc::c_int) << 4 as libc::c_int & loopdev_debug_mask != 0 {
                fprintf(
                    stderr,
                    b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                    getpid(),
                    b"loopdev\0" as *const u8 as *const libc::c_char,
                    b"SETUP\0" as *const u8 as *const libc::c_char,
                );
                ul_debugobj(
                    lc as *const libc::c_void,
                    b"LOOP_CONFIGURE: OK\0" as *const u8 as *const libc::c_char,
                );
            }
            current_block = 15462640364611497761;
        }
        match current_block {
            10781277991799156595 => {}
            _ => {
                if fallback != 0 {
                    if ioctl(dev_fd, 0x4c00 as libc::c_int as libc::c_ulong, file_fd)
                        < 0 as libc::c_int
                    {
                        rc = -*__errno_location();
                        errsv = *__errno_location();
                        if (1 as libc::c_int) << 4 as libc::c_int & loopdev_debug_mask
                            != 0
                        {
                            fprintf(
                                stderr,
                                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                                getpid(),
                                b"loopdev\0" as *const u8 as *const libc::c_char,
                                b"SETUP\0" as *const u8 as *const libc::c_char,
                            );
                            ul_debugobj(
                                lc as *const libc::c_void,
                                b"LOOP_SET_FD failed: %m\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        current_block = 10781277991799156595;
                    } else {
                        if (1 as libc::c_int) << 4 as libc::c_int & loopdev_debug_mask
                            != 0
                        {
                            fprintf(
                                stderr,
                                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                                getpid(),
                                b"loopdev\0" as *const u8 as *const libc::c_char,
                                b"SETUP\0" as *const u8 as *const libc::c_char,
                            );
                            ul_debugobj(
                                lc as *const libc::c_void,
                                b"LOOP_SET_FD: OK\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        if (*lc).blocksize > 0 as libc::c_int as libc::c_ulong
                            && {
                                rc = loopcxt_ioctl_blocksize(lc, (*lc).blocksize);
                                rc < 0 as libc::c_int
                            }
                        {
                            errsv = -rc;
                            current_block = 10781277991799156595;
                        } else {
                            rc = loopcxt_ioctl_status(lc);
                            if rc < 0 as libc::c_int {
                                errsv = -rc;
                                current_block = 10781277991799156595;
                            } else {
                                current_block = 2750570471926810434;
                            }
                        }
                    }
                } else {
                    current_block = 2750570471926810434;
                }
                match current_block {
                    10781277991799156595 => {}
                    _ => {
                        rc = loopcxt_check_size(lc, file_fd);
                        if !(rc != 0) {
                            close(file_fd);
                            memset(
                                &mut (*lc).config as *mut loop_config as *mut libc::c_void,
                                0 as libc::c_int,
                                ::core::mem::size_of::<loop_config>() as libc::c_ulong,
                            );
                            (*lc).set_has_info(0 as libc::c_int as libc::c_uint);
                            (*lc).set_info_failed(0 as libc::c_int as libc::c_uint);
                            if (1 as libc::c_int) << 4 as libc::c_int
                                & loopdev_debug_mask != 0
                            {
                                fprintf(
                                    stderr,
                                    b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                                    getpid(),
                                    b"loopdev\0" as *const u8 as *const libc::c_char,
                                    b"SETUP\0" as *const u8 as *const libc::c_char,
                                );
                                ul_debugobj(
                                    lc as *const libc::c_void,
                                    b"success [rc=0]\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            return 0 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
    if file_fd >= 0 as libc::c_int {
        close(file_fd);
    }
    if dev_fd >= 0 as libc::c_int && rc != -(16 as libc::c_int) {
        ioctl(dev_fd, 0x4c01 as libc::c_int as libc::c_ulong, 0 as libc::c_int);
    }
    if errsv != 0 {
        *__errno_location() = errsv;
    }
    if (1 as libc::c_int) << 4 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"SETUP\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"failed [rc=%d]\0" as *const u8 as *const libc::c_char,
            rc,
        );
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_ioctl_status(mut lc: *mut loopdev_cxt) -> libc::c_int {
    let mut dev_fd: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    *__errno_location() = 0 as libc::c_int;
    dev_fd = loopcxt_get_fd(lc);
    if dev_fd < 0 as libc::c_int {
        return -*__errno_location();
    }
    if (1 as libc::c_int) << 4 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"SETUP\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"calling LOOP_SET_STATUS64\0" as *const u8 as *const libc::c_char,
        );
    }
    rc = ({
        let mut _c: libc::c_int = 0 as libc::c_int;
        let mut _e: libc::c_int = 0;
        loop {
            *__errno_location() = 0 as libc::c_int;
            _e = ioctl(
                dev_fd,
                0x4c04 as libc::c_int as libc::c_ulong,
                &mut (*lc).config.info as *mut loop_info64,
            );
            if _e == 0 as libc::c_int || *__errno_location() != 11 as libc::c_int {
                break;
            }
            if _c >= 10 as libc::c_int {
                break;
            }
            xusleep(250000 as libc::c_int as useconds_t);
            _c += 1;
            _c;
        }
        if _e == 0 as libc::c_int {
            0 as libc::c_int
        } else if *__errno_location() != 0 {
            -*__errno_location()
        } else {
            -(1 as libc::c_int)
        }
    });
    if rc != 0 as libc::c_int {
        if (1 as libc::c_int) << 4 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"SETUP\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"LOOP_SET_STATUS64 failed: %m\0" as *const u8 as *const libc::c_char,
            );
        }
        return rc;
    }
    if (1 as libc::c_int) << 4 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"SETUP\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"LOOP_SET_STATUS64: OK\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_ioctl_capacity(
    mut lc: *mut loopdev_cxt,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut fd: libc::c_int = loopcxt_get_fd(lc);
    if fd < 0 as libc::c_int {
        return -(22 as libc::c_int);
    }
    if (1 as libc::c_int) << 4 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"SETUP\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"calling LOOP_SET_CAPACITY\0" as *const u8 as *const libc::c_char,
        );
    }
    rc = ({
        let mut _c: libc::c_int = 0 as libc::c_int;
        let mut _e: libc::c_int = 0;
        loop {
            *__errno_location() = 0 as libc::c_int;
            _e = ioctl(fd, 0x4c07 as libc::c_int as libc::c_ulong, 0 as libc::c_int);
            if _e == 0 as libc::c_int || *__errno_location() != 11 as libc::c_int {
                break;
            }
            if _c >= 10 as libc::c_int {
                break;
            }
            xusleep(250000 as libc::c_int as useconds_t);
            _c += 1;
            _c;
        }
        if _e == 0 as libc::c_int {
            0 as libc::c_int
        } else if *__errno_location() != 0 {
            -*__errno_location()
        } else {
            -(1 as libc::c_int)
        }
    });
    if rc != 0 as libc::c_int {
        if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"LOOP_SET_CAPACITY failed: %m\0" as *const u8 as *const libc::c_char,
            );
        }
        return rc;
    }
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"capacity set\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_ioctl_dio(
    mut lc: *mut loopdev_cxt,
    mut use_dio: libc::c_ulong,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut fd: libc::c_int = loopcxt_get_fd(lc);
    if fd < 0 as libc::c_int {
        return -(22 as libc::c_int);
    }
    if (1 as libc::c_int) << 4 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"SETUP\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"calling LOOP_SET_DIRECT_IO\0" as *const u8 as *const libc::c_char,
        );
    }
    rc = ({
        let mut _c: libc::c_int = 0 as libc::c_int;
        let mut _e: libc::c_int = 0;
        loop {
            *__errno_location() = 0 as libc::c_int;
            _e = ioctl(fd, 0x4c08 as libc::c_int as libc::c_ulong, use_dio);
            if _e == 0 as libc::c_int || *__errno_location() != 11 as libc::c_int {
                break;
            }
            if _c >= 10 as libc::c_int {
                break;
            }
            xusleep(250000 as libc::c_int as useconds_t);
            _c += 1;
            _c;
        }
        if _e == 0 as libc::c_int {
            0 as libc::c_int
        } else if *__errno_location() != 0 {
            -*__errno_location()
        } else {
            -(1 as libc::c_int)
        }
    });
    if rc != 0 as libc::c_int {
        if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"LOOP_SET_DIRECT_IO failed: %m\0" as *const u8 as *const libc::c_char,
            );
        }
        return rc;
    }
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"direct io set\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_ioctl_blocksize(
    mut lc: *mut loopdev_cxt,
    mut blocksize: uint64_t,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut fd: libc::c_int = loopcxt_get_fd(lc);
    if fd < 0 as libc::c_int {
        return -(22 as libc::c_int);
    }
    if (1 as libc::c_int) << 4 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"SETUP\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"calling LOOP_SET_BLOCK_SIZE\0" as *const u8 as *const libc::c_char,
        );
    }
    rc = ({
        let mut _c: libc::c_int = 0 as libc::c_int;
        let mut _e: libc::c_int = 0;
        loop {
            *__errno_location() = 0 as libc::c_int;
            _e = ioctl(fd, 0x4c09 as libc::c_int as libc::c_ulong, blocksize);
            if _e == 0 as libc::c_int || *__errno_location() != 11 as libc::c_int {
                break;
            }
            if _c >= 10 as libc::c_int {
                break;
            }
            xusleep(250000 as libc::c_int as useconds_t);
            _c += 1;
            _c;
        }
        if _e == 0 as libc::c_int {
            0 as libc::c_int
        } else if *__errno_location() != 0 {
            -*__errno_location()
        } else {
            -(1 as libc::c_int)
        }
    });
    if rc != 0 as libc::c_int {
        if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"LOOP_SET_BLOCK_SIZE failed: %m\0" as *const u8 as *const libc::c_char,
            );
        }
        return rc;
    }
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"logical block size set\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_delete_device(mut lc: *mut loopdev_cxt) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut fd: libc::c_int = loopcxt_get_fd(lc);
    if fd < 0 as libc::c_int {
        return -(22 as libc::c_int);
    }
    if (1 as libc::c_int) << 4 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"SETUP\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"calling LOOP_SET_CLR_FD\0" as *const u8 as *const libc::c_char,
        );
    }
    rc = ({
        let mut _c: libc::c_int = 0 as libc::c_int;
        let mut _e: libc::c_int = 0;
        loop {
            *__errno_location() = 0 as libc::c_int;
            _e = ioctl(fd, 0x4c01 as libc::c_int as libc::c_ulong, 0 as libc::c_int);
            if _e == 0 as libc::c_int || *__errno_location() != 11 as libc::c_int {
                break;
            }
            if _c >= 10 as libc::c_int {
                break;
            }
            xusleep(250000 as libc::c_int as useconds_t);
            _c += 1;
            _c;
        }
        if _e == 0 as libc::c_int {
            0 as libc::c_int
        } else if *__errno_location() != 0 {
            -*__errno_location()
        } else {
            -(1 as libc::c_int)
        }
    });
    if rc != 0 as libc::c_int {
        if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"LOOP_CLR_FD failed: %m\0" as *const u8 as *const libc::c_char,
            );
        }
        return rc;
    }
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"device removed\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_add_device(mut lc: *mut loopdev_cxt) -> libc::c_int {
    let mut rc: libc::c_int = -(22 as libc::c_int);
    let mut ctl: libc::c_int = 0;
    let mut nr: libc::c_int = -(1 as libc::c_int);
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut dev: *const libc::c_char = loopcxt_get_device(lc);
    if !dev.is_null() {
        if (*lc).flags & LOOPDEV_FL_CONTROL as libc::c_int == 0 {
            rc = -(38 as libc::c_int);
        } else {
            p = strrchr(dev, '/' as i32);
            if !(p.is_null()
                || sscanf(
                    p,
                    b"/loop%d\0" as *const u8 as *const libc::c_char,
                    &mut nr as *mut libc::c_int,
                ) != 1 as libc::c_int
                    && sscanf(
                        p,
                        b"/%d\0" as *const u8 as *const libc::c_char,
                        &mut nr as *mut libc::c_int,
                    ) != 1 as libc::c_int || nr < 0 as libc::c_int)
            {
                ctl = open(
                    b"/dev/loop-control\0" as *const u8 as *const libc::c_char,
                    0o2 as libc::c_int | 0o2000000 as libc::c_int,
                );
                if ctl >= 0 as libc::c_int {
                    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
                        fprintf(
                            stderr,
                            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                            getpid(),
                            b"loopdev\0" as *const u8 as *const libc::c_char,
                            b"CXT\0" as *const u8 as *const libc::c_char,
                        );
                        ul_debugobj(
                            lc as *const libc::c_void,
                            b"add_device %d\0" as *const u8 as *const libc::c_char,
                            nr,
                        );
                    }
                    rc = ioctl(ctl, 0x4c80 as libc::c_int as libc::c_ulong, nr);
                    close(ctl);
                }
                (*lc)
                    .set_control_ok(
                        (if rc >= 0 as libc::c_int {
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as libc::c_uint,
                    );
            }
        }
    }
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"add_device done [rc=%d]\0" as *const u8 as *const libc::c_char,
            rc,
        );
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_find_unused(mut lc: *mut loopdev_cxt) -> libc::c_int {
    let mut rc: libc::c_int = -(1 as libc::c_int);
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"find_unused requested\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*lc).flags & LOOPDEV_FL_CONTROL as libc::c_int != 0 {
        let mut ctl: libc::c_int = 0;
        if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"using loop-control\0" as *const u8 as *const libc::c_char,
            );
        }
        ctl = open(
            b"/dev/loop-control\0" as *const u8 as *const libc::c_char,
            0o2 as libc::c_int | 0o2000000 as libc::c_int,
        );
        if ctl >= 0 as libc::c_int {
            rc = ioctl(ctl, 0x4c82 as libc::c_int as libc::c_ulong);
        } else {
            rc = -*__errno_location();
        }
        if rc >= 0 as libc::c_int {
            let mut name: [libc::c_char; 16] = [0; 16];
            snprintf(
                name.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                b"loop%d\0" as *const u8 as *const libc::c_char,
                rc,
            );
            rc = loopiter_set_device(lc, name.as_mut_ptr());
        }
        (*lc)
            .set_control_ok(
                (if ctl >= 0 as libc::c_int && rc == 0 as libc::c_int {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as libc::c_uint,
            );
        if ctl >= 0 as libc::c_int {
            close(ctl);
        }
        if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"find_unused by loop-control [rc=%d]\0" as *const u8
                    as *const libc::c_char,
                rc,
            );
        }
    }
    if rc < 0 as libc::c_int {
        if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"using loop scan\0" as *const u8 as *const libc::c_char,
            );
        }
        rc = loopcxt_init_iterator(lc, LOOPITER_FL_FREE as libc::c_int);
        if rc != 0 {
            return rc;
        }
        rc = loopcxt_next(lc);
        loopcxt_deinit_iterator(lc);
        if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"find_unused by scan [rc=%d]\0" as *const u8 as *const libc::c_char,
                rc,
            );
        }
        if rc != 0 {
            return -(2 as libc::c_int);
        }
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn loopdev_is_autoclear(
    mut device: *const libc::c_char,
) -> libc::c_int {
    let mut lc: loopdev_cxt = loopdev_cxt {
        device: [0; 128],
        filename: 0 as *mut libc::c_char,
        fd: 0,
        mode: 0,
        blocksize: 0,
        flags: 0,
        has_info_extra_check_info_failed_control_ok: [0; 1],
        c2rust_padding: [0; 3],
        sysfs: 0 as *mut path_cxt,
        config: loop_config {
            fd: 0,
            block_size: 0,
            info: loop_info64 {
                lo_device: 0,
                lo_inode: 0,
                lo_rdevice: 0,
                lo_offset: 0,
                lo_sizelimit: 0,
                lo_number: 0,
                lo_encrypt_type: 0,
                lo_encrypt_key_size: 0,
                lo_flags: 0,
                lo_file_name: [0; 64],
                lo_crypt_name: [0; 64],
                lo_encrypt_key: [0; 32],
                lo_init: [0; 2],
            },
            __reserved: [0; 8],
        },
        iter: loopdev_iter {
            proc_0: 0 as *mut FILE,
            sysblock: 0 as *mut DIR,
            ncur: 0,
            minors: 0 as *mut libc::c_int,
            nminors: 0,
            ct_perm: 0,
            ct_succ: 0,
            done_default_check: [0; 1],
            c2rust_padding: [0; 3],
            flags: 0,
        },
    };
    let mut rc: libc::c_int = 0;
    if device.is_null() {
        return 0 as libc::c_int;
    }
    rc = loopcxt_init(&mut lc, 0 as libc::c_int);
    if rc == 0 {
        rc = loopcxt_set_device(&mut lc, device);
    }
    if rc == 0 {
        rc = loopcxt_is_autoclear(&mut lc);
    }
    loopcxt_deinit(&mut lc);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn loopdev_get_backing_file(
    mut device: *const libc::c_char,
) -> *mut libc::c_char {
    let mut lc: loopdev_cxt = loopdev_cxt {
        device: [0; 128],
        filename: 0 as *mut libc::c_char,
        fd: 0,
        mode: 0,
        blocksize: 0,
        flags: 0,
        has_info_extra_check_info_failed_control_ok: [0; 1],
        c2rust_padding: [0; 3],
        sysfs: 0 as *mut path_cxt,
        config: loop_config {
            fd: 0,
            block_size: 0,
            info: loop_info64 {
                lo_device: 0,
                lo_inode: 0,
                lo_rdevice: 0,
                lo_offset: 0,
                lo_sizelimit: 0,
                lo_number: 0,
                lo_encrypt_type: 0,
                lo_encrypt_key_size: 0,
                lo_flags: 0,
                lo_file_name: [0; 64],
                lo_crypt_name: [0; 64],
                lo_encrypt_key: [0; 32],
                lo_init: [0; 2],
            },
            __reserved: [0; 8],
        },
        iter: loopdev_iter {
            proc_0: 0 as *mut FILE,
            sysblock: 0 as *mut DIR,
            ncur: 0,
            minors: 0 as *mut libc::c_int,
            nminors: 0,
            ct_perm: 0,
            ct_succ: 0,
            done_default_check: [0; 1],
            c2rust_padding: [0; 3],
            flags: 0,
        },
    };
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    if device.is_null() {
        return 0 as *mut libc::c_char;
    }
    if loopcxt_init(&mut lc, 0 as libc::c_int) != 0 {
        return 0 as *mut libc::c_char;
    }
    if loopcxt_set_device(&mut lc, device) == 0 as libc::c_int {
        res = loopcxt_get_backing_file(&mut lc);
    }
    loopcxt_deinit(&mut lc);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn loopdev_has_backing_file(
    mut device: *const libc::c_char,
) -> libc::c_int {
    let mut tmp: *mut libc::c_char = loopdev_get_backing_file(device);
    if !tmp.is_null() {
        free(tmp as *mut libc::c_void);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loopdev_is_used(
    mut device: *const libc::c_char,
    mut filename: *const libc::c_char,
    mut offset: uint64_t,
    mut sizelimit: uint64_t,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut lc: loopdev_cxt = loopdev_cxt {
        device: [0; 128],
        filename: 0 as *mut libc::c_char,
        fd: 0,
        mode: 0,
        blocksize: 0,
        flags: 0,
        has_info_extra_check_info_failed_control_ok: [0; 1],
        c2rust_padding: [0; 3],
        sysfs: 0 as *mut path_cxt,
        config: loop_config {
            fd: 0,
            block_size: 0,
            info: loop_info64 {
                lo_device: 0,
                lo_inode: 0,
                lo_rdevice: 0,
                lo_offset: 0,
                lo_sizelimit: 0,
                lo_number: 0,
                lo_encrypt_type: 0,
                lo_encrypt_key_size: 0,
                lo_flags: 0,
                lo_file_name: [0; 64],
                lo_crypt_name: [0; 64],
                lo_encrypt_key: [0; 32],
                lo_init: [0; 2],
            },
            __reserved: [0; 8],
        },
        iter: loopdev_iter {
            proc_0: 0 as *mut FILE,
            sysblock: 0 as *mut DIR,
            ncur: 0,
            minors: 0 as *mut libc::c_int,
            nminors: 0,
            ct_perm: 0,
            ct_succ: 0,
            done_default_check: [0; 1],
            c2rust_padding: [0; 3],
            flags: 0,
        },
    };
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
    let mut rc: libc::c_int = 0 as libc::c_int;
    if device.is_null() || filename.is_null() {
        return 0 as libc::c_int;
    }
    rc = loopcxt_init(&mut lc, 0 as libc::c_int);
    if rc == 0 {
        rc = loopcxt_set_device(&mut lc, device);
    }
    if rc != 0 {
        return rc;
    }
    rc = (stat(filename, &mut st) == 0) as libc::c_int;
    rc = loopcxt_is_used(
        &mut lc,
        if rc != 0 { &mut st } else { 0 as *mut stat },
        filename,
        offset,
        sizelimit,
        flags,
    );
    loopcxt_deinit(&mut lc);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn loopdev_delete(mut device: *const libc::c_char) -> libc::c_int {
    let mut lc: loopdev_cxt = loopdev_cxt {
        device: [0; 128],
        filename: 0 as *mut libc::c_char,
        fd: 0,
        mode: 0,
        blocksize: 0,
        flags: 0,
        has_info_extra_check_info_failed_control_ok: [0; 1],
        c2rust_padding: [0; 3],
        sysfs: 0 as *mut path_cxt,
        config: loop_config {
            fd: 0,
            block_size: 0,
            info: loop_info64 {
                lo_device: 0,
                lo_inode: 0,
                lo_rdevice: 0,
                lo_offset: 0,
                lo_sizelimit: 0,
                lo_number: 0,
                lo_encrypt_type: 0,
                lo_encrypt_key_size: 0,
                lo_flags: 0,
                lo_file_name: [0; 64],
                lo_crypt_name: [0; 64],
                lo_encrypt_key: [0; 32],
                lo_init: [0; 2],
            },
            __reserved: [0; 8],
        },
        iter: loopdev_iter {
            proc_0: 0 as *mut FILE,
            sysblock: 0 as *mut DIR,
            ncur: 0,
            minors: 0 as *mut libc::c_int,
            nminors: 0,
            ct_perm: 0,
            ct_succ: 0,
            done_default_check: [0; 1],
            c2rust_padding: [0; 3],
            flags: 0,
        },
    };
    let mut rc: libc::c_int = 0;
    if device.is_null() {
        return -(22 as libc::c_int);
    }
    rc = loopcxt_init(&mut lc, 0 as libc::c_int);
    if rc == 0 {
        rc = loopcxt_set_device(&mut lc, device);
    }
    if rc == 0 {
        rc = loopcxt_delete_device(&mut lc);
    }
    loopcxt_deinit(&mut lc);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_find_by_backing_file(
    mut lc: *mut loopdev_cxt,
    mut filename: *const libc::c_char,
    mut offset: uint64_t,
    mut sizelimit: uint64_t,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut hasst: libc::c_int = 0;
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
    if filename.is_null() {
        return -(22 as libc::c_int);
    }
    hasst = (stat(filename, &mut st) == 0) as libc::c_int;
    rc = loopcxt_init_iterator(lc, LOOPITER_FL_USED as libc::c_int);
    if rc != 0 {
        return rc;
    }
    loop {
        rc = loopcxt_next(lc);
        if !(rc == 0 as libc::c_int) {
            break;
        }
        if loopcxt_is_used(
            lc,
            if hasst != 0 { &mut st } else { 0 as *mut stat },
            filename,
            offset,
            sizelimit,
            flags,
        ) != 0
        {
            break;
        }
    }
    loopcxt_deinit_iterator(lc);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn loopcxt_find_overlap(
    mut lc: *mut loopdev_cxt,
    mut filename: *const libc::c_char,
    mut offset: uint64_t,
    mut sizelimit: uint64_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    let mut hasst: libc::c_int = 0;
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
    if filename.is_null() {
        return -(22 as libc::c_int);
    }
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"find_overlap requested\0" as *const u8 as *const libc::c_char,
        );
    }
    hasst = (stat(filename, &mut st) == 0) as libc::c_int;
    rc = loopcxt_init_iterator(lc, LOOPITER_FL_USED as libc::c_int);
    if rc != 0 {
        return rc;
    }
    loop {
        rc = loopcxt_next(lc);
        if !(rc == 0 as libc::c_int) {
            current_block = 3160140712158701372;
            break;
        }
        let mut lc_sizelimit: uint64_t = 0;
        let mut lc_offset: uint64_t = 0;
        rc = loopcxt_is_used(
            lc,
            if hasst != 0 { &mut st } else { 0 as *mut stat },
            filename,
            offset,
            sizelimit,
            0 as libc::c_int,
        );
        if rc <= 0 as libc::c_int {
            continue;
        }
        if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"loopdev\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                lc as *const libc::c_void,
                b"found %s backed by %s\0" as *const u8 as *const libc::c_char,
                loopcxt_get_device(lc),
                filename,
            );
        }
        rc = loopcxt_get_offset(lc, &mut lc_offset);
        if rc != 0 {
            if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
                fprintf(
                    stderr,
                    b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                    getpid(),
                    b"loopdev\0" as *const u8 as *const libc::c_char,
                    b"CXT\0" as *const u8 as *const libc::c_char,
                );
                ul_debugobj(
                    lc as *const libc::c_void,
                    b"failed to get offset for device %s\0" as *const u8
                        as *const libc::c_char,
                    loopcxt_get_device(lc),
                );
            }
        } else {
            rc = loopcxt_get_sizelimit(lc, &mut lc_sizelimit);
            if rc != 0 {
                if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
                    fprintf(
                        stderr,
                        b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                        getpid(),
                        b"loopdev\0" as *const u8 as *const libc::c_char,
                        b"CXT\0" as *const u8 as *const libc::c_char,
                    );
                    ul_debugobj(
                        lc as *const libc::c_void,
                        b"failed to get sizelimit for device %s\0" as *const u8
                            as *const libc::c_char,
                        loopcxt_get_device(lc),
                    );
                }
            } else if lc_sizelimit == sizelimit && lc_offset == offset {
                if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
                    fprintf(
                        stderr,
                        b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                        getpid(),
                        b"loopdev\0" as *const u8 as *const libc::c_char,
                        b"CXT\0" as *const u8 as *const libc::c_char,
                    );
                    ul_debugobj(
                        lc as *const libc::c_void,
                        b"overlapping loop device %s (full match)\0" as *const u8
                            as *const libc::c_char,
                        loopcxt_get_device(lc),
                    );
                }
                rc = 2 as libc::c_int;
                current_block = 10754566852868215614;
                break;
            } else {
                if lc_sizelimit != 0 as libc::c_int as libc::c_ulong
                    && offset >= lc_offset.wrapping_add(lc_sizelimit)
                {
                    continue;
                }
                if sizelimit != 0 as libc::c_int as libc::c_ulong
                    && offset.wrapping_add(sizelimit) <= lc_offset
                {
                    continue;
                }
                if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
                    fprintf(
                        stderr,
                        b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                        getpid(),
                        b"loopdev\0" as *const u8 as *const libc::c_char,
                        b"CXT\0" as *const u8 as *const libc::c_char,
                    );
                    ul_debugobj(
                        lc as *const libc::c_void,
                        b"overlapping loop device %s\0" as *const u8
                            as *const libc::c_char,
                        loopcxt_get_device(lc),
                    );
                }
                rc = 1 as libc::c_int;
                current_block = 10754566852868215614;
                break;
            }
        }
    }
    match current_block {
        3160140712158701372 => {
            if rc == 1 as libc::c_int {
                rc = 0 as libc::c_int;
            }
        }
        _ => {}
    }
    loopcxt_deinit_iterator(lc);
    if (1 as libc::c_int) << 2 as libc::c_int & loopdev_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"loopdev\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            lc as *const libc::c_void,
            b"find_overlap done [rc=%d]\0" as *const u8 as *const libc::c_char,
            rc,
        );
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn loopdev_find_by_backing_file(
    mut filename: *const libc::c_char,
    mut offset: uint64_t,
    mut sizelimit: uint64_t,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    let mut lc: loopdev_cxt = loopdev_cxt {
        device: [0; 128],
        filename: 0 as *mut libc::c_char,
        fd: 0,
        mode: 0,
        blocksize: 0,
        flags: 0,
        has_info_extra_check_info_failed_control_ok: [0; 1],
        c2rust_padding: [0; 3],
        sysfs: 0 as *mut path_cxt,
        config: loop_config {
            fd: 0,
            block_size: 0,
            info: loop_info64 {
                lo_device: 0,
                lo_inode: 0,
                lo_rdevice: 0,
                lo_offset: 0,
                lo_sizelimit: 0,
                lo_number: 0,
                lo_encrypt_type: 0,
                lo_encrypt_key_size: 0,
                lo_flags: 0,
                lo_file_name: [0; 64],
                lo_crypt_name: [0; 64],
                lo_encrypt_key: [0; 32],
                lo_init: [0; 2],
            },
            __reserved: [0; 8],
        },
        iter: loopdev_iter {
            proc_0: 0 as *mut FILE,
            sysblock: 0 as *mut DIR,
            ncur: 0,
            minors: 0 as *mut libc::c_int,
            nminors: 0,
            ct_perm: 0,
            ct_succ: 0,
            done_default_check: [0; 1],
            c2rust_padding: [0; 3],
            flags: 0,
        },
    };
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    if filename.is_null() {
        return 0 as *mut libc::c_char;
    }
    if loopcxt_init(&mut lc, 0 as libc::c_int) != 0 {
        return 0 as *mut libc::c_char;
    }
    if loopcxt_find_by_backing_file(&mut lc, filename, offset, sizelimit, flags)
        == 0 as libc::c_int
    {
        res = loopcxt_strdup_device(&mut lc);
    }
    loopcxt_deinit(&mut lc);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn loopdev_count_by_backing_file(
    mut filename: *const libc::c_char,
    mut loopdev: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut lc: loopdev_cxt = loopdev_cxt {
        device: [0; 128],
        filename: 0 as *mut libc::c_char,
        fd: 0,
        mode: 0,
        blocksize: 0,
        flags: 0,
        has_info_extra_check_info_failed_control_ok: [0; 1],
        c2rust_padding: [0; 3],
        sysfs: 0 as *mut path_cxt,
        config: loop_config {
            fd: 0,
            block_size: 0,
            info: loop_info64 {
                lo_device: 0,
                lo_inode: 0,
                lo_rdevice: 0,
                lo_offset: 0,
                lo_sizelimit: 0,
                lo_number: 0,
                lo_encrypt_type: 0,
                lo_encrypt_key_size: 0,
                lo_flags: 0,
                lo_file_name: [0; 64],
                lo_crypt_name: [0; 64],
                lo_encrypt_key: [0; 32],
                lo_init: [0; 2],
            },
            __reserved: [0; 8],
        },
        iter: loopdev_iter {
            proc_0: 0 as *mut FILE,
            sysblock: 0 as *mut DIR,
            ncur: 0,
            minors: 0 as *mut libc::c_int,
            nminors: 0,
            ct_perm: 0,
            ct_succ: 0,
            done_default_check: [0; 1],
            c2rust_padding: [0; 3],
            flags: 0,
        },
    };
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut rc: libc::c_int = 0;
    if filename.is_null() {
        return -(1 as libc::c_int);
    }
    rc = loopcxt_init(&mut lc, 0 as libc::c_int);
    if rc != 0 {
        return rc;
    }
    if loopcxt_init_iterator(&mut lc, LOOPITER_FL_USED as libc::c_int) != 0 {
        return -(1 as libc::c_int);
    }
    while loopcxt_next(&mut lc) == 0 as libc::c_int {
        let mut backing: *mut libc::c_char = loopcxt_get_backing_file(&mut lc);
        if backing.is_null() || strcmp(backing, filename) != 0 as libc::c_int {
            free(backing as *mut libc::c_void);
        } else {
            free(backing as *mut libc::c_void);
            if !loopdev.is_null() && count == 0 as libc::c_int {
                *loopdev = loopcxt_strdup_device(&mut lc);
            }
            count += 1;
            count;
        }
    }
    loopcxt_deinit(&mut lc);
    if !loopdev.is_null() && count > 1 as libc::c_int {
        free(*loopdev as *mut libc::c_void);
        *loopdev = 0 as *mut libc::c_char;
    }
    return count;
}
