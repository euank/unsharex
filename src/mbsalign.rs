use ::libc;
extern "C" {
    fn __ctype_get_mb_cur_max() -> size_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn wcstombs(__s: *mut libc::c_char, __pwcs: *const wchar_t, __n: size_t) -> size_t;
    fn mbstowcs(__pwcs: *mut wchar_t, __s: *const libc::c_char, __n: size_t) -> size_t;
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn mempcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn mbrtowc(
        __pwc: *mut wchar_t,
        __s: *const libc::c_char,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    fn wcwidth(__c: wchar_t) -> libc::c_int;
    fn iswprint(__wc: wint_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
pub type mbs_align_t = libc::c_uint;
pub const MBS_ALIGN_CENTER: mbs_align_t = 2;
pub const MBS_ALIGN_RIGHT: mbs_align_t = 1;
pub const MBS_ALIGN_LEFT: mbs_align_t = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const MBA_UNIBYTE_FALLBACK: C2RustUnnamed_1 = 1;
pub type wint_t = libc::c_uint;
pub type mbstate_t = __mbstate_t;
#[no_mangle]
pub unsafe extern "C" fn mbs_nwidth(
    mut buf: *const libc::c_char,
    mut bufsz: size_t,
) -> size_t {
    let mut p: *const libc::c_char = buf;
    let mut last: *const libc::c_char = buf;
    let mut width: size_t = 0 as libc::c_int as size_t;
    let mut st: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    memset(
        &mut st as *mut mbstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    if !p.is_null() && *p as libc::c_int != 0 && bufsz != 0 {
        last = p.offset(bufsz.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    }
    while !p.is_null() && *p as libc::c_int != 0 && p <= last {
        if *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _IScntrl as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            p = p.offset(1);
            p;
            if *p as libc::c_int != 0 && *p as libc::c_int == '[' as i32 {
                let mut e: *const libc::c_char = p;
                while *e as libc::c_int != 0 && e < last
                    && *e as libc::c_int != 'm' as i32
                {
                    e = e.offset(1);
                    e;
                }
                if *e as libc::c_int == 'm' as i32 {
                    p = e.offset(1 as libc::c_int as isize);
                }
            }
        } else {
            let mut wc: wchar_t = 0;
            let mut len: size_t = mbrtowc(&mut wc, p, __ctype_get_mb_cur_max(), &mut st);
            if len == 0 as libc::c_int as libc::c_ulong {
                break;
            }
            if len > 0 as libc::c_int as libc::c_ulong && iswprint(wc as wint_t) != 0 {
                let mut x: libc::c_int = wcwidth(wc);
                if x > 0 as libc::c_int {
                    width = (width as libc::c_ulong).wrapping_add(x as libc::c_ulong)
                        as size_t as size_t;
                }
            } else if len == -(1 as libc::c_int) as size_t
                || len == -(2 as libc::c_int) as size_t
            {
                len = 1 as libc::c_int as size_t;
            }
            p = p.offset(len as isize);
        }
    }
    return width;
}
#[no_mangle]
pub unsafe extern "C" fn mbs_width(mut s: *const libc::c_char) -> size_t {
    if s.is_null() || *s == 0 {
        return 0 as libc::c_int as size_t;
    }
    return mbs_nwidth(s, strlen(s));
}
#[no_mangle]
pub unsafe extern "C" fn mbs_safe_nwidth(
    mut buf: *const libc::c_char,
    mut bufsz: size_t,
    mut sz: *mut size_t,
) -> size_t {
    let mut p: *const libc::c_char = buf;
    let mut last: *const libc::c_char = buf;
    let mut width: size_t = 0 as libc::c_int as size_t;
    let mut bytes: size_t = 0 as libc::c_int as size_t;
    let mut st: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    memset(
        &mut st as *mut mbstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    if !p.is_null() && *p as libc::c_int != 0 && bufsz != 0 {
        last = p.offset(bufsz.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    }
    while !p.is_null() && *p as libc::c_int != 0 && p <= last {
        if p < last && *p as libc::c_int == '\\' as i32
            && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
            || *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _IScntrl as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            width = (width as libc::c_ulong)
                .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
            bytes = (bytes as libc::c_ulong)
                .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
            p = p.offset(1);
            p;
        } else {
            let mut wc: wchar_t = 0;
            let mut len: size_t = mbrtowc(&mut wc, p, __ctype_get_mb_cur_max(), &mut st);
            if len == 0 as libc::c_int as libc::c_ulong {
                break;
            }
            if len == -(1 as libc::c_int) as size_t
                || len == -(2 as libc::c_int) as size_t
            {
                len = 1 as libc::c_int as size_t;
                if *(*__ctype_b_loc())
                    .offset(*p as libc::c_uchar as libc::c_int as isize) as libc::c_int
                    & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    width = (width as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                    bytes = (bytes as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                } else {
                    width = (width as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                    bytes = (bytes as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                }
            } else if iswprint(wc as wint_t) == 0 {
                width = (width as libc::c_ulong)
                    .wrapping_add(len.wrapping_mul(4 as libc::c_int as libc::c_ulong))
                    as size_t as size_t;
                bytes = (bytes as libc::c_ulong)
                    .wrapping_add(len.wrapping_mul(4 as libc::c_int as libc::c_ulong))
                    as size_t as size_t;
            } else {
                width = (width as libc::c_ulong)
                    .wrapping_add(wcwidth(wc) as libc::c_ulong) as size_t as size_t;
                bytes = (bytes as libc::c_ulong).wrapping_add(len) as size_t as size_t;
            }
            p = p.offset(len as isize);
        }
    }
    if !sz.is_null() {
        *sz = bytes;
    }
    return width;
}
#[no_mangle]
pub unsafe extern "C" fn mbs_safe_width(mut s: *const libc::c_char) -> size_t {
    if s.is_null() || *s == 0 {
        return 0 as libc::c_int as size_t;
    }
    return mbs_safe_nwidth(s, strlen(s), 0 as *mut size_t);
}
#[no_mangle]
pub unsafe extern "C" fn mbs_safe_encode_to_buffer(
    mut s: *const libc::c_char,
    mut width: *mut size_t,
    mut buf: *mut libc::c_char,
    mut safechars: *const libc::c_char,
) -> *mut libc::c_char {
    let mut p: *const libc::c_char = s;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sz: size_t = if !s.is_null() {
        strlen(s)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mut st: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    memset(
        &mut st as *mut mbstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    if sz == 0 || buf.is_null() {
        return 0 as *mut libc::c_char;
    }
    r = buf;
    *width = 0 as libc::c_int as size_t;
    while !p.is_null() && *p as libc::c_int != 0 {
        if !safechars.is_null() && !(strchr(safechars, *p as libc::c_int)).is_null() {
            let fresh0 = p;
            p = p.offset(1);
            let fresh1 = r;
            r = r.offset(1);
            *fresh1 = *fresh0;
        } else if *p as libc::c_int == '\\' as i32
            && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
            || *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _IScntrl as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            sprintf(
                r,
                b"\\x%02x\0" as *const u8 as *const libc::c_char,
                *p as libc::c_uchar as libc::c_int,
            );
            r = r.offset(4 as libc::c_int as isize);
            *width = (*width as libc::c_ulong)
                .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
            p = p.offset(1);
            p;
        } else {
            let mut wc: wchar_t = 0;
            let mut len: size_t = mbrtowc(&mut wc, p, __ctype_get_mb_cur_max(), &mut st);
            if len == 0 as libc::c_int as libc::c_ulong {
                break;
            }
            if len == -(1 as libc::c_int) as size_t
                || len == -(2 as libc::c_int) as size_t
            {
                len = 1 as libc::c_int as size_t;
                if *(*__ctype_b_loc())
                    .offset(*p as libc::c_uchar as libc::c_int as isize) as libc::c_int
                    & _ISprint as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    sprintf(
                        r,
                        b"\\x%02x\0" as *const u8 as *const libc::c_char,
                        *p as libc::c_uchar as libc::c_int,
                    );
                    r = r.offset(4 as libc::c_int as isize);
                    *width = (*width as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                } else {
                    *width = (*width).wrapping_add(1);
                    *width;
                    let fresh2 = r;
                    r = r.offset(1);
                    *fresh2 = *p;
                }
            } else if iswprint(wc as wint_t) == 0 {
                let mut i: size_t = 0;
                i = 0 as libc::c_int as size_t;
                while i < len {
                    sprintf(
                        r,
                        b"\\x%02x\0" as *const u8 as *const libc::c_char,
                        *p.offset(i as isize) as libc::c_uchar as libc::c_int,
                    );
                    r = r.offset(4 as libc::c_int as isize);
                    *width = (*width as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                    i = i.wrapping_add(1);
                    i;
                }
            } else {
                memcpy(r as *mut libc::c_void, p as *const libc::c_void, len);
                r = r.offset(len as isize);
                *width = (*width as libc::c_ulong)
                    .wrapping_add(wcwidth(wc) as libc::c_ulong) as size_t as size_t;
            }
            p = p.offset(len as isize);
        }
    }
    *r = '\0' as i32 as libc::c_char;
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn mbs_invalid_encode_to_buffer(
    mut s: *const libc::c_char,
    mut width: *mut size_t,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *const libc::c_char = s;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sz: size_t = if !s.is_null() {
        strlen(s)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mut st: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    memset(
        &mut st as *mut mbstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    if sz == 0 || buf.is_null() {
        return 0 as *mut libc::c_char;
    }
    r = buf;
    *width = 0 as libc::c_int as size_t;
    while !p.is_null() && *p as libc::c_int != 0 {
        let mut wc: wchar_t = 0;
        let mut len: size_t = mbrtowc(&mut wc, p, __ctype_get_mb_cur_max(), &mut st);
        if len == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        if len == -(1 as libc::c_int) as size_t || len == -(2 as libc::c_int) as size_t {
            len = 1 as libc::c_int as size_t;
            if *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                == 0
            {
                sprintf(
                    r,
                    b"\\x%02x\0" as *const u8 as *const libc::c_char,
                    *p as libc::c_uchar as libc::c_int,
                );
                r = r.offset(4 as libc::c_int as isize);
                *width = (*width as libc::c_ulong)
                    .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
            } else {
                *width = (*width).wrapping_add(1);
                *width;
                let fresh3 = r;
                r = r.offset(1);
                *fresh3 = *p;
            }
        } else if *p as libc::c_int == '\\' as i32
            && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
        {
            sprintf(
                r,
                b"\\x%02x\0" as *const u8 as *const libc::c_char,
                *p as libc::c_uchar as libc::c_int,
            );
            r = r.offset(4 as libc::c_int as isize);
            *width = (*width as libc::c_ulong)
                .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
        } else {
            r = mempcpy(r as *mut libc::c_void, p as *const libc::c_void, len)
                as *mut libc::c_char;
            *width = (*width as libc::c_ulong).wrapping_add(wcwidth(wc) as libc::c_ulong)
                as size_t as size_t;
        }
        p = p.offset(len as isize);
    }
    *r = '\0' as i32 as libc::c_char;
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn mbs_safe_encode_size(mut bytes: size_t) -> size_t {
    return bytes
        .wrapping_mul(4 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn mbs_safe_encode(
    mut s: *const libc::c_char,
    mut width: *mut size_t,
) -> *mut libc::c_char {
    let mut sz: size_t = if !s.is_null() {
        strlen(s)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    if sz == 0 {
        return 0 as *mut libc::c_char;
    }
    buf = malloc(mbs_safe_encode_size(sz)) as *mut libc::c_char;
    if !buf.is_null() {
        ret = mbs_safe_encode_to_buffer(s, width, buf, 0 as *const libc::c_char);
    }
    if ret.is_null() {
        free(buf as *mut libc::c_void);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mbs_invalid_encode(
    mut s: *const libc::c_char,
    mut width: *mut size_t,
) -> *mut libc::c_char {
    let mut sz: size_t = if !s.is_null() {
        strlen(s)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    if sz == 0 {
        return 0 as *mut libc::c_char;
    }
    buf = malloc(mbs_safe_encode_size(sz)) as *mut libc::c_char;
    if !buf.is_null() {
        ret = mbs_invalid_encode_to_buffer(s, width, buf);
    }
    if ret.is_null() {
        free(buf as *mut libc::c_void);
    }
    return ret;
}
unsafe extern "C" fn wc_ensure_printable(mut wchars: *mut wchar_t) -> bool {
    let mut replaced: bool = 0 as libc::c_int != 0;
    let mut wc: *mut wchar_t = wchars;
    while *wc != 0 {
        if iswprint(*wc as wint_t) == 0 {
            *wc = 0xfffd as libc::c_int;
            replaced = 1 as libc::c_int != 0;
        }
        wc = wc.offset(1);
        wc;
    }
    return replaced;
}
unsafe extern "C" fn wc_truncate(mut wc: *mut wchar_t, mut width: size_t) -> size_t {
    let mut cells: size_t = 0 as libc::c_int as size_t;
    let mut next_cells: libc::c_int = 0 as libc::c_int;
    while *wc != 0 {
        next_cells = wcwidth(*wc);
        if next_cells == -(1 as libc::c_int) {
            *wc = 0xfffd as libc::c_int;
            next_cells = 1 as libc::c_int;
        }
        if cells.wrapping_add(next_cells as libc::c_ulong) > width {
            break;
        }
        cells = (cells as libc::c_ulong).wrapping_add(next_cells as libc::c_ulong)
            as size_t as size_t;
        wc = wc.offset(1);
        wc;
    }
    *wc = '\0' as i32;
    return cells;
}
unsafe extern "C" fn rpl_wcswidth(mut s: *const wchar_t, mut n: size_t) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh4 = n;
        n = n.wrapping_sub(1);
        if !(fresh4 > 0 as libc::c_int as libc::c_ulong && *s != '\0' as i32) {
            break;
        }
        let fresh5 = s;
        s = s.offset(1);
        let mut nwidth: libc::c_int = wcwidth(*fresh5);
        if nwidth == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        if ret > 2147483647 as libc::c_int - nwidth {
            return -(1 as libc::c_int);
        }
        ret += nwidth;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mbs_truncate(
    mut str: *mut libc::c_char,
    mut width: *mut size_t,
) -> size_t {
    let mut bytes: ssize_t = strlen(str) as ssize_t;
    let mut sz: ssize_t = mbstowcs(0 as *mut wchar_t, str, 0 as libc::c_int as size_t)
        as ssize_t;
    let mut wcs: *mut wchar_t = 0 as *mut wchar_t;
    if !(sz == -(1 as libc::c_int) as ssize_t) {
        wcs = calloc(
            1 as libc::c_int as libc::c_ulong,
            ((sz + 1 as libc::c_int as libc::c_long) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<wchar_t>() as libc::c_ulong),
        ) as *mut wchar_t;
        if !wcs.is_null() {
            if !(mbstowcs(wcs, str, sz as size_t) == 0) {
                *width = wc_truncate(wcs, *width);
                bytes = wcstombs(str, wcs, bytes as size_t) as ssize_t;
            }
        }
    }
    free(wcs as *mut libc::c_void);
    if bytes >= 0 as libc::c_int as libc::c_long {
        *str.offset(bytes as isize) = '\0' as i32 as libc::c_char;
    }
    return bytes as size_t;
}
unsafe extern "C" fn mbs_align_pad(
    mut dest: *mut libc::c_char,
    mut dest_end: *const libc::c_char,
    mut n_spaces: size_t,
    mut padchar: libc::c_int,
) -> *mut libc::c_char {
    while n_spaces != 0 && dest < dest_end as *mut libc::c_char {
        let fresh6 = dest;
        dest = dest.offset(1);
        *fresh6 = padchar as libc::c_char;
        n_spaces = n_spaces.wrapping_sub(1);
        n_spaces;
    }
    *dest = '\0' as i32 as libc::c_char;
    return dest;
}
#[no_mangle]
pub unsafe extern "C" fn mbsalign(
    mut src: *const libc::c_char,
    mut dest: *mut libc::c_char,
    mut dest_size: size_t,
    mut width: *mut size_t,
    mut align: mbs_align_t,
    mut flags: libc::c_int,
) -> size_t {
    return mbsalign_with_padding(src, dest, dest_size, width, align, flags, ' ' as i32);
}
#[no_mangle]
pub unsafe extern "C" fn mbsalign_with_padding(
    mut src: *const libc::c_char,
    mut dest: *mut libc::c_char,
    mut dest_size: size_t,
    mut width: *mut size_t,
    mut align: mbs_align_t,
    mut flags: libc::c_int,
    mut padchar: libc::c_int,
) -> size_t {
    let mut current_block: u64;
    let mut ret: size_t = -(1 as libc::c_int) as size_t;
    let mut src_size: size_t = (strlen(src))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut newstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str_wc: *mut wchar_t = 0 as *mut wchar_t;
    let mut str_to_print: *const libc::c_char = src;
    let mut n_cols: size_t = src_size.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut n_used_bytes: size_t = n_cols;
    let mut n_spaces: size_t = 0 as libc::c_int as size_t;
    let mut space_left: size_t = 0;
    let mut conversion: bool = 0 as libc::c_int != 0;
    let mut wc_enabled: bool = 0 as libc::c_int != 0;
    if __ctype_get_mb_cur_max() > 1 as libc::c_int as libc::c_ulong {
        let mut src_chars: size_t = mbstowcs(
            0 as *mut wchar_t,
            src,
            0 as libc::c_int as size_t,
        );
        if src_chars == -(1 as libc::c_int) as size_t {
            if flags & MBA_UNIBYTE_FALLBACK as libc::c_int != 0 {
                current_block = 18409836299740197588;
            } else {
                current_block = 10972045911645593456;
            }
        } else {
            src_chars = (src_chars as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
            str_wc = malloc(
                src_chars
                    .wrapping_mul(::core::mem::size_of::<wchar_t>() as libc::c_ulong),
            ) as *mut wchar_t;
            if str_wc.is_null() {
                if flags & MBA_UNIBYTE_FALLBACK as libc::c_int != 0 {
                    current_block = 18409836299740197588;
                } else {
                    current_block = 10972045911645593456;
                }
            } else {
                if mbstowcs(str_wc, src, src_chars) != 0 as libc::c_int as libc::c_ulong
                {
                    *str_wc
                        .offset(
                            src_chars.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) = '\0' as i32;
                    wc_enabled = 1 as libc::c_int != 0;
                    conversion = wc_ensure_printable(str_wc);
                    n_cols = rpl_wcswidth(str_wc, src_chars) as size_t;
                }
                current_block = 7149356873433890176;
            }
        }
    } else {
        current_block = 7149356873433890176;
    }
    match current_block {
        7149356873433890176 => {
            if wc_enabled as libc::c_int != 0
                && (conversion as libc::c_int != 0 || n_cols > *width)
            {
                if conversion {
                    src_size = (wcstombs(
                        0 as *mut libc::c_char,
                        str_wc,
                        0 as libc::c_int as size_t,
                    ))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong);
                }
                newstr = malloc(src_size) as *mut libc::c_char;
                if newstr.is_null() {
                    if flags & MBA_UNIBYTE_FALLBACK as libc::c_int != 0 {
                        current_block = 18409836299740197588;
                    } else {
                        current_block = 10972045911645593456;
                    }
                } else {
                    str_to_print = newstr;
                    n_cols = wc_truncate(str_wc, *width);
                    n_used_bytes = wcstombs(newstr, str_wc, src_size);
                    current_block = 18409836299740197588;
                }
            } else {
                current_block = 18409836299740197588;
            }
        }
        _ => {}
    }
    match current_block {
        18409836299740197588 => {
            if n_cols > *width {
                n_cols = *width;
                n_used_bytes = n_cols;
            }
            if *width > n_cols {
                n_spaces = (*width).wrapping_sub(n_cols);
            }
            *width = n_cols;
            ret = n_used_bytes
                .wrapping_add(n_spaces.wrapping_mul(1 as libc::c_int as libc::c_ulong));
            if dest_size != 0 as libc::c_int as libc::c_ulong {
                let mut dest_end: *mut libc::c_char = dest
                    .offset(dest_size as isize)
                    .offset(-(1 as libc::c_int as isize));
                let mut start_spaces: size_t = 0;
                let mut end_spaces: size_t = 0;
                match align as libc::c_uint {
                    2 => {
                        start_spaces = n_spaces
                            .wrapping_div(2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                n_spaces.wrapping_rem(2 as libc::c_int as libc::c_ulong),
                            );
                        end_spaces = n_spaces
                            .wrapping_div(2 as libc::c_int as libc::c_ulong);
                    }
                    0 => {
                        start_spaces = 0 as libc::c_int as size_t;
                        end_spaces = n_spaces;
                    }
                    1 => {
                        start_spaces = n_spaces;
                        end_spaces = 0 as libc::c_int as size_t;
                    }
                    _ => {
                        abort();
                    }
                }
                dest = mbs_align_pad(dest, dest_end, start_spaces, padchar);
                space_left = dest_end.offset_from(dest) as libc::c_long as size_t;
                dest = mempcpy(
                    dest as *mut libc::c_void,
                    str_to_print as *const libc::c_void,
                    ({
                        let mut _min1: size_t = n_used_bytes;
                        let mut _min2: size_t = space_left;
                        &mut _min1 as *mut size_t;
                        &mut _min2 as *mut size_t;
                        if _min1 < _min2 { _min1 } else { _min2 }
                    }),
                ) as *mut libc::c_char;
                mbs_align_pad(dest, dest_end, end_spaces, padchar);
            }
        }
        _ => {}
    }
    free(str_wc as *mut libc::c_void);
    free(newstr as *mut libc::c_void);
    return ret;
}
