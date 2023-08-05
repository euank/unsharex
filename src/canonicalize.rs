use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
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
    fn mempcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn getuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn setgid(__gid: __gid_t) -> libc::c_int;
    fn fork() -> __pid_t;
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn realpath(
        __name: *const libc::c_char,
        __resolved: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub type useconds_t = __useconds_t;
pub type pid_t = __pid_t;
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
#[inline]
unsafe extern "C" fn drop_permissions() -> libc::c_int {
    *__errno_location() = 0 as libc::c_int;
    if !(setgid(getgid()) < 0 as libc::c_int) {
        if !(setuid(getuid()) < 0 as libc::c_int) {
            return 0 as libc::c_int;
        }
    }
    return if *__errno_location() != 0 {
        -*__errno_location()
    } else {
        -(1 as libc::c_int)
    };
}
#[inline]
unsafe extern "C" fn is_relative_path(mut path: *const libc::c_char) -> libc::c_int {
    if path.is_null() || *path as libc::c_int == '/' as i32 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn __canonicalize_dm_name(
    mut prefix: *const libc::c_char,
    mut ptname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut sz: size_t = 0;
    let mut path: [libc::c_char; 256] = [0; 256];
    let mut name: [libc::c_char; 244] = [0; 244];
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    if ptname.is_null() || *ptname == 0 {
        return 0 as *mut libc::c_char;
    }
    if prefix.is_null() {
        prefix = b"\0" as *const u8 as *const libc::c_char;
    }
    snprintf(
        path.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        b"%s/sys/block/%s/dm/name\0" as *const u8 as *const libc::c_char,
        prefix,
        ptname,
    );
    f = fopen(path.as_mut_ptr(), b"re\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        return 0 as *mut libc::c_char;
    }
    if !(fgets(
        name.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 244]>() as libc::c_ulong as libc::c_int,
        f,
    ))
        .is_null()
        && {
            sz = strlen(name.as_mut_ptr());
            sz > 1 as libc::c_int as libc::c_ulong
        }
    {
        name[sz.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] = '\0' as i32 as libc::c_char;
        snprintf(
            path.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            b"/dev/mapper/%s\0" as *const u8 as *const libc::c_char,
            name.as_mut_ptr(),
        );
        if !prefix.is_null() && *prefix as libc::c_int != 0
            || access(path.as_mut_ptr(), 0 as libc::c_int) == 0 as libc::c_int
        {
            res = strdup(path.as_mut_ptr());
        }
    }
    fclose(f);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn canonicalize_dm_name(
    mut ptname: *const libc::c_char,
) -> *mut libc::c_char {
    return __canonicalize_dm_name(0 as *const libc::c_char, ptname);
}
unsafe extern "C" fn is_dm_devname(
    mut canonical: *mut libc::c_char,
    mut name: *mut *mut libc::c_char,
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
    let mut p: *mut libc::c_char = strrchr(canonical, '/' as i32);
    *name = 0 as *mut libc::c_char;
    if p.is_null()
        || strncmp(
            p,
            b"/dm-\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
        || *(*__ctype_b_loc())
            .offset(*p.offset(4 as libc::c_int as isize) as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0 || stat(canonical, &mut sb) != 0 as libc::c_int
        || !(sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o60000 as libc::c_int as libc::c_uint)
    {
        return 0 as libc::c_int;
    }
    *name = p.offset(1 as libc::c_int as isize);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn absolute_path(
    mut path: *const libc::c_char,
) -> *mut libc::c_char {
    let mut cwd: [libc::c_char; 4096] = [0; 4096];
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut psz: size_t = 0;
    let mut csz: size_t = 0;
    if is_relative_path(path) == 0 {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut libc::c_char;
    }
    if (getcwd(
        cwd.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    ))
        .is_null()
    {
        return 0 as *mut libc::c_char;
    }
    if !(startswith(path, b"./\0" as *const u8 as *const libc::c_char)).is_null() {
        path = path.offset(2 as libc::c_int as isize);
    } else if strcmp(path, b".\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        path = 0 as *const libc::c_char;
    }
    if path.is_null() || *path == 0 {
        return strdup(cwd.as_mut_ptr());
    }
    csz = strlen(cwd.as_mut_ptr());
    psz = strlen(path);
    res = malloc(
        csz
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(psz)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    p = res;
    if res.is_null() {
        return 0 as *mut libc::c_char;
    }
    p = mempcpy(p as *mut libc::c_void, cwd.as_mut_ptr() as *const libc::c_void, csz)
        as *mut libc::c_char;
    let fresh1 = p;
    p = p.offset(1);
    *fresh1 = '/' as i32 as libc::c_char;
    memcpy(
        p as *mut libc::c_void,
        path as *const libc::c_void,
        psz.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn canonicalize_path(
    mut path: *const libc::c_char,
) -> *mut libc::c_char {
    let mut canonical: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dmname: *mut libc::c_char = 0 as *mut libc::c_char;
    if path.is_null() || *path == 0 {
        return 0 as *mut libc::c_char;
    }
    canonical = realpath(path, 0 as *mut libc::c_char);
    if canonical.is_null() {
        return strdup(path);
    }
    if is_dm_devname(canonical, &mut dmname) != 0 {
        let mut dm: *mut libc::c_char = canonicalize_dm_name(dmname);
        if !dm.is_null() {
            free(canonical as *mut libc::c_void);
            return dm;
        }
    }
    return canonical;
}
#[no_mangle]
pub unsafe extern "C" fn canonicalize_path_restricted(
    mut path: *const libc::c_char,
) -> *mut libc::c_char {
    let mut canonical: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut errsv: libc::c_int = 0 as libc::c_int;
    let mut pipes: [libc::c_int; 2] = [0; 2];
    let mut len: ssize_t = 0;
    let mut pid: pid_t = 0;
    if path.is_null() || *path == 0 {
        return 0 as *mut libc::c_char;
    }
    if pipe(pipes.as_mut_ptr()) != 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    pid = fork();
    match pid {
        -1 => {
            close(pipes[0 as libc::c_int as usize]);
            close(pipes[1 as libc::c_int as usize]);
            return 0 as *mut libc::c_char;
        }
        0 => {
            close(pipes[0 as libc::c_int as usize]);
            pipes[0 as libc::c_int as usize] = -(1 as libc::c_int);
            *__errno_location() = 0 as libc::c_int;
            if drop_permissions() != 0 as libc::c_int {
                canonical = 0 as *mut libc::c_char;
            } else {
                let mut dmname: *mut libc::c_char = 0 as *mut libc::c_char;
                canonical = realpath(path, 0 as *mut libc::c_char);
                if !canonical.is_null() && is_dm_devname(canonical, &mut dmname) != 0 {
                    let mut dm: *mut libc::c_char = canonicalize_dm_name(dmname);
                    if !dm.is_null() {
                        free(canonical as *mut libc::c_void);
                        canonical = dm;
                    }
                }
            }
            len = if !canonical.is_null() {
                strlen(canonical) as ssize_t
            } else {
                (if *__errno_location() != 0 {
                    -*__errno_location()
                } else {
                    -(22 as libc::c_int)
                }) as libc::c_long
            };
            write_all(
                pipes[1 as libc::c_int as usize],
                &mut len as *mut ssize_t as *mut libc::c_char as *const libc::c_void,
                ::core::mem::size_of::<ssize_t>() as libc::c_ulong,
            );
            if !canonical.is_null() {
                write_all(
                    pipes[1 as libc::c_int as usize],
                    canonical as *const libc::c_void,
                    len as size_t,
                );
            }
            exit(0 as libc::c_int);
        }
        _ => {}
    }
    close(pipes[1 as libc::c_int as usize]);
    pipes[1 as libc::c_int as usize] = -(1 as libc::c_int);
    if !(read_all(
        pipes[0 as libc::c_int as usize],
        &mut len as *mut ssize_t as *mut libc::c_char,
        ::core::mem::size_of::<ssize_t>() as libc::c_ulong,
    ) as libc::c_ulong != ::core::mem::size_of::<ssize_t>() as libc::c_ulong)
    {
        if len < 0 as libc::c_int as libc::c_long {
            errsv = -len as libc::c_int;
        } else {
            canonical = malloc((len + 1 as libc::c_int as libc::c_long) as libc::c_ulong)
                as *mut libc::c_char;
            if canonical.is_null() {
                errsv = 12 as libc::c_int;
            } else if read_all(
                pipes[0 as libc::c_int as usize],
                canonical,
                len as size_t,
            ) != len
            {
                errsv = *__errno_location();
            } else {
                *canonical.offset(len as isize) = '\0' as i32 as libc::c_char;
            }
        }
    }
    if errsv != 0 {
        free(canonical as *mut libc::c_void);
        canonical = 0 as *mut libc::c_char;
    }
    close(pipes[0 as libc::c_int as usize]);
    let mut __dummy: __pid_t = waitpid(pid, 0 as *mut libc::c_int, 0 as libc::c_int);
    *__errno_location() = errsv;
    return canonical;
}
