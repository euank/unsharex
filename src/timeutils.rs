use ::libc;
use ::c2rust_bitfields::BitfieldStruct;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn free(_: *mut libc::c_void);
    fn abs(_: libc::c_int) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn mktime(__tp: *mut tm) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn strptime(
        __s: *const libc::c_char,
        __fmt: *const libc::c_char,
        __tp: *mut tm,
    ) -> *mut libc::c_char;
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn warnx(__format: *const libc::c_char, _: ...);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type usec_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub name: *const libc::c_char,
    pub nr: libc::c_int,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_1 {}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_2 {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub suffix: *const libc::c_char,
    pub usec: usec_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_4 {}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const ISO_TIMESTAMP_COMMA_GT: C2RustUnnamed_5 = 407;
pub const ISO_TIMESTAMP_COMMA_G: C2RustUnnamed_5 = 279;
pub const ISO_TIMESTAMP_COMMA_T: C2RustUnnamed_5 = 151;
pub const ISO_TIMESTAMP_COMMA: C2RustUnnamed_5 = 23;
pub const ISO_TIMESTAMP_DOT_T: C2RustUnnamed_5 = 143;
pub const ISO_TIMESTAMP_DOT: C2RustUnnamed_5 = 15;
pub const ISO_TIMESTAMP_T: C2RustUnnamed_5 = 135;
pub const ISO_TIMESTAMP: C2RustUnnamed_5 = 7;
pub const ISO_GMTIME: C2RustUnnamed_5 = 256;
pub const ISO_T: C2RustUnnamed_5 = 128;
pub const ISO_COMMANSEC: C2RustUnnamed_5 = 64;
pub const ISO_DOTNSEC: C2RustUnnamed_5 = 32;
pub const ISO_COMMAUSEC: C2RustUnnamed_5 = 16;
pub const ISO_DOTUSEC: C2RustUnnamed_5 = 8;
pub const ISO_TIMEZONE: C2RustUnnamed_5 = 4;
pub const ISO_TIME: C2RustUnnamed_5 = 2;
pub const ISO_DATE: C2RustUnnamed_5 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub suffix: *const libc::c_char,
    pub width: libc::c_int,
    pub secs: int64_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_7 {}
#[inline]
unsafe extern "C" fn startswith(
    mut s: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> *const libc::c_char {
    let mut sz: size_t = if !prefix.is_null() {
        strlen(prefix)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    if !s.is_null() && sz != 0 && strncmp(s, prefix, sz) == 0 as libc::c_int {
        return s.offset(sz as isize);
    }
    return 0 as *const libc::c_char;
}
#[inline]
unsafe extern "C" fn startswith_no_case(
    mut s: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> *const libc::c_char {
    let mut sz: size_t = if !prefix.is_null() {
        strlen(prefix)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    if !s.is_null() && sz != 0 && strncasecmp(s, prefix, sz) == 0 as libc::c_int {
        return s.offset(sz as isize);
    }
    return 0 as *const libc::c_char;
}
#[inline]
unsafe extern "C" fn endswith(
    mut s: *const libc::c_char,
    mut postfix: *const libc::c_char,
) -> *const libc::c_char {
    let mut sl: size_t = if !s.is_null() {
        strlen(s)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mut pl: size_t = if !postfix.is_null() {
        strlen(postfix)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    if pl == 0 as libc::c_int as libc::c_ulong {
        return s.offset(sl as isize);
    }
    if sl < pl {
        return 0 as *const libc::c_char;
    }
    if memcmp(
        s.offset(sl as isize).offset(-(pl as isize)) as *const libc::c_void,
        postfix as *const libc::c_void,
        pl,
    ) != 0 as libc::c_int
    {
        return 0 as *const libc::c_char;
    }
    return s.offset(sl as isize).offset(-(pl as isize));
}
static mut table: [C2RustUnnamed_3; 28] = [C2RustUnnamed_3 {
    suffix: 0 as *const libc::c_char,
    usec: 0,
}; 28];
unsafe extern "C" fn parse_sec(
    mut t: *const libc::c_char,
    mut usec: *mut usec_t,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: usec_t = 0 as libc::c_int as usec_t;
    let mut something: libc::c_int = 0 as libc::c_int;
    if !t.is_null() {} else {
        __assert_fail(
            b"t\0" as *const u8 as *const libc::c_char,
            b"lib/timeutils.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"int parse_sec(const char *, usec_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7988: {
        if !t.is_null() {} else {
            __assert_fail(
                b"t\0" as *const u8 as *const libc::c_char,
                b"lib/timeutils.c\0" as *const u8 as *const libc::c_char,
                72 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"int parse_sec(const char *, usec_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !usec.is_null() {} else {
        __assert_fail(
            b"usec\0" as *const u8 as *const libc::c_char,
            b"lib/timeutils.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"int parse_sec(const char *, usec_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7952: {
        if !usec.is_null() {} else {
            __assert_fail(
                b"usec\0" as *const u8 as *const libc::c_char,
                b"lib/timeutils.c\0" as *const u8 as *const libc::c_char,
                73 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"int parse_sec(const char *, usec_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    p = t;
    loop {
        let mut l: libc::c_longlong = 0;
        let mut z: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
        let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut i: libc::c_uint = 0;
        let mut n: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        p = p
            .offset(
                strspn(p, b" \t\n\r\0" as *const u8 as *const libc::c_char) as isize,
            );
        if *p as libc::c_int == 0 as libc::c_int {
            if something == 0 {
                return -(22 as libc::c_int);
            }
            break;
        } else {
            *__errno_location() = 0 as libc::c_int;
            l = strtoll(p, &mut e, 10 as libc::c_int);
            if *__errno_location() > 0 as libc::c_int {
                return -*__errno_location();
            }
            if l < 0 as libc::c_int as libc::c_longlong {
                return -(34 as libc::c_int);
            }
            if *e as libc::c_int == '.' as i32 {
                let mut b: *mut libc::c_char = e.offset(1 as libc::c_int as isize);
                *__errno_location() = 0 as libc::c_int;
                z = strtoll(b, &mut e, 10 as libc::c_int);
                if *__errno_location() > 0 as libc::c_int {
                    return -*__errno_location();
                }
                if z < 0 as libc::c_int as libc::c_longlong {
                    return -(34 as libc::c_int);
                }
                if e == b {
                    return -(22 as libc::c_int);
                }
                n = e.offset_from(b) as libc::c_long as libc::c_uint;
            } else if e == p as *mut libc::c_char {
                return -(22 as libc::c_int)
            }
            e = e
                .offset(
                    strspn(e, b" \t\n\r\0" as *const u8 as *const libc::c_char) as isize,
                );
            i = 0 as libc::c_int as libc::c_uint;
            while (i as libc::c_ulong)
                < (::core::mem::size_of::<[C2RustUnnamed_3; 28]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong,
                    )
                    .wrapping_add(
                        ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong,
                    )
            {
                if !(startswith(e, table[i as usize].suffix)).is_null() {
                    let mut k: usec_t = (z as usec_t)
                        .wrapping_mul(table[i as usize].usec);
                    while n > 0 as libc::c_int as libc::c_uint {
                        k = (k as libc::c_ulong)
                            .wrapping_div(10 as libc::c_int as libc::c_ulong) as usec_t
                            as usec_t;
                        n = n.wrapping_sub(1);
                        n;
                    }
                    r = (r as libc::c_ulong)
                        .wrapping_add(
                            (l as usec_t)
                                .wrapping_mul(table[i as usize].usec)
                                .wrapping_add(k),
                        ) as usec_t as usec_t;
                    p = e.offset(strlen(table[i as usize].suffix) as isize);
                    something = 1 as libc::c_int;
                    break;
                } else {
                    i = i.wrapping_add(1);
                    i;
                }
            }
            if i as libc::c_ulong
                >= (::core::mem::size_of::<[C2RustUnnamed_3; 28]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong,
                    )
                    .wrapping_add(
                        ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong,
                    )
            {
                return -(22 as libc::c_int);
            }
        }
    }
    *usec = r;
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_subseconds(
    mut t: *const libc::c_char,
    mut usec: *mut usec_t,
) -> libc::c_int {
    let mut ret: usec_t = 0 as libc::c_int as usec_t;
    let mut factor: libc::c_int = (1000000 as libc::c_ulonglong)
        .wrapping_div(10 as libc::c_int as libc::c_ulonglong) as libc::c_int;
    if *t as libc::c_int != '.' as i32 && *t as libc::c_int != ',' as i32 {
        return -(1 as libc::c_int);
    }
    loop {
        t = t.offset(1);
        if !(*t != 0) {
            break;
        }
        if *(*__ctype_b_loc()).offset(*t as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
            || factor < 1 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        ret = (ret as libc::c_ulong)
            .wrapping_add(
                (*t as usec_t)
                    .wrapping_sub('0' as i32 as libc::c_ulong)
                    .wrapping_mul(factor as libc::c_ulong),
            ) as usec_t as usec_t;
        factor /= 10 as libc::c_int;
    }
    *usec = ret;
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_timestamp_reference(
    mut x: time_t,
    mut t: *const libc::c_char,
    mut usec: *mut usec_t,
) -> libc::c_int {
    static mut day_nr: [C2RustUnnamed_0; 14] = [
        {
            let mut init = C2RustUnnamed_0 {
                name: b"Sunday\0" as *const u8 as *const libc::c_char,
                nr: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                name: b"Sun\0" as *const u8 as *const libc::c_char,
                nr: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                name: b"Monday\0" as *const u8 as *const libc::c_char,
                nr: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                name: b"Mon\0" as *const u8 as *const libc::c_char,
                nr: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                name: b"Tuesday\0" as *const u8 as *const libc::c_char,
                nr: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                name: b"Tue\0" as *const u8 as *const libc::c_char,
                nr: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                name: b"Wednesday\0" as *const u8 as *const libc::c_char,
                nr: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                name: b"Wed\0" as *const u8 as *const libc::c_char,
                nr: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                name: b"Thursday\0" as *const u8 as *const libc::c_char,
                nr: 4 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                name: b"Thu\0" as *const u8 as *const libc::c_char,
                nr: 4 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                name: b"Friday\0" as *const u8 as *const libc::c_char,
                nr: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                name: b"Fri\0" as *const u8 as *const libc::c_char,
                nr: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                name: b"Saturday\0" as *const u8 as *const libc::c_char,
                nr: 6 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                name: b"Sat\0" as *const u8 as *const libc::c_char,
                nr: 6 as libc::c_int,
            };
            init
        },
    ];
    let mut k: *const libc::c_char = 0 as *const libc::c_char;
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut copy: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut plus: usec_t = 0 as libc::c_int as usec_t;
    let mut minus: usec_t = 0 as libc::c_int as usec_t;
    let mut ret: usec_t = 0 as libc::c_int as usec_t;
    let mut r: libc::c_int = 0;
    let mut weekday: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_uint = 0;
    if !t.is_null() {} else {
        __assert_fail(
            b"t\0" as *const u8 as *const libc::c_char,
            b"lib/timeutils.c\0" as *const u8 as *const libc::c_char,
            219 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"int parse_timestamp_reference(time_t, const char *, usec_t *)\0"))
                .as_ptr(),
        );
    }
    'c_8292: {
        if !t.is_null() {} else {
            __assert_fail(
                b"t\0" as *const u8 as *const libc::c_char,
                b"lib/timeutils.c\0" as *const u8 as *const libc::c_char,
                219 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"int parse_timestamp_reference(time_t, const char *, usec_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !usec.is_null() {} else {
        __assert_fail(
            b"usec\0" as *const u8 as *const libc::c_char,
            b"lib/timeutils.c\0" as *const u8 as *const libc::c_char,
            220 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"int parse_timestamp_reference(time_t, const char *, usec_t *)\0"))
                .as_ptr(),
        );
    }
    'c_8259: {
        if !usec.is_null() {} else {
            __assert_fail(
                b"usec\0" as *const u8 as *const libc::c_char,
                b"lib/timeutils.c\0" as *const u8 as *const libc::c_char,
                220 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"int parse_timestamp_reference(time_t, const char *, usec_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    localtime_r(&mut x, &mut tm);
    tm.tm_isdst = -(1 as libc::c_int);
    if !(strcmp(t, b"now\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int) {
        if strcmp(t, b"today\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            tm.tm_hour = 0 as libc::c_int;
            tm.tm_min = tm.tm_hour;
            tm.tm_sec = tm.tm_min;
        } else if strcmp(t, b"yesterday\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            tm.tm_mday -= 1;
            tm.tm_mday;
            tm.tm_hour = 0 as libc::c_int;
            tm.tm_min = tm.tm_hour;
            tm.tm_sec = tm.tm_min;
        } else if strcmp(t, b"tomorrow\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            tm.tm_mday += 1;
            tm.tm_mday;
            tm.tm_hour = 0 as libc::c_int;
            tm.tm_min = tm.tm_hour;
            tm.tm_sec = tm.tm_min;
        } else if *t.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32 {
            r = parse_sec(t.offset(1 as libc::c_int as isize), &mut plus);
            if r < 0 as libc::c_int {
                return r;
            }
        } else if *t.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
            r = parse_sec(t.offset(1 as libc::c_int as isize), &mut minus);
            if r < 0 as libc::c_int {
                return r;
            }
        } else if *t.offset(0 as libc::c_int as isize) as libc::c_int == '@' as i32 {
            k = strptime(
                t.offset(1 as libc::c_int as isize),
                b"%s\0" as *const u8 as *const libc::c_char,
                &mut tm,
            );
            if !(!k.is_null() && *k as libc::c_int == 0 as libc::c_int) {
                if !(!k.is_null() && parse_subseconds(k, &mut ret) == 0 as libc::c_int) {
                    return -(22 as libc::c_int);
                }
            }
        } else if !(endswith(t, b" ago\0" as *const u8 as *const libc::c_char)).is_null()
        {
            let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
            z = strndup(t, (strlen(t)).wrapping_sub(4 as libc::c_int as libc::c_ulong));
            if z.is_null() {
                return -(12 as libc::c_int);
            }
            r = parse_sec(z, &mut minus);
            free(z as *mut libc::c_void);
            if r < 0 as libc::c_int {
                return r;
            }
        } else {
            i = 0 as libc::c_int as libc::c_uint;
            while (i as libc::c_ulong)
                < (::core::mem::size_of::<[C2RustUnnamed_0; 14]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong,
                    )
                    .wrapping_add(
                        ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong,
                    )
            {
                let mut skip: size_t = 0;
                if !(startswith_no_case(t, day_nr[i as usize].name)).is_null() {
                    skip = strlen(day_nr[i as usize].name);
                    if !(*t.offset(skip as isize) as libc::c_int != ' ' as i32) {
                        weekday = day_nr[i as usize].nr;
                        t = t
                            .offset(
                                skip.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            );
                        break;
                    }
                }
                i = i.wrapping_add(1);
                i;
            }
            copy = tm;
            k = strptime(
                t,
                b"%y-%m-%d %H:%M:%S\0" as *const u8 as *const libc::c_char,
                &mut tm,
            );
            if !(!k.is_null() && *k as libc::c_int == 0 as libc::c_int) {
                if !(!k.is_null() && parse_subseconds(k, &mut ret) == 0 as libc::c_int) {
                    tm = copy;
                    k = strptime(
                        t,
                        b"%Y-%m-%d %H:%M:%S\0" as *const u8 as *const libc::c_char,
                        &mut tm,
                    );
                    if !(!k.is_null() && *k as libc::c_int == 0 as libc::c_int) {
                        if !(!k.is_null()
                            && parse_subseconds(k, &mut ret) == 0 as libc::c_int)
                        {
                            tm = copy;
                            k = strptime(
                                t,
                                b"%Y-%m-%dT%H:%M:%S\0" as *const u8 as *const libc::c_char,
                                &mut tm,
                            );
                            if !(!k.is_null() && *k as libc::c_int == 0 as libc::c_int) {
                                if !(!k.is_null()
                                    && parse_subseconds(k, &mut ret) == 0 as libc::c_int)
                                {
                                    tm = copy;
                                    k = strptime(
                                        t,
                                        b"%y-%m-%d %H:%M\0" as *const u8 as *const libc::c_char,
                                        &mut tm,
                                    );
                                    if !k.is_null() && *k as libc::c_int == 0 as libc::c_int {
                                        tm.tm_sec = 0 as libc::c_int;
                                    } else {
                                        tm = copy;
                                        k = strptime(
                                            t,
                                            b"%Y-%m-%d %H:%M\0" as *const u8 as *const libc::c_char,
                                            &mut tm,
                                        );
                                        if !k.is_null() && *k as libc::c_int == 0 as libc::c_int {
                                            tm.tm_sec = 0 as libc::c_int;
                                        } else {
                                            tm = copy;
                                            k = strptime(
                                                t,
                                                b"%y-%m-%d\0" as *const u8 as *const libc::c_char,
                                                &mut tm,
                                            );
                                            if !k.is_null() && *k as libc::c_int == 0 as libc::c_int {
                                                tm.tm_hour = 0 as libc::c_int;
                                                tm.tm_min = tm.tm_hour;
                                                tm.tm_sec = tm.tm_min;
                                            } else {
                                                tm = copy;
                                                k = strptime(
                                                    t,
                                                    b"%Y-%m-%d\0" as *const u8 as *const libc::c_char,
                                                    &mut tm,
                                                );
                                                if !k.is_null() && *k as libc::c_int == 0 as libc::c_int {
                                                    tm.tm_hour = 0 as libc::c_int;
                                                    tm.tm_min = tm.tm_hour;
                                                    tm.tm_sec = tm.tm_min;
                                                } else {
                                                    tm = copy;
                                                    k = strptime(
                                                        t,
                                                        b"%H:%M:%S\0" as *const u8 as *const libc::c_char,
                                                        &mut tm,
                                                    );
                                                    if !(!k.is_null() && *k as libc::c_int == 0 as libc::c_int)
                                                    {
                                                        if !(!k.is_null()
                                                            && parse_subseconds(k, &mut ret) == 0 as libc::c_int)
                                                        {
                                                            tm = copy;
                                                            k = strptime(
                                                                t,
                                                                b"%H:%M\0" as *const u8 as *const libc::c_char,
                                                                &mut tm,
                                                            );
                                                            if !k.is_null() && *k as libc::c_int == 0 as libc::c_int {
                                                                tm.tm_sec = 0 as libc::c_int;
                                                            } else {
                                                                tm = copy;
                                                                k = strptime(
                                                                    t,
                                                                    b"%Y%m%d%H%M%S\0" as *const u8 as *const libc::c_char,
                                                                    &mut tm,
                                                                );
                                                                if !(!k.is_null() && *k as libc::c_int == 0 as libc::c_int)
                                                                {
                                                                    if !(!k.is_null()
                                                                        && parse_subseconds(k, &mut ret) == 0 as libc::c_int)
                                                                    {
                                                                        return -(22 as libc::c_int);
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    x = mktime(&mut tm);
    if x == -(1 as libc::c_int) as time_t {
        return -(22 as libc::c_int);
    }
    if weekday >= 0 as libc::c_int && tm.tm_wday != weekday {
        return -(22 as libc::c_int);
    }
    ret = (ret as libc::c_ulonglong)
        .wrapping_add(
            (x as usec_t as libc::c_ulonglong).wrapping_mul(1000000 as libc::c_ulonglong),
        ) as usec_t as usec_t;
    ret = (ret as libc::c_ulong).wrapping_add(plus) as usec_t as usec_t;
    if ret > minus {
        ret = (ret as libc::c_ulong).wrapping_sub(minus) as usec_t as usec_t;
    } else {
        ret = 0 as libc::c_int as usec_t;
    }
    *usec = ret;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn parse_timestamp(
    mut t: *const libc::c_char,
    mut usec: *mut usec_t,
) -> libc::c_int {
    return parse_timestamp_reference(time(0 as *mut time_t), t, usec);
}
#[no_mangle]
pub unsafe extern "C" fn get_gmtoff(mut tp: *const tm) -> libc::c_int {
    if (*tp).tm_isdst < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return (*tp).tm_gmtoff as libc::c_int;
}
unsafe extern "C" fn format_iso_time(
    mut tm: *const tm,
    mut nsec: uint32_t,
    mut flags: libc::c_int,
    mut buf: *mut libc::c_char,
    mut bufsz: size_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut usec: uint32_t = (nsec as libc::c_ulonglong)
        .wrapping_div(1000 as libc::c_ulonglong) as uint32_t;
    let mut p: *mut libc::c_char = buf;
    let mut len: libc::c_int = 0;
    if flags & ISO_DATE as libc::c_int != 0 {
        len = snprintf(
            p,
            bufsz,
            b"%4ld-%.2d-%.2d\0" as *const u8 as *const libc::c_char,
            (*tm).tm_year as libc::c_long + 1900 as libc::c_int as libc::c_long,
            (*tm).tm_mon + 1 as libc::c_int,
            (*tm).tm_mday,
        );
        if len < 0 as libc::c_int || len as size_t > bufsz {
            current_block = 18374647011471551182;
        } else {
            bufsz = (bufsz as libc::c_ulong).wrapping_sub(len as libc::c_ulong) as size_t
                as size_t;
            p = p.offset(len as isize);
            current_block = 7095457783677275021;
        }
    } else {
        current_block = 7095457783677275021;
    }
    match current_block {
        7095457783677275021 => {
            if flags & ISO_DATE as libc::c_int != 0
                && flags & ISO_TIME as libc::c_int != 0
            {
                if bufsz < 1 as libc::c_int as libc::c_ulong {
                    current_block = 18374647011471551182;
                } else {
                    let fresh0 = p;
                    p = p.offset(1);
                    *fresh0 = (if flags & ISO_T as libc::c_int != 0 {
                        'T' as i32
                    } else {
                        ' ' as i32
                    }) as libc::c_char;
                    bufsz = bufsz.wrapping_sub(1);
                    bufsz;
                    current_block = 13513818773234778473;
                }
            } else {
                current_block = 13513818773234778473;
            }
            match current_block {
                18374647011471551182 => {}
                _ => {
                    if flags & ISO_TIME as libc::c_int != 0 {
                        len = snprintf(
                            p,
                            bufsz,
                            b"%02d:%02d:%02d\0" as *const u8 as *const libc::c_char,
                            (*tm).tm_hour,
                            (*tm).tm_min,
                            (*tm).tm_sec,
                        );
                        if len < 0 as libc::c_int || len as size_t > bufsz {
                            current_block = 18374647011471551182;
                        } else {
                            bufsz = (bufsz as libc::c_ulong)
                                .wrapping_sub(len as libc::c_ulong) as size_t as size_t;
                            p = p.offset(len as isize);
                            current_block = 11050875288958768710;
                        }
                    } else {
                        current_block = 11050875288958768710;
                    }
                    match current_block {
                        18374647011471551182 => {}
                        _ => {
                            if flags & ISO_DOTNSEC as libc::c_int != 0 {
                                len = snprintf(
                                    p,
                                    bufsz,
                                    b".%09u\0" as *const u8 as *const libc::c_char,
                                    nsec,
                                );
                                if len < 0 as libc::c_int || len as size_t > bufsz {
                                    current_block = 18374647011471551182;
                                } else {
                                    bufsz = (bufsz as libc::c_ulong)
                                        .wrapping_sub(len as libc::c_ulong) as size_t as size_t;
                                    p = p.offset(len as isize);
                                    current_block = 17788412896529399552;
                                }
                            } else if flags & ISO_COMMANSEC as libc::c_int != 0 {
                                len = snprintf(
                                    p,
                                    bufsz,
                                    b",%09u\0" as *const u8 as *const libc::c_char,
                                    nsec,
                                );
                                if len < 0 as libc::c_int || len as size_t > bufsz {
                                    current_block = 18374647011471551182;
                                } else {
                                    bufsz = (bufsz as libc::c_ulong)
                                        .wrapping_sub(len as libc::c_ulong) as size_t as size_t;
                                    p = p.offset(len as isize);
                                    current_block = 17788412896529399552;
                                }
                            } else if flags & ISO_DOTUSEC as libc::c_int != 0 {
                                len = snprintf(
                                    p,
                                    bufsz,
                                    b".%06u\0" as *const u8 as *const libc::c_char,
                                    usec,
                                );
                                if len < 0 as libc::c_int || len as size_t > bufsz {
                                    current_block = 18374647011471551182;
                                } else {
                                    bufsz = (bufsz as libc::c_ulong)
                                        .wrapping_sub(len as libc::c_ulong) as size_t as size_t;
                                    p = p.offset(len as isize);
                                    current_block = 17788412896529399552;
                                }
                            } else if flags & ISO_COMMAUSEC as libc::c_int != 0 {
                                len = snprintf(
                                    p,
                                    bufsz,
                                    b",%06u\0" as *const u8 as *const libc::c_char,
                                    usec,
                                );
                                if len < 0 as libc::c_int || len as size_t > bufsz {
                                    current_block = 18374647011471551182;
                                } else {
                                    bufsz = (bufsz as libc::c_ulong)
                                        .wrapping_sub(len as libc::c_ulong) as size_t as size_t;
                                    p = p.offset(len as isize);
                                    current_block = 17788412896529399552;
                                }
                            } else {
                                current_block = 17788412896529399552;
                            }
                            match current_block {
                                18374647011471551182 => {}
                                _ => {
                                    if flags & ISO_TIMEZONE as libc::c_int != 0 {
                                        let mut tmin: libc::c_int = get_gmtoff(tm)
                                            / 60 as libc::c_int;
                                        let mut zhour: libc::c_int = tmin / 60 as libc::c_int;
                                        let mut zmin: libc::c_int = abs(tmin % 60 as libc::c_int);
                                        len = snprintf(
                                            p,
                                            bufsz,
                                            b"%+03d:%02d\0" as *const u8 as *const libc::c_char,
                                            zhour,
                                            zmin,
                                        );
                                        if len < 0 as libc::c_int || len as size_t > bufsz {
                                            current_block = 18374647011471551182;
                                        } else {
                                            current_block = 11385396242402735691;
                                        }
                                    } else {
                                        current_block = 11385396242402735691;
                                    }
                                    match current_block {
                                        18374647011471551182 => {}
                                        _ => return 0 as libc::c_int,
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    warnx(
        dcgettext(
            0 as *const libc::c_char,
            b"format_iso_time: buffer overflow.\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn strtimespec_iso(
    mut ts: *const timespec,
    mut flags: libc::c_int,
    mut buf: *mut libc::c_char,
    mut bufsz: size_t,
) -> libc::c_int {
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut rc: *mut tm = 0 as *mut tm;
    if flags & ISO_GMTIME as libc::c_int != 0 {
        rc = gmtime_r(&(*ts).tv_sec, &mut tm);
    } else {
        rc = localtime_r(&(*ts).tv_sec, &mut tm);
    }
    if !rc.is_null() {
        return format_iso_time(&mut tm, (*ts).tv_nsec as uint32_t, flags, buf, bufsz);
    }
    warnx(
        dcgettext(
            0 as *const libc::c_char,
            b"time %ld is out of range.\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        (*ts).tv_sec,
    );
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn strtimeval_iso(
    mut tv: *const timeval,
    mut flags: libc::c_int,
    mut buf: *mut libc::c_char,
    mut bufsz: size_t,
) -> libc::c_int {
    let mut ts: timespec = {
        let mut init = timespec {
            tv_sec: (*tv).tv_sec,
            tv_nsec: ((*tv).tv_usec as libc::c_ulonglong)
                .wrapping_mul(1000 as libc::c_ulonglong) as __syscall_slong_t,
        };
        init
    };
    return strtimespec_iso(&mut ts, flags, buf, bufsz);
}
#[no_mangle]
pub unsafe extern "C" fn strtm_iso(
    mut tm: *const tm,
    mut flags: libc::c_int,
    mut buf: *mut libc::c_char,
    mut bufsz: size_t,
) -> libc::c_int {
    return format_iso_time(tm, 0 as libc::c_int as uint32_t, flags, buf, bufsz);
}
#[no_mangle]
pub unsafe extern "C" fn strtime_iso(
    mut t: *const time_t,
    mut flags: libc::c_int,
    mut buf: *mut libc::c_char,
    mut bufsz: size_t,
) -> libc::c_int {
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut rc: *mut tm = 0 as *mut tm;
    if flags & ISO_GMTIME as libc::c_int != 0 {
        rc = gmtime_r(t, &mut tm);
    } else {
        rc = localtime_r(t, &mut tm);
    }
    if !rc.is_null() {
        return format_iso_time(&mut tm, 0 as libc::c_int as uint32_t, flags, buf, bufsz);
    }
    warnx(
        dcgettext(
            0 as *const libc::c_char,
            b"time %ld is out of range.\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        *t,
    );
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn time_is_thisyear(tm: *const tm, tmnow: *const tm) -> libc::c_int {
    return ((*tm).tm_year == (*tmnow).tm_year) as libc::c_int;
}
#[inline]
unsafe extern "C" fn time_is_today(tm: *const tm, tmnow: *const tm) -> libc::c_int {
    return ((*tm).tm_yday == (*tmnow).tm_yday && time_is_thisyear(tm, tmnow) != 0)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn strtime_short(
    mut t: *const time_t,
    mut now: *mut timeval,
    mut flags: libc::c_int,
    mut buf: *mut libc::c_char,
    mut bufsz: size_t,
) -> libc::c_int {
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut tmnow: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut rc: libc::c_int = 0 as libc::c_int;
    if (*now).tv_sec == 0 as libc::c_int as libc::c_long {
        gettimeofday(now, 0 as *mut libc::c_void);
    }
    localtime_r(t, &mut tm);
    localtime_r(&mut (*now).tv_sec, &mut tmnow);
    if time_is_today(&mut tm, &mut tmnow) != 0 {
        rc = snprintf(
            buf,
            bufsz,
            b"%02d:%02d\0" as *const u8 as *const libc::c_char,
            tm.tm_hour,
            tm.tm_min,
        );
        if rc < 0 as libc::c_int || rc as size_t > bufsz {
            return -(1 as libc::c_int);
        }
        rc = 1 as libc::c_int;
    } else if time_is_thisyear(&mut tm, &mut tmnow) != 0 {
        if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            rc = strftime(
                buf,
                bufsz,
                b"%b%d/%H:%M\0" as *const u8 as *const libc::c_char,
                &mut tm,
            ) as libc::c_int;
        } else {
            rc = strftime(
                buf,
                bufsz,
                b"%b%d\0" as *const u8 as *const libc::c_char,
                &mut tm,
            ) as libc::c_int;
        }
    } else {
        rc = strftime(
            buf,
            bufsz,
            b"%Y-%b%d\0" as *const u8 as *const libc::c_char,
            &mut tm,
        ) as libc::c_int;
    }
    return if rc <= 0 as libc::c_int { -(1 as libc::c_int) } else { 0 as libc::c_int };
}
static mut table_0: [C2RustUnnamed_6; 5] = [C2RustUnnamed_6 {
    suffix: 0 as *const libc::c_char,
    width: 0,
    secs: 0,
}; 5];
#[no_mangle]
pub unsafe extern "C" fn strtimespec_relative(
    mut ts: *const timespec,
    mut buf: *mut libc::c_char,
    mut bufsz: size_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut secs: time_t = (*ts).tv_sec;
    let mut i: size_t = 0;
    let mut parts: size_t = 0 as libc::c_int as size_t;
    let mut rc: libc::c_int = 0;
    if bufsz != 0 {
        *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
    i = 0 as libc::c_int as size_t;
    loop {
        if !(i
            < (::core::mem::size_of::<[C2RustUnnamed_6; 5]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong)
                .wrapping_add(
                    ::core::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong,
                ))
        {
            current_block = 13536709405535804910;
            break;
        }
        if secs >= table_0[i as usize].secs {
            rc = snprintf(
                buf,
                bufsz,
                b"%*ld%s%s\0" as *const u8 as *const libc::c_char,
                if parts != 0 { table_0[i as usize].width } else { 0 as libc::c_int },
                secs / table_0[i as usize].secs,
                table_0[i as usize].suffix,
                if secs % table_0[i as usize].secs != 0 {
                    b" \0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
            if rc < 0 as libc::c_int || rc as size_t > bufsz {
                current_block = 16732268451210056868;
                break;
            }
            parts = parts.wrapping_add(1);
            parts;
            buf = buf.offset(rc as isize);
            bufsz = (bufsz as libc::c_ulong).wrapping_sub(rc as libc::c_ulong) as size_t
                as size_t;
            secs %= table_0[i as usize].secs;
        }
        i = i.wrapping_add(1);
        i;
    }
    match current_block {
        13536709405535804910 => {
            if (*ts).tv_nsec != 0 {
                if ((*ts).tv_nsec as libc::c_ulonglong)
                    .wrapping_rem(1000000 as libc::c_ulonglong) != 0
                {
                    rc = snprintf(
                        buf,
                        bufsz,
                        b"%*luns\0" as *const u8 as *const libc::c_char,
                        if parts != 0 { 10 as libc::c_int } else { 0 as libc::c_int },
                        (*ts).tv_nsec,
                    );
                    if rc < 0 as libc::c_int || rc as size_t > bufsz {
                        current_block = 16732268451210056868;
                    } else {
                        current_block = 8457315219000651999;
                    }
                } else {
                    rc = snprintf(
                        buf,
                        bufsz,
                        b"%*llums\0" as *const u8 as *const libc::c_char,
                        if parts != 0 { 4 as libc::c_int } else { 0 as libc::c_int },
                        ((*ts).tv_nsec as libc::c_ulonglong)
                            .wrapping_div(1000000 as libc::c_ulonglong),
                    );
                    if rc < 0 as libc::c_int || rc as size_t > bufsz {
                        current_block = 16732268451210056868;
                    } else {
                        current_block = 8457315219000651999;
                    }
                }
            } else {
                current_block = 8457315219000651999;
            }
            match current_block {
                16732268451210056868 => {}
                _ => return 0 as libc::c_int,
            }
        }
        _ => {}
    }
    warnx(
        dcgettext(
            0 as *const libc::c_char,
            b"format_reltime: buffer overflow.\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn run_static_initializers() {
    table = [
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"seconds\0" as *const u8 as *const libc::c_char,
                usec: 1000000 as libc::c_ulonglong as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"second\0" as *const u8 as *const libc::c_char,
                usec: 1000000 as libc::c_ulonglong as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"sec\0" as *const u8 as *const libc::c_char,
                usec: 1000000 as libc::c_ulonglong as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"s\0" as *const u8 as *const libc::c_char,
                usec: 1000000 as libc::c_ulonglong as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"minutes\0" as *const u8 as *const libc::c_char,
                usec: (60 as libc::c_ulonglong)
                    .wrapping_mul(1000000 as libc::c_ulonglong) as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"minute\0" as *const u8 as *const libc::c_char,
                usec: (60 as libc::c_ulonglong)
                    .wrapping_mul(1000000 as libc::c_ulonglong) as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"min\0" as *const u8 as *const libc::c_char,
                usec: (60 as libc::c_ulonglong)
                    .wrapping_mul(1000000 as libc::c_ulonglong) as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"months\0" as *const u8 as *const libc::c_char,
                usec: (2629800 as libc::c_ulonglong)
                    .wrapping_mul(1000000 as libc::c_ulonglong) as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"month\0" as *const u8 as *const libc::c_char,
                usec: (2629800 as libc::c_ulonglong)
                    .wrapping_mul(1000000 as libc::c_ulonglong) as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"msec\0" as *const u8 as *const libc::c_char,
                usec: 1000 as libc::c_ulonglong as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"ms\0" as *const u8 as *const libc::c_char,
                usec: 1000 as libc::c_ulonglong as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"m\0" as *const u8 as *const libc::c_char,
                usec: (60 as libc::c_ulonglong)
                    .wrapping_mul(1000000 as libc::c_ulonglong) as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"hours\0" as *const u8 as *const libc::c_char,
                usec: (60 as libc::c_ulonglong)
                    .wrapping_mul(
                        (60 as libc::c_ulonglong)
                            .wrapping_mul(1000000 as libc::c_ulonglong),
                    ) as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"hour\0" as *const u8 as *const libc::c_char,
                usec: (60 as libc::c_ulonglong)
                    .wrapping_mul(
                        (60 as libc::c_ulonglong)
                            .wrapping_mul(1000000 as libc::c_ulonglong),
                    ) as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"hr\0" as *const u8 as *const libc::c_char,
                usec: (60 as libc::c_ulonglong)
                    .wrapping_mul(
                        (60 as libc::c_ulonglong)
                            .wrapping_mul(1000000 as libc::c_ulonglong),
                    ) as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"h\0" as *const u8 as *const libc::c_char,
                usec: (60 as libc::c_ulonglong)
                    .wrapping_mul(
                        (60 as libc::c_ulonglong)
                            .wrapping_mul(1000000 as libc::c_ulonglong),
                    ) as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"days\0" as *const u8 as *const libc::c_char,
                usec: (24 as libc::c_ulonglong)
                    .wrapping_mul(
                        (60 as libc::c_ulonglong)
                            .wrapping_mul(
                                (60 as libc::c_ulonglong)
                                    .wrapping_mul(1000000 as libc::c_ulonglong),
                            ),
                    ) as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"day\0" as *const u8 as *const libc::c_char,
                usec: (24 as libc::c_ulonglong)
                    .wrapping_mul(
                        (60 as libc::c_ulonglong)
                            .wrapping_mul(
                                (60 as libc::c_ulonglong)
                                    .wrapping_mul(1000000 as libc::c_ulonglong),
                            ),
                    ) as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"d\0" as *const u8 as *const libc::c_char,
                usec: (24 as libc::c_ulonglong)
                    .wrapping_mul(
                        (60 as libc::c_ulonglong)
                            .wrapping_mul(
                                (60 as libc::c_ulonglong)
                                    .wrapping_mul(1000000 as libc::c_ulonglong),
                            ),
                    ) as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"weeks\0" as *const u8 as *const libc::c_char,
                usec: (7 as libc::c_ulonglong)
                    .wrapping_mul(
                        (24 as libc::c_ulonglong)
                            .wrapping_mul(
                                (60 as libc::c_ulonglong)
                                    .wrapping_mul(
                                        (60 as libc::c_ulonglong)
                                            .wrapping_mul(1000000 as libc::c_ulonglong),
                                    ),
                            ),
                    ) as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"week\0" as *const u8 as *const libc::c_char,
                usec: (7 as libc::c_ulonglong)
                    .wrapping_mul(
                        (24 as libc::c_ulonglong)
                            .wrapping_mul(
                                (60 as libc::c_ulonglong)
                                    .wrapping_mul(
                                        (60 as libc::c_ulonglong)
                                            .wrapping_mul(1000000 as libc::c_ulonglong),
                                    ),
                            ),
                    ) as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"w\0" as *const u8 as *const libc::c_char,
                usec: (7 as libc::c_ulonglong)
                    .wrapping_mul(
                        (24 as libc::c_ulonglong)
                            .wrapping_mul(
                                (60 as libc::c_ulonglong)
                                    .wrapping_mul(
                                        (60 as libc::c_ulonglong)
                                            .wrapping_mul(1000000 as libc::c_ulonglong),
                                    ),
                            ),
                    ) as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"years\0" as *const u8 as *const libc::c_char,
                usec: (31557600 as libc::c_ulonglong)
                    .wrapping_mul(1000000 as libc::c_ulonglong) as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"year\0" as *const u8 as *const libc::c_char,
                usec: (31557600 as libc::c_ulonglong)
                    .wrapping_mul(1000000 as libc::c_ulonglong) as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"y\0" as *const u8 as *const libc::c_char,
                usec: (31557600 as libc::c_ulonglong)
                    .wrapping_mul(1000000 as libc::c_ulonglong) as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"usec\0" as *const u8 as *const libc::c_char,
                usec: 1 as libc::c_ulonglong as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"us\0" as *const u8 as *const libc::c_char,
                usec: 1 as libc::c_ulonglong as usec_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                suffix: b"\0" as *const u8 as *const libc::c_char,
                usec: 1000000 as libc::c_ulonglong as usec_t,
            };
            init
        },
    ];
    table_0 = [
        {
            let mut init = C2RustUnnamed_6 {
                suffix: b"y\0" as *const u8 as *const libc::c_char,
                width: 4 as libc::c_int,
                secs: (31557600 as libc::c_ulonglong)
                    .wrapping_mul(1000000000 as libc::c_ulonglong)
                    .wrapping_div(1000000000 as libc::c_ulonglong) as int64_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                suffix: b"d\0" as *const u8 as *const libc::c_char,
                width: 3 as libc::c_int,
                secs: (24 as libc::c_ulonglong)
                    .wrapping_mul(
                        (60 as libc::c_ulonglong)
                            .wrapping_mul(
                                (60 as libc::c_ulonglong)
                                    .wrapping_mul(1000000000 as libc::c_ulonglong),
                            ),
                    )
                    .wrapping_div(1000000000 as libc::c_ulonglong) as int64_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                suffix: b"h\0" as *const u8 as *const libc::c_char,
                width: 2 as libc::c_int,
                secs: (60 as libc::c_ulonglong)
                    .wrapping_mul(
                        (60 as libc::c_ulonglong)
                            .wrapping_mul(1000000000 as libc::c_ulonglong),
                    )
                    .wrapping_div(1000000000 as libc::c_ulonglong) as int64_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                suffix: b"m\0" as *const u8 as *const libc::c_char,
                width: 2 as libc::c_int,
                secs: (60 as libc::c_ulonglong)
                    .wrapping_mul(1000000000 as libc::c_ulonglong)
                    .wrapping_div(1000000000 as libc::c_ulonglong) as int64_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                suffix: b"s\0" as *const u8 as *const libc::c_char,
                width: 2 as libc::c_int,
                secs: (1000000000 as libc::c_ulonglong)
                    .wrapping_div(1000000000 as libc::c_ulonglong) as int64_t,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
