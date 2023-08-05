use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
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
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[no_mangle]
pub unsafe extern "C" fn mangle(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut ss: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    if s.is_null() {
        return 0 as *mut libc::c_char;
    }
    sp = malloc(
        (4 as libc::c_int as libc::c_ulong)
            .wrapping_mul(strlen(s))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    ss = sp;
    if sp.is_null() {
        return 0 as *mut libc::c_char;
    }
    loop {
        if *s == 0 {
            *sp = '\0' as i32 as libc::c_char;
            break;
        } else {
            if !(strchr(
                b" \t\n\\\0" as *const u8 as *const libc::c_char,
                *s as libc::c_uint as libc::c_int,
            ))
                .is_null()
            {
                let fresh0 = sp;
                sp = sp.offset(1);
                *fresh0 = '\\' as i32 as libc::c_char;
                let fresh1 = sp;
                sp = sp.offset(1);
                *fresh1 = ('0' as i32
                    + ((*s as libc::c_int & 0o300 as libc::c_int) >> 6 as libc::c_int))
                    as libc::c_char;
                let fresh2 = sp;
                sp = sp.offset(1);
                *fresh2 = ('0' as i32
                    + ((*s as libc::c_int & 0o70 as libc::c_int) >> 3 as libc::c_int))
                    as libc::c_char;
                let fresh3 = sp;
                sp = sp.offset(1);
                *fresh3 = ('0' as i32 + (*s as libc::c_int & 0o7 as libc::c_int))
                    as libc::c_char;
            } else {
                let fresh4 = sp;
                sp = sp.offset(1);
                *fresh4 = *s;
            }
            s = s.offset(1);
            s;
        }
    }
    return ss;
}
#[no_mangle]
pub unsafe extern "C" fn unmangle_to_buffer(
    mut s: *const libc::c_char,
    mut buf: *mut libc::c_char,
    mut len: size_t,
) {
    let mut sz: size_t = 0 as libc::c_int as size_t;
    if s.is_null() {
        return;
    }
    while *s as libc::c_int != 0
        && sz < len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        if *s as libc::c_int == '\\' as i32
            && sz.wrapping_add(3 as libc::c_int as libc::c_ulong)
                < len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && *s.offset(1 as libc::c_int as isize) as libc::c_int & !(7 as libc::c_int)
                == '0' as i32
            && *s.offset(2 as libc::c_int as isize) as libc::c_int & !(7 as libc::c_int)
                == '0' as i32
            && *s.offset(3 as libc::c_int as isize) as libc::c_int & !(7 as libc::c_int)
                == '0' as i32
        {
            let fresh5 = buf;
            buf = buf.offset(1);
            *fresh5 = (64 as libc::c_int
                * (*s.offset(1 as libc::c_int as isize) as libc::c_int
                    & 7 as libc::c_int)
                + 8 as libc::c_int
                    * (*s.offset(2 as libc::c_int as isize) as libc::c_int
                        & 7 as libc::c_int)
                + (*s.offset(3 as libc::c_int as isize) as libc::c_int
                    & 7 as libc::c_int)) as libc::c_char;
            s = s.offset(4 as libc::c_int as isize);
            sz = (sz as libc::c_ulong).wrapping_add(4 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        } else {
            let fresh6 = s;
            s = s.offset(1);
            let fresh7 = buf;
            buf = buf.offset(1);
            *fresh7 = *fresh6;
            sz = sz.wrapping_add(1);
            sz;
        }
    }
    *buf = '\0' as i32 as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn unhexmangle_to_buffer(
    mut s: *const libc::c_char,
    mut buf: *mut libc::c_char,
    mut len: size_t,
) -> size_t {
    let mut sz: size_t = 0 as libc::c_int as size_t;
    let mut buf0: *const libc::c_char = buf;
    if s.is_null() {
        return 0 as libc::c_int as size_t;
    }
    while *s as libc::c_int != 0
        && sz < len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        if *s as libc::c_int == '\\' as i32
            && sz.wrapping_add(3 as libc::c_int as libc::c_ulong)
                < len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && *s.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
            && *(*__ctype_b_loc())
                .offset(*s.offset(2 as libc::c_int as isize) as libc::c_int as isize)
                as libc::c_int
                & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            && *(*__ctype_b_loc())
                .offset(*s.offset(3 as libc::c_int as isize) as libc::c_int as isize)
                as libc::c_int
                & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            let fresh8 = buf;
            buf = buf.offset(1);
            *fresh8 = ((if *(*__ctype_b_loc())
                .offset(*s.offset(2 as libc::c_int as isize) as libc::c_int as isize)
                as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                *s.offset(2 as libc::c_int as isize) as libc::c_int - '0' as i32
            } else {
                ({
                    let mut __res: libc::c_int = 0;
                    if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *s
                                .offset(2 as libc::c_int as isize) as libc::c_int;
                            __res = (if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            });
                        } else {
                            __res = tolower(
                                *s.offset(2 as libc::c_int as isize) as libc::c_int,
                            );
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(
                                *s.offset(2 as libc::c_int as isize) as libc::c_int as isize,
                            );
                    }
                    __res
                }) - 'a' as i32 + 10 as libc::c_int
            }) << 4 as libc::c_int
                | (if *(*__ctype_b_loc())
                    .offset(*s.offset(3 as libc::c_int as isize) as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    *s.offset(3 as libc::c_int as isize) as libc::c_int - '0' as i32
                } else {
                    ({
                        let mut __res: libc::c_int = 0;
                        if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                            > 1 as libc::c_int as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int = *s
                                    .offset(3 as libc::c_int as isize) as libc::c_int;
                                __res = (if __c < -(128 as libc::c_int)
                                    || __c > 255 as libc::c_int
                                {
                                    __c
                                } else {
                                    *(*__ctype_tolower_loc()).offset(__c as isize)
                                });
                            } else {
                                __res = tolower(
                                    *s.offset(3 as libc::c_int as isize) as libc::c_int,
                                );
                            }
                        } else {
                            __res = *(*__ctype_tolower_loc())
                                .offset(
                                    *s.offset(3 as libc::c_int as isize) as libc::c_int as isize,
                                );
                        }
                        __res
                    }) - 'a' as i32 + 10 as libc::c_int
                })) as libc::c_char;
            s = s.offset(4 as libc::c_int as isize);
            sz = (sz as libc::c_ulong).wrapping_add(4 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        } else {
            let fresh9 = s;
            s = s.offset(1);
            let fresh10 = buf;
            buf = buf.offset(1);
            *fresh10 = *fresh9;
            sz = sz.wrapping_add(1);
            sz;
        }
    }
    *buf = '\0' as i32 as libc::c_char;
    return (buf.offset_from(buf0) as libc::c_long + 1 as libc::c_int as libc::c_long)
        as size_t;
}
#[inline]
unsafe extern "C" fn skip_nonspaces(mut s: *const libc::c_char) -> *const libc::c_char {
    while !s.is_null() && *s as libc::c_int != 0
        && !(*s as libc::c_int == ' ' as i32 || *s as libc::c_int == '\t' as i32)
    {
        s = s.offset(1);
        s;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn unmangle(
    mut s: *const libc::c_char,
    mut end: *mut *const libc::c_char,
) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: *const libc::c_char = 0 as *const libc::c_char;
    let mut sz: size_t = 0;
    if s.is_null() {
        return 0 as *mut libc::c_char;
    }
    e = skip_nonspaces(s);
    sz = (e.offset_from(s) as libc::c_long + 1 as libc::c_int as libc::c_long) as size_t;
    if !end.is_null() {
        *end = e;
    }
    if e == s {
        return 0 as *mut libc::c_char;
    }
    buf = malloc(sz) as *mut libc::c_char;
    if buf.is_null() {
        return 0 as *mut libc::c_char;
    }
    unmangle_to_buffer(s, buf, sz);
    return buf;
}
