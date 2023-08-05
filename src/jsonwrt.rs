use ::libc;
use ::c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type uint64_t = __uint64_t;
pub type C2RustUnnamed = libc::c_uint;
pub const UL_JSON_VALUE: C2RustUnnamed = 2;
pub const UL_JSON_ARRAY: C2RustUnnamed = 1;
pub const UL_JSON_OBJECT: C2RustUnnamed = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct ul_jsonwrt {
    pub out: *mut FILE,
    pub indent: libc::c_int,
    #[bitfield(name = "after_close", ty = "libc::c_uint", bits = "0..=0")]
    pub after_close: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn c_tolower(mut c: libc::c_int) -> libc::c_int {
    match c {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
        | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => {
            return c - 'A' as i32 + 'a' as i32;
        }
        _ => return c,
    };
}
#[inline]
unsafe extern "C" fn c_toupper(mut c: libc::c_int) -> libc::c_int {
    match c {
        97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110
        | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 => {
            return c - 'a' as i32 + 'A' as i32;
        }
        _ => return c,
    };
}
unsafe extern "C" fn fputs_quoted_case_json(
    mut data: *const libc::c_char,
    mut out: *mut FILE,
    mut dir: libc::c_int,
) {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    fputc('"' as i32, out);
    p = data;
    while !p.is_null() && *p as libc::c_int != 0 {
        let c: libc::c_uint = *p as libc::c_uint;
        if c == '"' as i32 as libc::c_uint || c == '\\' as i32 as libc::c_uint {
            fputc('\\' as i32, out);
            fputc(c as libc::c_int, out);
        } else if c >= 0x20 as libc::c_int as libc::c_uint {
            if c <= 127 as libc::c_int as libc::c_uint {
                fputc(
                    if dir == 1 as libc::c_int {
                        c_toupper(c as libc::c_int)
                    } else if dir == -(1 as libc::c_int) {
                        c_tolower(c as libc::c_int)
                    } else {
                        *p as libc::c_int
                    },
                    out,
                );
            } else {
                fputc(
                    if dir == 1 as libc::c_int {
                        ({
                            let mut __res: libc::c_int = 0;
                            if ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong
                                > 1 as libc::c_int as libc::c_ulong
                            {
                                if 0 != 0 {
                                    let mut __c: libc::c_int = c as libc::c_int;
                                    __res = if __c < -(128 as libc::c_int)
                                        || __c > 255 as libc::c_int
                                    {
                                        __c
                                    } else {
                                        *(*__ctype_toupper_loc()).offset(__c as isize)
                                    };
                                } else {
                                    __res = toupper(c as libc::c_int);
                                }
                            } else {
                                __res = *(*__ctype_toupper_loc())
                                    .offset(c as libc::c_int as isize);
                            }
                            __res
                        })
                    } else if dir == -(1 as libc::c_int) {
                        ({
                            let mut __res: libc::c_int = 0;
                            if ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong
                                > 1 as libc::c_int as libc::c_ulong
                            {
                                if 0 != 0 {
                                    let mut __c: libc::c_int = c as libc::c_int;
                                    __res = if __c < -(128 as libc::c_int)
                                        || __c > 255 as libc::c_int
                                    {
                                        __c
                                    } else {
                                        *(*__ctype_tolower_loc()).offset(__c as isize)
                                    };
                                } else {
                                    __res = tolower(c as libc::c_int);
                                }
                            } else {
                                __res = *(*__ctype_tolower_loc())
                                    .offset(c as libc::c_int as isize);
                            }
                            __res
                        })
                    } else {
                        *p as libc::c_int
                    },
                    out,
                );
            }
        } else {
            match c {
                8 => {
                    fputs(b"\\b\0" as *const u8 as *const libc::c_char, out);
                }
                9 => {
                    fputs(b"\\t\0" as *const u8 as *const libc::c_char, out);
                }
                10 => {
                    fputs(b"\\n\0" as *const u8 as *const libc::c_char, out);
                }
                12 => {
                    fputs(b"\\f\0" as *const u8 as *const libc::c_char, out);
                }
                13 => {
                    fputs(b"\\r\0" as *const u8 as *const libc::c_char, out);
                }
                _ => {
                    fprintf(out, b"\\u00%02x\0" as *const u8 as *const libc::c_char, c);
                }
            }
        }
        p = p.offset(1);
        p;
    }
    fputc('"' as i32, out);
}
#[no_mangle]
pub unsafe extern "C" fn ul_jsonwrt_init(
    mut fmt: *mut ul_jsonwrt,
    mut out: *mut FILE,
    mut indent: libc::c_int,
) {
    (*fmt).out = out;
    (*fmt).indent = indent;
    (*fmt).set_after_close(0 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn ul_jsonwrt_is_ready(mut fmt: *mut ul_jsonwrt) -> libc::c_int {
    return if ((*fmt).out).is_null() { 0 as libc::c_int } else { 1 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn ul_jsonwrt_indent(mut fmt: *mut ul_jsonwrt) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*fmt).indent {
        fputs(b"   \0" as *const u8 as *const libc::c_char, (*fmt).out);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ul_jsonwrt_open(
    mut fmt: *mut ul_jsonwrt,
    mut name: *const libc::c_char,
    mut type_0: libc::c_int,
) {
    if !name.is_null() {
        if (*fmt).after_close() != 0 {
            fputs(b",\n\0" as *const u8 as *const libc::c_char, (*fmt).out);
        }
        ul_jsonwrt_indent(fmt);
        fputs_quoted_case_json(name, (*fmt).out, -(1 as libc::c_int));
    } else if (*fmt).after_close() != 0 {
        fputs(b",\0" as *const u8 as *const libc::c_char, (*fmt).out);
    } else {
        ul_jsonwrt_indent(fmt);
    }
    match type_0 {
        0 => {
            fputs(
                if !name.is_null() {
                    b": {\n\0" as *const u8 as *const libc::c_char
                } else {
                    b"{\n\0" as *const u8 as *const libc::c_char
                },
                (*fmt).out,
            );
            (*fmt).indent += 1;
            (*fmt).indent;
        }
        1 => {
            fputs(
                if !name.is_null() {
                    b": [\n\0" as *const u8 as *const libc::c_char
                } else {
                    b"[\n\0" as *const u8 as *const libc::c_char
                },
                (*fmt).out,
            );
            (*fmt).indent += 1;
            (*fmt).indent;
        }
        2 => {
            fputs(
                if !name.is_null() {
                    b": \0" as *const u8 as *const libc::c_char
                } else {
                    b" \0" as *const u8 as *const libc::c_char
                },
                (*fmt).out,
            );
        }
        _ => {}
    }
    (*fmt).set_after_close(0 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn ul_jsonwrt_close(
    mut fmt: *mut ul_jsonwrt,
    mut type_0: libc::c_int,
) {
    if (*fmt).indent == 1 as libc::c_int {
        fputs(b"\n}\n\0" as *const u8 as *const libc::c_char, (*fmt).out);
        (*fmt).indent -= 1;
        (*fmt).indent;
        (*fmt).set_after_close(1 as libc::c_int as libc::c_uint);
        return;
    }
    if (*fmt).indent > 0 as libc::c_int {} else {
        __assert_fail(
            b"fmt->indent > 0\0" as *const u8 as *const libc::c_char,
            b"lib/jsonwrt.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"void ul_jsonwrt_close(struct ul_jsonwrt *, int)\0"))
                .as_ptr(),
        );
    }
    'c_9017: {
        if (*fmt).indent > 0 as libc::c_int {} else {
            __assert_fail(
                b"fmt->indent > 0\0" as *const u8 as *const libc::c_char,
                b"lib/jsonwrt.c\0" as *const u8 as *const libc::c_char,
                163 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"void ul_jsonwrt_close(struct ul_jsonwrt *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    match type_0 {
        0 => {
            (*fmt).indent -= 1;
            (*fmt).indent;
            fputc('\n' as i32, (*fmt).out);
            ul_jsonwrt_indent(fmt);
            fputs(b"}\0" as *const u8 as *const libc::c_char, (*fmt).out);
        }
        1 => {
            (*fmt).indent -= 1;
            (*fmt).indent;
            fputc('\n' as i32, (*fmt).out);
            ul_jsonwrt_indent(fmt);
            fputs(b"]\0" as *const u8 as *const libc::c_char, (*fmt).out);
        }
        2 | _ => {}
    }
    (*fmt).set_after_close(1 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn ul_jsonwrt_value_raw(
    mut fmt: *mut ul_jsonwrt,
    mut name: *const libc::c_char,
    mut data: *const libc::c_char,
) {
    ul_jsonwrt_open(fmt, name, UL_JSON_VALUE as libc::c_int);
    if !data.is_null() && *data as libc::c_int != 0 {
        fputs(data, (*fmt).out);
    } else {
        fputs(b"null\0" as *const u8 as *const libc::c_char, (*fmt).out);
    }
    ul_jsonwrt_close(fmt, UL_JSON_VALUE as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ul_jsonwrt_value_s(
    mut fmt: *mut ul_jsonwrt,
    mut name: *const libc::c_char,
    mut data: *const libc::c_char,
) {
    ul_jsonwrt_open(fmt, name, UL_JSON_VALUE as libc::c_int);
    if !data.is_null() && *data as libc::c_int != 0 {
        fputs_quoted_case_json(data, (*fmt).out, 0 as libc::c_int);
    } else {
        fputs(b"null\0" as *const u8 as *const libc::c_char, (*fmt).out);
    }
    ul_jsonwrt_close(fmt, UL_JSON_VALUE as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ul_jsonwrt_value_u64(
    mut fmt: *mut ul_jsonwrt,
    mut name: *const libc::c_char,
    mut data: uint64_t,
) {
    ul_jsonwrt_open(fmt, name, UL_JSON_VALUE as libc::c_int);
    fprintf((*fmt).out, b"%lu\0" as *const u8 as *const libc::c_char, data);
    ul_jsonwrt_close(fmt, UL_JSON_VALUE as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ul_jsonwrt_value_boolean(
    mut fmt: *mut ul_jsonwrt,
    mut name: *const libc::c_char,
    mut data: libc::c_int,
) {
    ul_jsonwrt_open(fmt, name, UL_JSON_VALUE as libc::c_int);
    fputs(
        if data != 0 {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
        (*fmt).out,
    );
    ul_jsonwrt_close(fmt, UL_JSON_VALUE as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ul_jsonwrt_value_null(
    mut fmt: *mut ul_jsonwrt,
    mut name: *const libc::c_char,
) {
    ul_jsonwrt_open(fmt, name, UL_JSON_VALUE as libc::c_int);
    fputs(b"null\0" as *const u8 as *const libc::c_char, (*fmt).out);
    ul_jsonwrt_close(fmt, UL_JSON_VALUE as libc::c_int);
}
