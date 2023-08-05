use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn flock(__fd: libc::c_int, __operation: libc::c_int) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    static mut program_invocation_short_name: *mut libc::c_char;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
    fn warn(__format: *const libc::c_char, _: ...);
    fn warnx(__format: *const libc::c_char, _: ...);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
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
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type useconds_t = __useconds_t;
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
pub type __u8 = libc::c_uchar;
pub type __u32 = libc::c_uint;
pub type __u64 = libc::c_ulonglong;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floppy_struct {
    pub size: libc::c_uint,
    pub sect: libc::c_uint,
    pub head: libc::c_uint,
    pub track: libc::c_uint,
    pub stretch: libc::c_uint,
    pub gap: libc::c_uchar,
    pub rate: libc::c_uchar,
    pub spec1: libc::c_uchar,
    pub fmt_gap: libc::c_uchar,
    pub name: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blk_zone {
    pub start: __u64,
    pub len: __u64,
    pub wp: __u64,
    pub type_0: __u8,
    pub cond: __u8,
    pub non_seq: __u8,
    pub reset: __u8,
    pub resv: [__u8; 4],
    pub capacity: __u64,
    pub reserved: [__u8; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blk_zone_report {
    pub sector: __u64,
    pub nr_zones: __u32,
    pub flags: __u32,
    pub zones: [blk_zone; 0],
}
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
pub struct hd_geometry {
    pub heads: libc::c_uchar,
    pub sectors: libc::c_uchar,
    pub cylinders: libc::c_ushort,
    pub start: libc::c_ulong,
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
unsafe extern "C" fn is_same_inode(fd: libc::c_int, mut st: *const stat) -> libc::c_int {
    let mut f: stat = stat {
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
    if fstat(fd, &mut f) < 0 as libc::c_int {
        return 0 as libc::c_int
    } else if f.st_dev != (*st).st_dev || f.st_ino != (*st).st_ino {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn blkdev_valid_offset(
    mut fd: libc::c_int,
    mut offset: off_t,
) -> libc::c_long {
    let mut ch: libc::c_char = 0;
    if lseek(fd, offset, 0 as libc::c_int) < 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as libc::c_long;
    }
    if read_all(fd, &mut ch, 1 as libc::c_int as size_t)
        < 1 as libc::c_int as libc::c_long
    {
        return 0 as libc::c_int as libc::c_long;
    }
    return 1 as libc::c_int as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn is_blkdev(mut fd: libc::c_int) -> libc::c_int {
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
    return (fstat(fd, &mut st) == 0 as libc::c_int
        && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o60000 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn blkdev_find_size(mut fd: libc::c_int) -> off_t {
    let mut high: off_t = 0;
    let mut low: off_t = 0 as libc::c_int as off_t;
    high = 1024 as libc::c_int as off_t;
    while blkdev_valid_offset(fd, high) != 0 {
        if high
            == ((1 as libc::c_int as off_t)
                << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as off_t
                + ((1 as libc::c_int as off_t)
                    << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
        {
            *__errno_location() = 27 as libc::c_int;
            return -(1 as libc::c_int) as off_t;
        }
        low = high;
        if high
            >= (((1 as libc::c_int as off_t)
                << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as off_t
                + ((1 as libc::c_int as off_t)
                    << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong)))
                / 2 as libc::c_int as libc::c_long
        {
            high = ((1 as libc::c_int as off_t)
                << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as off_t
                + ((1 as libc::c_int as off_t)
                    << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong));
        } else {
            high *= 2 as libc::c_int as libc::c_long;
        }
    }
    while low < high - 1 as libc::c_int as libc::c_long {
        let mut mid: off_t = (low + high) / 2 as libc::c_int as libc::c_long;
        if blkdev_valid_offset(fd, mid) != 0 {
            low = mid;
        } else {
            high = mid;
        }
    }
    blkdev_valid_offset(fd, 0 as libc::c_int as off_t);
    return low + 1 as libc::c_int as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn blkdev_get_size(
    mut fd: libc::c_int,
    mut bytes: *mut libc::c_ulonglong,
) -> libc::c_int {
    if ioctl(
        fd,
        ((2 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | ((0x12 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int)
                as libc::c_uint
            | ((114 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
            as libc::c_ulong
            | (::core::mem::size_of::<size_t>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        bytes,
    ) >= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    let mut size: libc::c_ulong = 0;
    if ioctl(
        fd,
        ((0 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | ((0x12 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int)
                as libc::c_uint
            | ((96 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
            | ((0 as libc::c_int)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int)
                as libc::c_uint) as libc::c_ulong,
        &mut size as *mut libc::c_ulong,
    ) >= 0 as libc::c_int
    {
        *bytes = (size as libc::c_ulonglong) << 9 as libc::c_int;
        return 0 as libc::c_int;
    }
    let mut this_floppy: floppy_struct = floppy_struct {
        size: 0,
        sect: 0,
        head: 0,
        track: 0,
        stretch: 0,
        gap: 0,
        rate: 0,
        spec1: 0,
        fmt_gap: 0,
        name: 0 as *const libc::c_char,
    };
    if ioctl(
        fd,
        ((2 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | ((2 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((0x4 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
            as libc::c_ulong
            | (::core::mem::size_of::<floppy_struct>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut this_floppy as *mut floppy_struct,
    ) >= 0 as libc::c_int
    {
        *bytes = (this_floppy.size as libc::c_ulonglong) << 9 as libc::c_int;
        return 0 as libc::c_int;
    }
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
    if fstat(fd, &mut st) == 0 as libc::c_int
        && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint
    {
        *bytes = st.st_size as libc::c_ulonglong;
        return 0 as libc::c_int;
    }
    if !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o60000 as libc::c_int as libc::c_uint)
    {
        *__errno_location() = 15 as libc::c_int;
        return -(1 as libc::c_int);
    }
    *bytes = blkdev_find_size(fd) as libc::c_ulonglong;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn blkdev_get_sectors(
    mut fd: libc::c_int,
    mut sectors: *mut libc::c_ulonglong,
) -> libc::c_int {
    let mut bytes: libc::c_ulonglong = 0;
    if blkdev_get_size(fd, &mut bytes) == 0 as libc::c_int {
        *sectors = bytes >> 9 as libc::c_int;
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn blkdev_get_sector_size(
    mut fd: libc::c_int,
    mut sector_size: *mut libc::c_int,
) -> libc::c_int {
    if ioctl(
        fd,
        ((0 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | ((0x12 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int)
                as libc::c_uint
            | ((104 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
            | ((0 as libc::c_int)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int)
                as libc::c_uint) as libc::c_ulong,
        sector_size,
    ) >= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn blkdev_get_physector_size(
    mut fd: libc::c_int,
    mut sector_size: *mut libc::c_int,
) -> libc::c_int {
    if ioctl(
        fd,
        ((0 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | ((0x12 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int)
                as libc::c_uint
            | ((123 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
            | ((0 as libc::c_int)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int)
                as libc::c_uint) as libc::c_ulong,
        sector_size,
    ) >= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn blkdev_is_misaligned(mut fd: libc::c_int) -> libc::c_int {
    let mut aligned: libc::c_int = 0;
    if ioctl(
        fd,
        ((0 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | ((0x12 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int)
                as libc::c_uint
            | ((122 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
            | ((0 as libc::c_int)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int)
                as libc::c_uint) as libc::c_ulong,
        &mut aligned as *mut libc::c_int,
    ) < 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return if aligned != 0 as libc::c_int { 1 as libc::c_int } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn open_blkdev_or_file(
    mut st: *const stat,
    mut name: *const libc::c_char,
    oflag: libc::c_int,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    if (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o60000 as libc::c_int as libc::c_uint
    {
        fd = open(name, oflag | 0o200 as libc::c_int);
    } else {
        fd = open(name, oflag);
    }
    if -(1 as libc::c_int) < fd && is_same_inode(fd, st) == 0 {
        close(fd);
        *__errno_location() = 77 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if -(1 as libc::c_int) < fd
        && (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o60000 as libc::c_int as libc::c_uint && blkdev_is_misaligned(fd) != 0
    {
        warnx(
            dcgettext(
                0 as *const libc::c_char,
                b"warning: %s is misaligned\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn blkdev_is_cdrom(mut fd: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = ioctl(fd, 0x5331 as libc::c_int as libc::c_ulong, 0 as *mut libc::c_void);
    if ret < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn blkdev_get_geometry(
    mut fd: libc::c_int,
    mut h: *mut libc::c_uint,
    mut s: *mut libc::c_uint,
) -> libc::c_int {
    let mut geometry: hd_geometry = hd_geometry {
        heads: 0,
        sectors: 0,
        cylinders: 0,
        start: 0,
    };
    if ioctl(
        fd,
        0x301 as libc::c_int as libc::c_ulong,
        &mut geometry as *mut hd_geometry,
    ) == 0 as libc::c_int
    {
        *h = geometry.heads as libc::c_uint;
        *s = geometry.sectors as libc::c_uint;
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn blkdev_scsi_type_to_name(
    mut type_0: libc::c_int,
) -> *const libc::c_char {
    match type_0 {
        0 => return b"disk\0" as *const u8 as *const libc::c_char,
        1 => return b"tape\0" as *const u8 as *const libc::c_char,
        2 => return b"printer\0" as *const u8 as *const libc::c_char,
        3 => return b"processor\0" as *const u8 as *const libc::c_char,
        4 => return b"worm\0" as *const u8 as *const libc::c_char,
        5 => return b"rom\0" as *const u8 as *const libc::c_char,
        6 => return b"scanner\0" as *const u8 as *const libc::c_char,
        7 => return b"mo-disk\0" as *const u8 as *const libc::c_char,
        8 => return b"changer\0" as *const u8 as *const libc::c_char,
        9 => return b"comm\0" as *const u8 as *const libc::c_char,
        12 => return b"raid\0" as *const u8 as *const libc::c_char,
        13 => return b"enclosure\0" as *const u8 as *const libc::c_char,
        14 => return b"rbc\0" as *const u8 as *const libc::c_char,
        17 => return b"osd\0" as *const u8 as *const libc::c_char,
        127 => return b"no-lun\0" as *const u8 as *const libc::c_char,
        _ => {}
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn blkdev_lock(
    mut fd: libc::c_int,
    mut devname: *const libc::c_char,
    mut lockmode: *const libc::c_char,
) -> libc::c_int {
    let mut oper: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut msg: libc::c_int = 0 as libc::c_int;
    if lockmode.is_null() {
        lockmode = getenv(b"LOCK_BLOCK_DEVICE\0" as *const u8 as *const libc::c_char);
    }
    if lockmode.is_null() {
        return 0 as libc::c_int;
    }
    if strcasecmp(lockmode, b"yes\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp(lockmode, b"1\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        oper = 2 as libc::c_int;
    } else if strcasecmp(lockmode, b"nonblock\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        oper = 2 as libc::c_int | 4 as libc::c_int;
    } else if strcasecmp(lockmode, b"no\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp(lockmode, b"0\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        return 0 as libc::c_int
    } else {
        warnx(
            dcgettext(
                0 as *const libc::c_char,
                b"unsupported lock mode: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            lockmode,
        );
        return -(22 as libc::c_int);
    }
    if oper & 4 as libc::c_int == 0 {
        rc = flock(fd, oper | 4 as libc::c_int);
        if rc == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        if rc != 0 as libc::c_int && *__errno_location() == 11 as libc::c_int {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: %s: device already locked, waiting to get lock ... \0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                program_invocation_short_name,
                devname,
            );
            msg = 1 as libc::c_int;
        }
    }
    rc = flock(fd, oper);
    if rc != 0 as libc::c_int {
        match *__errno_location() {
            11 => {
                warnx(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: device already locked\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    devname,
                );
            }
            _ => {
                warn(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: failed to get lock\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    devname,
                );
            }
        }
    } else if msg != 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"OK\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn blkdev_get_zonereport(
    mut fd: libc::c_int,
    mut sector: uint64_t,
    mut nzones: uint32_t,
) -> *mut blk_zone_report {
    let mut rep: *mut blk_zone_report = 0 as *mut blk_zone_report;
    let mut rep_size: size_t = 0;
    let mut ret: libc::c_int = 0;
    rep_size = (::core::mem::size_of::<blk_zone_report>() as libc::c_ulong)
        .wrapping_add(
            (::core::mem::size_of::<blk_zone>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        );
    rep = calloc(1 as libc::c_int as libc::c_ulong, rep_size) as *mut blk_zone_report;
    if rep.is_null() {
        return 0 as *mut blk_zone_report;
    }
    (*rep).sector = sector as __u64;
    (*rep).nr_zones = nzones;
    ret = ioctl(
        fd,
        ((2 as libc::c_uint | 1 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | ((0x12 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int)
                as libc::c_uint
            | ((130 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
            as libc::c_ulong
            | (::core::mem::size_of::<blk_zone_report>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        rep,
    );
    if ret != 0 || (*rep).nr_zones != nzones {
        free(rep as *mut libc::c_void);
        return 0 as *mut blk_zone_report;
    }
    return rep;
}
