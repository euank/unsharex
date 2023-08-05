use ::libc;
use ::c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __xpg_basename(__path: *mut libc::c_char) -> *mut libc::c_char;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn faccessat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __type: libc::c_int,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn getpid() -> __pid_t;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getegid() -> __gid_t;
    fn readlink(
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
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
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn strsep(
        __stringp: *mut *mut libc::c_char,
        __delim: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn ul_new_path(dir: *const libc::c_char, _: ...) -> *mut path_cxt;
    fn ul_unref_path(pc: *mut path_cxt);
    fn ul_ref_path(pc: *mut path_cxt);
    fn ul_path_set_prefix(pc: *mut path_cxt, prefix: *const libc::c_char) -> libc::c_int;
    fn ul_path_get_prefix(pc: *mut path_cxt) -> *const libc::c_char;
    fn ul_path_set_dir(pc: *mut path_cxt, dir: *const libc::c_char) -> libc::c_int;
    fn ul_path_set_dialect(
        pc: *mut path_cxt,
        data: *mut libc::c_void,
        free_data: Option::<unsafe extern "C" fn(*mut path_cxt) -> ()>,
    ) -> libc::c_int;
    fn ul_path_get_dialect(pc: *mut path_cxt) -> *mut libc::c_void;
    fn ul_path_set_enoent_redirect(
        pc: *mut path_cxt,
        func: Option::<
            unsafe extern "C" fn(
                *mut path_cxt,
                *const libc::c_char,
                *mut libc::c_int,
            ) -> libc::c_int,
        >,
    ) -> libc::c_int;
    fn ul_path_get_dirfd(pc: *mut path_cxt) -> libc::c_int;
    fn ul_path_access(
        pc: *mut path_cxt,
        mode: libc::c_int,
        path: *const libc::c_char,
    ) -> libc::c_int;
    fn ul_path_opendir(pc: *mut path_cxt, path: *const libc::c_char) -> *mut DIR;
    fn ul_path_readlink(
        pc: *mut path_cxt,
        buf: *mut libc::c_char,
        bufsiz: size_t,
        path: *const libc::c_char,
    ) -> ssize_t;
    fn ul_path_read_string(
        pc: *mut path_cxt,
        str: *mut *mut libc::c_char,
        path: *const libc::c_char,
    ) -> libc::c_int;
    fn ul_path_read_buffer(
        pc: *mut path_cxt,
        buf: *mut libc::c_char,
        bufsz: size_t,
        path: *const libc::c_char,
    ) -> libc::c_int;
    fn ul_path_scanf(
        pc: *mut path_cxt,
        path: *const libc::c_char,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn ul_path_readf_majmin(
        pc: *mut path_cxt,
        res: *mut dev_t,
        path: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn ul_path_read_s32(
        pc: *mut path_cxt,
        res: *mut int32_t,
        path: *const libc::c_char,
    ) -> libc::c_int;
    fn ul_path_readf_s32(
        pc: *mut path_cxt,
        res: *mut int32_t,
        path: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn dirfd(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn stripoff_last_component(path: *mut libc::c_char) -> *mut libc::c_char;
    fn ul_buffer_free_data(buf: *mut ul_buffer);
    fn ul_buffer_append_data(
        buf: *mut ul_buffer,
        data: *const libc::c_char,
        sz: size_t,
    ) -> libc::c_int;
    fn ul_buffer_append_string(
        buf: *mut ul_buffer,
        str: *const libc::c_char,
    ) -> libc::c_int;
    fn ul_buffer_get_data(
        buf: *mut ul_buffer,
        sz: *mut size_t,
        width: *mut size_t,
    ) -> *mut libc::c_char;
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
pub type __int32_t = libc::c_int;
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
pub type size_t = libc::c_ulong;
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
pub type dev_t = __dev_t;
pub type ssize_t = __ssize_t;
pub type useconds_t = __useconds_t;
pub type int32_t = __int32_t;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct sysfs_blkdev {
    pub devno: dev_t,
    pub parent: *mut path_cxt,
    pub scsi_host: libc::c_uint,
    pub scsi_channel: libc::c_uint,
    pub scsi_target: libc::c_uint,
    pub scsi_lun: libc::c_uint,
    #[bitfield(name = "has_hctl", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "hctl_error", ty = "libc::c_uint", bits = "1..=1")]
    pub has_hctl_hctl_error: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ul_debug_maskname {
    pub name: *const libc::c_char,
    pub mask: libc::c_int,
    pub help: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ul_buffer {
    pub begin: *mut libc::c_char,
    pub end: *mut libc::c_char,
    pub sz: size_t,
    pub chunksize: size_t,
    pub encoded: *mut libc::c_char,
    pub encoded_sz: size_t,
    pub ptrs: *mut *mut libc::c_char,
    pub nptrs: size_t,
}
pub type sysfs_byteorder = libc::c_uint;
pub const SYSFS_BYTEORDER_BIG: sysfs_byteorder = 1;
pub const SYSFS_BYTEORDER_LITTLE: sysfs_byteorder = 0;
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
unsafe extern "C" fn sysfs_devname_sys_to_dev(mut name: *mut libc::c_char) {
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    if !name.is_null() {
        loop {
            c = strchr(name, '!' as i32);
            if c.is_null() {
                break;
            }
            *c.offset(0 as libc::c_int as isize) = '/' as i32 as libc::c_char;
        }
    }
}
#[inline]
unsafe extern "C" fn sysfs_devname_dev_to_sys(mut name: *mut libc::c_char) {
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    if !name.is_null() {
        loop {
            c = strchr(name, '/' as i32);
            if c.is_null() {
                break;
            }
            *c.offset(0 as libc::c_int as isize) = '!' as i32 as libc::c_char;
        }
    }
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
static mut ulsysfs_debug_mask: libc::c_int = 0;
static mut ulsysfs_masknames: [ul_debug_maskname; 1] = [
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
        && ulsysfs_debug_mask & (1 as libc::c_int) << 24 as libc::c_int == 0
    {
        fprintf(stderr, b"[%p]: \0" as *const u8 as *const libc::c_char, handler);
    }
    ap = args.clone();
    vfprintf(stderr, mesg, ap.as_va_list());
    fputc('\n' as i32, stderr);
}
#[no_mangle]
pub unsafe extern "C" fn ul_sysfs_init_debug() {
    if ulsysfs_debug_mask != 0 {
        return;
    }
    let mut envstr: *const libc::c_char = if 0 as libc::c_int != 0 {
        0 as *mut libc::c_char
    } else {
        getenv(b"ULSYSFS_DEBUG\0" as *const u8 as *const libc::c_char)
    };
    if !(ulsysfs_debug_mask & (1 as libc::c_int) << 1 as libc::c_int != 0) {
        if 0 as libc::c_int == 0 && !envstr.is_null() {
            ulsysfs_debug_mask = ul_debug_parse_mask(ulsysfs_masknames.as_ptr(), envstr);
        } else {
            ulsysfs_debug_mask = 0 as libc::c_int;
        }
    }
    if ulsysfs_debug_mask != 0 {
        if getuid() != geteuid() || getgid() != getegid() {
            ulsysfs_debug_mask |= (1 as libc::c_int) << 24 as libc::c_int;
            fprintf(
                stderr,
                b"%d: %s: don't print memory addresses (SUID executable).\n\0"
                    as *const u8 as *const libc::c_char,
                getpid(),
                b"ulsysfs\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    ulsysfs_debug_mask |= (1 as libc::c_int) << 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_new_sysfs_path(
    mut devno: dev_t,
    mut parent: *mut path_cxt,
    mut prefix: *const libc::c_char,
) -> *mut path_cxt {
    let mut pc: *mut path_cxt = ul_new_path(0 as *const libc::c_char);
    if pc.is_null() {
        return 0 as *mut path_cxt;
    }
    if !prefix.is_null() {
        ul_path_set_prefix(pc, prefix);
    }
    if sysfs_blkdev_init_path(pc, devno, parent) != 0 as libc::c_int {
        ul_unref_path(pc);
        return 0 as *mut path_cxt;
    }
    if (1 as libc::c_int) << 2 as libc::c_int & ulsysfs_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"ulsysfs\0" as *const u8 as *const libc::c_char,
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
pub unsafe extern "C" fn sysfs_blkdev_init_path(
    mut pc: *mut path_cxt,
    mut devno: dev_t,
    mut parent: *mut path_cxt,
) -> libc::c_int {
    let mut blk: *mut sysfs_blkdev = 0 as *mut sysfs_blkdev;
    let mut rc: libc::c_int = 0;
    let mut buf: [libc::c_char; 46] = [0; 46];
    snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong,
        b"/sys/dev/block/%d:%d\0" as *const u8 as *const libc::c_char,
        gnu_dev_major(devno),
        gnu_dev_minor(devno),
    );
    rc = ul_path_set_dir(pc, buf.as_mut_ptr());
    if rc != 0 {
        return rc;
    }
    rc = ul_path_get_dirfd(pc);
    if rc < 0 as libc::c_int {
        return rc;
    }
    blk = ul_path_get_dialect(pc) as *mut sysfs_blkdev;
    if blk.is_null() {
        if (1 as libc::c_int) << 2 as libc::c_int & ulsysfs_debug_mask != 0 {
            fprintf(
                stderr,
                b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                getpid(),
                b"ulsysfs\0" as *const u8 as *const libc::c_char,
                b"CXT\0" as *const u8 as *const libc::c_char,
            );
            ul_debugobj(
                pc as *const libc::c_void,
                b"alloc new sysfs handler\0" as *const u8 as *const libc::c_char,
            );
        }
        blk = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::core::mem::size_of::<sysfs_blkdev>() as libc::c_ulong,
        ) as *mut sysfs_blkdev;
        if blk.is_null() {
            return -(12 as libc::c_int);
        }
        ul_path_set_dialect(
            pc,
            blk as *mut libc::c_void,
            Some(sysfs_blkdev_deinit_path as unsafe extern "C" fn(*mut path_cxt) -> ()),
        );
        ul_path_set_enoent_redirect(
            pc,
            Some(
                sysfs_blkdev_enoent_redirect
                    as unsafe extern "C" fn(
                        *mut path_cxt,
                        *const libc::c_char,
                        *mut libc::c_int,
                    ) -> libc::c_int,
            ),
        );
    }
    if (1 as libc::c_int) << 2 as libc::c_int & ulsysfs_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"ulsysfs\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            pc as *const libc::c_void,
            b"init sysfs stuff\0" as *const u8 as *const libc::c_char,
        );
    }
    (*blk).devno = devno;
    sysfs_blkdev_set_parent(pc, parent);
    return 0 as libc::c_int;
}
unsafe extern "C" fn sysfs_blkdev_deinit_path(mut pc: *mut path_cxt) {
    let mut blk: *mut sysfs_blkdev = 0 as *mut sysfs_blkdev;
    if pc.is_null() {
        return;
    }
    if (1 as libc::c_int) << 2 as libc::c_int & ulsysfs_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"ulsysfs\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            pc as *const libc::c_void,
            b"deinit\0" as *const u8 as *const libc::c_char,
        );
    }
    blk = ul_path_get_dialect(pc) as *mut sysfs_blkdev;
    if blk.is_null() {
        return;
    }
    ul_unref_path((*blk).parent);
    free(blk as *mut libc::c_void);
    ul_path_set_dialect(pc, 0 as *mut libc::c_void, None);
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_blkdev_set_parent(
    mut pc: *mut path_cxt,
    mut parent: *mut path_cxt,
) -> libc::c_int {
    let mut blk: *mut sysfs_blkdev = ul_path_get_dialect(pc) as *mut sysfs_blkdev;
    if pc.is_null() || blk.is_null() {
        return -(22 as libc::c_int);
    }
    if !((*blk).parent).is_null() {
        ul_unref_path((*blk).parent);
        (*blk).parent = 0 as *mut path_cxt;
    }
    if !parent.is_null() {
        ul_ref_path(parent);
        (*blk).parent = parent;
    } else {
        (*blk).parent = 0 as *mut path_cxt;
    }
    if (1 as libc::c_int) << 2 as libc::c_int & ulsysfs_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"ulsysfs\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            pc as *const libc::c_void,
            b"new parent\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_blkdev_get_parent(
    mut pc: *mut path_cxt,
) -> *mut path_cxt {
    let mut blk: *mut sysfs_blkdev = ul_path_get_dialect(pc) as *mut sysfs_blkdev;
    return if !blk.is_null() { (*blk).parent } else { 0 as *mut path_cxt };
}
unsafe extern "C" fn sysfs_blkdev_enoent_redirect(
    mut pc: *mut path_cxt,
    mut path: *const libc::c_char,
    mut dirfd_0: *mut libc::c_int,
) -> libc::c_int {
    let mut blk: *mut sysfs_blkdev = ul_path_get_dialect(pc) as *mut sysfs_blkdev;
    if !blk.is_null() && !((*blk).parent).is_null() && !path.is_null() {
        *dirfd_0 = ul_path_get_dirfd((*blk).parent);
        if *dirfd_0 >= 0 as libc::c_int {
            if (1 as libc::c_int) << 2 as libc::c_int & ulsysfs_debug_mask != 0 {
                fprintf(
                    stderr,
                    b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
                    getpid(),
                    b"ulsysfs\0" as *const u8 as *const libc::c_char,
                    b"CXT\0" as *const u8 as *const libc::c_char,
                );
                ul_debugobj(
                    pc as *const libc::c_void,
                    b"%s redirected to parent\0" as *const u8 as *const libc::c_char,
                    path,
                );
            }
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_blkdev_get_name(
    mut pc: *mut path_cxt,
    mut buf: *mut libc::c_char,
    mut bufsiz: size_t,
) -> *mut libc::c_char {
    let mut link: [libc::c_char; 4096] = [0; 4096];
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sz: ssize_t = 0;
    sz = ul_path_readlink(
        pc,
        link.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        0 as *const libc::c_char,
    );
    if sz < 0 as libc::c_int as libc::c_long {
        return 0 as *mut libc::c_char;
    }
    name = strrchr(link.as_mut_ptr(), '/' as i32);
    if name.is_null() {
        return 0 as *mut libc::c_char;
    }
    name = name.offset(1);
    name;
    sz = strlen(name) as ssize_t;
    if (sz as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong) > bufsiz {
        return 0 as *mut libc::c_char;
    }
    memcpy(
        buf as *mut libc::c_void,
        name as *const libc::c_void,
        (sz + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
    );
    sysfs_devname_sys_to_dev(buf);
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_blkdev_is_partition_dirent(
    mut dir: *mut DIR,
    mut d: *mut dirent,
    mut parent_name: *const libc::c_char,
) -> libc::c_int {
    let mut path: [libc::c_char; 262] = [0; 262];
    if (*d).d_type as libc::c_int != DT_DIR as libc::c_int
        && (*d).d_type as libc::c_int != DT_LNK as libc::c_int
        && (*d).d_type as libc::c_int != DT_UNKNOWN as libc::c_int
    {
        return 0 as libc::c_int;
    }
    let mut len: size_t = 0 as libc::c_int as size_t;
    if !parent_name.is_null() {
        let mut p: *const libc::c_char = parent_name;
        if *parent_name as libc::c_int == '/' as i32 {
            p = strrchr(parent_name, '/' as i32);
            if p.is_null() {
                return 0 as libc::c_int;
            }
            p = p.offset(1);
            p;
        }
        len = strlen(p);
        if strlen(((*d).d_name).as_mut_ptr()) <= len
            || strncmp(p, ((*d).d_name).as_mut_ptr(), len) != 0 as libc::c_int
        {
            len = 0 as libc::c_int as size_t;
        }
    }
    if len > 0 as libc::c_int as libc::c_ulong {
        return (*((*d).d_name).as_mut_ptr().offset(len as isize) as libc::c_int
            == 'p' as i32
            && *(*__ctype_b_loc())
                .offset(
                    *((*d).d_name)
                        .as_mut_ptr()
                        .offset(len as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_int as isize,
                ) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            || *(*__ctype_b_loc())
                .offset(
                    *((*d).d_name).as_mut_ptr().offset(len as isize) as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
            as libc::c_int;
    }
    snprintf(
        path.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 262]>() as libc::c_ulong,
        b"%s/start\0" as *const u8 as *const libc::c_char,
        ((*d).d_name).as_mut_ptr(),
    );
    return (faccessat(dirfd(dir), path.as_mut_ptr(), 4 as libc::c_int, 0 as libc::c_int)
        == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_blkdev_count_partitions(
    mut pc: *mut path_cxt,
    mut devname: *const libc::c_char,
) -> libc::c_int {
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut d: *mut dirent = 0 as *mut dirent;
    let mut r: libc::c_int = 0 as libc::c_int;
    dir = ul_path_opendir(pc, 0 as *const libc::c_char);
    if dir.is_null() {
        return 0 as libc::c_int;
    }
    loop {
        d = xreaddir(dir);
        if d.is_null() {
            break;
        }
        if sysfs_blkdev_is_partition_dirent(dir, d, devname) != 0 {
            r += 1;
            r;
        }
    }
    closedir(dir);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_blkdev_partno_to_devno(
    mut pc: *mut path_cxt,
    mut partno: libc::c_int,
) -> dev_t {
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut d: *mut dirent = 0 as *mut dirent;
    let mut devno: dev_t = 0 as libc::c_int as dev_t;
    dir = ul_path_opendir(pc, 0 as *const libc::c_char);
    if dir.is_null() {
        return 0 as libc::c_int as dev_t;
    }
    loop {
        d = xreaddir(dir);
        if d.is_null() {
            break;
        }
        let mut n: libc::c_int = 0;
        if sysfs_blkdev_is_partition_dirent(dir, d, 0 as *const libc::c_char) == 0 {
            continue;
        }
        if ul_path_readf_s32(
            pc,
            &mut n as *mut libc::c_int,
            b"%s/partition\0" as *const u8 as *const libc::c_char,
            ((*d).d_name).as_mut_ptr(),
        ) != 0
        {
            continue;
        }
        if !(n == partno) {
            continue;
        }
        if ul_path_readf_majmin(
            pc,
            &mut devno as *mut dev_t,
            b"%s/dev\0" as *const u8 as *const libc::c_char,
            ((*d).d_name).as_mut_ptr(),
        ) == 0 as libc::c_int
        {
            break;
        }
    }
    closedir(dir);
    if (1 as libc::c_int) << 2 as libc::c_int & ulsysfs_debug_mask != 0 {
        fprintf(
            stderr,
            b"%d: %s: %8s: \0" as *const u8 as *const libc::c_char,
            getpid(),
            b"ulsysfs\0" as *const u8 as *const libc::c_char,
            b"CXT\0" as *const u8 as *const libc::c_char,
        );
        ul_debugobj(
            pc as *const libc::c_void,
            b"partno (%d) -> devno (%d)\0" as *const u8 as *const libc::c_char,
            partno,
            devno as libc::c_int,
        );
    }
    return devno;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_blkdev_get_slave(
    mut pc: *mut path_cxt,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut d: *mut dirent = 0 as *mut dirent;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    dir = ul_path_opendir(pc, b"slaves\0" as *const u8 as *const libc::c_char);
    if dir.is_null() {
        return 0 as *mut libc::c_char;
    }
    loop {
        d = xreaddir(dir);
        if d.is_null() {
            current_block = 15619007995458559411;
            break;
        }
        if !name.is_null() {
            current_block = 4220309844134831472;
            break;
        }
        name = strdup(((*d).d_name).as_mut_ptr());
    }
    match current_block {
        4220309844134831472 => {
            free(name as *mut libc::c_void);
            closedir(dir);
            return 0 as *mut libc::c_char;
        }
        _ => {
            closedir(dir);
            return name;
        }
    };
}
unsafe extern "C" fn get_subsystem(
    mut chain: *mut libc::c_char,
    mut buf: *mut libc::c_char,
    mut bufsz: size_t,
) -> *mut libc::c_char {
    let mut len: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if chain.is_null() || *chain == 0 {
        return 0 as *mut libc::c_char;
    }
    len = strlen(chain);
    if len.wrapping_add(::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
        > 4096 as libc::c_int as libc::c_ulong
    {
        return 0 as *mut libc::c_char;
    }
    loop {
        let mut sz: ssize_t = 0;
        memcpy(
            chain.offset(len as isize) as *mut libc::c_void,
            b"/subsystem\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            ::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong,
        );
        sz = readlink(chain, buf, bufsz.wrapping_sub(1 as libc::c_int as libc::c_ulong));
        *chain.offset(len as isize) = '\0' as i32 as libc::c_char;
        p = strrchr(chain, '/' as i32);
        if !p.is_null() {
            *p = '\0' as i32 as libc::c_char;
            len = p.offset_from(chain) as libc::c_long as size_t;
        }
        if sz > 0 as libc::c_int as libc::c_long {
            *buf.offset(sz as isize) = '\0' as i32 as libc::c_char;
            return __xpg_basename(buf);
        }
        if p.is_null() {
            break;
        }
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_blkdev_get_devchain(
    mut pc: *mut path_cxt,
    mut buf: *mut libc::c_char,
    mut bufsz: size_t,
) -> *mut libc::c_char {
    let mut ssz: ssize_t = 0;
    let mut sz: size_t = 0 as libc::c_int as size_t;
    let mut tmp: ul_buffer = {
        let mut init = ul_buffer {
            begin: 0 as *mut libc::c_char,
            end: 0 as *mut libc::c_char,
            sz: 0,
            chunksize: 0,
            encoded: 0 as *mut libc::c_char,
            encoded_sz: 0,
            ptrs: 0 as *mut *mut libc::c_char,
            nptrs: 0,
        };
        init
    };
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    ssz = ul_path_readlink(pc, buf, bufsz, 0 as *const libc::c_char);
    if ssz <= 0 as libc::c_int as libc::c_long {
        return 0 as *mut libc::c_char;
    }
    p = ul_path_get_prefix(pc);
    if !p.is_null() {
        ul_buffer_append_string(&mut tmp, p);
    }
    ul_buffer_append_string(
        &mut tmp,
        b"/sys/dev/block/\0" as *const u8 as *const libc::c_char,
    );
    ul_buffer_append_data(&mut tmp, buf, ssz as size_t);
    p = ul_buffer_get_data(&mut tmp, &mut sz, 0 as *mut size_t);
    if !p.is_null() && sz < bufsz {
        memcpy(buf as *mut libc::c_void, p as *const libc::c_void, sz);
        res = buf;
    }
    ul_buffer_free_data(&mut tmp);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_blkdev_next_subsystem(
    mut pc: *mut path_cxt,
    mut devchain: *mut libc::c_char,
    mut subsys: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut subbuf: [libc::c_char; 4096] = [0; 4096];
    let mut sub: *mut libc::c_char = 0 as *mut libc::c_char;
    if subsys.is_null() || devchain.is_null() {
        return -(22 as libc::c_int);
    }
    *subsys = 0 as *mut libc::c_char;
    sub = get_subsystem(
        devchain,
        subbuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    );
    if !sub.is_null() {
        *subsys = strdup(sub);
        if (*subsys).is_null() {
            return -(12 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn sysfs_devchain_is_removable(
    mut chain: *mut libc::c_char,
) -> libc::c_int {
    let mut len: size_t = 0;
    let mut buf: [libc::c_char; 20] = [0; 20];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if chain.is_null() || *chain == 0 {
        return 0 as libc::c_int;
    }
    len = strlen(chain);
    if len.wrapping_add(::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
        > 4096 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    loop {
        let mut fd: libc::c_int = 0;
        let mut rc: libc::c_int = 0;
        memcpy(
            chain.offset(len as isize) as *mut libc::c_void,
            b"/removable\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            ::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong,
        );
        fd = open(chain, 0 as libc::c_int);
        if fd != -(1 as libc::c_int) {
            rc = read_all(
                fd,
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
            ) as libc::c_int;
            close(fd);
            if rc > 0 as libc::c_int {
                if strncmp(
                    buf.as_mut_ptr(),
                    b"fixed\0" as *const u8 as *const libc::c_char,
                    ({
                        let mut _min1: libc::c_int = rc;
                        let mut _min2: libc::c_int = 5 as libc::c_int;
                        &mut _min1 as *mut libc::c_int;
                        &mut _min2 as *mut libc::c_int;
                        (if _min1 < _min2 { _min1 } else { _min2 })
                    }) as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return 0 as libc::c_int
                } else if strncmp(
                    buf.as_mut_ptr(),
                    b"removable\0" as *const u8 as *const libc::c_char,
                    ({
                        let mut _min1: libc::c_int = rc;
                        let mut _min2: libc::c_int = 9 as libc::c_int;
                        &mut _min1 as *mut libc::c_int;
                        &mut _min2 as *mut libc::c_int;
                        (if _min1 < _min2 { _min1 } else { _min2 })
                    }) as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    return 1 as libc::c_int
                }
            }
        }
        *chain.offset(len as isize) = '\0' as i32 as libc::c_char;
        p = strrchr(chain, '/' as i32);
        if !p.is_null() {
            *p = '\0' as i32 as libc::c_char;
            len = p.offset_from(chain) as libc::c_long as size_t;
        }
        if p.is_null() {
            break;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_blkdev_is_hotpluggable(
    mut pc: *mut path_cxt,
) -> libc::c_int {
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut chain: *mut libc::c_char = 0 as *mut libc::c_char;
    chain = sysfs_blkdev_get_devchain(
        pc,
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    );
    return sysfs_devchain_is_removable(chain);
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_blkdev_is_removable(
    mut pc: *mut path_cxt,
) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    if ul_path_read_s32(pc, &mut rc, b"removable\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return rc;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_dm_wholedisk(
    mut pc: *mut path_cxt,
    mut diskname: *mut libc::c_char,
    mut len: size_t,
    mut diskdevno: *mut dev_t,
) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    name = sysfs_blkdev_get_slave(pc);
    if name.is_null() {
        return -(1 as libc::c_int);
    }
    if !diskname.is_null() && len != 0 {
        xstrncpy(diskname, name, len);
    }
    if !diskdevno.is_null() {
        *diskdevno = __sysfs_devname_to_devno(
            ul_path_get_prefix(pc),
            name,
            0 as *const libc::c_char,
        );
        if *diskdevno == 0 {
            rc = -(1 as libc::c_int);
        }
    }
    free(name as *mut libc::c_void);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_blkdev_get_wholedisk(
    mut pc: *mut path_cxt,
    mut diskname: *mut libc::c_char,
    mut len: size_t,
    mut diskdevno: *mut dev_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut is_part: libc::c_int = 0 as libc::c_int;
    if pc.is_null() {
        return -(1 as libc::c_int);
    }
    is_part = (ul_path_access(
        pc,
        0 as libc::c_int,
        b"partition\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int) as libc::c_int;
    if is_part == 0 {
        let mut uuid: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut prefix: *mut libc::c_char = 0 as *mut libc::c_char;
        ul_path_read_string(
            pc,
            &mut uuid,
            b"dm/uuid\0" as *const u8 as *const libc::c_char,
        );
        tmp = uuid;
        prefix = if !uuid.is_null() {
            strsep(&mut tmp, b"-\0" as *const u8 as *const libc::c_char)
        } else {
            0 as *mut libc::c_char
        };
        if !prefix.is_null()
            && strncasecmp(
                prefix,
                b"part\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            is_part = 1 as libc::c_int;
        }
        free(uuid as *mut libc::c_void);
        if is_part != 0
            && get_dm_wholedisk(pc, diskname, len, diskdevno) == 0 as libc::c_int
        {
            current_block = 2004056152239539470;
        } else {
            is_part = 0 as libc::c_int;
            current_block = 1394248824506584008;
        }
    } else {
        current_block = 1394248824506584008;
    }
    match current_block {
        1394248824506584008 => {
            if is_part == 0 {
                if !diskname.is_null()
                    && (sysfs_blkdev_get_name(pc, diskname, len)).is_null()
                {
                    current_block = 1752895665913195110;
                } else {
                    if !diskdevno.is_null() {
                        *diskdevno = sysfs_blkdev_get_devno(pc);
                    }
                    current_block = 2004056152239539470;
                }
            } else {
                let mut linkpath: [libc::c_char; 4096] = [0; 4096];
                let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut linklen: ssize_t = 0;
                linklen = ul_path_readlink(
                    pc,
                    linkpath.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
                    0 as *const libc::c_char,
                );
                if linklen < 0 as libc::c_int as libc::c_long {
                    current_block = 1752895665913195110;
                } else {
                    stripoff_last_component(linkpath.as_mut_ptr());
                    name = stripoff_last_component(linkpath.as_mut_ptr());
                    if name.is_null() {
                        current_block = 1752895665913195110;
                    } else {
                        sysfs_devname_sys_to_dev(name);
                        if !diskname.is_null() && len != 0 {
                            xstrncpy(diskname, name, len);
                        }
                        if !diskdevno.is_null() {
                            *diskdevno = __sysfs_devname_to_devno(
                                ul_path_get_prefix(pc),
                                name,
                                0 as *const libc::c_char,
                            );
                            if *diskdevno == 0 {
                                current_block = 1752895665913195110;
                            } else {
                                current_block = 2004056152239539470;
                            }
                        } else {
                            current_block = 2004056152239539470;
                        }
                    }
                }
            }
            match current_block {
                2004056152239539470 => {}
                _ => return -(1 as libc::c_int),
            }
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_devno_to_wholedisk(
    mut devno: dev_t,
    mut diskname: *mut libc::c_char,
    mut len: size_t,
    mut diskdevno: *mut dev_t,
) -> libc::c_int {
    let mut pc: *mut path_cxt = 0 as *mut path_cxt;
    let mut rc: libc::c_int = 0 as libc::c_int;
    if devno == 0 {
        return -(22 as libc::c_int);
    }
    pc = ul_new_sysfs_path(devno, 0 as *mut path_cxt, 0 as *const libc::c_char);
    if pc.is_null() {
        return -(12 as libc::c_int);
    }
    rc = sysfs_blkdev_get_wholedisk(pc, diskname, len, diskdevno);
    ul_unref_path(pc);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_devno_is_dm_private(
    mut devno: dev_t,
    mut uuid: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut pc: *mut path_cxt = 0 as *mut path_cxt;
    let mut id: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rc: libc::c_int = 0 as libc::c_int;
    pc = ul_new_sysfs_path(devno, 0 as *mut path_cxt, 0 as *const libc::c_char);
    if !pc.is_null() {
        if !(ul_path_read_string(
            pc,
            &mut id,
            b"dm/uuid\0" as *const u8 as *const libc::c_char,
        ) <= 0 as libc::c_int || id.is_null())
        {
            if strncmp(
                id,
                b"LVM-\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                let mut p: *mut libc::c_char = strrchr(
                    id.offset(4 as libc::c_int as isize),
                    '-' as i32,
                );
                if !p.is_null()
                    && *p.offset(1 as libc::c_int as isize) as libc::c_int != 0
                {
                    rc = 1 as libc::c_int;
                }
            } else if strncmp(
                id,
                b"stratis-1-private\0" as *const u8 as *const libc::c_char,
                17 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                rc = 1 as libc::c_int;
            }
        }
    }
    ul_unref_path(pc);
    if !uuid.is_null() {
        *uuid = id;
    } else {
        free(id as *mut libc::c_void);
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_devno_is_wholedisk(mut devno: dev_t) -> libc::c_int {
    let mut disk: dev_t = 0;
    if sysfs_devno_to_wholedisk(
        devno,
        0 as *mut libc::c_char,
        0 as libc::c_int as size_t,
        &mut disk,
    ) != 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    return (devno == disk) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_blkdev_scsi_get_hctl(
    mut pc: *mut path_cxt,
    mut h: *mut libc::c_int,
    mut c: *mut libc::c_int,
    mut t: *mut libc::c_int,
    mut l: *mut libc::c_int,
) -> libc::c_int {
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut hctl: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut blk: *mut sysfs_blkdev = 0 as *mut sysfs_blkdev;
    let mut len: ssize_t = 0;
    blk = ul_path_get_dialect(pc) as *mut sysfs_blkdev;
    if blk.is_null() || (*blk).hctl_error() as libc::c_int != 0 {
        return -(22 as libc::c_int);
    }
    if !((*blk).has_hctl() != 0) {
        (*blk).set_hctl_error(1 as libc::c_int as libc::c_uint);
        len = ul_path_readlink(
            pc,
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
            b"device\0" as *const u8 as *const libc::c_char,
        );
        if len < 0 as libc::c_int as libc::c_long {
            return len as libc::c_int;
        }
        hctl = strrchr(buf.as_mut_ptr(), '/' as i32);
        if hctl.is_null() {
            return -(1 as libc::c_int);
        }
        hctl = hctl.offset(1);
        hctl;
        if sscanf(
            hctl,
            b"%u:%u:%u:%u\0" as *const u8 as *const libc::c_char,
            &mut (*blk).scsi_host as *mut libc::c_uint,
            &mut (*blk).scsi_channel as *mut libc::c_uint,
            &mut (*blk).scsi_target as *mut libc::c_uint,
            &mut (*blk).scsi_lun as *mut libc::c_uint,
        ) != 4 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        (*blk).set_has_hctl(1 as libc::c_int as libc::c_uint);
    }
    if !h.is_null() {
        *h = (*blk).scsi_host as libc::c_int;
    }
    if !c.is_null() {
        *c = (*blk).scsi_channel as libc::c_int;
    }
    if !t.is_null() {
        *t = (*blk).scsi_target as libc::c_int;
    }
    if !l.is_null() {
        *l = (*blk).scsi_lun as libc::c_int;
    }
    (*blk).set_hctl_error(0 as libc::c_int as libc::c_uint);
    return 0 as libc::c_int;
}
unsafe extern "C" fn scsi_host_attribute_path(
    mut pc: *mut path_cxt,
    mut type_0: *const libc::c_char,
    mut buf: *mut libc::c_char,
    mut bufsz: size_t,
    mut attr: *const libc::c_char,
) -> *mut libc::c_char {
    let mut len: libc::c_int = 0;
    let mut host: libc::c_int = 0;
    let mut prefix: *const libc::c_char = 0 as *const libc::c_char;
    if sysfs_blkdev_scsi_get_hctl(
        pc,
        &mut host,
        0 as *mut libc::c_int,
        0 as *mut libc::c_int,
        0 as *mut libc::c_int,
    ) != 0
    {
        return 0 as *mut libc::c_char;
    }
    prefix = ul_path_get_prefix(pc);
    if prefix.is_null() {
        prefix = b"\0" as *const u8 as *const libc::c_char;
    }
    if !attr.is_null() {
        len = snprintf(
            buf,
            bufsz,
            b"%s%s/%s_host/host%d/%s\0" as *const u8 as *const libc::c_char,
            prefix,
            b"/sys/class\0" as *const u8 as *const libc::c_char,
            type_0,
            host,
            attr,
        );
    } else {
        len = snprintf(
            buf,
            bufsz,
            b"%s%s/%s_host/host%d\0" as *const u8 as *const libc::c_char,
            prefix,
            b"/sys/class\0" as *const u8 as *const libc::c_char,
            type_0,
            host,
        );
    }
    return if len < 0 as libc::c_int || len as size_t >= bufsz {
        0 as *mut libc::c_char
    } else {
        buf
    };
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_blkdev_scsi_host_strdup_attribute(
    mut pc: *mut path_cxt,
    mut type_0: *const libc::c_char,
    mut attr: *const libc::c_char,
) -> *mut libc::c_char {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut rc: libc::c_int = 0;
    let mut f: *mut FILE = 0 as *mut FILE;
    if attr.is_null() || type_0.is_null()
        || (scsi_host_attribute_path(
            pc,
            type_0,
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            attr,
        ))
            .is_null()
    {
        return 0 as *mut libc::c_char;
    }
    f = fopen(buf.as_mut_ptr(), b"re\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        return 0 as *mut libc::c_char;
    }
    rc = fscanf(
        f,
        b"%1023[^\n]\0" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
    );
    fclose(f);
    return if rc == 1 as libc::c_int {
        strdup(buf.as_mut_ptr())
    } else {
        0 as *mut libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_blkdev_scsi_host_is(
    mut pc: *mut path_cxt,
    mut type_0: *const libc::c_char,
) -> libc::c_int {
    let mut buf: [libc::c_char; 4096] = [0; 4096];
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
    if type_0.is_null()
        || (scsi_host_attribute_path(
            pc,
            type_0,
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
            0 as *const libc::c_char,
        ))
            .is_null()
    {
        return 0 as libc::c_int;
    }
    return (stat(buf.as_mut_ptr(), &mut st) == 0 as libc::c_int
        && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn scsi_attribute_path(
    mut pc: *mut path_cxt,
    mut buf: *mut libc::c_char,
    mut bufsz: size_t,
    mut attr: *const libc::c_char,
) -> *mut libc::c_char {
    let mut len: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut prefix: *const libc::c_char = 0 as *const libc::c_char;
    if sysfs_blkdev_scsi_get_hctl(pc, &mut h, &mut c, &mut t, &mut l) != 0 as libc::c_int
    {
        return 0 as *mut libc::c_char;
    }
    prefix = ul_path_get_prefix(pc);
    if prefix.is_null() {
        prefix = b"\0" as *const u8 as *const libc::c_char;
    }
    if !attr.is_null() {
        len = snprintf(
            buf,
            bufsz,
            b"%s%s/devices/%d:%d:%d:%d/%s\0" as *const u8 as *const libc::c_char,
            prefix,
            b"/sys/bus/scsi\0" as *const u8 as *const libc::c_char,
            h,
            c,
            t,
            l,
            attr,
        );
    } else {
        len = snprintf(
            buf,
            bufsz,
            b"%s%s/devices/%d:%d:%d:%d\0" as *const u8 as *const libc::c_char,
            prefix,
            b"/sys/bus/scsi\0" as *const u8 as *const libc::c_char,
            h,
            c,
            t,
            l,
        );
    }
    return if len < 0 as libc::c_int || len as size_t >= bufsz {
        0 as *mut libc::c_char
    } else {
        buf
    };
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_blkdev_scsi_has_attribute(
    mut pc: *mut path_cxt,
    mut attr: *const libc::c_char,
) -> libc::c_int {
    let mut path: [libc::c_char; 4096] = [0; 4096];
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
    if (scsi_attribute_path(
        pc,
        path.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        attr,
    ))
        .is_null()
    {
        return 0 as libc::c_int;
    }
    return (stat(path.as_mut_ptr(), &mut st) == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_blkdev_scsi_path_contains(
    mut pc: *mut path_cxt,
    mut pattern: *const libc::c_char,
) -> libc::c_int {
    let mut path: [libc::c_char; 4096] = [0; 4096];
    let mut linkc: [libc::c_char; 4096] = [0; 4096];
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
    let mut len: ssize_t = 0;
    if (scsi_attribute_path(
        pc,
        path.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        0 as *const libc::c_char,
    ))
        .is_null()
    {
        return 0 as libc::c_int;
    }
    if stat(path.as_mut_ptr(), &mut st) != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    len = readlink(
        path.as_mut_ptr(),
        linkc.as_mut_ptr(),
        (::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    if len < 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    linkc[len as usize] = '\0' as i32 as libc::c_char;
    return (strstr(linkc.as_mut_ptr(), pattern)
        != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
}
unsafe extern "C" fn read_devno(mut path: *const libc::c_char) -> dev_t {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut maj: libc::c_int = 0 as libc::c_int;
    let mut min: libc::c_int = 0 as libc::c_int;
    let mut dev: dev_t = 0 as libc::c_int as dev_t;
    f = fopen(path, b"re\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        return 0 as libc::c_int as dev_t;
    }
    if fscanf(
        f,
        b"%d:%d\0" as *const u8 as *const libc::c_char,
        &mut maj as *mut libc::c_int,
        &mut min as *mut libc::c_int,
    ) == 2 as libc::c_int
    {
        dev = gnu_dev_makedev(maj as libc::c_uint, min as libc::c_uint);
    }
    fclose(f);
    return dev;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_devname_is_hidden(
    mut prefix: *const libc::c_char,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut hidden: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0;
    let mut f: *mut FILE = 0 as *mut FILE;
    if strncmp(
        b"/dev/\0" as *const u8 as *const libc::c_char,
        name,
        5 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if prefix.is_null() {
        prefix = b"\0" as *const u8 as *const libc::c_char;
    }
    len = snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        b"%s/sys/block/%s/hidden\0" as *const u8 as *const libc::c_char,
        prefix,
        name,
    );
    if len < 0 as libc::c_int
        || (len as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong)
            > ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    f = fopen(buf.as_mut_ptr(), b"re\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        return 0 as libc::c_int;
    }
    rc = fscanf(
        f,
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut hidden as *mut libc::c_int,
    );
    fclose(f);
    return if rc == 1 as libc::c_int { hidden } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn __sysfs_devname_to_devno(
    mut prefix: *const libc::c_char,
    mut name: *const libc::c_char,
    mut parent: *const libc::c_char,
) -> dev_t {
    let mut current_block: u64;
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut _name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut _parent: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dev: dev_t = 0 as libc::c_int as dev_t;
    let mut len: libc::c_int = 0;
    if prefix.is_null() {
        prefix = b"\0" as *const u8 as *const libc::c_char;
    }
    if !name.is_null() {} else {
        __assert_fail(
            b"name\0" as *const u8 as *const libc::c_char,
            b"lib/sysfs.c\0" as *const u8 as *const libc::c_char,
            921 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"dev_t __sysfs_devname_to_devno(const char *, const char *, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8734: {
        if !name.is_null() {} else {
            __assert_fail(
                b"name\0" as *const u8 as *const libc::c_char,
                b"lib/sysfs.c\0" as *const u8 as *const libc::c_char,
                921 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"dev_t __sysfs_devname_to_devno(const char *, const char *, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if strncmp(
        b"/dev/\0" as *const u8 as *const libc::c_char,
        name,
        5 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
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
        if stat(name, &mut st) == 0 as libc::c_int {
            dev = st.st_rdev;
            current_block = 12407301247641151756;
        } else {
            name = name.offset(5 as libc::c_int as isize);
            current_block = 10886091980245723256;
        }
    } else {
        current_block = 10886091980245723256;
    }
    match current_block {
        10886091980245723256 => {
            _name = strdup(name);
            if !_name.is_null() {
                sysfs_devname_dev_to_sys(_name);
                if !parent.is_null() {
                    _parent = strdup(parent);
                    if _parent.is_null() {
                        current_block = 12407301247641151756;
                    } else {
                        current_block = 13586036798005543211;
                    }
                } else {
                    current_block = 13586036798005543211;
                }
                match current_block {
                    12407301247641151756 => {}
                    _ => {
                        if !parent.is_null()
                            && strncmp(
                                b"dm-\0" as *const u8 as *const libc::c_char,
                                name,
                                3 as libc::c_int as libc::c_ulong,
                            ) != 0 as libc::c_int
                        {
                            sysfs_devname_dev_to_sys(_parent);
                            len = snprintf(
                                buf.as_mut_ptr(),
                                ::core::mem::size_of::<[libc::c_char; 4096]>()
                                    as libc::c_ulong,
                                b"%s/sys/block/%s/%s/dev\0" as *const u8
                                    as *const libc::c_char,
                                prefix,
                                _parent,
                                _name,
                            );
                            if !(len < 0 as libc::c_int
                                || len as size_t
                                    >= ::core::mem::size_of::<[libc::c_char; 4096]>()
                                        as libc::c_ulong)
                            {
                                dev = read_devno(buf.as_mut_ptr());
                            }
                        } else {
                            len = snprintf(
                                buf.as_mut_ptr(),
                                ::core::mem::size_of::<[libc::c_char; 4096]>()
                                    as libc::c_ulong,
                                b"%s/sys/block/%s/dev\0" as *const u8
                                    as *const libc::c_char,
                                prefix,
                                _name,
                            );
                            if !(len < 0 as libc::c_int
                                || len as size_t
                                    >= ::core::mem::size_of::<[libc::c_char; 4096]>()
                                        as libc::c_ulong)
                            {
                                dev = read_devno(buf.as_mut_ptr());
                                if dev == 0 && !parent.is_null()
                                    && !(startswith(name, parent)).is_null()
                                {
                                    len = snprintf(
                                        buf.as_mut_ptr(),
                                        ::core::mem::size_of::<[libc::c_char; 4096]>()
                                            as libc::c_ulong,
                                        b"%s/sys/block/%s/%s/dev\0" as *const u8
                                            as *const libc::c_char,
                                        prefix,
                                        _parent,
                                        _name,
                                    );
                                    if len < 0 as libc::c_int
                                        || len as size_t
                                            >= ::core::mem::size_of::<[libc::c_char; 4096]>()
                                                as libc::c_ulong
                                    {
                                        current_block = 12407301247641151756;
                                    } else {
                                        dev = read_devno(buf.as_mut_ptr());
                                        current_block = 5634871135123216486;
                                    }
                                } else {
                                    current_block = 5634871135123216486;
                                }
                                match current_block {
                                    12407301247641151756 => {}
                                    _ => {
                                        if dev == 0 {
                                            len = snprintf(
                                                buf.as_mut_ptr(),
                                                ::core::mem::size_of::<[libc::c_char; 4096]>()
                                                    as libc::c_ulong,
                                                b"%s/sys/block/%s/device/dev\0" as *const u8
                                                    as *const libc::c_char,
                                                prefix,
                                                _name,
                                            );
                                            if !(len < 0 as libc::c_int
                                                || len as size_t
                                                    >= ::core::mem::size_of::<[libc::c_char; 4096]>()
                                                        as libc::c_ulong)
                                            {
                                                dev = read_devno(buf.as_mut_ptr());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    free(_name as *mut libc::c_void);
    free(_parent as *mut libc::c_void);
    return dev;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_devname_to_devno(mut name: *const libc::c_char) -> dev_t {
    return __sysfs_devname_to_devno(
        0 as *const libc::c_char,
        name,
        0 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_blkdev_get_path(
    mut pc: *mut path_cxt,
    mut buf: *mut libc::c_char,
    mut bufsiz: size_t,
) -> *mut libc::c_char {
    let mut name: *const libc::c_char = sysfs_blkdev_get_name(pc, buf, bufsiz);
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sz: size_t = 0;
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
    if !name.is_null() {
        sz = strlen(name);
        if !(sz
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            > bufsiz)
        {
            memmove(
                buf.offset(5 as libc::c_int as isize) as *mut libc::c_void,
                name as *const libc::c_void,
                sz.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            memcpy(
                buf as *mut libc::c_void,
                b"/dev/\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                5 as libc::c_int as libc::c_ulong,
            );
            if stat(buf, &mut st) == 0
                && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o60000 as libc::c_int as libc::c_uint
                && st.st_rdev == sysfs_blkdev_get_devno(pc)
            {
                res = buf;
            }
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_blkdev_get_devno(mut pc: *mut path_cxt) -> dev_t {
    return (*(ul_path_get_dialect(pc) as *mut sysfs_blkdev)).devno;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_devno_to_devpath(
    mut devno: dev_t,
    mut buf: *mut libc::c_char,
    mut bufsiz: size_t,
) -> *mut libc::c_char {
    let mut pc: *mut path_cxt = ul_new_sysfs_path(
        devno,
        0 as *mut path_cxt,
        0 as *const libc::c_char,
    );
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    if !pc.is_null() {
        res = sysfs_blkdev_get_path(pc, buf, bufsiz);
        ul_unref_path(pc);
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_devno_to_devname(
    mut devno: dev_t,
    mut buf: *mut libc::c_char,
    mut bufsiz: size_t,
) -> *mut libc::c_char {
    let mut pc: *mut path_cxt = ul_new_sysfs_path(
        devno,
        0 as *mut path_cxt,
        0 as *const libc::c_char,
    );
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    if !pc.is_null() {
        res = sysfs_blkdev_get_name(pc, buf, bufsiz);
        ul_unref_path(pc);
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_devno_count_partitions(mut devno: dev_t) -> libc::c_int {
    let mut pc: *mut path_cxt = ul_new_sysfs_path(
        devno,
        0 as *mut path_cxt,
        0 as *const libc::c_char,
    );
    let mut n: libc::c_int = 0 as libc::c_int;
    if !pc.is_null() {
        let mut buf: [libc::c_char; 4097] = [0; 4097];
        let mut name: *mut libc::c_char = sysfs_blkdev_get_name(
            pc,
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 4097]>() as libc::c_ulong,
        );
        n = sysfs_blkdev_count_partitions(pc, name);
        ul_unref_path(pc);
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_chrdev_devno_to_devname(
    mut devno: dev_t,
    mut buf: *mut libc::c_char,
    mut bufsiz: size_t,
) -> *mut libc::c_char {
    let mut link: [libc::c_char; 4096] = [0; 4096];
    let mut pc: *mut path_cxt = 0 as *mut path_cxt;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sz: ssize_t = 0;
    pc = ul_new_path(
        b"/sys/dev/char/%u:%u\0" as *const u8 as *const libc::c_char,
        gnu_dev_major(devno),
        gnu_dev_minor(devno),
    );
    if pc.is_null() {
        return 0 as *mut libc::c_char;
    }
    sz = ul_path_readlink(
        pc,
        link.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        0 as *const libc::c_char,
    );
    ul_unref_path(pc);
    if sz < 0 as libc::c_int as libc::c_long {
        return 0 as *mut libc::c_char;
    }
    name = strrchr(link.as_mut_ptr(), '/' as i32);
    if name.is_null() {
        return 0 as *mut libc::c_char;
    }
    name = name.offset(1);
    name;
    sz = strlen(name) as ssize_t;
    if (sz as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong) > bufsiz {
        return 0 as *mut libc::c_char;
    }
    memcpy(
        buf as *mut libc::c_void,
        name as *const libc::c_void,
        (sz + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
    );
    sysfs_devname_sys_to_dev(buf);
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_get_byteorder(mut pc: *mut path_cxt) -> sysfs_byteorder {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut ret: sysfs_byteorder = SYSFS_BYTEORDER_LITTLE;
    rc = ul_path_read_buffer(
        pc,
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
        b"/sys/kernel/cpu_byteorder\0" as *const u8 as *const libc::c_char,
    );
    if rc < 0 as libc::c_int {
        current_block = 12405637863588088403;
    } else if strncmp(
        buf.as_mut_ptr(),
        b"little\0" as *const u8 as *const libc::c_char,
        ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        ret = SYSFS_BYTEORDER_LITTLE;
        current_block = 3556344217153581321;
    } else if strncmp(
        buf.as_mut_ptr(),
        b"big\0" as *const u8 as *const libc::c_char,
        ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        ret = SYSFS_BYTEORDER_BIG;
        current_block = 3556344217153581321;
    } else {
        current_block = 12405637863588088403;
    }
    match current_block {
        12405637863588088403 => {
            ret = SYSFS_BYTEORDER_LITTLE;
        }
        _ => {}
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn sysfs_get_address_bits(mut pc: *mut path_cxt) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut address_bits: libc::c_int = 0;
    rc = ul_path_scanf(
        pc,
        b"/sys/kernel/address_bits\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut address_bits as *mut libc::c_int,
    );
    if rc < 0 as libc::c_int {
        return rc;
    }
    if address_bits < 0 as libc::c_int {
        return -(22 as libc::c_int);
    }
    return address_bits;
}
