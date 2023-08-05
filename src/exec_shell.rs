use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn execl(
        __path: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn __xpg_basename(__path: *mut libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn err(__status: libc::c_int, __format: *const libc::c_char, _: ...) -> !;
}
pub type size_t = libc::c_ulong;
#[inline]
unsafe extern "C" fn xmalloc(size: size_t) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = malloc(size);
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
unsafe extern "C" fn xstrdup(mut str: *const libc::c_char) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    if !str.is_null() {} else {
        __assert_fail(
            b"str\0" as *const u8 as *const libc::c_char,
            b"./include/xalloc.h\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"char *xstrdup(const char *)\0"))
                .as_ptr(),
        );
    }
    'c_5021: {
        if !str.is_null() {} else {
            __assert_fail(
                b"str\0" as *const u8 as *const libc::c_char,
                b"./include/xalloc.h\0" as *const u8 as *const libc::c_char,
                80 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"char *xstrdup(const char *)\0"))
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
pub unsafe extern "C" fn exec_shell() -> ! {
    let mut shell: *const libc::c_char = getenv(
        b"SHELL\0" as *const u8 as *const libc::c_char,
    );
    let mut shellc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut shell_basename: *const libc::c_char = 0 as *const libc::c_char;
    let mut arg0: *mut libc::c_char = 0 as *mut libc::c_char;
    if shell.is_null() {
        shell = b"/bin/sh\0" as *const u8 as *const libc::c_char;
    }
    shellc = xstrdup(shell);
    shell_basename = __xpg_basename(shellc);
    arg0 = xmalloc(
        (strlen(shell_basename)).wrapping_add(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    *arg0.offset(0 as libc::c_int as isize) = '-' as i32 as libc::c_char;
    strcpy(arg0.offset(1 as libc::c_int as isize), shell_basename);
    execl(shell, arg0, 0 as *mut libc::c_void as *mut libc::c_char);
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
        shell,
    );
}
