use ::libc;
extern "C" {
    pub type __dirstream;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn asprintf(
        __ptr: *mut *mut libc::c_char,
        __fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn mkostemp(__template: *mut libc::c_char, __flags: libc::c_int) -> libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn readlink(
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn getdtablesize() -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
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
    fn sendfile(
        __out_fd: libc::c_int,
        __in_fd: libc::c_int,
        __offset: *mut off_t,
        __count: size_t,
    ) -> ssize_t;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn dirfd(__dirp: *mut DIR) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type mode_t = __mode_t;
pub type useconds_t = __useconds_t;
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
unsafe extern "C" fn sendfile_all(
    mut out: libc::c_int,
    mut in_0: libc::c_int,
    mut off: *mut off_t,
    mut count: size_t,
) -> ssize_t {
    let mut ret: ssize_t = 0;
    let mut c: ssize_t = 0 as libc::c_int as ssize_t;
    let mut tries: libc::c_int = 0 as libc::c_int;
    while count != 0 {
        ret = sendfile(out, in_0, off, count);
        if ret < 0 as libc::c_int as libc::c_long {
            if (*__errno_location() == 11 as libc::c_int
                || *__errno_location() == 4 as libc::c_int)
                && {
                    let fresh1 = tries;
                    tries = tries + 1;
                    fresh1 < 5 as libc::c_int
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
            c += ret;
        }
    }
    return c;
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
#[no_mangle]
pub unsafe extern "C" fn mkstemp_cloexec(
    mut template: *mut libc::c_char,
) -> libc::c_int {
    return mkostemp(
        template,
        0o2 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int
            | 0o2000000 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmkstemp(
    mut tmpname: *mut *mut libc::c_char,
    mut dir: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> libc::c_int {
    let mut localtmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmpenv: *const libc::c_char = 0 as *const libc::c_char;
    let mut old_mode: mode_t = 0;
    let mut fd: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    tmpenv = if !dir.is_null() {
        dir
    } else {
        getenv(b"TMPDIR\0" as *const u8 as *const libc::c_char) as *const libc::c_char
    };
    if tmpenv.is_null() {
        tmpenv = b"/tmp/\0" as *const u8 as *const libc::c_char;
    }
    rc = asprintf(
        &mut localtmp as *mut *mut libc::c_char,
        b"%s/%s.XXXXXX\0" as *const u8 as *const libc::c_char,
        tmpenv,
        prefix,
    );
    if rc < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    old_mode = umask(0o77 as libc::c_int as __mode_t);
    fd = mkstemp_cloexec(localtmp);
    umask(old_mode);
    if fd == -(1 as libc::c_int) {
        free(localtmp as *mut libc::c_void);
        localtmp = 0 as *mut libc::c_char;
    }
    *tmpname = localtmp;
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn dup_fd_cloexec(
    mut oldfd: libc::c_int,
    mut lowfd: libc::c_int,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut errno_save: libc::c_int = 0;
    fd = fcntl(oldfd, 1030 as libc::c_int, lowfd);
    if fd >= 0 as libc::c_int {
        return fd;
    }
    fd = dup(oldfd);
    if fd < 0 as libc::c_int {
        return fd;
    }
    flags = fcntl(fd, 1 as libc::c_int);
    if !(flags < 0 as libc::c_int) {
        if !(fcntl(fd, 2 as libc::c_int, flags | 1 as libc::c_int) < 0 as libc::c_int) {
            return fd;
        }
    }
    errno_save = *__errno_location();
    close(fd);
    *__errno_location() = errno_save;
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn get_fd_tabsize() -> libc::c_uint {
    let mut m: libc::c_int = 0;
    m = getdtablesize();
    return m as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn ul_close_all_fds(
    mut first: libc::c_uint,
    mut last: libc::c_uint,
) {
    let mut d: *mut dirent = 0 as *mut dirent;
    let mut dir: *mut DIR = 0 as *mut DIR;
    dir = opendir(b"/proc/self/fd\0" as *const u8 as *const libc::c_char);
    if !dir.is_null() {
        loop {
            d = xreaddir(dir);
            if d.is_null() {
                break;
            }
            let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut fd: libc::c_uint = 0;
            let mut dfd: libc::c_int = 0;
            *__errno_location() = 0 as libc::c_int;
            fd = strtoul(((*d).d_name).as_mut_ptr(), &mut end, 10 as libc::c_int)
                as libc::c_uint;
            if *__errno_location() != 0 || end == ((*d).d_name).as_mut_ptr()
                || end.is_null() || *end as libc::c_int != 0
            {
                continue;
            }
            dfd = dirfd(dir);
            if dfd < 0 as libc::c_int {
                continue;
            }
            if dfd as libc::c_uint == fd {
                continue;
            }
            if fd < first || last < fd {
                continue;
            }
            close(fd as libc::c_int);
        }
        closedir(dir);
    } else {
        let mut fd_0: libc::c_uint = 0;
        let mut tbsz: libc::c_uint = get_fd_tabsize();
        fd_0 = 0 as libc::c_int as libc::c_uint;
        while fd_0 < tbsz {
            if first <= fd_0 && fd_0 <= last {
                close(fd_0 as libc::c_int);
            }
            fd_0 = fd_0.wrapping_add(1);
            fd_0;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ul_mkdir_p(
    mut path: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rc: libc::c_int = 0 as libc::c_int;
    if path.is_null() || *path == 0 {
        return -(22 as libc::c_int);
    }
    p = strdup(path);
    dir = p;
    if dir.is_null() {
        return -(12 as libc::c_int);
    }
    if *p as libc::c_int == '/' as i32 {
        p = p.offset(1);
        p;
    }
    while !p.is_null() && *p as libc::c_int != 0 {
        let mut e: *mut libc::c_char = strchr(p, '/' as i32);
        if !e.is_null() {
            *e = '\0' as i32 as libc::c_char;
        }
        if *p != 0 {
            rc = mkdir(dir, mode);
            if rc != 0 && *__errno_location() != 17 as libc::c_int {
                break;
            }
            rc = 0 as libc::c_int;
        }
        if e.is_null() {
            break;
        }
        *e = '/' as i32 as libc::c_char;
        p = e.offset(1 as libc::c_int as isize);
    }
    free(dir as *mut libc::c_void);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn stripoff_last_component(
    mut path: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = if !path.is_null() {
        strrchr(path, '/' as i32)
    } else {
        0 as *mut libc::c_char
    };
    if p.is_null() {
        return 0 as *mut libc::c_char;
    }
    *p = '\0' as i32 as libc::c_char;
    return p.offset(1 as libc::c_int as isize);
}
unsafe extern "C" fn copy_file_simple(
    mut from: libc::c_int,
    mut to: libc::c_int,
) -> libc::c_int {
    let mut nr: ssize_t = 0;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    loop {
        nr = read_all(
            from,
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
        );
        if !(nr > 0 as libc::c_int as libc::c_long) {
            break;
        }
        if write_all(to, buf.as_mut_ptr() as *const libc::c_void, nr as size_t)
            == -(1 as libc::c_int)
        {
            return -(2 as libc::c_int);
        }
    }
    if nr < 0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int);
    }
    explicit_bzero(
        buf.as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_copy_file(
    mut from: libc::c_int,
    mut to: libc::c_int,
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
    let mut nw: ssize_t = 0;
    if fstat(from, &mut st) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint)
    {
        return copy_file_simple(from, to);
    }
    if sendfile_all(to, from, 0 as *mut off_t, st.st_size as size_t)
        < 0 as libc::c_int as libc::c_long
    {
        return copy_file_simple(from, to);
    }
    loop {
        nw = sendfile_all(
            to,
            from,
            0 as *mut off_t,
            (16 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int) as size_t,
        );
        if !(nw != 0 as libc::c_int as libc::c_long) {
            break;
        }
        if nw < 0 as libc::c_int as libc::c_long {
            return copy_file_simple(from, to);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_reopen(
    mut fd: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut ssz: ssize_t = 0;
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut fdpath: [libc::c_char; 25] = [0; 25];
    snprintf(
        fdpath.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong,
        b"/proc/self/fd/%d\0" as *const u8 as *const libc::c_char,
        fd,
    );
    ssz = readlink(
        fdpath.as_mut_ptr(),
        buf.as_mut_ptr(),
        (::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    if ssz < 0 as libc::c_int as libc::c_long {
        return -*__errno_location();
    }
    if ssz > 0 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"ssz > 0\0" as *const u8 as *const libc::c_char,
            b"lib/fileutils.c\0" as *const u8 as *const libc::c_char,
            308 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"int ul_reopen(int, int)\0"))
                .as_ptr(),
        );
    }
    'c_6603: {
        if ssz > 0 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"ssz > 0\0" as *const u8 as *const libc::c_char,
                b"lib/fileutils.c\0" as *const u8 as *const libc::c_char,
                308 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 24],
                    &[libc::c_char; 24],
                >(b"int ul_reopen(int, int)\0"))
                    .as_ptr(),
            );
        }
    };
    buf[ssz as usize] = '\0' as i32 as libc::c_char;
    return open(buf.as_mut_ptr(), flags);
}
