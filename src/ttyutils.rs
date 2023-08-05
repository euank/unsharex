use ::libc;
extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn ttyname(__fd: libc::c_int) -> *mut libc::c_char;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn getttynam(__tty: *const libc::c_char) -> *mut ttyent;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ttyent {
    pub ty_name: *mut libc::c_char,
    pub ty_getty: *mut libc::c_char,
    pub ty_type: *mut libc::c_char,
    pub ty_status: libc::c_int,
    pub ty_window: *mut libc::c_char,
    pub ty_comment: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
unsafe extern "C" fn get_env_int(mut name: *const libc::c_char) -> libc::c_int {
    let mut cp: *const libc::c_char = getenv(name);
    if !cp.is_null() {
        let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut x: libc::c_long = 0;
        *__errno_location() = 0 as libc::c_int;
        x = strtol(cp, &mut end, 10 as libc::c_int);
        if *__errno_location() == 0 as libc::c_int && !end.is_null()
            && *end as libc::c_int == '\0' as i32 && end > cp as *mut libc::c_char
            && x > 0 as libc::c_int as libc::c_long
            && x <= 2147483647 as libc::c_int as libc::c_long
        {
            return x as libc::c_int;
        }
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn get_terminal_dimension(
    mut cols: *mut libc::c_int,
    mut lines: *mut libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_int = 0 as libc::c_int;
    let mut l: libc::c_int = 0 as libc::c_int;
    let mut w_win: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    if ioctl(
        1 as libc::c_int,
        0x5413 as libc::c_int as libc::c_ulong,
        &mut w_win as *mut winsize,
    ) == 0 as libc::c_int
    {
        c = w_win.ws_col as libc::c_int;
        l = w_win.ws_row as libc::c_int;
    }
    if !cols.is_null() {
        if c <= 0 as libc::c_int {
            c = get_env_int(b"COLUMNS\0" as *const u8 as *const libc::c_char);
        }
        *cols = c;
    }
    if !lines.is_null() {
        if l <= 0 as libc::c_int {
            l = get_env_int(b"LINES\0" as *const u8 as *const libc::c_char);
        }
        *lines = l;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_terminal_width(
    mut default_width: libc::c_int,
) -> libc::c_int {
    let mut width: libc::c_int = 0 as libc::c_int;
    get_terminal_dimension(&mut width, 0 as *mut libc::c_int);
    return if width > 0 as libc::c_int { width } else { default_width };
}
#[no_mangle]
pub unsafe extern "C" fn get_terminal_stdfd() -> libc::c_int {
    if isatty(0 as libc::c_int) != 0 {
        return 0 as libc::c_int;
    }
    if isatty(1 as libc::c_int) != 0 {
        return 1 as libc::c_int;
    }
    if isatty(2 as libc::c_int) != 0 {
        return 2 as libc::c_int;
    }
    return -(22 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn get_terminal_name(
    mut path: *mut *const libc::c_char,
    mut name: *mut *const libc::c_char,
    mut number: *mut *const libc::c_char,
) -> libc::c_int {
    let mut tty: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut fd: libc::c_int = 0;
    if !name.is_null() {
        *name = 0 as *const libc::c_char;
    }
    if !path.is_null() {
        *path = 0 as *const libc::c_char;
    }
    if !number.is_null() {
        *number = 0 as *const libc::c_char;
    }
    fd = get_terminal_stdfd();
    if fd < 0 as libc::c_int {
        return fd;
    }
    tty = ttyname(fd);
    if tty.is_null() {
        return -(1 as libc::c_int);
    }
    if !path.is_null() {
        *path = tty;
    }
    if !name.is_null() || !number.is_null() {
        tty = if strncmp(
            tty,
            b"/dev/\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            tty.offset(5 as libc::c_int as isize)
        } else {
            tty
        };
    }
    if !name.is_null() {
        *name = tty;
    }
    if !number.is_null() {
        p = tty;
        while !p.is_null() && *p as libc::c_int != 0 {
            if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                *number = p;
                break;
            } else {
                p = p.offset(1);
                p;
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_terminal_type(
    mut type_0: *mut *const libc::c_char,
) -> libc::c_int {
    *type_0 = getenv(b"TERM\0" as *const u8 as *const libc::c_char);
    if !(*type_0).is_null() {
        return -(22 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_terminal_default_type(
    mut ttyname_0: *const libc::c_char,
    mut is_serial: libc::c_int,
) -> *mut libc::c_char {
    if !ttyname_0.is_null() {
        let mut ent: *mut ttyent = getttynam(ttyname_0);
        if !ent.is_null() && !((*ent).ty_type).is_null() {
            return strdup((*ent).ty_type);
        }
    }
    return strdup(
        if is_serial != 0 {
            b"vt102\0" as *const u8 as *const libc::c_char
        } else {
            b"linux\0" as *const u8 as *const libc::c_char
        },
    );
}
