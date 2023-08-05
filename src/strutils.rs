use ::libc;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vasprintf(
        __ptr: *mut *mut libc::c_char,
        __f: *const libc::c_char,
        __arg: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strtoimax(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> intmax_t;
    fn strtoumax(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> uintmax_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __errno_location() -> *mut libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn err(__status: libc::c_int, __format: *const libc::c_char, _: ...) -> !;
    fn errx(__status: libc::c_int, __format: *const libc::c_char, _: ...) -> !;
    fn localeconv() -> *mut lconv;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
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
pub type mode_t = __mode_t;
pub type time_t = __time_t;
pub type suseconds_t = __suseconds_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
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
pub struct lconv {
    pub decimal_point: *mut libc::c_char,
    pub thousands_sep: *mut libc::c_char,
    pub grouping: *mut libc::c_char,
    pub int_curr_symbol: *mut libc::c_char,
    pub currency_symbol: *mut libc::c_char,
    pub mon_decimal_point: *mut libc::c_char,
    pub mon_thousands_sep: *mut libc::c_char,
    pub mon_grouping: *mut libc::c_char,
    pub positive_sign: *mut libc::c_char,
    pub negative_sign: *mut libc::c_char,
    pub int_frac_digits: libc::c_char,
    pub frac_digits: libc::c_char,
    pub p_cs_precedes: libc::c_char,
    pub p_sep_by_space: libc::c_char,
    pub n_cs_precedes: libc::c_char,
    pub n_sep_by_space: libc::c_char,
    pub p_sign_posn: libc::c_char,
    pub n_sign_posn: libc::c_char,
    pub int_p_cs_precedes: libc::c_char,
    pub int_p_sep_by_space: libc::c_char,
    pub int_n_cs_precedes: libc::c_char,
    pub int_n_sep_by_space: libc::c_char,
    pub int_p_sign_posn: libc::c_char,
    pub int_n_sign_posn: libc::c_char,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const SIZE_DECIMAL_2DIGITS: C2RustUnnamed_0 = 4;
pub const SIZE_SUFFIX_SPACE: C2RustUnnamed_0 = 2;
pub const SIZE_SUFFIX_3LETTER: C2RustUnnamed_0 = 1;
pub const SIZE_SUFFIX_1LETTER: C2RustUnnamed_0 = 0;
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn xstrncpy(dest: *mut libc::c_char, src: *const libc::c_char, n: size_t) {
    let mut len: size_t = if !src.is_null() {
        strlen(src)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    if len == 0 {
        return;
    }
    len = {
        let mut _min1: size_t = len;
        let mut _min2: libc::c_ulong = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        &mut _min1 as *mut size_t;
        &mut _min2 as *mut libc::c_ulong;
        if _min1 < _min2 {
            _min1
        } else {
            _min2
        }
    };
    memcpy(dest as *mut libc::c_void, src as *const libc::c_void, len);
    *dest.offset(len as isize) = 0 as libc::c_int as libc::c_char;
}
static mut STRTOXX_EXIT_CODE: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn strutils_set_exitcode(ex: libc::c_int) {
    STRTOXX_EXIT_CODE = ex;
}
unsafe extern "C" fn do_scale_by_power(
    x: *mut uintmax_t,
    base: libc::c_int,
    mut power: libc::c_int,
) -> libc::c_int {
    loop {
        let fresh0 = power;
        power = power - 1;
        if !(fresh0 != 0) {
            break;
        }
        if (18446744073709551615 as libc::c_ulong).wrapping_div(base as libc::c_ulong) < *x {
            return -(34 as libc::c_int);
        }
        *x = (*x as libc::c_ulong).wrapping_mul(base as libc::c_ulong) as uintmax_t as uintmax_t;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn parse_size(
    str: *const libc::c_char,
    res: *mut uintmax_t,
    power: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: uintmax_t = 0;
    let mut frac: uintmax_t = 0 as libc::c_int as uintmax_t;
    let mut base: libc::c_int = 1024 as libc::c_int;
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut pwr: libc::c_int = 0 as libc::c_int;
    let mut frac_zeros: libc::c_int = 0 as libc::c_int;
    static mut suf: *const libc::c_char = b"KMGTPEZY\0" as *const u8 as *const libc::c_char;
    static mut suf2: *const libc::c_char = b"kmgtpezy\0" as *const u8 as *const libc::c_char;
    let mut sp: *const libc::c_char = 0 as *const libc::c_char;
    *res = 0 as libc::c_int as uintmax_t;
    if str.is_null() || *str == 0 {
        rc = -(22 as libc::c_int);
    } else {
        p = str;
        while *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int == '-' as i32 {
            rc = -(22 as libc::c_int);
        } else {
            *__errno_location() = 0 as libc::c_int;
            end = 0 as *mut libc::c_char;
            x = strtoumax(str, &mut end, 0 as libc::c_int);
            if end == str as *mut libc::c_char
                || *__errno_location() != 0 as libc::c_int
                    && (x == 18446744073709551615 as libc::c_ulong
                        || x == 0 as libc::c_int as libc::c_ulong)
            {
                rc = if *__errno_location() != 0 {
                    -*__errno_location()
                } else {
                    -(22 as libc::c_int)
                };
            } else {
                if end.is_null() || *end == 0 {
                    current_block = 17743634220615971465;
                } else {
                    p = end;
                    loop {
                        if *p.offset(1 as libc::c_int as isize) as libc::c_int == 'i' as i32
                            && (*p.offset(2 as libc::c_int as isize) as libc::c_int == 'B' as i32
                                || *p.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'b' as i32)
                            && *p.offset(3 as libc::c_int as isize) == 0
                        {
                            base = 1024 as libc::c_int;
                            current_block = 9853141518545631134;
                            break;
                        } else if (*p.offset(1 as libc::c_int as isize) as libc::c_int
                            == 'B' as i32
                            || *p.offset(1 as libc::c_int as isize) as libc::c_int == 'b' as i32)
                            && *p.offset(2 as libc::c_int as isize) == 0
                        {
                            base = 1000 as libc::c_int;
                            current_block = 9853141518545631134;
                            break;
                        } else {
                            if !(*p.offset(1 as libc::c_int as isize) != 0) {
                                current_block = 9853141518545631134;
                                break;
                            }
                            let l: *const lconv = localeconv();
                            let dp: *const libc::c_char = if !l.is_null() {
                                (*l).decimal_point
                            } else {
                                0 as *mut libc::c_char
                            };
                            let dpsz: size_t = if !dp.is_null() {
                                strlen(dp)
                            } else {
                                0 as libc::c_int as libc::c_ulong
                            };
                            if frac == 0 as libc::c_int as libc::c_ulong
                                && *p as libc::c_int != 0
                                && !dp.is_null()
                                && strncmp(dp, p, dpsz) == 0 as libc::c_int
                            {
                                let mut fstr: *const libc::c_char = p.offset(dpsz as isize);
                                p = fstr;
                                while *p as libc::c_int == '0' as i32 {
                                    frac_zeros += 1;
                                    frac_zeros;
                                    p = p.offset(1);
                                    p;
                                }
                                fstr = p;
                                if *(*__ctype_b_loc()).offset(*fstr as libc::c_int as isize)
                                    as libc::c_int
                                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                                {
                                    *__errno_location() = 0 as libc::c_int;
                                    end = 0 as *mut libc::c_char;
                                    frac = strtoumax(fstr, &mut end, 0 as libc::c_int);
                                    if end == fstr as *mut libc::c_char
                                        || *__errno_location() != 0 as libc::c_int
                                            && (frac == 18446744073709551615 as libc::c_ulong
                                                || frac == 0 as libc::c_int as libc::c_ulong)
                                    {
                                        rc = if *__errno_location() != 0 {
                                            -*__errno_location()
                                        } else {
                                            -(22 as libc::c_int)
                                        };
                                        current_block = 17718845201884699228;
                                        break;
                                    }
                                } else {
                                    end = p as *mut libc::c_char;
                                }
                                if frac != 0 && (end.is_null() || *end == 0) {
                                    rc = -(22 as libc::c_int);
                                    current_block = 17718845201884699228;
                                    break;
                                } else {
                                    p = end;
                                }
                            } else {
                                rc = -(22 as libc::c_int);
                                current_block = 17718845201884699228;
                                break;
                            }
                        }
                    }
                    match current_block {
                        17718845201884699228 => {}
                        _ => {
                            sp = strchr(suf, *p as libc::c_int);
                            if !sp.is_null() {
                                pwr = (sp.offset_from(suf) as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                                    as libc::c_int;
                                current_block = 3934796541983872331;
                            } else {
                                sp = strchr(suf2, *p as libc::c_int);
                                if !sp.is_null() {
                                    pwr = (sp.offset_from(suf2) as libc::c_long
                                        + 1 as libc::c_int as libc::c_long)
                                        as libc::c_int;
                                    current_block = 3934796541983872331;
                                } else {
                                    rc = -(22 as libc::c_int);
                                    current_block = 17718845201884699228;
                                }
                            }
                            match current_block {
                                17718845201884699228 => {}
                                _ => {
                                    rc = do_scale_by_power(&mut x, base, pwr);
                                    if !power.is_null() {
                                        *power = pwr;
                                    }
                                    if frac != 0 && pwr != 0 {
                                        let mut i: libc::c_int = 0;
                                        let mut frac_div: uintmax_t =
                                            10 as libc::c_int as uintmax_t;
                                        let mut frac_poz: uintmax_t = 1 as libc::c_int as uintmax_t;
                                        let mut frac_base: uintmax_t =
                                            1 as libc::c_int as uintmax_t;
                                        do_scale_by_power(&mut frac_base, base, pwr);
                                        while frac_div < frac {
                                            if frac_div
                                                <= (18446744073709551615 as libc::c_ulong)
                                                    .wrapping_div(
                                                        10 as libc::c_int as libc::c_ulong,
                                                    )
                                            {
                                                frac_div = (frac_div as libc::c_ulong).wrapping_mul(
                                                    10 as libc::c_int as libc::c_ulong,
                                                )
                                                    as uintmax_t
                                                    as uintmax_t;
                                            } else {
                                                frac = (frac as libc::c_ulong).wrapping_div(
                                                    10 as libc::c_int as libc::c_ulong,
                                                )
                                                    as uintmax_t
                                                    as uintmax_t;
                                            }
                                        }
                                        i = 0 as libc::c_int;
                                        while i < frac_zeros {
                                            if frac_div
                                                <= (18446744073709551615 as libc::c_ulong)
                                                    .wrapping_div(
                                                        10 as libc::c_int as libc::c_ulong,
                                                    )
                                            {
                                                frac_div = (frac_div as libc::c_ulong).wrapping_mul(
                                                    10 as libc::c_int as libc::c_ulong,
                                                )
                                                    as uintmax_t
                                                    as uintmax_t;
                                            } else {
                                                frac = (frac as libc::c_ulong).wrapping_div(
                                                    10 as libc::c_int as libc::c_ulong,
                                                )
                                                    as uintmax_t
                                                    as uintmax_t;
                                            }
                                            i += 1;
                                            i;
                                        }
                                        loop {
                                            let seg: libc::c_uint = frac
                                                .wrapping_rem(10 as libc::c_int as libc::c_ulong)
                                                as libc::c_uint;
                                            let seg_div: uintmax_t =
                                                frac_div.wrapping_div(frac_poz);
                                            frac = (frac as libc::c_ulong)
                                                .wrapping_div(10 as libc::c_int as libc::c_ulong)
                                                as uintmax_t
                                                as uintmax_t;
                                            frac_poz = (frac_poz as libc::c_ulong)
                                                .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                                                as uintmax_t
                                                as uintmax_t;
                                            if seg != 0
                                                && seg_div.wrapping_div(seg as libc::c_ulong) != 0
                                            {
                                                x = (x as libc::c_ulong).wrapping_add(
                                                    frac_base.wrapping_div(
                                                        seg_div.wrapping_div(seg as libc::c_ulong),
                                                    ),
                                                )
                                                    as uintmax_t
                                                    as uintmax_t;
                                            }
                                            if !(frac != 0) {
                                                break;
                                            }
                                        }
                                    }
                                    current_block = 17743634220615971465;
                                }
                            }
                        }
                    }
                }
                match current_block {
                    17718845201884699228 => {}
                    _ => {
                        *res = x;
                    }
                }
            }
        }
    }
    if rc < 0 as libc::c_int {
        *__errno_location() = -rc;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn strtosize(str: *const libc::c_char, res: *mut uintmax_t) -> libc::c_int {
    return parse_size(str, res, 0 as *mut libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn isdigit_strend(
    str: *const libc::c_char,
    end: *mut *const libc::c_char,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = str;
    while !p.is_null()
        && *p as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        p = p.offset(1);
        p;
    }
    if !end.is_null() {
        *end = p;
    }
    return (!p.is_null() && p > str && *p == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn isxdigit_strend(
    str: *const libc::c_char,
    end: *mut *const libc::c_char,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = str;
    while !p.is_null()
        && *p as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        p = p.offset(1);
        p;
    }
    if !end.is_null() {
        *end = p;
    }
    return (!p.is_null() && p > str && *p == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn parse_switch(
    arg: *const libc::c_char,
    errmesg: *const libc::c_char,
    args: ...
) -> libc::c_int {
    let mut a: *const libc::c_char = 0 as *const libc::c_char;
    let mut b: *const libc::c_char = 0 as *const libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    loop {
        a = ap.arg::<*mut libc::c_char>();
        if a.is_null() {
            break;
        }
        b = ap.arg::<*mut libc::c_char>();
        if b.is_null() {
            break;
        }
        if strcmp(arg, a) == 0 as libc::c_int {
            return 1 as libc::c_int;
        }
        if strcmp(arg, b) == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    errx(
        STRTOXX_EXIT_CODE,
        b"%s: '%s'\0" as *const u8 as *const libc::c_char,
        errmesg,
        arg,
    );
}
#[no_mangle]
pub unsafe extern "C" fn strnchr(
    mut s: *const libc::c_char,
    mut maxlen: size_t,
    c: libc::c_int,
) -> *mut libc::c_char {
    loop {
        let fresh1 = maxlen;
        maxlen = maxlen.wrapping_sub(1);
        if !(fresh1 != 0 && *s as libc::c_int != '\0' as i32) {
            break;
        }
        if *s as libc::c_int == c as libc::c_char as libc::c_int {
            return s as *mut libc::c_char;
        }
        s = s.offset(1);
        s;
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn ul_strtos64(
    str: *const libc::c_char,
    num: *mut int64_t,
    base: libc::c_int,
) -> libc::c_int {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    if str.is_null() || *str as libc::c_int == '\0' as i32 {
        let ref mut fresh2 = *__errno_location();
        *fresh2 = 22 as libc::c_int;
        return -*fresh2;
    }
    *__errno_location() = 0 as libc::c_int;
    *num = strtoimax(str, &mut end, base);
    if *__errno_location() != 0 as libc::c_int {
        return -*__errno_location();
    }
    if str == end as *const libc::c_char || !end.is_null() && *end as libc::c_int != 0 {
        let ref mut fresh3 = *__errno_location();
        *fresh3 = 22 as libc::c_int;
        return -*fresh3;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_strtou64(
    str: *const libc::c_char,
    num: *mut uint64_t,
    base: libc::c_int,
) -> libc::c_int {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: int64_t = 0;
    if str.is_null() || *str as libc::c_int == '\0' as i32 {
        let ref mut fresh4 = *__errno_location();
        *fresh4 = 22 as libc::c_int;
        return -*fresh4;
    }
    *__errno_location() = 0 as libc::c_int;
    tmp = strtoimax(str, &mut end, base);
    if tmp < 0 as libc::c_int as libc::c_long {
        *__errno_location() = 34 as libc::c_int;
    } else {
        *__errno_location() = 0 as libc::c_int;
        *num = strtoumax(str, &mut end, base);
    }
    if *__errno_location() != 0 as libc::c_int {
        return -*__errno_location();
    }
    if str == end as *const libc::c_char || !end.is_null() && *end as libc::c_int != 0 {
        let ref mut fresh5 = *__errno_location();
        *fresh5 = 22 as libc::c_int;
        return -*fresh5;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_strtos32(
    str: *const libc::c_char,
    num: *mut int32_t,
    base: libc::c_int,
) -> libc::c_int {
    let mut tmp: int64_t = 0;
    let mut rc: libc::c_int = 0;
    rc = ul_strtos64(str, &mut tmp, base);
    if rc == 0 as libc::c_int
        && (tmp < (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_long
            || tmp > 2147483647 as libc::c_int as libc::c_long)
    {
        let ref mut fresh6 = *__errno_location();
        *fresh6 = 34 as libc::c_int;
        rc = -*fresh6;
    }
    if rc == 0 as libc::c_int {
        *num = tmp as int32_t;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn ul_strtou32(
    str: *const libc::c_char,
    num: *mut uint32_t,
    base: libc::c_int,
) -> libc::c_int {
    let mut tmp: uint64_t = 0;
    let mut rc: libc::c_int = 0;
    rc = ul_strtou64(str, &mut tmp, base);
    if rc == 0 as libc::c_int && tmp > 4294967295 as libc::c_uint as libc::c_ulong {
        let ref mut fresh7 = *__errno_location();
        *fresh7 = 34 as libc::c_int;
        rc = -*fresh7;
    }
    if rc == 0 as libc::c_int {
        *num = tmp as uint32_t;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn str2num_or_err(
    str: *const libc::c_char,
    base: libc::c_int,
    errmesg: *const libc::c_char,
    low: int64_t,
    up: int64_t,
) -> int64_t {
    let mut num: int64_t = 0 as libc::c_int as int64_t;
    let mut rc: libc::c_int = 0;
    rc = ul_strtos64(str, &mut num, base);
    if rc == 0 as libc::c_int && (low != 0 && num < low || up != 0 && num > up) {
        let ref mut fresh8 = *__errno_location();
        *fresh8 = 34 as libc::c_int;
        rc = -*fresh8;
    }
    if rc != 0 {
        if *__errno_location() == 34 as libc::c_int {
            err(
                STRTOXX_EXIT_CODE,
                b"%s: '%s'\0" as *const u8 as *const libc::c_char,
                errmesg,
                str,
            );
        }
        errx(
            STRTOXX_EXIT_CODE,
            b"%s: '%s'\0" as *const u8 as *const libc::c_char,
            errmesg,
            str,
        );
    }
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn str2unum_or_err(
    str: *const libc::c_char,
    base: libc::c_int,
    errmesg: *const libc::c_char,
    up: uint64_t,
) -> uint64_t {
    let mut num: uint64_t = 0 as libc::c_int as uint64_t;
    let mut rc: libc::c_int = 0;
    rc = ul_strtou64(str, &mut num, base);
    if rc == 0 as libc::c_int && (up != 0 && num > up) {
        let ref mut fresh9 = *__errno_location();
        *fresh9 = 34 as libc::c_int;
        rc = -*fresh9;
    }
    if rc != 0 {
        if *__errno_location() == 34 as libc::c_int {
            err(
                STRTOXX_EXIT_CODE,
                b"%s: '%s'\0" as *const u8 as *const libc::c_char,
                errmesg,
                str,
            );
        }
        errx(
            STRTOXX_EXIT_CODE,
            b"%s: '%s'\0" as *const u8 as *const libc::c_char,
            errmesg,
            str,
        );
    }
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn strtod_or_err(
    str: *const libc::c_char,
    errmesg: *const libc::c_char,
) -> libc::c_double {
    let mut num: libc::c_double = 0.;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    *__errno_location() = 0 as libc::c_int;
    if !(str.is_null() || *str as libc::c_int == '\0' as i32) {
        num = strtod(str, &mut end);
        if !(*__errno_location() != 0
            || str == end as *const libc::c_char
            || !end.is_null() && *end as libc::c_int != 0)
        {
            return num;
        }
    }
    if *__errno_location() == 34 as libc::c_int {
        err(
            STRTOXX_EXIT_CODE,
            b"%s: '%s'\0" as *const u8 as *const libc::c_char,
            errmesg,
            str,
        );
    }
    errx(
        STRTOXX_EXIT_CODE,
        b"%s: '%s'\0" as *const u8 as *const libc::c_char,
        errmesg,
        str,
    );
}
#[no_mangle]
pub unsafe extern "C" fn strtosize_or_err(
    str: *const libc::c_char,
    errmesg: *const libc::c_char,
) -> uintmax_t {
    let mut num: uintmax_t = 0;
    if strtosize(str, &mut num) == 0 as libc::c_int {
        return num;
    }
    if *__errno_location() != 0 {
        err(
            STRTOXX_EXIT_CODE,
            b"%s: '%s'\0" as *const u8 as *const libc::c_char,
            errmesg,
            str,
        );
    }
    errx(
        STRTOXX_EXIT_CODE,
        b"%s: '%s'\0" as *const u8 as *const libc::c_char,
        errmesg,
        str,
    );
}
#[no_mangle]
pub unsafe extern "C" fn strtotime_or_err(
    str: *const libc::c_char,
    errmesg: *const libc::c_char,
) -> time_t {
    let mut user_input: int64_t = 0;
    user_input = str2num_or_err(
        str,
        10 as libc::c_int,
        errmesg,
        0 as libc::c_int as int64_t,
        0 as libc::c_int as int64_t,
    );
    return user_input;
}
#[no_mangle]
pub unsafe extern "C" fn xstrmode(mode: mode_t, str: *mut libc::c_char) -> *mut libc::c_char {
    let mut i: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    if mode & 0o170000 as libc::c_int as libc::c_uint == 0o40000 as libc::c_int as libc::c_uint {
        let fresh10 = i;
        i = i.wrapping_add(1);
        *str.offset(fresh10 as isize) = 'd' as i32 as libc::c_char;
    } else if mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o120000 as libc::c_int as libc::c_uint
    {
        let fresh11 = i;
        i = i.wrapping_add(1);
        *str.offset(fresh11 as isize) = 'l' as i32 as libc::c_char;
    } else if mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o20000 as libc::c_int as libc::c_uint
    {
        let fresh12 = i;
        i = i.wrapping_add(1);
        *str.offset(fresh12 as isize) = 'c' as i32 as libc::c_char;
    } else if mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o60000 as libc::c_int as libc::c_uint
    {
        let fresh13 = i;
        i = i.wrapping_add(1);
        *str.offset(fresh13 as isize) = 'b' as i32 as libc::c_char;
    } else if mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o140000 as libc::c_int as libc::c_uint
    {
        let fresh14 = i;
        i = i.wrapping_add(1);
        *str.offset(fresh14 as isize) = 's' as i32 as libc::c_char;
    } else if mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o10000 as libc::c_int as libc::c_uint
    {
        let fresh15 = i;
        i = i.wrapping_add(1);
        *str.offset(fresh15 as isize) = 'p' as i32 as libc::c_char;
    } else if mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
    {
        let fresh16 = i;
        i = i.wrapping_add(1);
        *str.offset(fresh16 as isize) = '-' as i32 as libc::c_char;
    }
    let fresh17 = i;
    i = i.wrapping_add(1);
    *str.offset(fresh17 as isize) = (if mode & 0o400 as libc::c_int as libc::c_uint != 0 {
        'r' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    let fresh18 = i;
    i = i.wrapping_add(1);
    *str.offset(fresh18 as isize) = (if mode & 0o200 as libc::c_int as libc::c_uint != 0 {
        'w' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    let fresh19 = i;
    i = i.wrapping_add(1);
    *str.offset(fresh19 as isize) = (if mode & 0o4000 as libc::c_int as libc::c_uint != 0 {
        if mode & 0o100 as libc::c_int as libc::c_uint != 0 {
            's' as i32
        } else {
            'S' as i32
        }
    } else if mode & 0o100 as libc::c_int as libc::c_uint != 0 {
        'x' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    let fresh20 = i;
    i = i.wrapping_add(1);
    *str.offset(fresh20 as isize) =
        (if mode & (0o400 as libc::c_int >> 3 as libc::c_int) as libc::c_uint != 0 {
            'r' as i32
        } else {
            '-' as i32
        }) as libc::c_char;
    let fresh21 = i;
    i = i.wrapping_add(1);
    *str.offset(fresh21 as isize) =
        (if mode & (0o200 as libc::c_int >> 3 as libc::c_int) as libc::c_uint != 0 {
            'w' as i32
        } else {
            '-' as i32
        }) as libc::c_char;
    let fresh22 = i;
    i = i.wrapping_add(1);
    *str.offset(fresh22 as isize) = (if mode & 0o2000 as libc::c_int as libc::c_uint != 0 {
        if mode & (0o100 as libc::c_int >> 3 as libc::c_int) as libc::c_uint != 0 {
            's' as i32
        } else {
            'S' as i32
        }
    } else if mode & (0o100 as libc::c_int >> 3 as libc::c_int) as libc::c_uint != 0 {
        'x' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    let fresh23 = i;
    i = i.wrapping_add(1);
    *str.offset(fresh23 as isize) = (if mode
        & (0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_uint
        != 0
    {
        'r' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    let fresh24 = i;
    i = i.wrapping_add(1);
    *str.offset(fresh24 as isize) = (if mode
        & (0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_uint
        != 0
    {
        'w' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    let fresh25 = i;
    i = i.wrapping_add(1);
    *str.offset(fresh25 as isize) = (if mode & 0o1000 as libc::c_int as libc::c_uint != 0 {
        if mode & (0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_uint
            != 0
        {
            't' as i32
        } else {
            'T' as i32
        }
    } else if mode & (0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_uint
        != 0
    {
        'x' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    *str.offset(i as isize) = '\0' as i32 as libc::c_char;
    return str;
}
unsafe extern "C" fn get_exp(n: uint64_t) -> libc::c_int {
    let mut shft: libc::c_int = 0;
    shft = 10 as libc::c_int;
    while shft <= 60 as libc::c_int {
        if (n as libc::c_ulonglong) < (1 as libc::c_ulonglong) << shft {
            break;
        }
        shft += 10 as libc::c_int;
    }
    return shft - 10 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn size_to_human_string(
    options: libc::c_int,
    bytes: uint64_t,
) -> *mut libc::c_char {
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut dec: libc::c_int = 0;
    let mut exp: libc::c_int = 0;
    let mut frac: uint64_t = 0;
    let letters: *const libc::c_char = b"BKMGTPE\0" as *const u8 as *const libc::c_char;
    let mut suffix: [libc::c_char; 5] = [0; 5];
    let mut psuf: *mut libc::c_char = suffix.as_mut_ptr();
    let mut c: libc::c_char = 0;
    if options & SIZE_SUFFIX_SPACE as libc::c_int != 0 {
        let fresh26 = psuf;
        psuf = psuf.offset(1);
        *fresh26 = ' ' as i32 as libc::c_char;
    }
    exp = get_exp(bytes);
    c = *letters.offset(
        (if exp != 0 {
            exp / 10 as libc::c_int
        } else {
            0 as libc::c_int
        }) as isize,
    );
    dec = (if exp != 0 {
        (bytes as libc::c_ulonglong).wrapping_div((1 as libc::c_ulonglong) << exp)
    } else {
        bytes as libc::c_ulonglong
    }) as libc::c_int;
    frac = (if exp != 0 {
        (bytes as libc::c_ulonglong).wrapping_rem((1 as libc::c_ulonglong) << exp)
    } else {
        0 as libc::c_int as libc::c_ulonglong
    }) as uint64_t;
    let fresh27 = psuf;
    psuf = psuf.offset(1);
    *fresh27 = c;
    if options & SIZE_SUFFIX_3LETTER as libc::c_int != 0 && c as libc::c_int != 'B' as i32 {
        let fresh28 = psuf;
        psuf = psuf.offset(1);
        *fresh28 = 'i' as i32 as libc::c_char;
        let fresh29 = psuf;
        psuf = psuf.offset(1);
        *fresh29 = 'B' as i32 as libc::c_char;
    }
    *psuf = '\0' as i32 as libc::c_char;
    if frac != 0 {
        if frac
            >= (18446744073709551615 as libc::c_ulong)
                .wrapping_div(1000 as libc::c_int as libc::c_ulong)
        {
            frac = (frac
                .wrapping_div(1024 as libc::c_int as libc::c_ulong)
                .wrapping_mul(1000 as libc::c_int as libc::c_ulong)
                as libc::c_ulonglong)
                .wrapping_div((1 as libc::c_ulonglong) << exp - 10 as libc::c_int)
                as uint64_t;
        } else {
            frac = (frac.wrapping_mul(1000 as libc::c_int as libc::c_ulong) as libc::c_ulonglong)
                .wrapping_div((1 as libc::c_ulonglong) << exp) as uint64_t;
        }
        if options & SIZE_DECIMAL_2DIGITS as libc::c_int != 0 {
            frac = frac
                .wrapping_add(5 as libc::c_int as libc::c_ulong)
                .wrapping_div(10 as libc::c_int as libc::c_ulong);
        } else {
            frac = frac
                .wrapping_add(50 as libc::c_int as libc::c_ulong)
                .wrapping_div(100 as libc::c_int as libc::c_ulong)
                .wrapping_mul(10 as libc::c_int as libc::c_ulong);
        }
        if frac == 100 as libc::c_int as libc::c_ulong {
            dec += 1;
            dec;
            frac = 0 as libc::c_int as uint64_t;
        }
    }
    if frac != 0 {
        let l: *const lconv = localeconv();
        let mut dp: *mut libc::c_char = if !l.is_null() {
            (*l).decimal_point
        } else {
            0 as *mut libc::c_char
        };
        let mut len: libc::c_int = 0;
        if dp.is_null() || *dp == 0 {
            dp = b".\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        len = snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%d%s%02lu\0" as *const u8 as *const libc::c_char,
            dec,
            dp,
            frac,
        );
        if len > 0 as libc::c_int
            && (len as size_t) < ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong
        {
            if buf[(len - 1 as libc::c_int) as usize] as libc::c_int == '0' as i32 {
                let fresh30 = len;
                len = len - 1;
                buf[fresh30 as usize] = '\0' as i32 as libc::c_char;
            }
            xstrncpy(
                buf.as_mut_ptr().offset(len as isize),
                suffix.as_mut_ptr(),
                (::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                    .wrapping_sub(len as libc::c_ulong),
            );
        } else {
            *buf.as_mut_ptr() = '\0' as i32 as libc::c_char;
        }
    } else {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%d%s\0" as *const u8 as *const libc::c_char,
            dec,
            suffix.as_mut_ptr(),
        );
    }
    return strdup(buf.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn string_to_idarray(
    list: *const libc::c_char,
    ary: *mut libc::c_int,
    arysz: size_t,
    name2id: Option<unsafe extern "C" fn(*const libc::c_char, size_t) -> libc::c_int>,
) -> libc::c_int {
    let mut begin: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: size_t = 0 as libc::c_int as size_t;
    if list.is_null() || *list == 0 || ary.is_null() || arysz == 0 || name2id.is_none() {
        return -(1 as libc::c_int);
    }
    p = list;
    while !p.is_null() && *p as libc::c_int != 0 {
        let mut end: *const libc::c_char = 0 as *const libc::c_char;
        let mut id: libc::c_int = 0;
        if n >= arysz {
            return -(2 as libc::c_int);
        }
        if begin.is_null() {
            begin = p;
        }
        if *p as libc::c_int == ',' as i32 {
            end = p;
        }
        if *p.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
            end = p.offset(1 as libc::c_int as isize);
        }
        if !(begin.is_null() || end.is_null()) {
            if end <= begin {
                return -(1 as libc::c_int);
            }
            id = name2id.expect("non-null function pointer")(
                begin,
                end.offset_from(begin) as libc::c_long as size_t,
            );
            if id == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            let fresh31 = n;
            n = n.wrapping_add(1);
            *ary.offset(fresh31 as isize) = id;
            begin = 0 as *const libc::c_char;
            if !end.is_null() && *end == 0 {
                break;
            }
        }
        p = p.offset(1);
        p;
    }
    return n as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn string_add_to_idarray(
    list: *const libc::c_char,
    ary: *mut libc::c_int,
    arysz: size_t,
    ary_pos: *mut size_t,
    name2id: Option<unsafe extern "C" fn(*const libc::c_char, size_t) -> libc::c_int>,
) -> libc::c_int {
    let mut list_add: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: libc::c_int = 0;
    if list.is_null() || *list == 0 || ary_pos.is_null() || *ary_pos > arysz {
        return -(1 as libc::c_int);
    }
    if *list.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32 {
        list_add = &*list.offset(1 as libc::c_int as isize) as *const libc::c_char;
    } else {
        list_add = list;
        *ary_pos = 0 as libc::c_int as size_t;
    }
    r = string_to_idarray(
        list_add,
        &mut *ary.offset(*ary_pos as isize),
        arysz.wrapping_sub(*ary_pos),
        name2id,
    );
    if r > 0 as libc::c_int {
        *ary_pos = (*ary_pos as libc::c_ulong).wrapping_add(r as libc::c_ulong) as size_t as size_t;
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn string_to_bitarray(
    list: *const libc::c_char,
    ary: *mut libc::c_char,
    name2bit: Option<unsafe extern "C" fn(*const libc::c_char, size_t) -> libc::c_int>,
    allow_range: size_t,
) -> libc::c_int {
    let mut begin: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if list.is_null() || name2bit.is_none() || ary.is_null() {
        return -(22 as libc::c_int);
    }
    p = list;
    while !p.is_null() && *p as libc::c_int != 0 {
        let mut end: *const libc::c_char = 0 as *const libc::c_char;
        let mut bit: libc::c_int = 0;
        let mut set_lower: libc::c_int = 0 as libc::c_int;
        let mut set_higher: libc::c_int = 0 as libc::c_int;
        if begin.is_null() {
            begin = p;
        }
        if *p as libc::c_int == ',' as i32 {
            end = p;
        }
        if *p.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
            end = p.offset(1 as libc::c_int as isize);
        }
        if !(begin.is_null() || end.is_null()) {
            if end <= begin {
                return -(1 as libc::c_int);
            }
            if allow_range != 0 {
                if *end.offset(-(1 as libc::c_int as isize)) as libc::c_int == '+' as i32 {
                    end = end.offset(-1);
                    end;
                    set_lower = 1 as libc::c_int;
                } else if *begin as libc::c_int == '+' as i32 {
                    begin = begin.offset(1);
                    begin;
                    set_higher = 1 as libc::c_int;
                }
            }
            bit = name2bit.expect("non-null function pointer")(
                begin,
                end.offset_from(begin) as libc::c_long as size_t,
            );
            if bit < 0 as libc::c_int {
                return bit;
            }
            let ref mut fresh32 = *ary.offset((bit / 8 as libc::c_int) as isize);
            *fresh32 = (*fresh32 as libc::c_int | (1 as libc::c_int) << bit % 8 as libc::c_int)
                as libc::c_char;
            if set_lower != 0 {
                loop {
                    bit -= 1;
                    if !(bit >= 0 as libc::c_int) {
                        break;
                    }
                    let ref mut fresh33 = *ary.offset((bit / 8 as libc::c_int) as isize);
                    *fresh33 = (*fresh33 as libc::c_int
                        | (1 as libc::c_int) << bit % 8 as libc::c_int)
                        as libc::c_char;
                }
            } else if set_higher != 0 {
                loop {
                    bit += 1;
                    if !(bit < allow_range as libc::c_int) {
                        break;
                    }
                    let ref mut fresh34 = *ary.offset((bit / 8 as libc::c_int) as isize);
                    *fresh34 = (*fresh34 as libc::c_int
                        | (1 as libc::c_int) << bit % 8 as libc::c_int)
                        as libc::c_char;
                }
            }
            begin = 0 as *const libc::c_char;
            if !end.is_null() && *end == 0 {
                break;
            }
        }
        p = p.offset(1);
        p;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn string_to_bitmask(
    list: *const libc::c_char,
    mask: *mut libc::c_ulong,
    name2flag: Option<unsafe extern "C" fn(*const libc::c_char, size_t) -> libc::c_long>,
) -> libc::c_int {
    let mut begin: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if list.is_null() || name2flag.is_none() || mask.is_null() {
        return -(22 as libc::c_int);
    }
    p = list;
    while !p.is_null() && *p as libc::c_int != 0 {
        let mut end: *const libc::c_char = 0 as *const libc::c_char;
        let mut flag: libc::c_long = 0;
        if begin.is_null() {
            begin = p;
        }
        if *p as libc::c_int == ',' as i32 {
            end = p;
        }
        if *p.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
            end = p.offset(1 as libc::c_int as isize);
        }
        if !(begin.is_null() || end.is_null()) {
            if end <= begin {
                return -(1 as libc::c_int);
            }
            flag = name2flag.expect("non-null function pointer")(
                begin,
                end.offset_from(begin) as libc::c_long as size_t,
            );
            if flag < 0 as libc::c_int as libc::c_long {
                return flag as libc::c_int;
            }
            *mask |= flag as libc::c_ulong;
            begin = 0 as *const libc::c_char;
            if !end.is_null() && *end == 0 {
                break;
            }
        }
        p = p.offset(1);
        p;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn parse_range(
    mut str: *const libc::c_char,
    lower: *mut libc::c_int,
    upper: *mut libc::c_int,
    def: libc::c_int,
) -> libc::c_int {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    if str.is_null() {
        return 0 as libc::c_int;
    }
    *lower = def;
    *upper = *lower;
    *__errno_location() = 0 as libc::c_int;
    if *str as libc::c_int == ':' as i32 {
        str = str.offset(1);
        str;
        *upper = strtol(str, &mut end, 10 as libc::c_int) as libc::c_int;
        if *__errno_location() != 0
            || end.is_null()
            || *end as libc::c_int != 0
            || end == str as *mut libc::c_char
        {
            return -(1 as libc::c_int);
        }
    } else {
        *lower = strtol(str, &mut end, 10 as libc::c_int) as libc::c_int;
        *upper = *lower;
        if *__errno_location() != 0 || end.is_null() || end == str as *mut libc::c_char {
            return -(1 as libc::c_int);
        }
        if *end as libc::c_int == ':' as i32 && *end.offset(1 as libc::c_int as isize) == 0 {
            *upper = def;
        } else if *end as libc::c_int == '-' as i32 || *end as libc::c_int == ':' as i32 {
            str = end.offset(1 as libc::c_int as isize);
            end = 0 as *mut libc::c_char;
            *__errno_location() = 0 as libc::c_int;
            *upper = strtol(str, &mut end, 10 as libc::c_int) as libc::c_int;
            if *__errno_location() != 0
                || end.is_null()
                || *end as libc::c_int != 0
                || end == str as *mut libc::c_char
            {
                return -(1 as libc::c_int);
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn next_path_segment(
    str: *const libc::c_char,
    sz: *mut size_t,
) -> *const libc::c_char {
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    start = str;
    *sz = 0 as libc::c_int as size_t;
    while !start.is_null()
        && *start as libc::c_int == '/' as i32
        && *start.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        start = start.offset(1);
        start;
    }
    if start.is_null() || *start == 0 {
        return 0 as *const libc::c_char;
    }
    *sz = 1 as libc::c_int as size_t;
    p = start.offset(1 as libc::c_int as isize);
    while *p as libc::c_int != 0 && *p as libc::c_int != '/' as i32 {
        *sz = (*sz).wrapping_add(1);
        *sz;
        p = p.offset(1);
        p;
    }
    return start;
}
#[no_mangle]
pub unsafe extern "C" fn streq_paths(
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
) -> libc::c_int {
    while !a.is_null() && !b.is_null() {
        let mut a_sz: size_t = 0;
        let mut b_sz: size_t = 0;
        let a_seg: *const libc::c_char = next_path_segment(a, &mut a_sz);
        let b_seg: *const libc::c_char = next_path_segment(b, &mut b_sz);
        if a_sz.wrapping_add(b_sz) == 0 as libc::c_int as libc::c_ulong {
            return 1 as libc::c_int;
        }
        if a_sz.wrapping_add(b_sz) == 1 as libc::c_int as libc::c_ulong
            && (!a_seg.is_null() && *a_seg as libc::c_int == '/' as i32
                || !b_seg.is_null() && *b_seg as libc::c_int == '/' as i32)
        {
            return 1 as libc::c_int;
        }
        if a_seg.is_null() || b_seg.is_null() {
            break;
        }
        if a_sz != b_sz || strncmp(a_seg, b_seg, a_sz) != 0 as libc::c_int {
            break;
        }
        a = a_seg.offset(a_sz as isize);
        b = b_seg.offset(b_sz as isize);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn strnconcat(
    s: *const libc::c_char,
    suffix: *const libc::c_char,
    b: size_t,
) -> *mut libc::c_char {
    let mut a: size_t = 0;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    if s.is_null() && suffix.is_null() {
        return strdup(b"\0" as *const u8 as *const libc::c_char);
    }
    if s.is_null() {
        return strndup(suffix, b);
    }
    if suffix.is_null() {
        return strdup(s);
    }
    if !s.is_null() {
    } else {
        __assert_fail(
            b"s\0" as *const u8 as *const libc::c_char,
            b"lib/strutils.c\0" as *const u8 as *const libc::c_char,
            945 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                b"char *strnconcat(const char *, const char *, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    'c_10751: {
        if !s.is_null() {
        } else {
            __assert_fail(
                b"s\0" as *const u8 as *const libc::c_char,
                b"lib/strutils.c\0" as *const u8 as *const libc::c_char,
                945 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                    b"char *strnconcat(const char *, const char *, size_t)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if !suffix.is_null() {
    } else {
        __assert_fail(
            b"suffix\0" as *const u8 as *const libc::c_char,
            b"lib/strutils.c\0" as *const u8 as *const libc::c_char,
            946 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                b"char *strnconcat(const char *, const char *, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    'c_10714: {
        if !suffix.is_null() {
        } else {
            __assert_fail(
                b"suffix\0" as *const u8 as *const libc::c_char,
                b"lib/strutils.c\0" as *const u8 as *const libc::c_char,
                946 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                    b"char *strnconcat(const char *, const char *, size_t)\0",
                ))
                .as_ptr(),
            );
        }
    };
    a = strlen(s);
    if b > (-(1 as libc::c_int) as size_t).wrapping_sub(a) {
        return 0 as *mut libc::c_char;
    }
    r = malloc(
        a.wrapping_add(b)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if r.is_null() {
        return 0 as *mut libc::c_char;
    }
    memcpy(r as *mut libc::c_void, s as *const libc::c_void, a);
    memcpy(
        r.offset(a as isize) as *mut libc::c_void,
        suffix as *const libc::c_void,
        b,
    );
    *r.offset(a.wrapping_add(b) as isize) = 0 as libc::c_int as libc::c_char;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn strconcat(
    s: *const libc::c_char,
    suffix: *const libc::c_char,
) -> *mut libc::c_char {
    return strnconcat(
        s,
        suffix,
        if !suffix.is_null() {
            strlen(suffix)
        } else {
            0 as libc::c_int as libc::c_ulong
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn strfconcat(
    s: *const libc::c_char,
    format: *const libc::c_char,
    args: ...
) -> *mut libc::c_char {
    let mut ap: ::core::ffi::VaListImpl;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sz: libc::c_int = 0;
    ap = args.clone();
    sz = vasprintf(&mut val, format, ap.as_va_list());
    if sz < 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    res = strnconcat(s, val, sz as size_t);
    free(val as *mut libc::c_void);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn strappend(
    a: *mut *mut libc::c_char,
    b: *const libc::c_char,
) -> libc::c_int {
    let mut al: size_t = 0;
    let mut bl: size_t = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    if a.is_null() {
        return -(22 as libc::c_int);
    }
    if b.is_null() || *b == 0 {
        return 0 as libc::c_int;
    }
    if (*a).is_null() {
        *a = strdup(b);
        return if (*a).is_null() {
            -(12 as libc::c_int)
        } else {
            0 as libc::c_int
        };
    }
    al = strlen(*a);
    bl = strlen(b);
    tmp = realloc(
        *a as *mut libc::c_void,
        al.wrapping_add(bl)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if tmp.is_null() {
        return -(12 as libc::c_int);
    }
    *a = tmp;
    memcpy(
        (*a).offset(al as isize) as *mut libc::c_void,
        b as *const libc::c_void,
        bl.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn strcspn_escaped(
    s: *const libc::c_char,
    reject: *const libc::c_char,
) -> size_t {
    let mut escaped: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while *s.offset(n as isize) != 0 {
        if escaped != 0 {
            escaped = 0 as libc::c_int;
        } else if *s.offset(n as isize) as libc::c_int == '\\' as i32 {
            escaped = 1 as libc::c_int;
        } else if !(strchr(reject, *s.offset(n as isize) as libc::c_int)).is_null() {
            break;
        }
        n += 1;
        n;
    }
    return (n - escaped) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ul_strchr_escaped(
    s: *const libc::c_char,
    c: libc::c_int,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut esc: libc::c_int = 0 as libc::c_int;
    p = s as *mut libc::c_char;
    while !p.is_null() && *p as libc::c_int != 0 {
        if esc == 0 && *p as libc::c_int == '\\' as i32 {
            esc = 1 as libc::c_int;
        } else {
            if *p as libc::c_int == c && (esc == 0 || c == '\\' as i32) {
                return p;
            }
            esc = 0 as libc::c_int;
        }
        p = p.offset(1);
        p;
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn split(
    state: *mut *const libc::c_char,
    l: *mut size_t,
    separator: *const libc::c_char,
    quoted: libc::c_int,
) -> *const libc::c_char {
    let mut current: *const libc::c_char = 0 as *const libc::c_char;
    current = *state;
    if *current == 0 {
        if **state as libc::c_int == '\0' as i32 {
        } else {
            __assert_fail(
                b"**state == '\\0'\0" as *const u8 as *const libc::c_char,
                b"lib/strutils.c\0" as *const u8 as *const libc::c_char,
                1070 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                    b"const char *split(const char **, size_t *, const char *, int)\0",
                ))
                .as_ptr(),
            );
        }
        'c_11392: {
            if **state as libc::c_int == '\0' as i32 {
            } else {
                __assert_fail(
                    b"**state == '\\0'\0" as *const u8 as *const libc::c_char,
                    b"lib/strutils.c\0" as *const u8 as *const libc::c_char,
                    1070 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                        b"const char *split(const char **, size_t *, const char *, int)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        return 0 as *const libc::c_char;
    }
    current = current.offset(strspn(current, separator) as isize);
    if *current == 0 {
        *state = current;
        return 0 as *const libc::c_char;
    }
    if quoted != 0
        && !(strchr(
            b"'\"\0" as *const u8 as *const libc::c_char,
            *current as libc::c_int,
        ))
        .is_null()
    {
        let mut quotechars: [libc::c_char; 2] = [*current, '\0' as i32 as libc::c_char];
        *l = strcspn_escaped(
            current.offset(1 as libc::c_int as isize),
            quotechars.as_mut_ptr(),
        );
        if *current.offset((*l).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int
            == '\0' as i32
            || *current.offset((*l).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                != quotechars[0 as libc::c_int as usize] as libc::c_int
            || *current.offset((*l).wrapping_add(2 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                != 0
                && (strchr(
                    separator,
                    *current.offset((*l).wrapping_add(2 as libc::c_int as libc::c_ulong) as isize)
                        as libc::c_int,
                ))
                .is_null()
        {
            *state = current;
            return 0 as *const libc::c_char;
        }
        let fresh35 = current;
        current = current.offset(1);
        *state = fresh35
            .offset(*l as isize)
            .offset(2 as libc::c_int as isize);
    } else if quoted != 0 {
        *l = strcspn_escaped(current, separator);
        if *current.offset(*l as isize) as libc::c_int != 0
            && (strchr(separator, *current.offset(*l as isize) as libc::c_int)).is_null()
        {
            *state = current;
            return 0 as *const libc::c_char;
        }
        *state = current.offset(*l as isize);
    } else {
        *l = strcspn(current, separator);
        *state = current.offset(*l as isize);
    }
    return current;
}
#[no_mangle]
pub unsafe extern "C" fn skip_fline(fp: *mut FILE) -> libc::c_int {
    let mut ch: libc::c_int = 0;
    loop {
        ch = fgetc(fp);
        if ch == -(1 as libc::c_int) {
            return 1 as libc::c_int;
        }
        if ch == '\n' as i32 {
            return 0 as libc::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ul_stralnumcmp(
    p1: *const libc::c_char,
    p2: *const libc::c_char,
) -> libc::c_int {
    let mut s1: *const libc::c_uchar = p1 as *const libc::c_uchar;
    let mut s2: *const libc::c_uchar = p2 as *const libc::c_uchar;
    let mut c1: libc::c_uchar = 0;
    let mut c2: libc::c_uchar = 0;
    loop {
        loop {
            let fresh36 = s1;
            s1 = s1.offset(1);
            c1 = *fresh36;
            if !(c1 as libc::c_int != '\0' as i32
                && *(*__ctype_b_loc()).offset(c1 as libc::c_uint as libc::c_int as isize)
                    as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                    == 0)
            {
                break;
            }
        }
        loop {
            let fresh37 = s2;
            s2 = s2.offset(1);
            c2 = *fresh37;
            if !(c2 as libc::c_int != '\0' as i32
                && *(*__ctype_b_loc()).offset(c2 as libc::c_uint as libc::c_int as isize)
                    as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                    == 0)
            {
                break;
            }
        }
        if c1 as libc::c_int != '\0' as i32 {
            c1 = ({
                let mut __res: libc::c_int = 0;
                if ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = c1 as libc::c_int;
                        __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = tolower(c1 as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc()).offset(c1 as libc::c_int as isize);
                }
                __res
            }) as libc::c_uchar;
        }
        if c2 as libc::c_int != '\0' as i32 {
            c2 = ({
                let mut __res: libc::c_int = 0;
                if ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = c2 as libc::c_int;
                        __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = tolower(c2 as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc()).offset(c2 as libc::c_int as isize);
                }
                __res
            }) as libc::c_uchar;
        }
        if c1 as libc::c_int == '\0' as i32 {
            return c1 as libc::c_int - c2 as libc::c_int;
        }
        if !(c1 as libc::c_int == c2 as libc::c_int) {
            break;
        }
    }
    return c1 as libc::c_int - c2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ul_optstr_next(
    optstr: *mut *mut libc::c_char,
    name: *mut *mut libc::c_char,
    namesz: *mut size_t,
    value: *mut *mut libc::c_char,
    valsz: *mut size_t,
) -> libc::c_int {
    let mut open_quote: libc::c_int = 0 as libc::c_int;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stop: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut optstr0: *mut libc::c_char = 0 as *mut libc::c_char;
    if !optstr.is_null() {
    } else {
        __assert_fail(
            b"optstr\0" as *const u8 as *const libc::c_char,
            b"lib/strutils.c\0" as *const u8 as *const libc::c_char,
            1165 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 66], &[libc::c_char; 66]>(
                b"int ul_optstr_next(char **, char **, size_t *, char **, size_t *)\0",
            ))
            .as_ptr(),
        );
    }
    'c_12200: {
        if !optstr.is_null() {
        } else {
            __assert_fail(
                b"optstr\0" as *const u8 as *const libc::c_char,
                b"lib/strutils.c\0" as *const u8 as *const libc::c_char,
                1165 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 66], &[libc::c_char; 66]>(
                    b"int ul_optstr_next(char **, char **, size_t *, char **, size_t *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if !(*optstr).is_null() {
    } else {
        __assert_fail(
            b"*optstr\0" as *const u8 as *const libc::c_char,
            b"lib/strutils.c\0" as *const u8 as *const libc::c_char,
            1166 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 66], &[libc::c_char; 66]>(
                b"int ul_optstr_next(char **, char **, size_t *, char **, size_t *)\0",
            ))
            .as_ptr(),
        );
    }
    'c_12163: {
        if !(*optstr).is_null() {
        } else {
            __assert_fail(
                b"*optstr\0" as *const u8 as *const libc::c_char,
                b"lib/strutils.c\0" as *const u8 as *const libc::c_char,
                1166 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 66], &[libc::c_char; 66]>(
                    b"int ul_optstr_next(char **, char **, size_t *, char **, size_t *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    optstr0 = *optstr;
    if !name.is_null() {
        *name = 0 as *mut libc::c_char;
    }
    if !namesz.is_null() {
        *namesz = 0 as libc::c_int as size_t;
    }
    if !value.is_null() {
        *value = 0 as *mut libc::c_char;
    }
    if !valsz.is_null() {
        *valsz = 0 as libc::c_int as size_t;
    }
    while !optstr0.is_null() && *optstr0 as libc::c_int == ',' as i32 {
        optstr0 = optstr0.offset(1);
        optstr0;
    }
    p = optstr0;
    while !p.is_null() && *p as libc::c_int != 0 {
        if start.is_null() {
            start = p;
        }
        if *p as libc::c_int == '"' as i32 {
            open_quote ^= 1 as libc::c_int;
        }
        if !(open_quote != 0) {
            if sep.is_null() && p > start && *p as libc::c_int == '=' as i32 {
                sep = p;
            }
            if *p as libc::c_int == ',' as i32 {
                stop = p;
            } else if *p.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
                stop = p.offset(1 as libc::c_int as isize);
            }
            if !(start.is_null() || stop.is_null()) {
                if stop <= start {
                    return -(22 as libc::c_int);
                }
                if !name.is_null() {
                    *name = start;
                }
                if !namesz.is_null() {
                    *namesz = (if !sep.is_null() {
                        sep.offset_from(start) as libc::c_long
                    } else {
                        stop.offset_from(start) as libc::c_long
                    }) as size_t;
                }
                *optstr = if *stop as libc::c_int != 0 {
                    stop.offset(1 as libc::c_int as isize)
                } else {
                    stop
                };
                if !sep.is_null() {
                    if !value.is_null() {
                        *value = sep.offset(1 as libc::c_int as isize);
                    }
                    if !valsz.is_null() {
                        *valsz = (stop.offset_from(sep) as libc::c_long
                            - 1 as libc::c_int as libc::c_long)
                            as size_t;
                    }
                }
                return 0 as libc::c_int;
            }
        }
        p = p.offset(1);
        p;
    }
    return 1 as libc::c_int;
}
