use ::libc;
use ::c2rust_bitfields::BitfieldStruct;
extern "C" {
    fn asprintf(
        __ptr: *mut *mut libc::c_char,
        __fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ul_color_name {
    pub name: *const libc::c_char,
    pub seq: *const libc::c_char,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed {}
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
#[inline]
unsafe extern "C" fn bsearch(
    mut __key: *const libc::c_void,
    mut __base: *const libc::c_void,
    mut __nmemb: size_t,
    mut __size: size_t,
    mut __compar: __compar_fn_t,
) -> *mut libc::c_void {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p: *const libc::c_void = 0 as *const libc::c_void;
    let mut __comparison: libc::c_int = 0;
    __l = 0 as libc::c_int as size_t;
    __u = __nmemb;
    while __l < __u {
        __idx = __l.wrapping_add(__u).wrapping_div(2 as libc::c_int as libc::c_ulong);
        __p = (__base as *const libc::c_char).offset(__idx.wrapping_mul(__size) as isize)
            as *const libc::c_void;
        __comparison = (Some(__compar.expect("non-null function pointer")))
            .expect("non-null function pointer")(__key, __p);
        if __comparison < 0 as libc::c_int {
            __u = __idx;
        } else if __comparison > 0 as libc::c_int {
            __l = __idx.wrapping_add(1 as libc::c_int as libc::c_ulong);
        } else {
            return __p as *mut libc::c_void
        }
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn cmp_color_name(
    mut a0: *const libc::c_void,
    mut b0: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const ul_color_name = a0 as *const ul_color_name;
    let mut b: *const ul_color_name = b0 as *const ul_color_name;
    return strcmp((*a).name, (*b).name);
}
#[no_mangle]
pub unsafe extern "C" fn color_sequence_from_colorname(
    mut str: *const libc::c_char,
) -> *const libc::c_char {
    static mut basic_schemes: [ul_color_name; 22] = [
        {
            let mut init = ul_color_name {
                name: b"black\0" as *const u8 as *const libc::c_char,
                seq: b"\x1B[30m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ul_color_name {
                name: b"blink\0" as *const u8 as *const libc::c_char,
                seq: b"\x1B[5m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ul_color_name {
                name: b"blue\0" as *const u8 as *const libc::c_char,
                seq: b"\x1B[34m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ul_color_name {
                name: b"bold\0" as *const u8 as *const libc::c_char,
                seq: b"\x1B[1m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ul_color_name {
                name: b"brown\0" as *const u8 as *const libc::c_char,
                seq: b"\x1B[33m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ul_color_name {
                name: b"cyan\0" as *const u8 as *const libc::c_char,
                seq: b"\x1B[36m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ul_color_name {
                name: b"darkgray\0" as *const u8 as *const libc::c_char,
                seq: b"\x1B[1;30m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ul_color_name {
                name: b"gray\0" as *const u8 as *const libc::c_char,
                seq: b"\x1B[37m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ul_color_name {
                name: b"green\0" as *const u8 as *const libc::c_char,
                seq: b"\x1B[32m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ul_color_name {
                name: b"halfbright\0" as *const u8 as *const libc::c_char,
                seq: b"\x1B[2m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ul_color_name {
                name: b"lightblue\0" as *const u8 as *const libc::c_char,
                seq: b"\x1B[1;34m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ul_color_name {
                name: b"lightcyan\0" as *const u8 as *const libc::c_char,
                seq: b"\x1B[1;36m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ul_color_name {
                name: b"lightgray,\0" as *const u8 as *const libc::c_char,
                seq: b"\x1B[37m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ul_color_name {
                name: b"lightgreen\0" as *const u8 as *const libc::c_char,
                seq: b"\x1B[1;32m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ul_color_name {
                name: b"lightmagenta\0" as *const u8 as *const libc::c_char,
                seq: b"\x1B[1;35m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ul_color_name {
                name: b"lightred\0" as *const u8 as *const libc::c_char,
                seq: b"\x1B[1;31m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ul_color_name {
                name: b"magenta\0" as *const u8 as *const libc::c_char,
                seq: b"\x1B[35m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ul_color_name {
                name: b"red\0" as *const u8 as *const libc::c_char,
                seq: b"\x1B[31m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ul_color_name {
                name: b"reset\0" as *const u8 as *const libc::c_char,
                seq: b"\x1B[0m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ul_color_name {
                name: b"reverse\0" as *const u8 as *const libc::c_char,
                seq: b"\x1B[7m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ul_color_name {
                name: b"yellow\0" as *const u8 as *const libc::c_char,
                seq: b"\x1B[1;33m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ul_color_name {
                name: b"white\0" as *const u8 as *const libc::c_char,
                seq: b"\x1B[1;37m\0" as *const u8 as *const libc::c_char,
            };
            init
        },
    ];
    let mut key: ul_color_name = {
        let mut init = ul_color_name {
            name: str,
            seq: 0 as *const libc::c_char,
        };
        init
    };
    let mut res: *mut ul_color_name = 0 as *mut ul_color_name;
    if str.is_null() {
        return 0 as *const libc::c_char;
    }
    res = bsearch(
        &mut key as *mut ul_color_name as *const libc::c_void,
        basic_schemes.as_ptr() as *const libc::c_void,
        (::core::mem::size_of::<[ul_color_name; 22]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<ul_color_name>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<C2RustUnnamed>() as libc::c_ulong),
        ::core::mem::size_of::<ul_color_name>() as libc::c_ulong,
        Some(
            cmp_color_name
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    ) as *mut ul_color_name;
    return if !res.is_null() { (*res).seq } else { 0 as *const libc::c_char };
}
#[no_mangle]
pub unsafe extern "C" fn color_is_sequence(
    mut color: *const libc::c_char,
) -> libc::c_int {
    if !color.is_null() && *color as libc::c_int == 0x1b as libc::c_int {
        let mut len: size_t = strlen(color);
        if len >= 4 as libc::c_int as libc::c_ulong
            && *color.offset(1 as libc::c_int as isize) as libc::c_int == '[' as i32
            && *(*__ctype_b_loc())
                .offset(*color.offset(2 as libc::c_int as isize) as libc::c_int as isize)
                as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            && *color.offset(len as isize).offset(-(1 as libc::c_int as isize))
                as libc::c_int == 'm' as i32
        {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __color_canonicalize(
    mut str: *const libc::c_char,
    mut seq: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut in_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    if str.is_null() {
        return -(22 as libc::c_int);
    }
    *seq = 0 as *mut libc::c_char;
    if *str as libc::c_int != '\\' as i32
        && *(*__ctype_b_loc()).offset(*str as libc::c_int as isize) as libc::c_int
            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        let mut s: *const libc::c_char = color_sequence_from_colorname(str);
        *seq = strdup(if !s.is_null() { s } else { str });
        return if !(*seq).is_null() { 0 as libc::c_int } else { -(12 as libc::c_int) };
    }
    len = asprintf(seq, b"\x1B[%sm\0" as *const u8 as *const libc::c_char, str);
    if len < 1 as libc::c_int {
        return -(12 as libc::c_int);
    }
    in_0 = *seq;
    out = *seq;
    while !in_0.is_null() && *in_0 as libc::c_int != 0 {
        if *in_0 as libc::c_int != '\\' as i32 {
            let fresh0 = out;
            out = out.offset(1);
            *fresh0 = *in_0;
        } else {
            match *in_0.offset(1 as libc::c_int as isize) as libc::c_int {
                97 => {
                    let fresh1 = out;
                    out = out.offset(1);
                    *fresh1 = '\u{7}' as i32 as libc::c_char;
                }
                98 => {
                    let fresh2 = out;
                    out = out.offset(1);
                    *fresh2 = '\u{8}' as i32 as libc::c_char;
                }
                101 => {
                    let fresh3 = out;
                    out = out.offset(1);
                    *fresh3 = '\u{1b}' as i32 as libc::c_char;
                }
                102 => {
                    let fresh4 = out;
                    out = out.offset(1);
                    *fresh4 = '\u{c}' as i32 as libc::c_char;
                }
                110 => {
                    let fresh5 = out;
                    out = out.offset(1);
                    *fresh5 = '\n' as i32 as libc::c_char;
                }
                114 => {
                    let fresh6 = out;
                    out = out.offset(1);
                    *fresh6 = '\r' as i32 as libc::c_char;
                }
                116 => {
                    let fresh7 = out;
                    out = out.offset(1);
                    *fresh7 = '\t' as i32 as libc::c_char;
                }
                118 => {
                    let fresh8 = out;
                    out = out.offset(1);
                    *fresh8 = '\u{b}' as i32 as libc::c_char;
                }
                92 => {
                    let fresh9 = out;
                    out = out.offset(1);
                    *fresh9 = '\\' as i32 as libc::c_char;
                }
                95 => {
                    let fresh10 = out;
                    out = out.offset(1);
                    *fresh10 = ' ' as i32 as libc::c_char;
                }
                35 => {
                    let fresh11 = out;
                    out = out.offset(1);
                    *fresh11 = '#' as i32 as libc::c_char;
                }
                63 => {
                    let fresh12 = out;
                    out = out.offset(1);
                    *fresh12 = '?' as i32 as libc::c_char;
                }
                _ => {
                    let fresh13 = out;
                    out = out.offset(1);
                    *fresh13 = *in_0;
                    let fresh14 = out;
                    out = out.offset(1);
                    *fresh14 = *in_0.offset(1 as libc::c_int as isize);
                }
            }
            in_0 = in_0.offset(1);
            in_0;
        }
        in_0 = in_0.offset(1);
        in_0;
    }
    if !out.is_null() {
        if out.offset_from(*seq) as libc::c_long <= len as libc::c_long {} else {
            __assert_fail(
                b"(out - *seq) <= len\0" as *const u8 as *const libc::c_char,
                b"lib/color-names.c\0" as *const u8 as *const libc::c_char,
                158 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"int __color_canonicalize(const char *, char **)\0"))
                    .as_ptr(),
            );
        }
        'c_4762: {
            if out.offset_from(*seq) as libc::c_long <= len as libc::c_long {} else {
                __assert_fail(
                    b"(out - *seq) <= len\0" as *const u8 as *const libc::c_char,
                    b"lib/color-names.c\0" as *const u8 as *const libc::c_char,
                    158 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 48],
                        &[libc::c_char; 48],
                    >(b"int __color_canonicalize(const char *, char **)\0"))
                        .as_ptr(),
                );
            }
        };
        *out = '\0' as i32 as libc::c_char;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn color_get_sequence(
    mut color: *const libc::c_char,
) -> *mut libc::c_char {
    let mut seq: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rc: libc::c_int = __color_canonicalize(color, &mut seq);
    return if rc != 0 { 0 as *mut libc::c_char } else { seq };
}
