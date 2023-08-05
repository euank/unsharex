use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn secure_getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
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
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn prctl(__option: libc::c_int, _: ...) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    static mut environ: *mut *mut libc::c_char;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getegid() -> __gid_t;
    fn __errno_location() -> *mut libc::c_int;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type useconds_t = __useconds_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ul_env_list {
    pub env: *mut libc::c_char,
    pub next: *mut ul_env_list,
}
#[inline]
unsafe extern "C" fn remote_entry(
    mut argv: *mut *mut libc::c_char,
    mut remove: libc::c_int,
    mut last: libc::c_int,
) -> libc::c_int {
    memmove(
        argv.offset(remove as isize) as *mut libc::c_void,
        argv.offset(remove as isize).offset(1 as libc::c_int as isize)
            as *const libc::c_void,
        (::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul((last - remove) as libc::c_ulong),
    );
    return last - 1 as libc::c_int;
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
unsafe extern "C" fn read_all_alloc(
    mut fd: libc::c_int,
    mut buf: *mut *mut libc::c_char,
) -> ssize_t {
    let mut size: size_t = 1024 as libc::c_int as size_t;
    let mut c: size_t = 0 as libc::c_int as size_t;
    let mut ret: ssize_t = 0;
    *buf = malloc(size) as *mut libc::c_char;
    if (*buf).is_null() {
        return -(1 as libc::c_int) as ssize_t;
    }
    loop {
        ret = read_all(fd, (*buf).offset(c as isize), size.wrapping_sub(c));
        if ret < 0 as libc::c_int as libc::c_long {
            free(*buf as *mut libc::c_void);
            *buf = 0 as *mut libc::c_char;
            return -(1 as libc::c_int) as ssize_t;
        }
        if ret == 0 as libc::c_int as libc::c_long {
            return c as ssize_t;
        }
        c = (c as libc::c_ulong).wrapping_add(ret as libc::c_ulong) as size_t as size_t;
        if c == size {
            size = (size as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            *buf = realloc(*buf as *mut libc::c_void, size) as *mut libc::c_char;
            if (*buf).is_null() {
                return -(1 as libc::c_int) as ssize_t;
            }
        }
    };
}
static mut forbid: [*mut libc::c_char; 13] = [
    b"BASH_ENV=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ENV=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"HOME=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"IFS=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"KRB_CONF=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"LD_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"LIBPATH=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MAIL=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"NLSPATH=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"PATH=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SHELL=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SHLIB_PATH=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
static mut noslash: [*mut libc::c_char; 4] = [
    b"LANG=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"LANGUAGE=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"LC_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
unsafe extern "C" fn env_list_add(
    mut ls0: *mut ul_env_list,
    mut str: *const libc::c_char,
) -> *mut ul_env_list {
    let mut ls: *mut ul_env_list = 0 as *mut ul_env_list;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sz: size_t = 0 as libc::c_int as size_t;
    if str.is_null() || *str == 0 {
        return ls0;
    }
    sz = (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    p = malloc((::core::mem::size_of::<ul_env_list>() as libc::c_ulong).wrapping_add(sz))
        as *mut libc::c_char;
    if p.is_null() {
        return ls0;
    }
    ls = p as *mut ul_env_list;
    p = p.offset(::core::mem::size_of::<ul_env_list>() as libc::c_ulong as isize);
    memcpy(p as *mut libc::c_void, str as *const libc::c_void, sz);
    (*ls).env = p;
    (*ls).next = ls0;
    return ls;
}
#[no_mangle]
pub unsafe extern "C" fn env_from_fd(mut fd: libc::c_int) -> *mut ul_env_list {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rc: size_t = 0 as libc::c_int as size_t;
    let mut ls: *mut ul_env_list = 0 as *mut ul_env_list;
    rc = read_all_alloc(fd, &mut buf) as size_t;
    if rc < 1 as libc::c_int as libc::c_ulong {
        return 0 as *mut ul_env_list;
    }
    *buf.offset(rc as isize) = '\0' as i32 as libc::c_char;
    p = buf;
    while rc > 0 as libc::c_int as libc::c_ulong {
        ls = env_list_add(ls, p);
        p = p
            .offset(
                (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
        rc = (rc as libc::c_ulong)
            .wrapping_sub((strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong))
            as size_t as size_t;
    }
    free(buf as *mut libc::c_void);
    return ls;
}
#[no_mangle]
pub unsafe extern "C" fn env_list_setenv(mut ls: *mut ul_env_list) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    while !ls.is_null() && rc == 0 as libc::c_int {
        if !((*ls).env).is_null() {
            let mut val: *mut libc::c_char = strchr((*ls).env, '=' as i32);
            if val.is_null() {
                continue;
            }
            *val = '\0' as i32 as libc::c_char;
            rc = setenv(
                (*ls).env,
                val.offset(1 as libc::c_int as isize),
                0 as libc::c_int,
            );
            *val = '=' as i32 as libc::c_char;
        }
        ls = (*ls).next;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn env_list_free(mut ls: *mut ul_env_list) {
    while !ls.is_null() {
        let mut x: *mut ul_env_list = ls;
        ls = (*ls).next;
        free(x as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn __sanitize_env(mut org: *mut *mut ul_env_list) {
    let mut envp: *mut *mut libc::c_char = environ;
    let mut bad: *const *mut libc::c_char = 0 as *const *mut libc::c_char;
    let mut cur: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut last: libc::c_int = 0 as libc::c_int;
    cur = envp;
    while !(*cur).is_null() {
        last += 1;
        last;
        cur = cur.offset(1);
        cur;
    }
    cur = envp;
    while !(*cur).is_null() {
        bad = forbid.as_ptr();
        while !(*bad).is_null() {
            if strncmp(*cur, *bad, strlen(*bad)) == 0 as libc::c_int {
                if !org.is_null() {
                    *org = env_list_add(*org, *cur);
                }
                last = remote_entry(
                    envp,
                    cur.offset_from(envp) as libc::c_long as libc::c_int,
                    last,
                );
                cur = cur.offset(-1);
                cur;
                break;
            } else {
                bad = bad.offset(1);
                bad;
            }
        }
        cur = cur.offset(1);
        cur;
    }
    cur = envp;
    while !(*cur).is_null() {
        bad = noslash.as_ptr();
        while !(*bad).is_null() {
            if !(strncmp(*cur, *bad, strlen(*bad)) != 0 as libc::c_int) {
                if !(strchr(*cur, '/' as i32)).is_null() {
                    if !org.is_null() {
                        *org = env_list_add(*org, *cur);
                    }
                    last = remote_entry(
                        envp,
                        cur.offset_from(envp) as libc::c_long as libc::c_int,
                        last,
                    );
                    cur = cur.offset(-1);
                    cur;
                    break;
                }
            }
            bad = bad.offset(1);
            bad;
        }
        cur = cur.offset(1);
        cur;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sanitize_env() {
    __sanitize_env(0 as *mut *mut ul_env_list);
}
#[no_mangle]
pub unsafe extern "C" fn safe_getenv(mut arg: *const libc::c_char) -> *mut libc::c_char {
    if getuid() != geteuid() || getgid() != getegid() {
        return 0 as *mut libc::c_char;
    }
    if prctl(
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {
        return 0 as *mut libc::c_char;
    }
    return secure_getenv(arg);
}
