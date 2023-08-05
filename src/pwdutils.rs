use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn getuid() -> __uid_t;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn getgrnam_r(
        __name: *const libc::c_char,
        __resultbuf: *mut group,
        __buffer: *mut libc::c_char,
        __buflen: size_t,
        __result: *mut *mut group,
    ) -> libc::c_int;
    fn err(__status: libc::c_int, __format: *const libc::c_char, _: ...) -> !;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getpwuid_r(
        __uid: __uid_t,
        __resultbuf: *mut passwd,
        __buffer: *mut libc::c_char,
        __buflen: size_t,
        __result: *mut *mut passwd,
    ) -> libc::c_int;
    fn getpwnam_r(
        __name: *const libc::c_char,
        __resultbuf: *mut passwd,
        __buffer: *mut libc::c_char,
        __buflen: size_t,
        __result: *mut *mut passwd,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type uid_t = __uid_t;
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
unsafe extern "C" fn xcalloc(nelems: size_t, size: size_t) -> *mut libc::c_void {
    let ret: *mut libc::c_void = calloc(nelems, size);
    if ret.is_null() && size != 0 && nelems != 0 {
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
    'c_5094: {
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
#[no_mangle]
pub unsafe extern "C" fn xgetpwnam(
    username: *const libc::c_char,
    pwdbuf: *mut *mut libc::c_char,
) -> *mut passwd {
    let mut pwd: *mut passwd = 0 as *mut passwd;
    let mut res: *mut passwd = 0 as *mut passwd;
    let mut rc: libc::c_int = 0;
    if !pwdbuf.is_null() {
    } else {
        __assert_fail(
            b"pwdbuf\0" as *const u8 as *const libc::c_char,
            b"lib/pwdutils.c\0" as *const u8 as *const libc::c_char,
            21 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                b"struct passwd *xgetpwnam(const char *, char **)\0",
            ))
            .as_ptr(),
        );
    }
    'c_4646: {
        if !pwdbuf.is_null() {
        } else {
            __assert_fail(
                b"pwdbuf\0" as *const u8 as *const libc::c_char,
                b"lib/pwdutils.c\0" as *const u8 as *const libc::c_char,
                21 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"struct passwd *xgetpwnam(const char *, char **)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if !username.is_null() {
    } else {
        __assert_fail(
            b"username\0" as *const u8 as *const libc::c_char,
            b"lib/pwdutils.c\0" as *const u8 as *const libc::c_char,
            22 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                b"struct passwd *xgetpwnam(const char *, char **)\0",
            ))
            .as_ptr(),
        );
    }
    'c_4609: {
        if !username.is_null() {
        } else {
            __assert_fail(
                b"username\0" as *const u8 as *const libc::c_char,
                b"lib/pwdutils.c\0" as *const u8 as *const libc::c_char,
                22 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"struct passwd *xgetpwnam(const char *, char **)\0",
                ))
                .as_ptr(),
            );
        }
    };
    *pwdbuf = xmalloc((16 as libc::c_int * 1024 as libc::c_int) as size_t) as *mut libc::c_char;
    pwd = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<passwd>() as libc::c_ulong,
    ) as *mut passwd;
    *__errno_location() = 0 as libc::c_int;
    rc = getpwnam_r(
        username,
        pwd,
        *pwdbuf,
        (16 as libc::c_int * 1024 as libc::c_int) as size_t,
        &mut res,
    );
    if rc != 0 as libc::c_int {
        *__errno_location() = rc;
    } else if res.is_null() {
        *__errno_location() = 22 as libc::c_int;
    } else {
        return pwd;
    }
    free(pwd as *mut libc::c_void);
    free(*pwdbuf as *mut libc::c_void);
    return 0 as *mut passwd;
}
#[no_mangle]
pub unsafe extern "C" fn xgetgrnam(
    groupname: *const libc::c_char,
    grpbuf: *mut *mut libc::c_char,
) -> *mut group {
    let mut grp: *mut group = 0 as *mut group;
    let mut res: *mut group = 0 as *mut group;
    let mut rc: libc::c_int = 0;
    if !grpbuf.is_null() {
    } else {
        __assert_fail(
            b"grpbuf\0" as *const u8 as *const libc::c_char,
            b"lib/pwdutils.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                b"struct group *xgetgrnam(const char *, char **)\0",
            ))
            .as_ptr(),
        );
    }
    'c_4842: {
        if !grpbuf.is_null() {
        } else {
            __assert_fail(
                b"grpbuf\0" as *const u8 as *const libc::c_char,
                b"lib/pwdutils.c\0" as *const u8 as *const libc::c_char,
                53 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                    b"struct group *xgetgrnam(const char *, char **)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if !groupname.is_null() {
    } else {
        __assert_fail(
            b"groupname\0" as *const u8 as *const libc::c_char,
            b"lib/pwdutils.c\0" as *const u8 as *const libc::c_char,
            54 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                b"struct group *xgetgrnam(const char *, char **)\0",
            ))
            .as_ptr(),
        );
    }
    'c_4808: {
        if !groupname.is_null() {
        } else {
            __assert_fail(
                b"groupname\0" as *const u8 as *const libc::c_char,
                b"lib/pwdutils.c\0" as *const u8 as *const libc::c_char,
                54 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                    b"struct group *xgetgrnam(const char *, char **)\0",
                ))
                .as_ptr(),
            );
        }
    };
    *grpbuf = xmalloc((16 as libc::c_int * 1024 as libc::c_int) as size_t) as *mut libc::c_char;
    grp = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<group>() as libc::c_ulong,
    ) as *mut group;
    *__errno_location() = 0 as libc::c_int;
    rc = getgrnam_r(
        groupname,
        grp,
        *grpbuf,
        (16 as libc::c_int * 1024 as libc::c_int) as size_t,
        &mut res,
    );
    if rc != 0 as libc::c_int {
        *__errno_location() = rc;
    } else if res.is_null() {
        *__errno_location() = 22 as libc::c_int;
    } else {
        return grp;
    }
    free(grp as *mut libc::c_void);
    free(*grpbuf as *mut libc::c_void);
    return 0 as *mut group;
}
#[no_mangle]
pub unsafe extern "C" fn xgetpwuid(uid: uid_t, pwdbuf: *mut *mut libc::c_char) -> *mut passwd {
    let mut pwd: *mut passwd = 0 as *mut passwd;
    let mut res: *mut passwd = 0 as *mut passwd;
    let mut rc: libc::c_int = 0;
    if !pwdbuf.is_null() {
    } else {
        __assert_fail(
            b"pwdbuf\0" as *const u8 as *const libc::c_char,
            b"lib/pwdutils.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                b"struct passwd *xgetpwuid(uid_t, char **)\0",
            ))
            .as_ptr(),
        );
    }
    'c_5003: {
        if !pwdbuf.is_null() {
        } else {
            __assert_fail(
                b"pwdbuf\0" as *const u8 as *const libc::c_char,
                b"lib/pwdutils.c\0" as *const u8 as *const libc::c_char,
                81 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                    b"struct passwd *xgetpwuid(uid_t, char **)\0",
                ))
                .as_ptr(),
            );
        }
    };
    *pwdbuf = xmalloc((16 as libc::c_int * 1024 as libc::c_int) as size_t) as *mut libc::c_char;
    pwd = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<passwd>() as libc::c_ulong,
    ) as *mut passwd;
    *__errno_location() = 0 as libc::c_int;
    rc = getpwuid_r(
        uid,
        pwd,
        *pwdbuf,
        (16 as libc::c_int * 1024 as libc::c_int) as size_t,
        &mut res,
    );
    if rc != 0 as libc::c_int {
        *__errno_location() = rc;
    } else if res.is_null() {
        *__errno_location() = 22 as libc::c_int;
    } else {
        return pwd;
    }
    free(pwd as *mut libc::c_void);
    free(*pwdbuf as *mut libc::c_void);
    return 0 as *mut passwd;
}
#[no_mangle]
pub unsafe extern "C" fn xgetlogin() -> *mut libc::c_char {
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut ruid: uid_t = 0;
    *__errno_location() = 0 as libc::c_int;
    ruid = getuid();
    if *__errno_location() == 0 as libc::c_int {
        pw = getpwuid(ruid);
    }
    if !pw.is_null() && !((*pw).pw_name).is_null() && *(*pw).pw_name as libc::c_int != 0 {
        return xstrdup((*pw).pw_name);
    }
    return 0 as *mut libc::c_char;
}
